use crate::html_comment_ext::{html_comment_ranges, normalize_quote_prefixes};
use crate::list_ext::AnyListItem;
use crate::{
    AnyMdBlock, AnyMdCodeBlock, AnyMdContainerBlock, AnyMdInline, AnyMdLeafBlock,
    MarkdownSyntaxKind, MarkdownSyntaxNode, MdFencedCodeBlock, MdHtmlBlock, MdIndentTokenList,
    MdParagraph, MdQuote,
};
use biome_rowan::{AstNode, AstNodeList};

const TAB_STOP_SPACES: usize = 4;
const MAX_BLOCK_PREFIX_INDENT: usize = 3;

impl AnyMdBlock {
    pub const fn is_fenced_block(&self) -> bool {
        matches!(
            self,
            Self::AnyMdLeafBlock(AnyMdLeafBlock::AnyMdCodeBlock(
                AnyMdCodeBlock::MdFencedCodeBlock(_)
            ))
        )
    }

    pub const fn is_indent_block(&self) -> bool {
        matches!(
            self,
            Self::AnyMdLeafBlock(AnyMdLeafBlock::AnyMdCodeBlock(
                AnyMdCodeBlock::MdIndentCodeBlock(_)
            ))
        )
    }

    pub const fn is_list(&self) -> bool {
        matches!(
            self,
            Self::AnyMdContainerBlock(
                AnyMdContainerBlock::MdBulletListItem(_)
                    | AnyMdContainerBlock::MdOrderedListItem(_)
            )
        )
    }

    pub const fn is_continuation_indent(&self) -> bool {
        matches!(
            self,
            Self::AnyMdLeafBlock(AnyMdLeafBlock::MdContinuationIndent(_))
        )
    }

    pub const fn is_paragraph(&self) -> bool {
        matches!(self, Self::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(_)))
    }

    /// Whether the block is a header or setext header.
    pub const fn is_any_header(&self) -> bool {
        matches!(
            self,
            Self::AnyMdLeafBlock(AnyMdLeafBlock::MdHeader(_) | AnyMdLeafBlock::MdSetextHeader(_))
        )
    }

    pub const fn is_link_reference_definition(&self) -> bool {
        matches!(
            self,
            Self::AnyMdLeafBlock(AnyMdLeafBlock::MdLinkReferenceDefinition(_))
        )
    }

    pub const fn is_thematic_break(&self) -> bool {
        matches!(
            self,
            Self::AnyMdLeafBlock(AnyMdLeafBlock::MdThematicBreakBlock(_))
        )
    }

    pub const fn is_newline(&self) -> bool {
        matches!(self, Self::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_)))
    }

    pub const fn is_html_block(&self) -> bool {
        matches!(self, Self::AnyMdLeafBlock(AnyMdLeafBlock::MdHtmlBlock(_)))
    }

    pub fn is_html_comment(&self) -> bool {
        match self {
            Self::AnyMdLeafBlock(AnyMdLeafBlock::MdHtmlBlock(block)) => block.is_html_comment(),
            Self::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph)) => {
                paragraph.is_html_comment()
            }
            _ => false,
        }
    }

    pub fn as_any_list_item(&self) -> Option<AnyListItem> {
        match self {
            Self::AnyMdContainerBlock(AnyMdContainerBlock::MdBulletListItem(item)) => {
                Some(AnyListItem::MdBulletListItem(item.clone()))
            }
            Self::AnyMdContainerBlock(AnyMdContainerBlock::MdOrderedListItem(item)) => {
                Some(AnyListItem::MdOrderedListItem(item.clone()))
            }
            _ => None,
        }
    }
}

impl MdFencedCodeBlock {
    /// Returns the normalized info string following the opening fence.
    ///
    /// Boundary whitespace is omitted by the CST. This method resolves
    /// backslash escapes and character references using CommonMark semantics.
    pub fn info_string(&self) -> String {
        crate::text_ext::unescape_markdown(&self.code_list().syntax().text_trimmed().to_string())
    }

    /// Returns the opening fence's structural indentation in visual columns.
    ///
    /// CommonMark permits at most three columns before a fence. Any surrounding
    /// list or quote indentation belongs to the container and is not included.
    pub fn opening_indent_width(&self) -> usize {
        self.indent().width().min(MAX_BLOCK_PREFIX_INDENT)
    }

    /// Returns fenced content before container indentation is removed.
    ///
    /// The structural line ending after the opening fence is omitted, and all
    /// remaining line endings are normalized to `\n`. Content nested in lists
    /// or block quotes still contains those structural prefixes; consumers that
    /// need executable code must remove them using the active container context.
    pub fn content_text(&self) -> String {
        let content = self.content().literal_text();
        let content = content
            .strip_prefix("\r\n")
            .or_else(|| content.strip_prefix('\r'))
            .or_else(|| content.strip_prefix('\n'))
            .unwrap_or(&content);
        content.replace("\r\n", "\n").replace('\r', "\n")
    }

