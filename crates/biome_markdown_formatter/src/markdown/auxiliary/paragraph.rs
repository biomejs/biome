use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdParagraph, MdParagraphFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdParagraph {
    trim_start: bool,
}
impl FormatNodeRule<MdParagraph> for FormatMdParagraph {
    fn fmt_fields(&self, node: &MdParagraph, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdParagraphFields { list, hard_line } = node.as_fields();
        write!(
            f,
            [list
                .format()
                .with_options(FormatMdFormatInlineItemListOptions {
                    trime_start: self.trim_start
                })]
        )?;
        if let Some(hard_line) = hard_line {
            write!(f, [hard_line.format()])?;
        }
        Ok(())
    }
}

pub(crate) struct FormatMdParagraphOptions {
    /// Whether to trim the start of the paragraph. Usually signaled by the headers
    pub trim_start: bool,
}

impl FormatRuleWithOptions<MdParagraph> for FormatMdParagraph {
    type Options = FormatMdParagraphOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.trim_start = options.trim_start;
        self
    }
}
