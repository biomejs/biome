use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, AnyJsMemberExpression,
    JsAssignmentExpression, JsCallExpression, JsSyntaxToken, TextRange,
};
use biome_rowan::{AstNode, AstSeparatedList, declare_node_union};
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
        version: "next",
        name: "noExtendNative",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-extend-native").same()],
    }
}

const NATIVE_BUILTINS: &[&str] = &[
    "Object", "Function", "Array", "Number", "Boolean", "String", "Symbol", "BigInt",
    "Date", "RegExp", "Error", "EvalError", "RangeError", "ReferenceError", "SyntaxError",
    "TypeError", "URIError", "AggregateError", "Map", "Set", "WeakMap", "WeakSet", "Promise",
    "ArrayBuffer", "SharedArrayBuffer", "DataView", "Int8Array", "Uint8Array",
    "Uint8ClampedArray", "Int16Array", "Uint16Array", "Int32Array", "Uint32Array",
    "Float32Array", "Float64Array", "BigInt64Array", "BigUint64Array",
];

declare_node_union! {
    pub AnyExtendNativeCandidate = JsAssignmentExpression | JsCallExpression
}

pub struct NoExtendNativeState {
    range: TextRange,
    builtin_name: Box<str>,
}

impl Rule for NoExtendNative {
    type Query = Ast<AnyExtendNativeCandidate>;
    type State = NoExtendNativeState;
    type Signals = Option<Self::State>;
    type Options = NoExtendNativeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyExtendNativeCandidate::JsAssignmentExpression(assignment) => {
                let AnyJsAssignmentPattern::AnyJsAssignment(target) = assignment.left().ok()? else {
                    return None;
                };
                let object = match target {
                    AnyJsAssignment::JsStaticMemberAssignment(m) => m.object().ok()?,
                    AnyJsAssignment::JsComputedMemberAssignment(m) => m.object().ok()?,
                    _ => return None,
                };
                let builtin_name = is_native_prototype(&object)?;
                Some(NoExtendNativeState { range: assignment.range(), builtin_name })
            }
            AnyExtendNativeCandidate::JsCallExpression(call) => {
                let callee = call.callee().ok()?;
                let member_expr = callee.as_js_static_member_expression()?;
                if identifier_name(&member_expr.object().ok()?)?.text_trimmed() != "Object" {
                    return None;
                }
                let method = member_expr.member().ok()?;
                let method = method.as_js_name()?.value_token().ok()?;
                if !matches!(method.text_trimmed(), "defineProperty" | "defineProperties") {
                    return None;
                }
                let first_arg = call.arguments().ok()?.args().iter().next()?.ok()?;
                let target = first_arg.as_any_js_expression()?;
                let builtin_name = is_native_prototype(target)?;
                Some(NoExtendNativeState { range: call.range(), builtin_name })
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Avoid extending the prototype of '" {&state.builtin_name} "'."
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

/// Returns the built-in name if `expr` is `<Builtin>.prototype`.
/// Handles both static (`Array.prototype`) and computed (`Array["prototype"]`) access.
fn is_native_prototype(expr: &AnyJsExpression) -> Option<Box<str>> {
    let member_expr = AnyJsMemberExpression::cast_ref(expr.syntax())?;
    let (object_expr, is_prototype) = match &member_expr {
        AnyJsMemberExpression::JsStaticMemberExpression(static_member) => {
            let member = static_member.member().ok()?;
            let name = member.as_js_name()?;
            let is_proto = name.value_token().ok()?.text_trimmed() == "prototype";
            (static_member.object().ok()?, is_proto)
        }
        AnyJsMemberExpression::JsComputedMemberExpression(computed_member) => {
            let member = computed_member.member().ok()?;
            let is_proto = member
                .as_static_value()
                .as_ref()
                .and_then(|v| v.as_string_constant())
                .is_some_and(|s| s == "prototype");
            (computed_member.object().ok()?, is_proto)
        }
    };
    if !is_prototype {
        return None;
    }
    let name_token = identifier_name(&object_expr)?;
    let name = name_token.text_trimmed();
    if NATIVE_BUILTINS.contains(&name) {
        Some(Box::from(name))
    } else {
        None
    }
}

fn identifier_name(expr: &AnyJsExpression) -> Option<JsSyntaxToken> {
    expr.as_js_identifier_expression()?
        .name()
        .ok()?
        .value_token()
        .ok()
}
