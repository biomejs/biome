use crate::prelude::*;
use biome_css_syntax::{CssCounterStyleAtRule, CssCounterStyleAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCounterStyleAtRule;

impl FormatNodeRule<CssCounterStyleAtRule> for FormatCssCounterStyleAtRule {
    fn fmt_fields(&self, node: &CssCounterStyleAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssCounterStyleAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
