use crate::prelude::*;
use biome_css_syntax::{CssMediaAtRule, CssMediaAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaAtRule;

impl FormatNodeRule<CssMediaAtRule> for FormatCssMediaAtRule {
    fn fmt_fields(&self, node: &CssMediaAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
