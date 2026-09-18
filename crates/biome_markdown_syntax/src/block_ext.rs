use crate::html_comment_ext::{html_comment_ranges, normalize_quote_prefixes};
use crate::list_ext::AnyListItem;
use crate::{
    AnyMdBlock, AnyMdCodeBlock, AnyMdContainerBlock, AnyMdInline, AnyMdLeafBlock, MdHtmlBlock,
    MdParagraph, MdQuote,
};
use biome_rowan::{AstNode, AstNodeList};

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
        self.list().last().is_some_and(|item| {
            item.as_md_textual()
                .is_some_and(|textual| textual.is_newline().unwrap_or_default())
        })
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

        let quote_depth = self
            .syntax()
            .ancestors()
            .filter(|node| MdQuote::can_cast(node.kind()))
            .count();
        html_comment_ranges(&normalize_quote_prefixes(token.text_trimmed(), quote_depth)).is_some()
    }
}