    /// Returns the number of block quotes that contain this fenced block.
    ///
    /// Each level contributes one structural `>` prefix to every content line.
    pub fn quote_depth(&self) -> usize {
        self.syntax().ancestors().filter_map(MdQuote::cast).count()
    }

    /// Returns the opening fence's visual column after quote prefixes are
    /// removed.
    ///
    /// Tabs advance to four-column tab stops. The result accounts for tabs
    /// assigned wholly to a neighboring CST node even when a tab crosses a
    /// structural indentation boundary.
    pub fn line_start_column(&self) -> usize {
        line_start_column(self.syntax())
    }
}

impl MdIndentTokenList {
    /// Returns the visual width of this indentation using four-column tab stops.
    pub fn width(&self) -> usize {
        let mut width = 0usize;
        for character in self.syntax().to_string().chars() {
            match character {
                ' ' => width += 1,
                '\t' => width += TAB_STOP_SPACES - (width % TAB_STOP_SPACES),
                _ => {}
            }
        }
        width
    }
}

fn line_start_column(node: &MarkdownSyntaxNode) -> usize {
    let Some(first) = node.first_token() else {
        return 0;
    };
    let mut anchor = first.prev_token();
    while let Some(token) = anchor.clone() {
        if token.kind() == MarkdownSyntaxKind::NEWLINE
            || token.text().contains('\n')
            || token.text().contains('\r')
        {
            break;
        }
        anchor = token.prev_token();
    }

    let mut column = 0usize;
    let mut cursor = match anchor {
        Some(token) => token.next_token(),
        None => {
            let mut token = Some(first.clone());
            loop {
                let Some(current) = token.clone() else {
                    break None;
                };
                let Some(previous) = current.prev_token() else {
                    break Some(current);
                };
                token = Some(previous);
            }
        }
    };

    while let Some(token) = cursor {
        if token == first {
            break;
        }
        let in_quote_prefix = token.parent().is_some_and(|parent| {
            parent
                .ancestors()
                .any(|node| node.kind() == MarkdownSyntaxKind::MD_QUOTE_PREFIX)
        });
        if !in_quote_prefix
            && !matches!(
                token.kind(),
                MarkdownSyntaxKind::R_ANGLE | MarkdownSyntaxKind::MD_QUOTE_POST_MARKER_SPACE
            )
        {
            for character in token.text().chars() {
                column += if character == '\t' {
                    TAB_STOP_SPACES - (column % TAB_STOP_SPACES)
                } else {
                    1
                };
            }
        }
        cursor = token.next_token();
    }

    column
}

impl MdParagraph {
    pub fn is_html_comment(&self) -> bool {
        let quote_depth = self
            .syntax()
            .ancestors()
            .filter(|node| MdQuote::can_cast(node.kind()))
            .count();
        let mut has_comment = false;
        for item in self.list() {
            match item {
                AnyMdInline::MdInlineHtml(html) => {
                    let Ok(token) = html.value_token() else {
                        return false;
                    };
                    if token.text_trimmed().trim().is_empty() && token.has_leading_comments() {
                        has_comment = true;
                        continue;
                    }
                    let text = normalize_quote_prefixes(token.text_trimmed(), quote_depth);
                    if html_comment_ranges(&text).is_none() {
                        return false;
                    }
                    has_comment = true;
                }
                AnyMdInline::MdTextual(textual) => {
                    if !textual
                        .value_token()
                        .is_ok_and(|token| token.text_trimmed().chars().all(char::is_whitespace))
                    {
                        return false;
                    }
                }
                AnyMdInline::MdQuotePrefix(_) => {}
                _ => return false,
            }
        }
        has_comment
    }

    pub fn ends_with_newline(&self) -> bool {
        self.list().ends_with_newline()
    }

    pub fn ends_with_double_newline(&self) -> bool {
        let mut iter = self.list().iter();
        let last = iter.next_back();
        let penultimate = iter.next_back();

        match (last, penultimate) {
            (Some(last), Some(penultimate)) => {
                last.as_md_textual()
                    .is_some_and(|textual| textual.is_newline().unwrap_or_default())
                    && penultimate
                        .as_md_textual()
                        .is_some_and(|textual| textual.is_newline().unwrap_or_default())
            }
            _ => false,
        }
    }
}

impl MdHtmlBlock {
    pub fn is_html_comment(&self) -> bool {
        let Ok(content) = self.content() else {
            return false;
        };
        let Ok(token) = content.value_token() else {
            return false;
        };
        if token.text_trimmed().trim().is_empty() && token.has_leading_comments() {
            return true;
        }

        let quote_depth = self
            .syntax()
            .ancestors()
            .filter(|node| MdQuote::can_cast(node.kind()))
            .count();
        html_comment_ranges(&normalize_quote_prefixes(token.text_trimmed(), quote_depth)).is_some()
    }
}
