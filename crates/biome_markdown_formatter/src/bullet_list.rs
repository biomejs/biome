use crate::context::MarkdownFormatContext;
use crate::markdown::auxiliary::continuation_indent::FormatMdContinuationIndentOptions;
use crate::markdown::auxiliary::fenced_code_block::FormatMdFencedCodeBlockOptions;
use crate::markdown::auxiliary::indent_code_block::FormatMdIndentCodeBlockOptions;
use crate::markdown::auxiliary::list_marker_prefix::{
    FormatMdListMarkerPrefixOptions, OrderedMarker, TargetMarker,
};
use crate::markdown::auxiliary::newline::FormatMdNewlineOptions;
use crate::markdown::auxiliary::paragraph::FormatMdParagraphOptions;
use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::quote::quote_line_prefix;
use crate::shared::{TextContext, TextPrintMode};
use crate::{AsFormat, MarkdownFormatter};
use biome_formatter::prelude::*;
use biome_formatter::{Format, FormatResult, format_args, write};
use biome_markdown_syntax::list_ext::{AnyListItem, ListMarker, OrderedListDelimiter};
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
        BulletListPrinter::new(&list, list_sibling_index(&self.node)).fmt(f)
    }
}

pub(crate) struct BulletListPrinter {
    bullets: Vec<ListBullet>,
}

impl BulletListPrinter {
    /// Creates a printer for a single parsed list node.
    ///
    /// `list_sibling_index` is not the number of an item inside the list. It is
    /// the position of this parsed list among the adjacent sibling lists of the
    /// same kind. For example, in `1. a\n2) b`, the parser creates two ordered
    /// list nodes. The first list has sibling index `0`, and the second has
    /// sibling index `1`.
    ///
    /// The formatter uses this index to choose markers that keep adjacent parsed
    /// lists separate after formatting. Without this, two separate lists can be
    /// printed in a way that Markdown parses back as one list.
    pub(crate) fn new(node: &MdBulletList, list_sibling_index: usize) -> Self {
        let marker_plan = ListMarkerPlan::from_list(node, list_sibling_index);

        Self {
            bullets: node
                .iter()
                .enumerate()
                .map(|(index, item)| ListBullet {
                    node: item,
                    unordered_marker: marker_plan.unordered_marker,
                    target_ordered_marker: marker_plan.ordered_marker_for_index(index),
                })
                .collect(),
        }
    }
}

/// The marker choices that are shared by all bullets in a single parsed list node.
///
/// The marker must be chosen once per list, not once per item. If some items in
/// the same parsed list used `-` and others used `*`, Markdown could parse the
/// formatted output as multiple lists.
struct ListMarkerPlan {
    unordered_marker: ListMarker,
    ordered_marker: Option<OrderedMarkerPlan>,
}

impl ListMarkerPlan {
    fn from_list(node: &MdBulletList, list_sibling_index: usize) -> Self {
        let has_dash_thematic_break = node
            .iter()
            .any(|bullet| first_block_is_dash_thematic_break(&bullet.content()));

        Self {
            unordered_marker: unordered_marker_for_list(
                list_sibling_index,
                has_dash_thematic_break,
            ),
            ordered_marker: OrderedMarkerPlan::from_list(node, list_sibling_index),
        }
    }

    fn ordered_marker_for_index(&self, index: usize) -> Option<TargetMarker> {
        self.ordered_marker
            .as_ref()
            .map(|marker| TargetMarker::Ordered(marker.marker_for_index(index)))
    }
}

/// The marker choice for a single parsed ordered list node.
struct OrderedMarkerPlan {
    start: usize,
    delimiter: OrderedListDelimiter,
    use_git_diff_friendly_numbering: bool,
}

impl OrderedMarkerPlan {
    fn from_list(node: &MdBulletList, list_sibling_index: usize) -> Option<Self> {
        let numbers = node
            .iter()
            .filter_map(|bullet| bullet.ordered_marker_number())
            .collect::<Vec<_>>();
        let start = numbers.first().copied()?;
        let use_git_diff_friendly_numbering = has_git_diff_friendly_ordered_list(&numbers);

        Some(Self {
            start,
            delimiter: ordered_delimiter_for_list(list_sibling_index),
            use_git_diff_friendly_numbering,
        })
    }

    /// Returns the ordered marker for a bullet, including its delimiter.
    fn marker_for_index(&self, index: usize) -> OrderedMarker {
        let number = if index == 0 {
            self.start
        } else if self.use_git_diff_friendly_numbering {
            1
        } else {
            self.start.saturating_add(index)
        };

        OrderedMarker::new(number, self.delimiter)
    }
}

impl Format<MarkdownFormatContext> for BulletListPrinter {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        let mut joiner = f.join();

