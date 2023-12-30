use crate::{prelude::*, utils::string_utils::FormatTokenAsLowercase};
use biome_css_syntax::{CssRegularDimension, CssRegularDimensionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularDimension;
impl FormatNodeRule<CssRegularDimension> for FormatCssRegularDimension {
    fn fmt_fields(&self, node: &CssRegularDimension, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRegularDimensionFields {
            value_token,
            unit_token,
        } = node.as_fields();

        write!(
            f,
            [
                FormatTokenAsLowercase::from(value_token?),
                FormatTokenAsLowercase::from(unit_token?),
            ]
        )
    }
}
