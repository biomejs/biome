use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdParagraph, MdParagraphFields};

#[derive(Debug, Clone)]
pub(crate) struct FormatMdParagraph {
    trim_mode: TextPrintMode,
    text_context: TextContext,
}

impl Default for FormatMdParagraph {
    fn default() -> Self {
        Self {
            trim_mode: TextPrintMode::fill(),
            text_context: TextContext::Neutral,
        }
    }
}
impl FormatNodeRule<MdParagraph> for FormatMdParagraph {
    fn fmt_fields(&self, node: &MdParagraph, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdParagraphFields { list, hard_line } = node.as_fields();
        write!(
            f,
            [list
                .format()
                .with_options(FormatMdFormatInlineItemListOptions {
                    print_mode: self.trim_mode,
                    keep_fences_in_italics: false,
                    text_context: self.text_context,
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
    pub trim_mode: TextPrintMode,
    /// Where the paragraph is located in the document structure.
    pub text_context: TextContext,
}

impl FormatRuleWithOptions<MdParagraph> for FormatMdParagraph {
    type Options = FormatMdParagraphOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.trim_mode = options.trim_mode;
        self.text_context = options.text_context;
        self
    }
}
