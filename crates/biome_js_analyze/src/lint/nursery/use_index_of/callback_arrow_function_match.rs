use crate::lint::nursery::use_index_of::{
    extract_parameter_name::extract_parameter_name,
    find_index_comparable_expression::find_index_comparable_expression, types::JsSyntaxMatchPair,
};
use biome_js_syntax::{JsArrowFunctionExpression, JsSyntaxToken};

pub fn callback_arrow_function_match(
    function: &JsArrowFunctionExpression,
    member_name_token: &JsSyntaxToken,
) -> Option<JsSyntaxMatchPair> {
    if function.async_token().is_some() {
        return None;
    }

    let parameters = function.parameters().ok()?;
    let parameter_name = extract_parameter_name(&parameters)?;
    let body = function.body().ok()?;

    let matched = find_index_comparable_expression(&body, &parameter_name, false);

    matched.as_ref().map(|token_match| JsSyntaxMatchPair {
        matching_array_element: token_match.clone(),
        member_name: member_name_token.clone(),
    })
}