        for (index, item) in self.bullets.iter().enumerate() {
            if index > 0 && content_ends_with_quote_prefix(&self.bullets[index - 1].node) {
                let line_prefix = quote_line_prefix(item.node.syntax())?;
                if !line_prefix.is_empty() {
                    joiner.entry(&format_with(|f| {
                        write!(
                            f,
                            [dedent_to_root(&format_args![
                                hard_line_break(),
                                line_prefix.format(true)
                            ])]
                        )
                    }));
                }
            }
            joiner.entry(item);
        }
        joiner.finish()
    }
}

/// A trailing [MdQuotePrefix] in a bullet's content is the `> ` of the line
/// that starts the next bullet: the parser attaches continuation-line
/// prefixes to the previous sibling's content.
fn content_ends_with_quote_prefix(bullet: &MdBullet) -> bool {
    matches!(
        bullet.content().iter().last(),
        Some(AnyMdBlock::MdQuotePrefix(_))
    )
}

pub(crate) struct ListBullet {
    node: MdBullet,
    /// The marker to print when this bullet belongs to an unordered list.
    ///
    /// This is shared by every bullet in the same parsed list node. For example,
    /// if the list marker plan chooses [ListMarker::Star], then the source
    /// markers [ListMarker::Minus] and [ListMarker::Plus] are both replaced with
    /// `*`.
    unordered_marker: ListMarker,
    /// The marker to print when this bullet belongs to an ordered list.
    ///
    /// This includes both the number and the delimiter. It is `None` for
    /// unordered lists.
    target_ordered_marker: Option<TargetMarker>,
}

impl ListBullet {
    /// Checks whether the indentation before the marker needs to be kept.
    ///
    /// This indentation needs to be kept when the bullet item is followed by a
    /// blank line and an indented code block.
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

