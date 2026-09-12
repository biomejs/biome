use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunction, AnyJsMemberExpression, JsCallArgumentList,
    JsCallExpression, JsForStatement, JsLogicalExpression, JsLogicalOperator, JsSyntaxKind,
    JsSyntaxNode, JsUnaryExpression, JsUnaryOperator, is_in_boolean_context,
};
use biome_js_type_info::BooleanCoercion;
use biome_rowan::{AstNode, AstSeparatedList};
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
    /// ```ts,file=explicit-conditions.ts
    /// function run(count: number | undefined, enabled?: boolean) {
    ///     if (count != null) {}
    ///     if (enabled === true) {}
    /// }
    /// ```
    ///
    /// ```ts,file=allowed-types.ts
    /// function run(text: string, count: number, object: object | null) {
    ///     if (text) {}
    ///     if (count) {}
    ///     if (object) {}
    /// }
    /// ```
    pub UseStrictBooleanExpressions {
        version: "next",
        name: "useStrictBooleanExpressions",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("strict-boolean-expressions").inspired()],
        domains: &[RuleDomain::Types],
    }
}

pub struct UseStrictBooleanExpressionsState {
    coercion: BooleanCoercion,
    predicate: bool,
}

impl Rule for UseStrictBooleanExpressions {
    type Query = Typed<AnyJsExpression>;
    type State = UseStrictBooleanExpressionsState;
    type Signals = Option<Self::State>;
    type Options = UseStrictBooleanExpressionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query();
        if matches!(expression, AnyJsExpression::JsParenthesizedExpression(_)) {
            return None;
        }
        let predicate = !is_condition(ctx, expression.syntax());
        let coercion = if !predicate {
            if matches!(expression, AnyJsExpression::JsLogicalExpression(logical) if logical.operator().ok()? != JsLogicalOperator::NullishCoalescing)
            {
                return None;
            }
            ctx.type_of_expression(expression)?
                .boolean_coercion()
                .ok()?
        } else if is_array_predicate(ctx, expression) {
            if let Some(function) = AnyJsFunction::cast(expression.syntax().clone()) {
                ctx.return_type_of_function(&function)
                    .or_else(|| ctx.type_of_expression(expression)?.callable_return_type())?
                    .boolean_coercion()
                    .ok()?
            } else {
                ctx.type_of_expression(expression)?
                    .callable_return_type()?
                    .boolean_coercion()
                    .ok()?
            }
        } else {
            return None;
        };
        if coercion == BooleanCoercion::Safe {
            return None;
        }
        Some(UseStrictBooleanExpressionsState {
            coercion,
            predicate,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
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
                ctx.query().range(),
                markup! { {context}<Emphasis>{description}</Emphasis>"." },
            )
            .note(reason)
            .note(advice),
        )
    }
}

fn is_condition(ctx: &RuleContext<UseStrictBooleanExpressions>, node: &JsSyntaxNode) -> bool {
    let mut current = node.clone();
    for parent in node.ancestors().skip(1) {
        match parent.kind() {
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => current = parent,
            JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                let Some(logical) = JsLogicalExpression::cast(parent.clone()) else {
                    return false;
                };
                if logical.operator().ok() == Some(JsLogicalOperator::NullishCoalescing) {
                    return false;
                }
                if logical.left().is_ok_and(|left| left.syntax() == &current) {
                    return true;
                }
                current = parent;
            }
            JsSyntaxKind::JS_FOR_STATEMENT => {
                return JsForStatement::cast(parent)
                    .and_then(|statement| statement.test())
                    .is_some_and(|test| test.syntax() == &current);
            }
            JsSyntaxKind::JS_UNARY_EXPRESSION => {
                return JsUnaryExpression::cast(parent).is_some_and(|unary| {
                    unary.operator().ok() == Some(JsUnaryOperator::LogicalNot)
                });
            }
            JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
                return AnyJsExpression::cast(current)
                    .zip(JsCallArgumentList::cast(parent))
                    .is_some_and(|(argument, arguments)| {
                        is_asserted_argument(ctx, &argument, &arguments)
                    });
            }
            _ => return is_in_boolean_context(&current).unwrap_or(false),
        }
    }
    false
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

fn is_array_predicate(
    ctx: &RuleContext<UseStrictBooleanExpressions>,
    expression: &AnyJsExpression,
) -> bool {
    let argument_node = expression
        .syntax()
        .ancestors()
        .skip(1)
        .take_while(|parent| parent.kind() == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION)
        .last()
        .unwrap_or_else(|| expression.syntax().clone());
    let Some(arguments) = argument_node.parent().and_then(JsCallArgumentList::cast) else {
        return false;
    };
    if !arguments
        .iter()
        .next()
        .is_some_and(|argument| argument.is_ok_and(|argument| argument.syntax() == &argument_node))
    {
        return false;
    }
    let Some(call) = arguments
        .syntax()
        .grand_parent()
        .and_then(JsCallExpression::cast)
    else {
        return false;
    };
    let Some(member) = call
        .callee()
        .ok()
        .and_then(|callee| AnyJsMemberExpression::cast(callee.omit_parentheses().into_syntax()))
    else {
        return false;
    };
    if !member
        .member_name()
        .is_some_and(|name| ARRAY_PREDICATE_METHODS.binary_search(&name.text()).is_ok())
    {
        return false;
    }
    let Some(object) = member.object().ok() else {
        return false;
    };
    ctx.type_of_expression(&object)
        .is_some_and(|ty| ty.is_array_or_tuple())
}

fn is_asserted_argument(
    ctx: &RuleContext<UseStrictBooleanExpressions>,
    node: &AnyJsExpression,
    arguments: &JsCallArgumentList,
) -> bool {
    let Some(call) = arguments
        .syntax()
        .grand_parent()
        .and_then(JsCallExpression::cast)
    else {
        return false;
    };
    let Some(callee) = call.callee().ok() else {
        return false;
    };
    let Some(index) = ctx
        .type_of_expression(&callee)
        .and_then(|ty| ty.truthiness_asserted_argument())
    else {
        return false;
    };
    for (argument_index, argument) in arguments.iter().enumerate() {
        let Ok(AnyJsCallArgument::AnyJsExpression(argument)) = argument else {
            return false;
        };
        if argument_index == index {
            return argument == *node;
        }
    }
    false
}

#[test]
fn array_predicate_methods_are_sorted() {
    assert!(ARRAY_PREDICATE_METHODS.is_sorted_by(|left, right| left.cmp(right).is_le()));
}
