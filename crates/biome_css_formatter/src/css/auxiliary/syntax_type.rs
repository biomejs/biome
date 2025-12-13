use crate::prelude::*;
use biome_css_syntax::{CssSyntaxType, CssSyntaxTypeFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSyntaxType;

impl FormatNodeRule<CssSyntaxType> for FormatCssSyntaxType {
    fn fmt_fields(&self, node: &CssSyntaxType, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSyntaxTypeFields {
            l_angle_token,
            type_name,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_angle_token.format(),
                type_name.format(),
                r_angle_token.format(),
            ]
        )
    }
}
