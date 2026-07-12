use crate::{
    prelude::*,
    utils::{
        case::should_preserve_interpolated_property_dimension_unit_case,
        string_utils::FormatDimensionUnit,
    },
};
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

        let unit_token = unit_token?;
        let unit = if should_preserve_interpolated_property_dimension_unit_case(node.syntax()) {
            FormatDimensionUnit::preserve_source_case(unit_token)
        } else {
            FormatDimensionUnit::from(unit_token)
        };

        let var_name = write!(
            f,
            [
                value_token.format()?.with_text_case(CssCase::Lowercase),
                unit,
            ]
        );
        var_name
    }
}
