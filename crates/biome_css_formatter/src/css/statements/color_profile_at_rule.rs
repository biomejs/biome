use crate::prelude::*;
use biome_css_syntax::{CssColorProfileAtRule, CssColorProfileAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssColorProfileAtRule;

impl FormatNodeRule<CssColorProfileAtRule> for FormatCssColorProfileAtRule {
    fn fmt_fields(&self, node: &CssColorProfileAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssColorProfileAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