        let target_marker = if is_ordered_marker {
            self.target_ordered_marker
        } else {
            let target_marker = self.unordered_marker.unordered_marker_text();
            if target_marker.is_some_and(|target| marker.text_trimmed() == target) {
                None
            } else {
                target_marker.map(|_| TargetMarker::Unordered(self.unordered_marker))
            }
        };
        let marker_width = target_marker
            .as_ref()
            .map_or_else(|| marker.text_trimmed().len(), |target| target.width());

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
                4usize.saturating_sub(marker_width)
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
        let alignment = pre_marker_width + (marker_width as u8) + post_marker_len;

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

/// Chooses the marker for a single unordered list node.
///
/// Adjacent unordered list nodes cannot always be printed with the same marker.
/// If two parsed sibling lists are both printed with `-`, Markdown can parse
/// them back as one loose list. Alternating between `-` and `*` keeps the
/// boundary visible in the formatted text.
///
/// There is one safety rule: do not use `-` when one of the items starts with a
/// thematic break made from dashes. Printing `- ---` is parsed as a thematic
/// break, not as a list item that contains a thematic break. In that case, use
/// `+` for even sibling lists and `*` for odd sibling lists, so the chosen marker
/// still differs from adjacent unordered lists.
fn unordered_marker_for_list(
    list_sibling_index: usize,
    has_dash_thematic_break: bool,
) -> ListMarker {
    if has_dash_thematic_break && list_sibling_index.is_multiple_of(2) {
        ListMarker::Plus
    } else if !list_sibling_index.is_multiple_of(2) {
        ListMarker::Star
    } else {
        ListMarker::Minus
    }
}

/// Chooses the delimiter for a single ordered list node.
///
/// Ordered lists can use either `.` or `)` after the number. If two parsed
/// ordered list nodes are adjacent, using a different delimiter for the next one
/// keeps them separate when the formatted text is parsed again.
fn ordered_delimiter_for_list(list_sibling_index: usize) -> OrderedListDelimiter {
    if list_sibling_index.is_multiple_of(2) {
        OrderedListDelimiter::Dot
    } else {
        OrderedListDelimiter::Paren
    }
}

/// Returns `true` when an ordered list uses "Git diff-friendly" numbering.
///
/// "Git diff-friendly" means this style:
///
/// ```md
/// 1. First item
/// 1. Second item
/// 1. Third item
/// ```
///
/// Markdown still renders this as `1, 2, 3`, but the source keeps every marker
/// as `1`. That makes Git diffs smaller. If a new item is inserted in the
/// middle, only the inserted line changes. In a sequential source list like
/// `1., 2., 3.`, the following lines also need to be renumbered.
///
/// The formatter preserves the first marker number because Markdown uses it as
/// the start value. For example, a list that starts with `10.` should still start
/// at `10` after formatting.
///
/// The decision is based on the source marker numbers:
///
/// - `1, 2, 3` is sequential, so it formats as `1, 2, 3`.
/// - `1, 1, 1` is Git diff-friendly, so it formats as `1, 1, 1`.
/// - `10, 1, 2` is Git diff-friendly, so it formats as `10, 1, 1`.
/// - `0, 1` is sequential, because `1` naturally follows `0`.
/// - `0, 1, 1` is Git diff-friendly, because the third `1` makes the intent
///   clear.
fn has_git_diff_friendly_ordered_list(numbers: &[usize]) -> bool {
    if numbers.len() < 2 || numbers[1] != 1 {
        return false;
    }

    if numbers[0] != 0 {
        return true;
    }

    numbers.get(2).is_some_and(|number| *number == 1)
}

/// Counts the previous adjacent parsed list nodes of the same kind.
///
/// This is a list-node index, not a bullet-item index. It answers: "Is this the
/// first, second, third, ... parsed list among adjacent sibling lists?"
///
/// The formatter uses this to alternate markers for adjacent lists. Alternating
/// keeps two parsed lists from becoming one parsed list after formatting.
/// Newline and continuation-indent nodes are skipped because they are separators,
/// not real content between the lists.
fn list_sibling_index(node: &AnyListItem) -> usize {
    let is_ordered = node.is_ordered();
    let mut index = 0;

    for sibling in node.syntax().siblings(Direction::Prev).skip(1) {
        let Some(block) = AnyMdBlock::cast(sibling) else {
            break;
        };

        if block.is_newline() || block.is_continuation_indent() {
            continue;
        }

        let Some(list_item) = block.as_any_list_item() else {
            break;
        };

        if list_item.is_ordered() != is_ordered {
            break;
        }

        index += 1;
    }

    index
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
        previous_content_was_fenced_code_block: bool,
        previous_content_needs_blank_before_fenced_code_block: bool,
        content: &AnyMdBlock,
        f: &mut Formatter<MarkdownFormatContext>,
    ) -> FormatResult<()> {
        let breaks = if pending_breaks > 0
            && (previous_content_was_fenced_code_block
                || content.is_thematic_break()
                || (content.is_fenced_block()
                    && previous_content_needs_blank_before_fenced_code_block))
        {
            2
        } else if content.is_list() {
            pending_breaks.min(1)
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
        let mut last_content_was_fenced_code_block = false;
        let mut last_content_needs_blank_before_fenced_code_block = false;
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

                    if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) = &content
                        && let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(middle_newline)) =
                            &middle_block
                    {
                        if at_line_terminator {
                            at_line_terminator = false;
                        } else {
                            if pending_breaks > 0 {
                                last_content_has_trailing_newline = true;
                            }
                            pending_breaks += 1;
                        }

                        if pending_breaks > 0 {
                            last_content_has_trailing_newline = true;
                        }
                        pending_breaks += 1;

                        write!(
                            f,
                            [
                                newline.format().with_options(FormatMdNewlineOptions {
                                    print_mode: TextPrintMode::Remove,
                                }),
                                middle_newline
                                    .format()
                                    .with_options(FormatMdNewlineOptions {
                                        print_mode: TextPrintMode::Remove,
                                    }),
                                continuation.format().with_options(
                                    FormatMdContinuationIndentOptions {
                                        should_remove: true
                                    }
                                )
                            ]
                        )?;

                        continue;
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
                        Self::emit_pending_breaks(
                            pending_breaks,
                            last_content_was_fenced_code_block,
                            last_content_needs_blank_before_fenced_code_block,
                            &content,
                            f,
                        )?;
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
                    last_content_was_fenced_code_block = content.is_fenced_block();
                    last_content_needs_blank_before_fenced_code_block =
                        content_needs_blank_before_fenced_code_block(&content);
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

                    if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) = &content
                    {
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
                        write!(
                            f,
                            [continuation.format().with_options(
                                FormatMdContinuationIndentOptions {
                                    should_remove: true
                                }
                            )]
                        )?;
                        continue;
                    }

                    Self::emit_pending_breaks(
                        pending_breaks,
                        last_content_was_fenced_code_block,
                        last_content_needs_blank_before_fenced_code_block,
                        &content,
                        f,
                    )?;
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
                    last_content_was_fenced_code_block = content.is_fenced_block();
                    last_content_needs_blank_before_fenced_code_block =
                        content_needs_blank_before_fenced_code_block(&content);
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
                        Self::emit_pending_breaks(
                            pending_breaks,
                            last_content_was_fenced_code_block,
                            last_content_needs_blank_before_fenced_code_block,
                            &content,
                            f,
                        )?;
                        Self::fmt_list_content(&content, f)?;
                        pending_breaks = if content.is_thematic_break() { 2 } else { 1 };
                        last_content_was_thematic_break = content.is_thematic_break();
                        last_content_was_fenced_code_block = content.is_fenced_block();
                        last_content_needs_blank_before_fenced_code_block =
                            content_needs_blank_before_fenced_code_block(&content);
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

fn content_needs_blank_before_fenced_code_block(content: &AnyMdBlock) -> bool {
    matches!(
        content,
        AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(_))
    ) || content.is_list()
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
