use crate::prelude::*;
use biome_css_syntax::{ScssErrorAtRule, ScssErrorAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssErrorAtRule;

impl FormatNodeRule<ScssErrorAtRule> for FormatScssErrorAtRule {
    fn fmt_fields(&self, node: &ScssErrorAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssErrorAtRuleFields {
            error_token,
            value,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                error_token.format(),
                space(),
                value.format(),
                semicolon_token.format()
            ]
        )
    }
}
