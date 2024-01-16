use crate::prelude::*;
use biome_css_syntax::{CssBorderProperty, CssBorderPropertyFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBorderProperty;
impl FormatNodeRule<CssBorderProperty> for FormatCssBorderProperty {
    fn fmt_fields(&self, node: &CssBorderProperty, f: &mut CssFormatter) -> FormatResult<()> {
        let CssBorderPropertyFields {
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
