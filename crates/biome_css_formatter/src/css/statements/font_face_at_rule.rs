use crate::prelude::*;
use biome_css_syntax::{CssFontFaceAtRule, CssFontFaceAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFaceAtRule;

impl FormatNodeRule<CssFontFaceAtRule> for FormatCssFontFaceAtRule {
    fn fmt_fields(&self, node: &CssFontFaceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssFontFaceAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
