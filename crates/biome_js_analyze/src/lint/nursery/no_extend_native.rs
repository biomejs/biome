use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, JsAssignmentExpression,
    JsCallExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::no_extend_native::NoExtendNativeOptions;

declare_lint_rule! {
    /// Disallow extending the prototypes of built-in objects.
    ///
    /// Adding properties to the prototype of a built-in such as `Object`,
    /// `Array`, or `Error` leaks into every value of that type. The new
    /// property shows up in every `for...in`, collides with other libraries
    /// that patch the same prototype, and breaks assumptions across the whole
    /// program. Extend a subclass or use a standalone helper instead.
    ///
    /// This rule flags a direct prototype assignment
    /// (`Builtin.prototype.x = …`) and `Object.defineProperty` /
    /// `Object.defineProperties` targeting a built-in prototype.
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

/// Built-in constructors whose prototypes should not be extended.
const NATIVE_BUILTINS: &[&str] = &[
    "Object",
    "Function",
    "Array",
    "Number",
    "Boolean",
    "String",
    "Symbol",
    "BigInt",
    "Date",
    "RegExp",
    "Error",
    "EvalError",
    "RangeError",
    "ReferenceError",
    "SyntaxError",
    "TypeError",
    "URIError",
    "AggregateError",
    "Map",
    "Set",
    "WeakMap",
    "WeakSet",
    "Promise",
    "ArrayBuffer",
    "SharedArrayBuffer",
    "DataView",
    "Int8Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "Int16Array",
    "Uint16Array",
    "Int32Array",
    "Uint32Array",
    "Float32Array",
    "Float64Array",
    "BigInt64Array",
    "BigUint64Array",
];

declare_node_union! {
    pub AnyExtendNativeCandidate = JsAssignmentExpression | JsCallExpression
}

impl Rule for NoExtendNative {
    type Query = Ast<AnyExtendNativeCandidate>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoExtendNativeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            // `Builtin.prototype.x = …` (or `Builtin.prototype["x"] = …`)
            AnyExtendNativeCandidate::JsAssignmentExpression(assignment) => {
                let AnyJsAssignmentPattern::AnyJsAssignment(target) = assignment.left().ok()?
                else {
                    return None;
                };
                let object = match target {
                    AnyJsAssignment::JsStaticMemberAssignment(member) => member.object().ok()?,
                    AnyJsAssignment::JsComputedMemberAssignment(member) => member.object().ok()?,
                    _ => return None,
                };
                is_native_prototype(&object).then(|| assignment.range())
            }
            // `Object.defineProperty(Builtin.prototype, …)` / `defineProperties`
            AnyExtendNativeCandidate::JsCallExpression(call) => {
                let callee = call.callee().ok()?;
                let member_expr = callee.as_js_static_member_expression()?;

                // Callee object must be the global `Object`.
                if identifier_name(&member_expr.object().ok()?)? != "Object" {
                    return None;
                }
                // Method must be defineProperty / defineProperties.
                let method = member_expr.member().ok()?;
                let method = method.as_js_name()?.value_token().ok()?;
                if !matches!(method.text_trimmed(), "defineProperty" | "defineProperties") {
                    return None;
                }

                // First argument must be a built-in prototype.
                let first_arg = call.arguments().ok()?.args().iter().next()?.ok()?;
                let target = first_arg.as_any_js_expression()?;
                is_native_prototype(target).then(|| call.range())
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Avoid extending the prototype of a built-in object."
                },
            )
            .note(markup! {
                "Patching a native prototype leaks the property into every value of that type and can collide with other code. Use a subclass or a standalone helper instead."
            }),
        )
    }
}

/// Returns `true` when `expr` is `<Builtin>.prototype` for a known native
/// constructor.
fn is_native_prototype(expr: &AnyJsExpression) -> bool {
    let Some(member_expr) = expr.as_js_static_member_expression() else {
        return false;
    };
    // The accessed member must be `prototype`.
    let is_prototype = member_expr
        .member()
        .ok()
        .and_then(|member| member.as_js_name().cloned())
        .and_then(|name| name.value_token().ok())
        .is_some_and(|token| token.text_trimmed() == "prototype");
    if !is_prototype {
        return false;
    }
    // The object must be one of the native built-ins.
    member_expr
        .object()
        .ok()
        .as_ref()
        .and_then(identifier_name)
        .is_some_and(|name| NATIVE_BUILTINS.contains(&name.as_str()))
}

/// Extracts the identifier name from a plain identifier expression.
fn identifier_name(expr: &AnyJsExpression) -> Option<String> {
    Some(
        expr.as_js_identifier_expression()?
            .name()
            .ok()?
            .value_token()
            .ok()?
            .text_trimmed()
            .to_string(),
    )
}
