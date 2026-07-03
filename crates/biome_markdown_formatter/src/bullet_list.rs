use crate::context::MarkdownFormatContext;
use crate::markdown::auxiliary::continuation_indent::FormatMdContinuationIndentOptions;
use crate::markdown::auxiliary::fenced_code_block::FormatMdFencedCodeBlockOptions;
use crate::markdown::auxiliary::indent_code_block::FormatMdIndentCodeBlockOptions;
use crate::markdown::auxiliary::list_marker_prefix::FormatMdListMarkerPrefixOptions;
use crate::markdown::auxiliary::newline::FormatMdNewlineOptions;
use crate::markdown::auxiliary::paragraph::FormatMdParagraphOptions;
use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::shared::{TextContext, TextPrintMode};
use crate::{AsFormat, MarkdownFormatter};
use biome_formatter::prelude::*;
use biome_formatter::{Format, FormatResult, write};
use biome_markdown_syntax::list_ext::{AnyListItem, ListMarker};
use biome_markdown_syntax::thematic_break_ext::MdThematicBreakMarker;
use biome_markdown_syntax::{
    AnyMdBlock, AnyMdCodeBlock, AnyMdLeafBlock, MarkdownLanguage, MdBlockList, MdBullet,
    MdBulletFields, MdBulletList, MdBulletListItem, MdContinuationIndent, MdIndentCodeBlock,
    MdOrderedListItem, MdQuotePrefix,
};
use biome_rowan::{AstNode, AstNodeList, AstNodeListIterator, Direction};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::iter::FusedIterator;

/// Thin wrapper around [AnyListItem]
pub struct FmtAnyList {
    node: AnyListItem,
}

impl FmtAnyList {
    pub(crate) fn new(node: AnyListItem) -> Self {
        Self { node }
    }
}

impl Format<MarkdownFormatContext> for FmtAnyList {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        f.context().comments().is_suppressed(self.node.syntax());
        let list = self.node.list();
        BulletListPrinter::new(&list).fmt(f)
    }
}

pub(crate) struct BulletListPrinter {
    bullets: Vec<ListBullet>,
}

impl BulletListPrinter {
    pub(crate) fn new(node: &MdBulletList) -> Self {
        // The marker decision must be the same for every bullet of the list:
        // normalizing only some markers changes the marker mid-list, which
        // splits the list in two per CommonMark.
        let prefer_star_marker = node
            .iter()
            .any(|bullet| first_block_is_dash_thematic_break(&bullet.content()));
        Self {
            bullets: node
                .iter()
                .map(|item| ListBullet {
                    node: item,
                    prefer_star_marker,
                })
                .collect(),
        }
    }
}

impl Format<MarkdownFormatContext> for BulletListPrinter {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        let mut joiner = f.join();

        for item in self.bullets.iter() {
            joiner.entry(item);
        }
        joiner.finish()
    }
}

pub(crate) struct ListBullet {
    node: MdBullet,
    /// When true, unordered list markers are normalized to `*` instead of `-`.
    /// Computed once per list by [BulletListPrinter].
    prefer_star_marker: bool,
}

impl ListBullet {
    /// This functions checks if the pre-marker of a bullet item needs to be kept.
    ///
    /// Following, the cases when it needs to be kept
    ///
    /// ## Cases
    ///
    /// When the bullet item is followed by a newline and a inline code block
    ///
    /// ```md
    ///  -    one
    ///
    ///      two
    /// ```
    ///
    /// Source: <https://spec.commonmark.org/dingus/?text=%20-%20%20%20%20one%0A%0A%20%20%20%20%20two%0A>
    fn keep_pre_marker(&self) -> bool {
        self.node
            .syntax()
            .ancestors()
            .find(|a| MdBulletListItem::can_cast(a.kind()) || MdOrderedListItem::can_cast(a.kind()))
            .is_some_and(|list_item| {
                list_item
                    .siblings(Direction::Next)
                    // We skip 1 because usually the next sibling is a MdNewline
                    .skip(1)
                    .any(|s| MdIndentCodeBlock::can_cast(s.kind()))
            })
    }
}

impl Format<MarkdownFormatContext> for ListBullet {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.context().comments().is_suppressed(self.node.syntax());

        let MdBulletFields { content, prefix } = self.node.as_fields();

        let prefix = prefix?;
        let marker = prefix.marker()?;
        let list_marker = prefix.list_marker()?;

        let is_ordered_marker = list_marker.is_ordered();
        let is_minus_marker = list_marker.is_minus();
        let is_star_marker = matches!(&list_marker, ListMarker::Star);

