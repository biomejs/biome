use crate::bullet_list::FmtAnyList;
use crate::markdown::auxiliary::newline::FormatMdNewlineOptions;
use crate::markdown::auxiliary::paragraph::FormatMdParagraphOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode};
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::write;
use biome_markdown_syntax::list_ext::AnyListItem;
use biome_markdown_syntax::{AnyMdBlock, AnyMdLeafBlock, MdBlockList, MdBullet};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBlockList {
    /// When true, it removes all leading newlines and trailing newlines
    paragraph_print_mode: TextPrintMode,

    trim: bool,
}
impl FormatRule<MdBlockList> for FormatMdBlockList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdBlockList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut joiner = f.join();

        let text_context = if node
            .syntax()
            .parent()
            .is_some_and(|n| MdBullet::can_cast(n.kind()))
        {
            TextContext::List
        } else {
            TextContext::Neutral
        };

        if !self.trim {
            let mut prev_content = PrevContentBlock::None;
            let mut iter = node.iter().peekable();

            while let Some(node) = iter.next() {
                match &node {
                    AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph)) => {
                        prev_content = PrevContentBlock::Paragraph;
                        joiner.entry(&paragraph.format().with_options(FormatMdParagraphOptions {
                            trim_mode: self.paragraph_print_mode,
                            text_context,
                        }));
                    }

                    node if node.is_any_header() => {
                        prev_content = PrevContentBlock::Header;
                        joiner.entry(&node.format());
                    }

                    node if node.is_newline() => {
                        if prev_content == PrevContentBlock::Header
                            && iter.peek().is_some_and(|next| !next.is_newline())
                        {
                            let entry =
                                format_with(|f| write!(f, [node.format(), hard_line_break()]));
                            joiner.entry(&entry);
                        } else {
                            joiner.entry(&node.format());
                        }
                        prev_content = PrevContentBlock::Other;
                    }

                    AnyMdBlock::MdQuotePrefix(prefix)
                        if prev_content == PrevContentBlock::Paragraph
                            && iter.peek().is_some_and(|next| next.is_fenced_block()) =>
                    {
                        prev_content = PrevContentBlock::Other;
                        let entry = format_with(|f| {
                            write!(f, [token(">")])?;
                            write!(f, [hard_line_break()])?;
                            write!(f, [prefix.format()])
                        });
                        joiner.entry(&entry);
                    }

                    AnyMdBlock::MdQuotePrefix(_) => {
                        joiner.entry(&node.format());
                    }

                    _ => {
                        prev_content = PrevContentBlock::Other;
                        if let Some(list_item) = node.as_any_list_item() {
                            joiner
                                .entry(&format_with(|f| FmtAnyList::new(list_item.clone()).fmt(f)));
                        } else {
                            joiner.entry(&node.format());
                        }
                    }
                }
            }

            return joiner.finish();
        }

        DefaultBlockListFormatter { node: node.clone() }.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PrevContentBlock {
    None,
    Header,
    Paragraph,
    Other,
}

pub(crate) struct FormatMdBlockListOptions {
    /// Signals how [MdParagraph] should be formatted
    pub(crate) paragraph_print_mode: TextPrintMode,

    /// When true, leading and trailing newlines are removed
    pub(crate) trim: bool,
}

impl FormatRuleWithOptions<MdBlockList> for FormatMdBlockList {
    type Options = FormatMdBlockListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.paragraph_print_mode = options.paragraph_print_mode;
        self.trim = options.trim;
        self
    }
}

pub(crate) struct DefaultBlockListFormatter {
    node: MdBlockList,
}

impl Format<MarkdownFormatContext> for DefaultBlockListFormatter {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        f.context().comments().is_suppressed(self.node.syntax());

        let mut joiner = f.join();

        let mut iter = self.node.iter();

        // Count trailing newlines using next_back
        let mut trailing_count = 0;
        while iter.next_back().is_some_and(|block| block.is_newline()) {
            trailing_count += 1;
        }

        // we don't need the iter anymore
        drop(iter);

