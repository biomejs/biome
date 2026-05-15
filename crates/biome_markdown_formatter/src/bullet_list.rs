use crate::context::MarkdownFormatContext;
use crate::markdown::auxiliary::continuation_indent::FormatMdContinuationIndentOptions;
use crate::markdown::auxiliary::list_marker_prefix::FormatMdListMarkerPrefixOptions;
use crate::markdown::auxiliary::newline::FormatMdNewlineOptions;
use crate::markdown::auxiliary::paragraph::FormatMdParagraphOptions;
use crate::shared::TextPrintMode;
use crate::{AsFormat, MarkdownFormatter};
use biome_formatter::prelude::*;
use biome_formatter::{Format, FormatResult, format_args, write};
use biome_markdown_syntax::{
    AnyMdBlock, AnyMdBulletListMember, AnyMdLeafBlock, MarkdownLanguage, MarkdownSyntaxKind,
    MdBlockList, MdBullet, MdBulletFields, MdBulletList, MdContinuationIndent, MdNewline,
};
use biome_rowan::{AstNode, AstNodeList, AstNodeListIterator};
use std::collections::VecDeque;
use std::fmt::Debug;

pub(crate) struct BulletListPrinter {
    bullets: Vec<ListItem>,
}

impl BulletListPrinter {
    pub(crate) fn new(node: &MdBulletList, is_ordered: bool) -> Self {
        let mut bullets = Vec::new();
        for item in node.iter() {
            match item {
                AnyMdBulletListMember::MdBullet(bullet) => {
                    bullets.push(ListItem::Bullet(ListBullet {
                        node: bullet,
                        is_ordered,
                    }));
                }
                AnyMdBulletListMember::MdNewline(newline) => {
                    bullets.push(ListItem::Newline(newline));
                }
            }
        }

        Self { bullets }
    }
}

impl Format<MarkdownFormatContext> for BulletListPrinter {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        let mut iter = self.bullets.iter().peekable();
        let mut joiner = f.join();

        while let Some(item) = iter.next() {
            match item {
                ListItem::Bullet(bullet) => {
                    let fmt_bullet = format_with(|f| {
                        if bullet.has_continuation_indent() {
                            write!(f, [bullet])
                        } else {
                            write!(f, [bullet])
                        }
                    });

                    joiner.entry(&fmt_bullet);
                }

                ListItem::Newline(newline) => {
                    joiner.entry(&newline.format());
                }
            }
        }

        joiner.finish()
    }
}

pub(crate) enum ListItem {
    Bullet(ListBullet),
    Newline(MdNewline),
}

pub(crate) struct ListBullet {
    node: MdBullet,
    is_ordered: bool,
}

impl ListBullet {
    fn has_continuation_indent(&self) -> bool {
        let mut content = self.node.content().iter();
        let first = content.next();
        let second = content.next();
        let third = content.next();
        first.is_some()
            && second.is_some_and(|second| second.is_newline())
            && third.is_some_and(|third| third.is_continuation_indent())
    }
}

impl Format<MarkdownFormatContext> for ListBullet {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.context().comments().is_suppressed(self.node.syntax());

        let MdBulletFields { content, prefix } = self.node.as_fields();

        let prefix = prefix?;
        let marker = prefix.marker()?;

        // `* - - -` is a bullet containing a `-` thematic break. Normalizing `*`
        // to `-` produces `- - - -` which CommonMark 4.1 parses as a thematic
        // break, not a list item. Same for `+ - - -`. Skip normalization for marker
        // but still format content through child formatters.
        let target_marker = if marker.kind() == MarkdownSyntaxKind::MINUS
            || first_block_is_dash_thematic_break(&content)
            || marker.kind() == MarkdownSyntaxKind::MD_ORDERED_LIST_MARKER
        {
            None
        } else {
            Some("-")
        };

        biome_formatter::write!(
            f,
            [prefix
                .format()
                .with_options(FormatMdListMarkerPrefixOptions { target_marker })]
        )?;

        let content = ListBlockList {
            content: content.clone(),
        };
        let alignment = if self.is_ordered { 3 } else { 2 };
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
        .and_then(|c| c.value().ok())
        .is_some_and(|t| t.text_trimmed() == "-")
}

