use crate::prelude::*;
use biome_css_syntax::{CssColor, CssColorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssColor;
impl FormatNodeRule<CssColor> for FormatCssColor {
    fn fmt_fields(&self, node: &CssColor, f: &mut CssFormatter) -> FormatResult<()> {
        let CssColorFields {
            hash_token,
            value_token,
        } = node.as_fields();

        write!(
            f,
            [
                hash_token.format(),
                value_token.format()?.with_text_case(CssCase::Lowercase)
            ]
        )
    }
}