        // `* - - -` is a bullet containing a `-` thematic break. Normalizing `*`
        // to `-` produces `- ---`, which CommonMark 4.1 parses as a thematic
        // break, not a list item. Same for `+ - - -`, so use `*` for the whole
        // unordered list when one item starts with a dash thematic break.
        let target_marker = if is_ordered_marker {
            None
        } else if self.prefer_star_marker {
            if is_star_marker { None } else { Some("*") }
        } else if is_minus_marker {
            None
        } else {
            Some("-")
        };

        let keep_pre_marker = self.keep_pre_marker();
        let pre_marker_width = if keep_pre_marker {
            prefix.pre_marker_indent().len() as u8
        } else {
            0
        };
        let min_post_marker_len =
            if is_ordered_marker && has_indented_code_block_after_content(&content) {
                // CommonMark indented code blocks use four spaces:
                // https://spec.commonmark.org/0.31.2/#indented-code-blocks
                4usize.saturating_sub(marker.text_trimmed().len())
            } else {
                0
            };

        write!(
            f,
            [prefix
                .format()
                .with_options(FormatMdListMarkerPrefixOptions {
                    target_marker,
                    keep_pre_marker,
                    min_post_marker_len,
                })]
        )?;

        // The alignment is the sum of the pre-marker width, the marker width and the post-marker width.
        let post_marker_len = prefix
            .post_marker_len()
            .unwrap_or(2)
            .max(min_post_marker_len) as u8;
        let alignment = pre_marker_width + (marker.text_trimmed().len() as u8) + post_marker_len;

        let content = ListBlockList {
            content: content.clone(),
        };
        write!(f, [align(alignment, &content),])
    }
}

/// Returns true if the first block in `content` is a thematic break using `-`.
fn first_block_is_dash_thematic_break(content: &MdBlockList) -> bool {
    let Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdThematicBreakBlock(block))) =
        content.iter().next()
    else {
        return false;
    };
    block
        .parts()
        .into_iter()
        .find_map(|p| p.as_md_thematic_break_char().cloned())
        .is_some_and(|c| {
            c.marker()
                .is_ok_and(|marker| marker == MdThematicBreakMarker::Hyphen)
        })
}

fn has_indented_code_block_after_content(content: &MdBlockList) -> bool {
    let mut seen_content = false;

    for content in content.iter() {
        if content.is_newline() {
            continue;
        }

        if content.is_indent_block() {
            return seen_content;
        }

        seen_content = true;
    }

    false
}

/// It's responsible for formatting a [MdBlockList] that is inside a bullet list
struct ListBlockList {
    content: MdBlockList,
}

impl ListBlockList {
    fn emit_pending_breaks(
        pending_breaks: u8,
        content: &AnyMdBlock,
        f: &mut Formatter<MarkdownFormatContext>,
    ) -> FormatResult<()> {
        let breaks = if content.is_thematic_break() && pending_breaks > 0 {
            2
        } else if content.is_list() {
            pending_breaks.min(1)
        } else if should_separate_fenced_code_block(content) && pending_breaks > 0 {
            2
        } else {
            pending_breaks
        };
        match breaks {
            0 => {}
            1 => write!(f, [hard_line_break()])?,
            // NOTE: Prettier emits a double hardline, but our Printer is different, it deduplicates continues hardlines.
            // Our IR has an empty_line for that.
            _ => write!(f, [empty_line()])?,
        }
        Ok(())
    }

    fn fmt_list_content(content: &AnyMdBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph)) = content {
            let line_break = format_with(|f| {
                if paragraph.ends_with_double_newline() {
                    write!(f, [empty_line()])
                } else {
                    write!(f, [hard_line_break()])
                }
            });
            write!(
                f,
                [
                    paragraph.format().with_options(FormatMdParagraphOptions {
                        trim_mode: TextPrintMode::fill(),
                        text_context: TextContext::List,
                    }),
                    line_break
                ]
            )
        } else if let Some(list_item) = content.as_any_list_item() {
            FmtAnyList::new(list_item.clone()).fmt(f)
        } else if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::AnyMdCodeBlock(
            AnyMdCodeBlock::MdIndentCodeBlock(code_block),
        )) = content
        {
            write!(
                f,
                [code_block
                    .format()
                    .with_options(FormatMdIndentCodeBlockOptions { in_list: true })]
            )
        } else if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::AnyMdCodeBlock(
            AnyMdCodeBlock::MdFencedCodeBlock(code_block),
        )) = content
        {
            write!(
                f,
                [code_block
                    .format()
                    .with_options(FormatMdFencedCodeBlockOptions {
                        text_context: TextContext::List,
                    })]
            )
        } else {
            write!(f, [content.format()])
        }
    }
}

