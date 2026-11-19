use crate::frameworks::{framework_api_name, is_framework_api_reference};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, JsCallExpression, static_value::StaticValue,
};
use biome_rowan::AstNode;

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

/// Checks if the given expression is a reference to a Vue API function (e.g., from `vue` package or `Vue` global).
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

/// Checks if the given expression is a reference to Vue's `nextTick` function: `nextTick`
/// imported from `vue` (also when renamed), `Vue.nextTick`, or `this.$nextTick`.
pub fn is_vue_next_tick_reference(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    is_vue_api_reference(expression, model, "nextTick")
        || expression
            .inner_expression()
            .is_some_and(|expression| is_vue_instance_next_tick(&expression))
}

fn is_vue_instance_next_tick(expression: &AnyJsExpression) -> bool {
    let Some(member_expression) = AnyJsMemberExpression::cast_ref(expression.syntax()) else {
        return false;
    };
    member_expression
        .member_name()
        .is_some_and(|name| name.text() == "$nextTick")
        && member_expression.object().is_ok_and(|object| {
            matches!(
                object.omit_parentheses(),
                AnyJsExpression::JsThisExpression(_)
            )
        })
}

/// Returns the name of the Vue API function that `expression` refers to, such as `onMounted` for
/// `onMounted` imported from `vue` (also when renamed) or for `Vue.onMounted`.
pub fn vue_api_name(expression: &AnyJsExpression, model: &SemanticModel) -> Option<StaticValue> {
    framework_api_name(expression, model, VUE_PACKAGE_NAMES, VUE_GLOBAL_NAME)
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
    "withDefaults",
];
