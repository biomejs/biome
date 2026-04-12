use crate::markdown::auxiliary::newline::FormatMdNewlineOptions;
use crate::prelude::*;
use biome_formatter::FormatRuleWithOptions;
use biome_markdown_syntax::{AnyMdBlock, AnyMdLeafBlock, MdBlockList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBlockList {
    /// When true, it removes all leading newlines and trailing newlines
    trim: bool,
}
impl FormatRule<MdBlockList> for FormatMdBlockList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdBlockList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut joiner = f.join();

        if !self.trim {
            return f.join().entries(node.iter().formatted()).finish();
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
    pub(crate) trim: bool,
}

impl FormatRuleWithOptions<MdBlockList> for FormatMdBlockList {
    type Options = FormatMdBlockListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.trim = options.trim;
        self
    }
}
