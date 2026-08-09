use crate::prelude::*;
use biome_css_syntax::{CssCustomMediaAtRule, CssCustomMediaAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomMediaAtRule;

impl FormatNodeRule<CssCustomMediaAtRule> for FormatCssCustomMediaAtRule {
    fn fmt_fields(&self, node: &CssCustomMediaAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssCustomMediaAtRuleFields {
            declarator,
            semicolon_token,
        } = node.as_fields();

        write!(f, [declarator.format(), semicolon_token.format()])
    }
}
