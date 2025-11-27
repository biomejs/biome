use crate::prelude::*;
use biome_css_syntax::{CssPositionTryAtRule, CssPositionTryAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPositionTryAtRule;
impl FormatNodeRule<CssPositionTryAtRule> for FormatCssPositionTryAtRule {
    fn fmt_fields(&self, node: &CssPositionTryAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPositionTryAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
