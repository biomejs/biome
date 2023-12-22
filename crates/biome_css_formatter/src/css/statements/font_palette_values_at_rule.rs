use crate::prelude::*;
use biome_css_syntax::CssFontPaletteValuesAtRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontPaletteValuesAtRule;

impl FormatNodeRule<CssFontPaletteValuesAtRule> for FormatCssFontPaletteValuesAtRule {
    fn fmt_fields(
        &self,
        node: &CssFontPaletteValuesAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