        // Single forward pass in document order
        let mut still_leading = true;
        let mut prev_was_header = false;
        let mut prev_was_list = false;
        let mut prev_ends_with_line_break = false;
        let content_count = self.node.len() - trailing_count;
        let mut iter = self.node.iter().enumerate().peekable();
        while let Some((index, node)) = iter.next() {
            if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) = &node {
                let is_leading = still_leading;
                let is_trailing = index >= content_count;
                let next_is_bull_item = iter.peek().is_some_and(|(_, next)| next.is_list());

                if prev_was_header && !is_leading && !is_trailing {
                    joiner.entry(&newline.format().with_options(FormatMdNewlineOptions {
                        should_remove: true,
                    }));
                    while iter.peek().is_some_and(|(_, next)| next.is_newline()) {
                        if let Some((
                            _,
                            AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(extra)),
                        )) = iter.next()
                        {
                            joiner.entry(&extra.format().with_options(FormatMdNewlineOptions {
                                should_remove: true,
                            }));
                        }
                    }
                    if prev_was_header {
                        joiner.entry(&empty_line());
                    }
                } else if prev_was_list && !is_leading && !is_trailing {
                    // A list always flushes its own trailing line break, so
                    // the newlines that follow it at this level must be
                    // re-evaluated as a whole run: printing them one by one
                    // double-counts the line ending when the last block of
                    // the list (e.g. a thematic break) doesn't swallow it.
                    let mut run = vec![newline.clone()];
                    while iter
                        .peek()
                        .is_some_and(|(i, next)| next.is_newline() && *i < content_count)
                    {
                        if let Some((
                            _,
                            AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(nl)),
                        )) = iter.next()
                        {
                            run.push(nl);
                        }
                    }
                    let next_is_list = iter.peek().is_some_and(|(_, next)| next.is_list());
                    if next_is_list {
                        // Sibling lists always need exactly one blank line to
                        // stay separate lists.
                        for nl in &run {
                            joiner.entry(&nl.format().with_options(FormatMdNewlineOptions {
                                should_remove: true,
                            }));
                        }
                        joiner.entry(&empty_line());
                    } else {
                        let mut blank_lines = run.iter();
                        if !prev_ends_with_line_break {
                            // The first newline is the line terminator of the
                            // list itself: the list has already flushed its own
                            // line break, so printing it would create a
                            // spurious blank line.
                            if let Some(line_terminator) = blank_lines.next() {
                                joiner.entry(&line_terminator.format().with_options(
                                    FormatMdNewlineOptions {
                                        should_remove: true,
                                    },
                                ));
                            }
                        }
                        // The remaining newlines are real blank lines, kept as
                        // they are.
                        for blank_line in blank_lines {
                            joiner.entry(&blank_line.format());
                        }
                    }
                } else {
                    joiner.entry(&newline.format().with_options(FormatMdNewlineOptions {
                        should_remove: is_leading || is_trailing || next_is_bull_item,
                    }));
                    if next_is_bull_item {
                        joiner.entry(&hard_line_break());
                    }
                }
                prev_was_header = false;
            } else {
                still_leading = false;
                prev_was_header = node.is_any_header();
                prev_was_list = node.is_list();
                prev_ends_with_line_break = node
                    .as_any_list_item()
                    .is_some_and(|item| list_ends_with_line_break(&item));
                if let Some(list_item) = node.as_any_list_item() {
                    joiner.entry(&format_with(|f| FmtAnyList::new(list_item.clone()).fmt(f)));
                } else {
                    joiner.entry(&node.format());
                }
            }
        }

        joiner.finish()
    }
}

/// Whether the list terminates its own line.
///
/// This is the case when the last block of its last bullet carries the line
/// ending: a paragraph ending with a newline, or a blank line swallowed by
/// the bullet as [MdNewline]. Blocks that end with their last visible
/// character, like a thematic break, leave the line terminator to the
/// enclosing block list.
///
/// [MdNewline]: biome_markdown_syntax::MdNewline
fn list_ends_with_line_break(item: &AnyListItem) -> bool {
    let Some(last_bullet) = item.list().iter().last() else {
        return false;
    };
    let mut content = last_bullet.content();
    loop {
        match content.iter().last() {
            Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_))) => return true,
            Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph))) => {
                return paragraph.ends_with_newline();
            }
            Some(block) => match block
                .as_any_list_item()
                .and_then(|nested| nested.list().iter().last())
            {
                // Nested list: the line ending is carried by its last bullet.
                Some(bullet) => content = bullet.content(),
                None => return false,
            },
            None => return false,
        }
    }
}
