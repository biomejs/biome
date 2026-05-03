use crate::prelude::*;
use biome_css_syntax::{ScssReturnAtRule, ScssReturnAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssReturnAtRule;

impl FormatNodeRule<ScssReturnAtRule> for FormatScssReturnAtRule {
    fn fmt_fields(&self, node: &ScssReturnAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssReturnAtRuleFields {
            return_token,
            value,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                return_token.format(),
                space(),
                value.format(),
                semicolon_token.format()
            ]
        )
    }
}
