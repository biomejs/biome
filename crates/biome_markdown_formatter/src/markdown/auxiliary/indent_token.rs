use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::MdIndentToken;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentToken {
    replace_tabs_with_spaces: bool,
    should_remove: bool,
}

pub(crate) struct FormatMdIndentTokenOptions {
    pub(crate) replace_tabs_with_spaces: bool,
    pub(crate) should_remove: bool,
}

impl FormatRuleWithOptions<MdIndentToken> for FormatMdIndentToken {
    type Options = FormatMdIndentTokenOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.replace_tabs_with_spaces = options.replace_tabs_with_spaces;
        self.should_remove = options.should_remove;
        self
    }
}

impl FormatNodeRule<MdIndentToken> for FormatMdIndentToken {
    fn fmt_fields(&self, node: &MdIndentToken, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let token = node.md_indent_char_token()?;

        if self.should_remove {
            return write!(f, [format_removed(&token)]);
        }

        if self.replace_tabs_with_spaces && token.text() == "\t" {
            format_replaced(&token, &text("    ", Some(token.text_range().start()))).fmt(f)
        } else {
            token.format().fmt(f)
        }
    }
}
