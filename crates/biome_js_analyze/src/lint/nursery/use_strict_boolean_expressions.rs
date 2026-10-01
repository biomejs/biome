use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunction, AnyJsMemberExpression, AnyJsOptionalChainExpression,
    JsCallExpression, JsConditionalExpression, JsDoWhileStatement, JsForStatement, JsIfStatement,
    JsLogicalExpression, JsLogicalOperator, JsUnaryExpression, JsUnaryOperator, JsWhileStatement,
};
use biome_js_type_info::BooleanCoercion;
use biome_rowan::{AstNode, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::use_strict_boolean_expressions::UseStrictBooleanExpressionsOptions;

use crate::services::typed::Typed;

declare_lint_rule! {
    /// Require unambiguous boolean expressions in conditions.
    ///
    /// Truthiness checks on nullable primitives can confuse missing values with
    /// `false`, an empty string, zero, or `NaN`. Check for nullish values explicitly
    /// or convert the value with `Boolean()` when that distinction is intentional.
    ///
    /// This rule allows booleans, non-nullable strings and numbers, and nullable
    /// objects, functions, and symbols. Nullable `true`, nonempty string literal
    /// types, and nonzero number literal types are also allowed. Numbers include
    /// bigints, but nullable bigints are rejected.
    ///
    /// It checks conditions, logical negation, the operands of `&&` and `||`, and
    /// array predicate callbacks and truthiness assertion arguments. The last operand of a logical expression is
    /// checked only when its result is used as a condition.
    ///
    /// Types that cannot be inferred are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=nullable-number.ts
    /// function display(count: number | undefined) {
    ///     if (count) {}
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=nullable-boolean.ts
    /// function run(enabled?: boolean) {
    ///     if (enabled) {}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,file=object-condition.js
    /// if ({}) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function run(maybeCount: number | undefined, count: number, text: string, object: object | null, enabled?: boolean) {
    ///     if (maybeCount != null) {}
    ///     if (enabled === true) {}
    ///     if (text) {}
    ///     if (count) {}
    ///     if (object) {}
    /// }
    /// ```
    pub UseStrictBooleanExpressions {
        version: "2.5.15",
        name: "useStrictBooleanExpressions",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("strict-boolean-expressions").inspired()],
        domains: &[RuleDomain::Types],
    }
}

declare_node_union! {
    /// Nodes that coerce one of their operands to a boolean.
    pub AnyBooleanContext =
        JsIfStatement
        | JsWhileStatement
        | JsDoWhileStatement
        | JsForStatement
        | JsConditionalExpression
        | JsUnaryExpression
        | JsLogicalExpression
        | JsCallExpression
}

pub struct UseStrictBooleanExpressionsState {
    range: TextRange,
    coercion: BooleanCoercion,
    /// Whether the expression is an array predicate callback, in which case
    /// `coercion` describes its return type.
    predicate: bool,
}

