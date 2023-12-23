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
        let CssFontPaletteValuesAtRuleFields {
            font_palette_values_token,
            name,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                font_palette_values_token.format(),
                space(),
                name.format(),
                space(),
                block.format()
            ]
        )
    }
}
