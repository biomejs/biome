use crate::frameworks::is_framework_api_reference;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsExpression, JsCallExpression};

/// Given a Vue compiler macro, checks if the given call expression is a call to that macro.
pub fn is_vue_compiler_macro_call(
    call: &JsCallExpression,
    model: &SemanticModel,
    macro_name: &str,
) -> bool {
    // we bail straight away if the macro doesn't exist in Vue
    debug_assert!(VUE_COMPILER_MACROS.contains(&macro_name));
    let Some(callee) = call
        .callee()
        .ok()
        .and_then(|callee| callee.inner_expression())
    else {
        return false;
    };
    let Some(reference) = callee.as_js_reference_identifier() else {
        return false;
    };

    if let Ok(value_token) = reference.value_token() {
        if value_token.text_trimmed() != macro_name {
            return false;
        }
    } else {
        // If the reference doesn't have a value token, we can't match it against the macro name
        return false;
    }

    model.binding(&reference).is_none()
}

pub fn is_vue_api_reference(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    api_name: &str,
) -> bool {
    is_framework_api_reference(
        expression,
        model,
        api_name,
        VUE_PACKAGE_NAMES,
        VUE_GLOBAL_NAME,
    )
}

const VUE_PACKAGE_NAMES: &[&str] = &["vue"];
const VUE_GLOBAL_NAME: Option<&str> = Some("Vue");

const VUE_COMPILER_MACROS: &[&str] = &[
    "defineEmits",
    "defineExpose",
    "defineModel",
    "defineOptions",
    "defineProps",
    "defineSlots",
];
