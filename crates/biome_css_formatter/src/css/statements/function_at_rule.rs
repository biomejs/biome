use biome_css_syntax::{CssFunctionAtRule, CssFunctionAtRuleFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionAtRule;

impl FormatNodeRule<CssFunctionAtRule> for FormatCssFunctionAtRule {
    fn fmt_fields(&self, node: &CssFunctionAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssFunctionAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
