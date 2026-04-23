use crate::markdown::auxiliary::newline::FormatMdNewlineOptions;
use crate::markdown::auxiliary::paragraph::FormatMdParagraphOptions;
use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::FormatRuleWithOptions;
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
            for node in node.iter() {
                match &node {
                    AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph)) => {
                        joiner.entry(&paragraph.format().with_options(FormatMdParagraphOptions {
                            trim_mode: self.paragraph_print_mode,
                            inside_list,
                        }));
                    }

                    _ => {
                        joiner.entry(&node.format());
                    }
                }
            }

            return joiner.finish();
        }

        let mut iter = node.iter();

        // Count trailing newlines using next_back
        let mut trailing_count = 0;
        while let Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_))) = iter.next_back()
        {
            trailing_count += 1;
        }

        // we don't need the iter anymore
        drop(iter);

        // Single forward pass in document order
        let mut still_leading = true;
        let content_count = node.len() - trailing_count;
        for (index, node) in node.iter().enumerate() {
            if let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) = node {
                let is_leading = still_leading;
                let is_trailing = index >= content_count;
                joiner.entry(&newline.format().with_options(FormatMdNewlineOptions {
                    should_remove: is_leading || is_trailing,
                }));
            } else {
                still_leading = false;
                joiner.entry(&node.format());
            }
        }

        joiner.finish()
    }
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
