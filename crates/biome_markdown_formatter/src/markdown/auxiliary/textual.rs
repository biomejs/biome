use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdTextual, MdTextualFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdTextual {
    should_remove: bool,
}
impl FormatNodeRule<MdTextual> for FormatMdTextual {
    fn fmt_fields(&self, node: &MdTextual, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdTextualFields { value_token } = node.as_fields();

        if self.should_remove {
            format_removed(&value_token?).fmt(f)
        } else {
            write!(f, [value_token.format()])
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FormatMdTextualOptions {
    pub(crate) should_remove: bool,
}

impl FormatRuleWithOptions<MdTextual> for FormatMdTextual {
    type Options = FormatMdTextualOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.should_remove = options.should_remove;
        self
    }
}
