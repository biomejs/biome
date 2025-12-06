use crate::prelude::*;
use biome_css_syntax::{
    CssFontPaletteValuesAtRuleDeclarator, CssFontPaletteValuesAtRuleDeclaratorFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontPaletteValuesAtRuleDeclarator;

impl FormatNodeRule<CssFontPaletteValuesAtRuleDeclarator>
    for FormatCssFontPaletteValuesAtRuleDeclarator
{
    fn fmt_fields(
        &self,
        node: &CssFontPaletteValuesAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssFontPaletteValuesAtRuleDeclaratorFields {
            font_palette_values_token,
            name,
        } = node.as_fields();

        write!(
            f,
            [font_palette_values_token.format(), space(), name.format()]
        )
    }
}
