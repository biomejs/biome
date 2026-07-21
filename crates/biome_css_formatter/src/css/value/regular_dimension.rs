use crate::{
    prelude::*,
    utils::{case::is_supports_test_declaration, string_utils::FormatDimensionUnit},
};
use biome_css_syntax::{
    AnyCssDeclarationName, CssDeclaration, CssGenericProperty, CssRegularDimension,
    CssRegularDimensionFields,
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

/// Matches units in interpolated declaration names such as `--foo-#{$size + 1PX}`.
fn is_in_interpolated_property_name(node: &CssRegularDimension) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .filter_map(AnyCssDeclarationName::cast)
        .find_map(|name| name.parent::<CssGenericProperty>())
        .and_then(|property| property.parent::<CssDeclaration>())
        .is_some_and(|declaration| !is_supports_test_declaration(&declaration))
}
