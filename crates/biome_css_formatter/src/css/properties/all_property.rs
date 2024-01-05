use crate::prelude::*;
use biome_css_syntax::{CssAllProperty, CssAllPropertyFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAllProperty;
impl FormatNodeRule<CssAllProperty> for FormatCssAllProperty {
    fn fmt_fields(&self, node: &CssAllProperty, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAllPropertyFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [name.format(), colon_token.format(), space(), value.format()]
        )
    }
}
