use biome_css_syntax::{CssSupportsAtRule, CssSupportsAtRuleFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsAtRule;

impl FormatNodeRule<CssSupportsAtRule> for FormatCssSupportsAtRule {
    fn fmt_fields(&self, node: &CssSupportsAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSupportsAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
