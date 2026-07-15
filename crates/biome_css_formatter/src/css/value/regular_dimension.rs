use crate::{prelude::*, utils::string_utils::FormatDimensionUnit};
use biome_css_syntax::{
    CssGenericProperty, CssRegularDimension, CssRegularDimensionFields, ScssInterpolatedIdentifier,
};
use biome_formatter::{token::number::NumberFormatOptions, write};
use biome_rowan::AstNode as _;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularDimension;
impl FormatNodeRule<CssRegularDimension> for FormatCssRegularDimension {
    fn fmt_fields(&self, node: &CssRegularDimension, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRegularDimensionFields {
            value_token,
            unit_token,
        } = node.as_fields();
        let unit_token = unit_token?;
        let unit = if is_in_interpolated_property_name(node) {
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

/// Matches units in interpolated property names such as `#{foo-#{$size + 1PX}}`.
fn is_in_interpolated_property_name(node: &CssRegularDimension) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .filter_map(ScssInterpolatedIdentifier::cast)
        .any(|identifier| identifier.parent::<CssGenericProperty>().is_some())
}
