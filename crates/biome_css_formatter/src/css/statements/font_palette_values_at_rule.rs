use crate::prelude::*;
use biome_css_syntax::{CssFontPaletteValuesAtRule, CssFontPaletteValuesAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontPaletteValuesAtRule;

impl FormatNodeRule<CssFontPaletteValuesAtRule> for FormatCssFontPaletteValuesAtRule {
    fn fmt_fields(
        &self,
        node: &CssFontPaletteValuesAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssFontPaletteValuesAtRuleFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
