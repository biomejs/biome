use crate::lint::nursery::use_index_of::{
    find_index_comparable_expression::find_index_comparable_expression, types::JsSyntaxMatchPair,
};
use biome_js_syntax::{AnyJsArrowFunctionParameters, JsArrowFunctionExpression, JsSyntaxToken};
use biome_rowan::{AstNode, AstSeparatedList};

fn extract_parameter_name(parameters: &AnyJsArrowFunctionParameters) -> Option<String> {
    if parameters.len() != 1 {
        return None;
    }

    match parameters {
        AnyJsArrowFunctionParameters::AnyJsBinding(binding) => Some(binding.to_trimmed_string()),
        AnyJsArrowFunctionParameters::JsParameters(param) => param
            .items()
            .first()?
            .ok()
            .map(|item| item.to_trimmed_string()),
    }
}

pub fn callback_arrow_function_match(
    function: &JsArrowFunctionExpression,
    member_name_token: JsSyntaxToken,
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
        member_name: member_name_token,
    })
}
