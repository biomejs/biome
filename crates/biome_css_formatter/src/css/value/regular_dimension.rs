use crate::{
    prelude::*,
    utils::{
        case::should_preserve_interpolated_property_dimension_unit_case,
        string_utils::FormatDimensionUnit,
    },
};
use biome_css_syntax::{CssRegularDimension, CssRegularDimensionFields};
use biome_formatter::{token::number::NumberFormatOptions, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularDimension;
impl FormatNodeRule<CssRegularDimension> for FormatCssRegularDimension {
    fn fmt_fields(&self, node: &CssRegularDimension, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRegularDimensionFields {
            value_token,
            unit_token,
        } = node.as_fields();

        let unit_token = unit_token?;
        let unit = if should_preserve_interpolated_property_dimension_unit_case(node.syntax()) {
            FormatDimensionUnit::preserve_source_case(unit_token)
        } else {
            FormatDimensionUnit::from(unit_token)
        };

        write!(
            f,
            [
                format_number_token(&value_token?, NumberFormatOptions::default()),
                unit,
            ]
        )
    }
}
