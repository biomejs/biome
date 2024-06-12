use crate::prelude::*;
use biome_css_syntax::{CssComposesProperty, CssComposesPropertyFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComposesProperty;
impl FormatNodeRule<CssComposesProperty> for FormatCssComposesProperty {
    fn fmt_fields(&self, node: &CssComposesProperty, f: &mut CssFormatter) -> FormatResult<()> {
        let CssComposesPropertyFields {
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
