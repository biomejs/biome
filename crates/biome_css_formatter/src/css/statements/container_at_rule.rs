use crate::prelude::*;
use biome_css_syntax::{CssContainerAtRule, CssContainerAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerAtRule;

impl FormatNodeRule<CssContainerAtRule> for FormatCssContainerAtRule {
    fn fmt_fields(&self, node: &CssContainerAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssContainerAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
