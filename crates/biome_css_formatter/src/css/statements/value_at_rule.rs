use crate::prelude::*;
use biome_css_syntax::{CssValueAtRule, CssValueAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRule;
impl FormatNodeRule<CssValueAtRule> for FormatCssValueAtRule {
    fn fmt_fields(&self, node: &CssValueAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssValueAtRuleFields {
            value_token,
            clause,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                value_token.format(),
                space(),
                clause.format(),
                semicolon_token.format(),
            ]
        )
    }
}
