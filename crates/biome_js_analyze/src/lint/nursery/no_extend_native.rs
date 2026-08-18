use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, AnyJsMemberExpression,
    JsAssignmentExpression, JsCallExpression, JsReferenceIdentifier, TextRange,
};
use biome_rowan::{AstNode, AstSeparatedList, TokenText, declare_node_union};
use biome_rule_options::no_extend_native::NoExtendNativeOptions;

declare_lint_rule! {
    /// Disallow extending the prototype of built-in objects.
    ///
    /// Adding properties to the prototype of a built-in such as `Object`,
    /// `Array`, or `Error` leaks into every value of that type. The new
    /// property shows up in every `for...in`, collides with other libraries
    /// that patch the same prototype, and breaks assumptions across the whole
    /// program. Extend a subclass or use a standalone helper instead.
    ///
    /// This rule flags a direct prototype assignment
    /// (`Builtin.prototype.x = ...`), computed prototype access
    /// (`Builtin["prototype"].x = ...`), and
    /// `Object.defineProperty`/`Object.defineProperties` targeting a
    /// built-in prototype.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Object.prototype.extra = "a";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Array.prototype.times = function () {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.defineProperty(Array.prototype, "times", { value: 999 });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class CustomArray extends Array {}
    /// ```
    ///
    /// ```js
    /// const obj = {};
    /// obj.extra = "a";
    /// ```
    ///
    pub NoExtendNative {
        version: "2.5.7",
        name: "noExtendNative",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-extend-native").same()],
    }
}

declare_node_union! {
    pub AnyExtendNativeCandidate = JsAssignmentExpression | JsCallExpression
}

pub struct NoExtendNativeState {
    range: TextRange,
    builtin_name: TokenText,
}

impl Rule for NoExtendNative {
    type Query = Semantic<AnyExtendNativeCandidate>;
    type State = NoExtendNativeState;
    type Signals = Option<Self::State>;
    type Options = NoExtendNativeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyExtendNativeCandidate::JsAssignmentExpression(assignment) => {
                let AnyJsAssignmentPattern::AnyJsAssignment(target) = assignment.left().ok()?
                else {
                    return None;
                };
                let object = match target {
                    AnyJsAssignment::JsStaticMemberAssignment(m) => m.object().ok()?,
                    AnyJsAssignment::JsComputedMemberAssignment(m) => m.object().ok()?,
                    _ => return None,
                };
                let builtin_name = native_builtin_name(ctx, &object)?;
                Some(NoExtendNativeState {
                    range: assignment.range(),
                    builtin_name,
                })
            }
            AnyExtendNativeCandidate::JsCallExpression(call) => {
                let callee = call.callee().ok()?.omit_parentheses();
                let member_expr = AnyJsMemberExpression::cast_ref(callee.syntax())?;
                let member_token = member_expr.member_name()?;
                let method = member_token.text();
                if method != "defineProperty" && method != "defineProperties" {
                    return None;
                }
                let object_expr = member_expr.object().ok()?.omit_parentheses();
                let obj_ref = object_expr.as_js_reference_identifier()?;
                let obj_token = obj_ref.value_token().ok()?;
                if obj_token.text_trimmed() != "Object" {
                    return None;
                }
                // Reject shadowed `Object` identifiers
                if ctx.model().binding(&obj_ref).is_some() {
                    return None;
                }
                let first_arg = call.arguments().ok()?.args().iter().next()?.ok()?;
                let target = first_arg.as_any_js_expression()?;
                let builtin_name = native_builtin_name(ctx, target)?;
                Some(NoExtendNativeState {
                    range: call.range(),
                    builtin_name,
                })
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Avoid extending the prototype of '" {state.builtin_name.as_ref()} "'."
                },
            )
            .note(markup! {
                "Patching a native prototype leaks the property into every value of that type and can collide with other code."
            })
            .note(markup! {
                "Use a subclass or a standalone helper instead."
            }),
        )
    }
}

/// Checks if `expr` (after stripping parentheses) is `<Builtin>.prototype`
/// and returns the `JsReferenceIdentifier` for the builtin name.
fn is_native_prototype(expr: &AnyJsExpression) -> Option<JsReferenceIdentifier> {
    let expr = expr.clone().omit_parentheses();
    let member = AnyJsMemberExpression::cast_ref(expr.syntax())?;
    let member_token = member.member_name()?;
    let proto_name = member_token.text();
    if proto_name != "prototype" {
        return None;
    }
    let object = member.object().ok()?.omit_parentheses();
    object.as_js_reference_identifier()
}

/// Validates that `expr` is a `JsAssignmentExpression` or `JsCallExpression`.
/// Returns the builtin `TokenText` name if:
/// 1. The expression accesses `<Builtin>.prototype` for a known builtin.
/// 2. The builtin identifier is not shadowed by a local binding.
/// 3. The builtin name is not in the `ignore` options list.
fn native_builtin_name(
    ctx: &RuleContext<NoExtendNative>,
    expr: &AnyJsExpression,
) -> Option<TokenText> {
    let builtin_ref = is_native_prototype(expr)?;
    let name_token = builtin_ref.value_token().ok()?;
    let name = name_token.text_trimmed();
    if NATIVE_BUILTINS.binary_search(&name).is_err() {
        return None;
    }
    if ctx.model().binding(&builtin_ref).is_some() {
        return None;
    }
    if ctx
        .options()
        .ignore
        .iter()
        .flatten()
        .any(|n| n.as_ref() == name)
    {
        return None;
    }
    Some(name_token.token_text_trimmed())
}

// IMPORTANT: Keep this array sorted for binary search
const NATIVE_BUILTINS: &[&str] = &[
    "AggregateError",
    "Array",
    "ArrayBuffer",
    "BigInt",
    "BigInt64Array",
    "BigUint64Array",
    "Boolean",
    "DataView",
    "Date",
    "Error",
    "EvalError",
    "FinalizationRegistry",
    "Float32Array",
    "Float64Array",
    "Function",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "Map",
    "Number",
    "Object",
    "Promise",
    "RangeError",
    "ReferenceError",
    "RegExp",
    "Set",
    "SharedArrayBuffer",
    "String",
    "Symbol",
    "SyntaxError",
    "TypeError",
    "URIError",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "WeakMap",
    "WeakRef",
    "WeakSet",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_builtins_sorted() {
        assert!(NATIVE_BUILTINS.is_sorted());
    }
}
