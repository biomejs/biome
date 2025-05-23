use crate::lint::nursery::use_index_of::{
    find_index_comparable_expression::find_index_comparable_expression, types::JsSyntaxMatchPair,
};
use biome_rowan::{AstNode, AstSeparatedList};

use biome_js_syntax::{AnyJsFunctionBody, JsFunctionExpression, JsParameterList, JsSyntaxToken};

fn extract_function_parameter_name(parameters: &JsParameterList) -> Option<String> {
    if parameters.len() != 1 {
        return None;
    }

    Some(parameters.first().unwrap().unwrap().to_trimmed_string())
}

pub fn callback_function_match(
    function: &JsFunctionExpression,
    member_name_token: &JsSyntaxToken,
) -> Option<JsSyntaxMatchPair> {
    if function.async_token().is_some() || function.star_token().is_some() {
        return None;
    }

    let function_parameters = function.parameters().unwrap().items();
    let parameter_name = extract_function_parameter_name(&function_parameters)?;
    let binding = function.body().ok()?;
    let body = binding
        .syntax()
        .descendants()
        .find_map(AnyJsFunctionBody::cast)?;

    let matched = find_index_comparable_expression(&body, &parameter_name, true);

    matched.as_ref().map(|token_match| JsSyntaxMatchPair {
        matching_array_element: token_match.clone(),
        member_name: member_name_token.clone(),
    })
}
