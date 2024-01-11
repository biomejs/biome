use crate::prelude::*;
use biome_css_syntax::{CssZIndexProperty, CssZIndexPropertyFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssZIndexProperty;
impl FormatNodeRule<CssZIndexProperty> for FormatCssZIndexProperty {
    fn fmt_fields(&self, node: &CssZIndexProperty, f: &mut CssFormatter) -> FormatResult<()> {
        let CssZIndexPropertyFields {
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