impl Rule for UseStrictBooleanExpressions {
    type Query = Typed<AnyBooleanContext>;
    type State = UseStrictBooleanExpressionsState;
    type Signals = Box<[Self::State]>;
    type Options = UseStrictBooleanExpressionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut signals = Vec::new();
        let condition = match ctx.query() {
            AnyBooleanContext::JsIfStatement(statement) => statement.test().ok(),
            AnyBooleanContext::JsWhileStatement(statement) => statement.test().ok(),
            AnyBooleanContext::JsDoWhileStatement(statement) => statement.test().ok(),
            AnyBooleanContext::JsForStatement(statement) => statement.test(),
            AnyBooleanContext::JsConditionalExpression(expression) => expression.test().ok(),
            AnyBooleanContext::JsUnaryExpression(expression) => expression
                .operator()
                .is_ok_and(|operator| operator == JsUnaryOperator::LogicalNot)
                .then(|| expression.argument().ok())
                .flatten(),
            AnyBooleanContext::JsLogicalExpression(expression) => {
                // Only the left operand is always coerced. The right operand is
                // coerced only when the whole expression is, so it is checked
                // by the enclosing context.
                if expression.operator().ok() == Some(JsLogicalOperator::NullishCoalescing) {
                    None
                } else {
                    expression.left().ok()
                }
            }
            AnyBooleanContext::JsCallExpression(call) => {
                let asserted = asserted_argument(ctx, call);
                if let Some(callback) = array_predicate(ctx, call)
                    && asserted.as_ref() != Some(&callback)
                {
                    signals.extend(check_predicate(ctx, &callback));
                }
                asserted
            }
        };
        if let Some(condition) = condition {
            check_condition(ctx, condition, &mut signals);
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (description, reason, advice) = match state.coercion {
            BooleanCoercion::Safe => return None,
            BooleanCoercion::NullableBoolean => (
                "a nullable boolean",
                markup! { "This check treats "<Emphasis>"false"</Emphasis>" and nullish values alike." },
                markup! { "Compare with "<Emphasis>"true"</Emphasis>" or provide an explicit default with "<Emphasis>"?? false"</Emphasis>"." },
            ),
            BooleanCoercion::NullableString => (
                "a nullable string",
                markup! { "This check treats "<Emphasis>"empty strings"</Emphasis>" and nullish values alike." },
                markup! { "Check for nullish values and empty strings explicitly, or use "<Emphasis>"Boolean()"</Emphasis>" to test truthiness." },
            ),
            BooleanCoercion::NullableNumber => (
                "a nullable number",
                markup! { "This check treats "<Emphasis>"zero"</Emphasis>", "<Emphasis>"NaN"</Emphasis>", and nullish values alike." },
                markup! { "Check for nullish values and falsy numbers explicitly, or use "<Emphasis>"Boolean()"</Emphasis>" to test truthiness." },
            ),
            BooleanCoercion::AlwaysTruthy => (
                "an always-truthy value",
                markup! { "Objects, functions, and symbols are "<Emphasis>"always truthy"</Emphasis>"." },
                markup! { "Check a property of the value or remove the condition." },
            ),
            BooleanCoercion::AlwaysNullish => (
                "an always-nullish value",
                markup! { "Nullish values are "<Emphasis>"always falsy"</Emphasis>"." },
                markup! { "Remove the condition or check a different value." },
            ),
            BooleanCoercion::Unrestricted => (
                "an unrestricted type",
                markup! { "This type can contain both truthy and falsy values of different kinds." },
                markup! { "Narrow the type before checking it, or use "<Emphasis>"Boolean()"</Emphasis>" to test truthiness." },
            ),
            BooleanCoercion::Mixed => (
                "a union of different types",
                markup! { "This check can conflate falsy values of different types." },
                markup! { "Handle each type explicitly, or use "<Emphasis>"Boolean()"</Emphasis>" to test truthiness." },
            ),
        };
        let context = if state.predicate {
            "This array predicate returns "
        } else {
            "This condition uses "
        };
        let advice = if state.predicate
            && matches!(
                state.coercion,
                BooleanCoercion::AlwaysTruthy | BooleanCoercion::AlwaysNullish
            ) {
            markup! { "Return a "<Emphasis>"boolean"</Emphasis>" that expresses the intended condition." }
        } else {
            advice
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! { {context}<Emphasis>{description}</Emphasis>"." },
            )
            .note(reason)
            .note(advice),
        )
    }
}

/// Reports `condition` if coercing it to a boolean is ambiguous.
///
/// When `condition` is a `&&` or `||` expression, its right operand is checked
/// instead, because the expression's own query checks its left operand.
fn check_condition(
    ctx: &RuleContext<UseStrictBooleanExpressions>,
    condition: AnyJsExpression,
    signals: &mut Vec<UseStrictBooleanExpressionsState>,
) {
    let condition = condition.omit_parentheses();
    if let AnyJsExpression::JsLogicalExpression(logical) = &condition
        && logical.operator().ok() != Some(JsLogicalOperator::NullishCoalescing)
    {
        if let Ok(right) = logical.right() {
            check_condition(ctx, right, signals);
        }
        return;
    }
    let Some(coercion) = ctx
        .type_of_expression(&condition)
        .and_then(|ty| ty.boolean_coercion().ok())
    else {
        return;
    };
    if coercion != BooleanCoercion::Safe {
        signals.push(UseStrictBooleanExpressionsState {
            range: condition.range(),
            coercion,
            predicate: false,
        });
    }
}