impl Format<MarkdownFormatContext> for ListBlockList {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        let iter = BlockListIterator::new(self.content.iter());
        let mut pending_breaks: u8 = 0;
        let mut last_content_was_thematic_break = false;
        let mut last_content_has_trailing_newline = false;
        let mut at_line_terminator = false;
        for item in iter {
            match item {
                BlockListIteratorItem::WithContinuationIndent {
                    continuation,
                    middle_block,
                    content,
                    quote_prefix,
                } => {
                    f.context().comments().is_suppressed(continuation.syntax());

                    for prefix in quote_prefix {
                        write!(
                            f,
                            [prefix.format().with_options(FormatMdQuotePrefixOptions {
                                should_remove: true
                            })]
                        )?;
                    }

                    // A newline right after a block that doesn't carry its
                    // own trailing newline (like an HTML block or a fenced
                    // code block) is that block's line terminator, not a
                    // blank line. The pending break already provides its
                    // line ending, so the newline is removed instead of
                    // printed.
                    if at_line_terminator && content.is_newline() {
                        if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) =
                            &content
                        {
                            write!(
                                f,
                                [newline.format().with_options(FormatMdNewlineOptions {
                                    print_mode: TextPrintMode::Remove,
                                })]
                            )?;
                        }
                    } else {
                        Self::emit_pending_breaks(pending_breaks, &content, f)?;
                        Self::fmt_list_content(&content, f)?;
                    }

                    let middle_is_terminator = middle_block.is_newline()
                        && (content.is_html_block() || content.is_fenced_block());
                    if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) =
                        &middle_block
                    {
                        write!(
                            f,
                            [newline.format().with_options(FormatMdNewlineOptions {
                                print_mode: TextPrintMode::Remove,
                            })]
                        )?;
                    } else {
                        write!(f, [middle_block.format()])?;
                    }

                    write!(
                        f,
                        [continuation
                            .format()
                            .with_options(FormatMdContinuationIndentOptions {
                                should_remove: true
                            })]
                    )?;

                    pending_breaks = if middle_block.is_newline() && !middle_is_terminator {
                        2
                    } else {
                        1
                    };
                    at_line_terminator = false;
                    last_content_was_thematic_break = content.is_thematic_break();
                    last_content_has_trailing_newline = middle_block.is_newline();
                }

                BlockListIteratorItem::OnlyContinuationIndent {
                    content,
                    continuation,
                    quote_prefix,
                } => {
                    f.context().comments().is_suppressed(continuation.syntax());

                    for prefix in quote_prefix {
                        write!(
                            f,
                            [prefix.format().with_options(FormatMdQuotePrefixOptions {
                                should_remove: true
                            })]
                        )?;
                    }

                    Self::emit_pending_breaks(pending_breaks, &content, f)?;
                    Self::fmt_list_content(&content, f)?;
                    write!(
                        f,
                        [continuation
                            .format()
                            .with_options(FormatMdContinuationIndentOptions {
                                should_remove: true
                            })]
                    )?;
                    pending_breaks = if content.is_thematic_break() { 2 } else { 1 };
                    last_content_was_thematic_break = content.is_thematic_break();
                    last_content_has_trailing_newline = false;
                    at_line_terminator = content.is_html_block() || content.is_fenced_block();
                }

                BlockListIteratorItem::Simple((content, quote_prefix)) => {
                    for prefix in quote_prefix {
                        write!(
                            f,
                            [prefix.format().with_options(FormatMdQuotePrefixOptions {
                                should_remove: true
                            })]
                        )?;
                    }
                    if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) = &content
                    {
                        // A newline right after a block that doesn't carry
                        // its own trailing newline is that block's line
                        // terminator; only the newlines after it are blank
                        // lines. The pending break already provides the
                        // terminator's line ending.
                        if at_line_terminator {
                            at_line_terminator = false;
                        } else {
                            if pending_breaks > 0 {
                                last_content_has_trailing_newline = true;
                            }
                            pending_breaks += 1;
                        }
                        write!(
                            f,
                            [newline.format().with_options(FormatMdNewlineOptions {
                                print_mode: TextPrintMode::Remove,
                            })]
                        )?;
                    } else {
                        Self::emit_pending_breaks(pending_breaks, &content, f)?;
                        Self::fmt_list_content(&content, f)?;
                        pending_breaks = if content.is_thematic_break() { 2 } else { 1 };
                        last_content_was_thematic_break = content.is_thematic_break();
                        last_content_has_trailing_newline = false;
                        at_line_terminator = content.is_html_block() || content.is_fenced_block();
                    }
                }
            }
        }

        if pending_breaks > 0 {
            match (
                pending_breaks,
                last_content_was_thematic_break && !last_content_has_trailing_newline,
            ) {
                (_, true) => write!(f, [hard_line_break()])?,
                (1, false) => write!(f, [hard_line_break()])?,
                (_, false) => write!(f, [empty_line()])?,
            }
        }
        Ok(())
    }
}

