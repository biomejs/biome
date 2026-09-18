use crate::{markdown::auxiliary::indent_token::FormatMdIndentTokenOptions, prelude::*};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdHeader, MdIndentTokenList, MdListMarkerPrefix};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentTokenList {
    should_remove: bool,
}
impl FormatRule<MdIndentTokenList> for FormatMdIndentTokenList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdIndentTokenList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let inside_list_prefix = node
            .syntax()
            .parent()
            .is_some_and(|node| MdListMarkerPrefix::can_cast(node.kind()));

        let inside_header = node
            .syntax()
            .parent()
            .is_some_and(|node| MdHeader::can_cast(node.kind()));

        // inside lists, alignment is handled by alignments
        if self.should_remove || inside_list_prefix || inside_header {
            for token in node.iter() {
                write!(
                    f,
                    [token.format().with_options(FormatMdIndentTokenOptions {
                        replace_tabs_with_spaces: false,
                        should_remove: true,
                    })]
                )?;
            }
            Ok(())
        } else {
            f.join().entries(node.iter().formatted()).finish()
        }
    }
}

pub(crate) struct FormatMdIndentTokenListOptions {
    pub(crate) should_remove: bool,
}

impl FormatRuleWithOptions<MdIndentTokenList> for FormatMdIndentTokenList {
    type Options = FormatMdIndentTokenListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.should_remove = options.should_remove;
        self
    }
}