/// It's responsible for formatting a [MdBlockList] that is inside a bullet list
struct ListBlockList {
    content: MdBlockList,
}

impl Format<MarkdownFormatContext> for ListBlockList {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        let mut iter = BlockListIterator::new(self.content.iter()).peekable();

        let mut seen_continuation_indent = false;

        while let Some(item) = iter.next() {
            dbg!(&item);
            match item {
                BlockListIteratorItem::WithContinuationIndent {
                    continuation,
                    middle_block,
                    content,
                } => {
                    f.context().comments().is_suppressed(continuation.syntax());

                    let middle_block = format_with(|f| {
                        if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) =
                            &middle_block
                        {
                            write!(
                                f,
                                [newline.format().with_options(FormatMdNewlineOptions {
                                    should_remove: true
                                })]
                            )
                        } else {
                            write!(f, [middle_block.format()])
                        }
                    });

                    write!(
                        f,
                        [&align(
                            1,
                            &biome_formatter::format_args![
                                middle_block,
                                continuation.format().with_options(
                                    FormatMdContinuationIndentOptions {
                                        should_remove: true
                                    }
                                ),
                                content.format()
                            ],
                        ),]
                    )?;
                    seen_continuation_indent = true;
                }
                BlockListIteratorItem::OnlyContinuationIndent {
                    content,
                    continuation,
                } => {
                    f.context().comments().is_suppressed(continuation.syntax());

                    write!(
                        f,
                        [&align(
                            1,
                            &format_args![
                                continuation.format().with_options(
                                    FormatMdContinuationIndentOptions {
                                        should_remove: true
                                    }
                                ),
                                content.format()
                            ],
                        ),]
                    )?;
                }

                BlockListIteratorItem::Simple(content) => {
                    if seen_continuation_indent && !content.is_list() {
                        write!(f, [empty_line()])?;
                    }

                    if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph)) =
                        content
                    {
                        write!(
                            f,
                            [paragraph.format().with_options(FormatMdParagraphOptions {
                                trim_mode: TextPrintMode::fill(),
                                inside_list: true,
                            })]
                        )?;
                    } else {
                        write!(f, [&content.format()])?;
                    }

                    seen_continuation_indent = false;
                }
            }
        }

        Ok(())
    }
}

struct BlockListIterator {
    content: AstNodeListIterator<MarkdownLanguage, AnyMdBlock>,
    queue: VecDeque<Option<AnyMdBlock>>,
}
impl BlockListIterator {
    fn new(content: AstNodeListIterator<MarkdownLanguage, AnyMdBlock>) -> Self {
        Self {
            content,
            queue: VecDeque::new(),
        }
    }
}

enum BlockListIteratorItem {
    WithContinuationIndent {
        content: AnyMdBlock,
        middle_block: AnyMdBlock,
        continuation: MdContinuationIndent,
    },
    OnlyContinuationIndent {
        content: AnyMdBlock,
        continuation: MdContinuationIndent,
    },
    Simple(AnyMdBlock),
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
        if !self.queue.is_empty() {
            if let Some(block) = self.queue.pop_front().flatten() {
                return Some(BlockListIteratorItem::Simple(block));
            }
        }

        let content = self.content.next()?;
        let second_block = self.content.next();
        if let Some(second_block) = second_block {
            if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdContinuationIndent(continuation)) =
                second_block
            {
                Some(BlockListIteratorItem::OnlyContinuationIndent {
                    content,
                    continuation,
                })
            } else {
                let third_block = self.content.next();
                match third_block {
                    Some(third_block) => {
                        if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdContinuationIndent(
                            continuation,
                        )) = third_block
                        {
                            Some(BlockListIteratorItem::WithContinuationIndent {
                                content,
                                middle_block: second_block,
                                continuation,
                            })
                        } else {
                            self.queue.push_back(Some(second_block));
                            self.queue.push_back(Some(third_block));
                            Some(BlockListIteratorItem::Simple(content))
                        }
                    }
                    None => {
                        self.queue.push_back(Some(second_block));
                        Some(BlockListIteratorItem::Simple(content))
                    }
                }
            }
        } else {
            self.queue.push_back(second_block);
            Some(BlockListIteratorItem::Simple(content))
        }
    }
}
