use crate::prelude::*;
use biome_css_syntax::{
    CssSyntaxComponentWithoutMultiplier, CssSyntaxComponentWithoutMultiplierFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSyntaxComponentWithoutMultiplier;

impl FormatNodeRule<CssSyntaxComponentWithoutMultiplier>
    for FormatCssSyntaxComponentWithoutMultiplier
{
    fn fmt_fields(
        &self,
        node: &CssSyntaxComponentWithoutMultiplier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssSyntaxComponentWithoutMultiplierFields {
            l_angle_token,
            type_name_token,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_angle_token.format(),
                type_name_token.format(),
                r_angle_token.format()
            ]
        )
    }
}
