use crate::markdown::auxiliary::newline::FormatMdNewlineOptions;
use crate::markdown::auxiliary::paragraph::FormatMdParagraphOptions;
use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::write;
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

        let inside_list = node
            .syntax()
            .parent()
            .is_some_and(|n| MdBullet::can_cast(n.kind()));

        if !self.trim {
            let mut prev_content = PrevContentBlock::None;
            let mut iter = node.iter().peekable();

            while let Some(node) = iter.next() {
                match &node {
                    AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph)) => {
                        prev_content = PrevContentBlock::Paragraph;
                        joiner.entry(&paragraph.format().with_options(FormatMdParagraphOptions {
                            trim_mode: self.paragraph_print_mode,
                            inside_list,
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
                        joiner.entry(&node.format());
                    }
                }
            }

            return joiner.finish();
        }

        let mut iter = node.iter();

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
        let content_count = node.len() - trailing_count;
        let mut iter = node.iter().enumerate().peekable();
        while let Some((index, node)) = iter.next() {
            if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) = &node {
                let is_leading = still_leading;
                let is_trailing = index >= content_count;
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
                    joiner.entry(&empty_line());
                } else {
                    joiner.entry(&newline.format().with_options(FormatMdNewlineOptions {
                        should_remove: is_leading || is_trailing,
                    }));
                }
                prev_was_header = false;
            } else {
                still_leading = false;
                prev_was_header = node.is_any_header();
                joiner.entry(&node.format());
            }
        }

        joiner.finish()
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
