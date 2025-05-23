use biome_js_syntax::AnyJsArrowFunctionParameters;
use biome_rowan::{AstNode, AstSeparatedList};

pub fn extract_parameter_name(parameters: &AnyJsArrowFunctionParameters) -> Option<String> {
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