/// Returns a signal if the return type of the array predicate `callback` is
/// ambiguous when coerced to a boolean.
fn check_predicate(
    ctx: &RuleContext<UseStrictBooleanExpressions>,
    callback: &AnyJsExpression,
) -> Option<UseStrictBooleanExpressionsState> {
    let callback = callback.clone().omit_parentheses();
    let return_type = match AnyJsFunction::cast(callback.syntax().clone()) {
        Some(function) => ctx
            .return_type_of_function(&function)
            .or_else(|| ctx.type_of_expression(&callback)?.callable_return_type())?,
        None => ctx.type_of_expression(&callback)?.callable_return_type()?,
    };
    let coercion = return_type.boolean_coercion().ok()?;
    (coercion != BooleanCoercion::Safe).then(|| UseStrictBooleanExpressionsState {
        range: callback.range(),
        coercion,
        predicate: true,
    })
}

const ARRAY_PREDICATE_METHODS: [&str; 7] = [
    "every",
    "filter",
    "find",
    "findIndex",
    "findLast",
    "findLastIndex",
    "some",
];

/// Returns the callback passed to `call` if `call` is an array method that
/// coerces the callback's return value to a boolean, such as `filter` or `some`.
///
/// The callback is the first argument. The method name must be in
/// [`ARRAY_PREDICATE_METHODS`], and the receiver must be inferred as an array or
/// tuple. In an optional chain such as `items?.filter(...)`, the receiver may
/// also be nullish, because the call is skipped when it is. Receivers with an
/// unresolved type return `None`, so methods with the same name on other objects
/// are not matched.
///
/// In the following example, only the first call has an array predicate:
///
/// ```ts
/// declare const items: string[];
/// declare const set: { filter(cb: (value: string) => unknown): void };
/// items.filter((item) => item); // array predicate
/// set.filter((item) => item);   // not an array predicate
/// ```
fn array_predicate(
    ctx: &RuleContext<UseStrictBooleanExpressions>,
    call: &JsCallExpression,
) -> Option<AnyJsExpression> {
    let callee = call.callee().ok()?.omit_parentheses();
    let member = AnyJsMemberExpression::cast(callee.into_syntax())?;
    let name = member.member_name()?;
    ARRAY_PREDICATE_METHODS.binary_search(&name.text()).ok()?;
    let Ok(AnyJsCallArgument::AnyJsExpression(callback)) =
        call.arguments().ok()?.args().iter().next()?
    else {
        return None;
    };
    let receiver = ctx.type_of_expression(&member.object().ok()?)?;
    let is_array = if AnyJsOptionalChainExpression::cast_ref(member.syntax())
        .is_some_and(|member| member.is_optional_chain())
    {
        receiver.is_nullable_array_or_tuple()
    } else {
        receiver.is_array_or_tuple()
    };
    is_array.then_some(callback)
}

/// Returns the argument of `call` that the called function asserts to be truthy,
/// such as `value` in a call to a function declared with `asserts value`.
///
/// Type predicates such as `asserts value is string` narrow a type but do not
/// assert truthiness, so they return `None`.
///
/// If a spread argument or a missing argument appears at or before the asserted
/// position, argument positions no longer map to parameters, and the function
/// returns `None`.
///
/// In the following example, `flag` is the asserted argument:
///
/// ```ts
/// declare function assert(value: unknown, message?: string): asserts value;
/// declare const flag: boolean | undefined;
/// declare const message: string | undefined;
/// assert(flag, message);
/// ```
fn asserted_argument(
    ctx: &RuleContext<UseStrictBooleanExpressions>,
    call: &JsCallExpression,
) -> Option<AnyJsExpression> {
    let arguments = call.arguments().ok()?.args();
    if arguments.is_empty() {
        return None;
    }
    let index = ctx
        .type_of_expression(&call.callee().ok()?)?
        .truthiness_asserted_argument()?;
    for (argument_index, argument) in arguments.iter().enumerate() {
        let Ok(AnyJsCallArgument::AnyJsExpression(argument)) = argument else {
            return None;
        };
        if argument_index == index {
            return Some(argument);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_predicate_methods_are_sorted() {
        for items in ARRAY_PREDICATE_METHODS.windows(2) {
            assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
        }
    }
}
