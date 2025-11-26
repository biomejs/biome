use crate::prelude::*;
use biome_css_syntax::{CssViewTransitionAtRule, CssViewTransitionAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssViewTransitionAtRule;
impl FormatNodeRule<CssViewTransitionAtRule> for FormatCssViewTransitionAtRule {
    fn fmt_fields(&self, node: &CssViewTransitionAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssViewTransitionAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
