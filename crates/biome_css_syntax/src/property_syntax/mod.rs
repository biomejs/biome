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

use crate::{AnyCssValue, CssFunction, CssSyntaxToken, decode_css_identifier};
use biome_rowan::{AstNodeList, AstSeparatedList};

/// Returns the custom-property name from a well-formed `var()` function.
pub fn custom_property_name_from_var_function(function: &CssFunction) -> Option<CssSyntaxToken> {
    let name = function
        .name()
        .ok()?
        .as_css_identifier()?
        .value_token()
        .ok()?;
    if !decode_css_identifier(name.text_trimmed()).eq_ignore_ascii_case("var") {
        return None;
    }

    let value = function
        .items()
        .iter()
        .next()?
        .ok()?
        .as_css_list_of_component_values_expression()?
        .css_component_value_list()
        .iter()
        .next()?;
    let token = match value {
        AnyCssValue::AnyCssDashedIdentifier(identifier) => {
            identifier.as_css_dashed_identifier()?.value_token().ok()?
        }
        AnyCssValue::CssCustomIdentifier(identifier) => identifier.value_token().ok()?,
        AnyCssValue::CssIdentifier(identifier) => identifier.value_token().ok()?,
        _ => return None,
    };
    decode_css_identifier(token.text_trimmed())
        .starts_with("--")
        .then_some(token)
}
