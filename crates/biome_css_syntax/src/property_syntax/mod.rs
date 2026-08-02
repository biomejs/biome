mod data;
mod matcher;
mod parser;
mod serializer;

pub use data::{
    PropertySyntax, PropertySyntaxComponent, PropertySyntaxComponentName, PropertySyntaxErrorKind,
    PropertySyntaxMultiplier, PropertySyntaxParseDiagnostic, PropertySyntaxResult,
    PropertySyntaxType,
};
pub use parser::encode;
pub use serializer::decode;

use crate::{CssDashedIdentifier, CssFunction, decode_css_identifier};
use biome_rowan::{AstNodeList, AstSeparatedList};

/// Returns the custom-property name from a well-formed `var()` function.
pub fn custom_property_name_from_var_function(
    function: &CssFunction,
) -> Option<CssDashedIdentifier> {
    let name = function
        .name()
        .ok()?
        .as_css_identifier()?
        .value_token()
        .ok()?;
    if !decode_css_identifier(name.text_trimmed()).eq_ignore_ascii_case("var") {
        return None;
    }

    function
        .items()
        .iter()
        .next()?
        .ok()?
        .as_css_list_of_component_values_expression()?
        .css_component_value_list()
        .iter()
        .next()?
        .as_any_css_dashed_identifier()?
        .as_css_dashed_identifier()
        .cloned()
}
