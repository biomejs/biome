use crate::{prelude::*, utils::string_utils::FormatTokenAsLowercase};
use biome_css_syntax::{CssUnknownDimension, CssUnknownDimensionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownDimension;
impl FormatNodeRule<CssUnknownDimension> for FormatCssUnknownDimension {
    fn fmt_fields(&self, node: &CssUnknownDimension, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnknownDimensionFields {
            value_token,
            unit_token,
        } = node.as_fields();

        let var_name = write!(
            f,
            [
                FormatTokenAsLowercase::from(value_token?),
                FormatTokenAsLowercase::from(unit_token?),
            ]
        );
        var_name
    }
}
