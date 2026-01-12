use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{SvelteDirectiveModifier, SvelteDirectiveModifierFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDirectiveModifier {
    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: bool,
}

impl FormatNodeRule<SvelteDirectiveModifier> for FormatSvelteDirectiveModifier {
    fn fmt_fields(
        &self,
        node: &SvelteDirectiveModifier,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteDirectiveModifierFields {
            bitwise_or_token,
            name,
        } = node.as_fields();
        if self.compact {
            let bitwise_or_token = bitwise_or_token?;
            let name = name?;
            write!(
                f,
                [
                    format_removed(&bitwise_or_token),
                    name.format().with_options(true)
                ]
            )
        } else {
            write!(f, [bitwise_or_token.format(), name.format()])
        }
    }
}

impl FormatRuleWithOptions<SvelteDirectiveModifier> for FormatSvelteDirectiveModifier {
    type Options = bool;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
