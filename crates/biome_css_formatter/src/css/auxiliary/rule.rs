use crate::prelude::*;
use biome_css_syntax::{CssRule, CssRuleFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRule;
impl FormatNodeRule<CssRule> for FormatCssRule {
    fn fmt_fields(&self, node: &CssRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRuleFields { prelude, block } = node.as_fields();

        write!(f, [group(&prelude.format()), space(), &block?.format()])
    }
}
