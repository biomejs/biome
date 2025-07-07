use crate::frameworks::is_framework_api_reference;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsExpression, JsCallExpression};

/// This function checks if a given call expression is a call to a Vue compiler macro.
/// Note that we are not using semantic model here because vue macro compiler doesn't check
/// if it's a global macro call or a locally defined function call.
/// So this is a working code in vue:
/// ```js
/// const definedProps = 123;
/// defineProps(['x', 'y']);
/// ```
pub fn is_vue_compiler_macro_call(call: &JsCallExpression, macro_name: &str) -> bool {
    // we bail straight away if the macro doesn't exist in Vue
    debug_assert!(VUE_COMPILER_MACROS.contains(&macro_name));
    let Some(AnyJsExpression::JsIdentifierExpression(callee_ident)) = call
        .callee()
        .ok()
        .and_then(|callee| callee.inner_expression())
    else {
        return false;
    };
    let Ok(name_token) = callee_ident.name().and_then(|name| name.value_token()) else {
        return false;
    };
    name_token.text_trimmed() == macro_name
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
