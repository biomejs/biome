use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsArrowFunctionExpression, JsFunctionExpression, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::use_typed_iterable_callback_return::UseTypedIterableCallbackReturnOptions;

use crate::{
    services::control_flow::TypedControlFlowGraph,
    utils::iterable::{ITERABLE_METHOD_INFOS, IterableMethodInfo},
};

declare_lint_rule! {
    /// Enforce consistent return values in iterable callbacks using type information.
    ///
    /// This rule ensures that callbacks passed to certain iterable methods either always return a
    /// value or never return a value, depending on the method's requirements.
    ///
    /// For methods that need a result, each branch must return a value or throw.
    /// Explicitly returning `undefined` is allowed. Async and generator callbacks are ignored.
    /// Calls with unknown object types are skipped.
    ///
    /// This is a type-aware alternative to [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/).
    /// Enable only one of these rules to avoid overlapping reports.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid.ts
    /// function log(value: number): void {}
    /// [1, 2, 3].map(value => log(value));
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=discarded.ts
    /// [1, 2, 3].forEach(value => 42);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// function log(value: number): void {}
    /// [1, 2, 3].forEach(value => log(value));
    /// [1, 2, 3].map(value => 42);
    /// ```
    ///
    /// ## Options
    ///
    /// ### `checkForEach`
    ///
    /// Default: `true`
    ///
    /// Set to `false` to allow values returned from `forEach` callbacks.
    ///
    /// ```json,options
    /// { "options": { "checkForEach": false } }
    /// ```
    ///
    /// ```ts,use_options,file=unchecked.ts
    /// [1, 2, 3].forEach(value => 42);
    /// ```
    ///
    /// ### `allowImplicit`
    ///
    /// Default: `false`.
    ///
    /// When set to `true`, allows callbacks to implicitly return `undefined`
    /// using `return;`. This is useful for patterns like `.filter(Boolean)`
    /// chaining where some callbacks intentionally return `undefined`.
    ///
    /// ```json,options
    /// { "options": { "allowImplicit": true } }
    /// ```
    ///
    /// ```ts,use_options,file=implicit.ts
    /// [1, 2, 3].map(value => { return; });
    /// ```
    ///
    pub UseTypedIterableCallbackReturn {
        version: "next",
        name: "useTypedIterableCallbackReturn",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Types],
    }
}

pub struct RuleState {
    method: &'static IterableMethodInfo,
    range: TextRange,
    invalid_returns: Vec<TextRange>,
    falls_through: bool,
}

declare_node_union! {
    pub AnyJsCallback = JsArrowFunctionExpression | JsFunctionExpression
}

impl Rule for UseTypedIterableCallbackReturn {
    type Query = TypedControlFlowGraph<AnyJsCallback>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseTypedIterableCallbackReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();
        let function = AnyJsFunction::cast_ref(&cfg.node)?;
        if function.is_async() || function.is_generator() {
            return None;
        }
        let expression = AnyJsExpression::cast_ref(&cfg.node)?.outer_expression()?;
        let arguments = expression.parent::<JsCallArgumentList>()?;
        let call = arguments
            .parent::<JsCallArguments>()?
            .parent::<JsCallExpression>()?;
        let callee = call.callee().ok()?.omit_parentheses();
        let member = callee.as_js_static_member_expression()?;
        let name = member.member().ok()?;
        let token = name.as_js_name()?.value_token().ok()?;
        let method = ITERABLE_METHOD_INFOS.get(token.text_trimmed())?;
        if !method.return_value_required && !ctx.options().check_for_each() {
            return None;
        }
        let position = arguments.iter().position(|argument| {
            argument.is_ok_and(|argument| argument.syntax() == expression.syntax())
        })?;
        if position != method.callback_argument_position {
            return None;
        }

        let object = member.object().ok()?.omit_parentheses();
        if let Some(global_name) = method.global_name {
            let (reference, name) =
                global_identifier(&object.as_any_global_identifier_expression()?)?;
            if name.text() != global_name || ctx.has_binding(&reference) {
                return None;
            }
        } else {
            let ty = ctx.type_of_expression(&object)?;
            let recognized = if method.return_value_required {
                ty.is_all_array_or_tuple()
            } else {
                ty.is_all_array_map_or_set()
            };
            if !recognized {
                return None;
            }
        }

        let mut invalid_returns = Vec::new();
        let falls_through = match function.body().ok()? {
            AnyJsFunctionBody::AnyJsExpression(expression) => {
                if invalid_return(ctx, method, Some(&expression)) {
                    invalid_returns.push(expression.range());
                }
                false
            }
            AnyJsFunctionBody::JsFunctionBody(_) => cfg.visit_return_paths(|statement| {
                if invalid_return(ctx, method, statement.argument().as_ref()) {
                    invalid_returns.push(statement.range());
                }
            }),
        };
        let falls_through = falls_through && method.return_value_required;
        (falls_through || !invalid_returns.is_empty()).then_some(RuleState {
            method,
            range: member.member().ok()?.range(),
            invalid_returns,
            falls_through,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = if state.method.return_value_required {
            RuleDiagnostic::new(rule_category!(), state.range, markup! {
                "This callback passed to "<Emphasis>{state.method}</Emphasis>" does not return a value on every path."
            }).note(markup! {
                "This method uses the callback's result. Return a value on each non-throwing path."
            })
        } else {
            RuleDiagnostic::new(rule_category!(), state.range, markup! {
                "This callback passed to "<Emphasis>{state.method}</Emphasis>" returns a value that is discarded."
            }).note(markup! {
                "Use a block body without returning the value, or choose a method that uses the callback's result."
            })
        };
        if state.falls_through {
            diagnostic =
                diagnostic.note(markup! { "The callback can reach its end without returning." });
        }
        for range in &state.invalid_returns {
            diagnostic = diagnostic.detail(
                *range,
                if state.method.return_value_required {
                    markup! { "Return a value instead of an empty or void-valued result." }
                } else {
                    markup! { "This returned value is not used." }
                },
            );
        }
        Some(diagnostic)
    }
}

fn invalid_return(
    ctx: &RuleContext<UseTypedIterableCallbackReturn>,
    method: &IterableMethodInfo,
    expression: Option<&AnyJsExpression>,
) -> bool {
    if method.return_value_required && ctx.options().allow_implicit() {
        return false;
    }
    let Some(expression) = expression else {
        return method.return_value_required;
    };
    let Some(ty) = ctx.type_of_expression(expression) else {
        return false;
    };
    if method.return_value_required {
        ty.has_void_variant() == Some(true)
    } else {
        ty.has_non_void_return_value() == Some(true)
    }
}