/// Returns `true` for fenced code blocks nested deeply enough in lists that a
/// blank line keeps the fence separated from the preceding list paragraph.
///
/// This is intentionally narrower than "any fenced code block after list
/// content": shallow list code fences can be followed by regular paragraphs,
/// and forcing a blank line there changes existing idempotency-sensitive cases.
fn should_separate_fenced_code_block(content: &AnyMdBlock) -> bool {
    if !content.is_fenced_block() {
        return false;
    }

    content
        .syntax()
        .ancestors()
        .filter(|ancestor| {
            MdBulletListItem::can_cast(ancestor.kind())
                || MdOrderedListItem::can_cast(ancestor.kind())
        })
        .count()
        >= 3
}

/// Iterator in charge or formatting a [MdBlockList] that is inside a bullet list
struct BlockListIterator {
    content: AstNodeListIterator<MarkdownLanguage, AnyMdBlock>,
    queue: VecDeque<Option<AnyMdBlock>>,
    quote_prefixes: Vec<MdQuotePrefix>,
}
impl BlockListIterator {
    fn new(content: AstNodeListIterator<MarkdownLanguage, AnyMdBlock>) -> Self {
        Self {
            content,
            queue: VecDeque::new(),
            quote_prefixes: Vec::new(),
        }
    }

    fn next_block(&mut self) -> Option<AnyMdBlock> {
        loop {
            let block = if let Some(queued) = self.queue.pop_front().flatten() {
                queued
            } else {
                self.content.next()?
            };

            if let AnyMdBlock::MdQuotePrefix(prefix) = &block {
                self.quote_prefixes.push(prefix.clone());
                continue;
            }
            return Some(block);
        }
    }

    fn drain_quote_prefixes(&mut self) -> Vec<MdQuotePrefix> {
        std::mem::take(&mut self.quote_prefixes)
    }
}

enum BlockListIteratorItem {
    WithContinuationIndent {
        content: AnyMdBlock,
        middle_block: AnyMdBlock,
        continuation: MdContinuationIndent,
        quote_prefix: Vec<MdQuotePrefix>,
    },
    OnlyContinuationIndent {
        content: AnyMdBlock,
        continuation: MdContinuationIndent,
        quote_prefix: Vec<MdQuotePrefix>,
    },
    Simple((AnyMdBlock, Vec<MdQuotePrefix>)),
}

impl Debug for BlockListIteratorItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockListIteratorItem::WithContinuationIndent { .. } => f
                .debug_struct("BlockListIteratorItem::WithContinuationIndent")
                .finish(),
            BlockListIteratorItem::OnlyContinuationIndent { .. } => f
                .debug_struct("BlockListIteratorItem::OnlyContinuationIndent")
                .finish(),
            BlockListIteratorItem::Simple(_) => {
                f.debug_struct("BlockListIteratorItem::Simple").finish()
            }
        }
    }
}

impl Iterator for BlockListIterator {
    type Item = BlockListIteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        let content = self.next_block()?;
        let second_block = self.next_block();
        if let Some(second_block) = second_block {
            if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdContinuationIndent(continuation)) =
                second_block
            {
                let quote_prefix = self.drain_quote_prefixes();
                Some(BlockListIteratorItem::OnlyContinuationIndent {
                    content,
                    continuation,
                    quote_prefix,
                })
            } else {
                let third_block = self.next_block();
                match third_block {
                    Some(third_block) => {
                        if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdContinuationIndent(
                            continuation,
                        )) = third_block
                        {
                            let quote_prefix = self.drain_quote_prefixes();
                            Some(BlockListIteratorItem::WithContinuationIndent {
                                content,
                                middle_block: second_block,
                                quote_prefix,
                                continuation,
                            })
                        } else {
                            self.queue.push_back(Some(second_block));
                            self.queue.push_back(Some(third_block));
                            Some(BlockListIteratorItem::Simple((
                                content,
                                self.drain_quote_prefixes(),
                            )))
                        }
                    }
                    None => {
                        self.queue.push_back(Some(second_block));
                        Some(BlockListIteratorItem::Simple((
                            content,
                            self.drain_quote_prefixes(),
                        )))
                    }
                }
            }
        } else {
            Some(BlockListIteratorItem::Simple((
                content,
                self.drain_quote_prefixes(),
            )))
        }
    }
}

impl FusedIterator for BlockListIterator {}
impl ExactSizeIterator for BlockListIterator {}
