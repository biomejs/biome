use crate::prelude::*;
use biome_css_syntax::{CssScopeAtRule, CssScopeAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeAtRule;

impl FormatNodeRule<CssScopeAtRule> for FormatCssScopeAtRule {
    fn fmt_fields(&self, node: &CssScopeAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssScopeAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
