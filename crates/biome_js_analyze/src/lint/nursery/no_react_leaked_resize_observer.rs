use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, JsCallExpression, JsNewExpression, JsSyntaxKind, JsVariableDeclarator,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_react_leaked_resize_observer::NoReactLeakedResizeObserverOptions;

use crate::react::{effect_callback, is_effect_call};

declare_lint_rule! {
    /// Disallow forgetting to disconnect `ResizeObserver` within `useEffect`.
    ///
    /// This rule detects `ResizeObserver` instances created within `useEffect` hooks that don't have a corresponding
    /// `disconnect()` call in the cleanup function. Forgetting to disconnect a ResizeObserver can lead to memory
    /// leaks and unexpected behavior when components unmount or dependencies change.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function MyComponent() {
    ///   useEffect(() => {
    ///     const observer = new ResizeObserver(() => {
    ///       console.log("resize");
    ///     });
    ///     observer.observe(document.body);
    ///   }, []);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// import { useEffect } from "react";
    ///
    /// function MyComponent() {
    ///   useEffect(() => {
    ///     const observer = new ResizeObserver(() => {
    ///       console.log("resize");
    ///     });
    ///     observer.observe(document.body);
    ///     return () => observer.disconnect();
    ///   }, []);
    /// }
    /// ```
    ///
    pub NoReactLeakedResizeObserver {
        version: "next",
        name: "noReactLeakedResizeObserver",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReactXyz("web-api-no-leaked-resize-observer").same(), RuleSource::EslintReactWebApi("no-leaked-resize-observer").same()],
    }
}

impl Rule for NoReactLeakedResizeObserver {
    type Query = Ast<JsCallExpression>;
    type State = NoReactLeakedResizeObserverState;
    type Signals = Option<Self::State>;
    type Options = NoReactLeakedResizeObserverOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let effect_call = ctx.query();
        if !is_effect_call(effect_call) {
            return None;
        }

        let callback = effect_callback(effect_call)?;
        let callback_node = callback.syntax();

        let mut observer_ranges = Vec::new();
        for new_expr in callback_node
            .descendants()
            .filter_map(JsNewExpression::cast)
        {
            if !is_new_resize_observer(&new_expr) {
                continue;
            }

            if !new_expr
                .syntax()
                .ancestors()
                .any(|ancestor| JsVariableDeclarator::can_cast(ancestor.kind()))
            {
                return Some(NoReactLeakedResizeObserverState {
                    range: new_expr.range(),
                    kind: ResizeObserverLeakKind::UnexpectedFloatingInstance,
                });
            }

            observer_ranges.push(new_expr.range());
        }

        for observer_range in observer_ranges {
            let mut has_disconnect = false;
            let mut has_dynamic_observe = false;
            let mut observe_count = 0usize;
            let mut unobserve_count = 0usize;

            for call in callback_node
                .descendants()
                .filter_map(JsCallExpression::cast)
            {
                let Some(method_name) = observer_call_kind(&call) else {
                    continue;
                };

                match method_name {
                    "disconnect" => {
                        has_disconnect = true;
                    }
                    "observe" => {
                        observe_count += 1;

                        if is_dynamic_observe_call(&call, callback_node) {
                            has_dynamic_observe = true;
                        }
                    }
                    "unobserve" => {
                        unobserve_count += 1;
                    }
                    _ => {}
                }
            }

            if has_disconnect {
                continue;
            }

            if has_dynamic_observe {
                return Some(NoReactLeakedResizeObserverState {
                    range: observer_range,
                    kind: ResizeObserverLeakKind::ExpectedDisconnectInControlFlow,
                });
            }

            if observe_count == 0 || observe_count > unobserve_count {
                return Some(NoReactLeakedResizeObserverState {
                    range: observer_range,
                    kind: ResizeObserverLeakKind::ExpectedDisconnectOrUnobserveInCleanup,
                });
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let message = match state.kind {
            ResizeObserverLeakKind::ExpectedDisconnectInControlFlow => {
                markup! {
                    "Dynamically observed elements should be cleaned up with "<Emphasis>"disconnect()"</Emphasis>" in the cleanup function."
                }
            }
            ResizeObserverLeakKind::ExpectedDisconnectOrUnobserveInCleanup => {
                markup! {
                    "A "<Emphasis>"ResizeObserver"</Emphasis>" created in an effect must be disconnected or fully unobserved in the cleanup function."
                }
            }
            ResizeObserverLeakKind::UnexpectedFloatingInstance => {
                markup! {
                    "Assign this "<Emphasis>"ResizeObserver"</Emphasis>" instance to a variable so it can be cleaned up in the effect cleanup function."
                }
            }
        };

        Some(RuleDiagnostic::new(rule_category!(), state.range, message).note(markup! {
            "Not clearing the observer can cause memory leaks and trigger duplicate observations on an unmounted component."
        })
        .note(markup! {
            "Return a cleanup function from the effect that calls "<Emphasis>"disconnect"</Emphasis>" on the observer object."
        }))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResizeObserverLeakKind {
    ExpectedDisconnectInControlFlow,
    ExpectedDisconnectOrUnobserveInCleanup,
    UnexpectedFloatingInstance,
}

#[derive(Debug, Clone, Copy)]
pub struct NoReactLeakedResizeObserverState {
    range: TextRange,
    kind: ResizeObserverLeakKind,
}

fn is_new_resize_observer(new_expr: &JsNewExpression) -> bool {
    let Ok(callee) = new_expr.callee() else {
        return false;
    };

    let Some(name) = callee.get_callee_member_name() else {
        return false;
    };

    name.text_trimmed() == "ResizeObserver"
}

fn observer_call_kind(call: &JsCallExpression) -> Option<&'static str> {
    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;

    let member_name = member.member().ok()?.value_token().ok()?;
    match member_name.text_trimmed() {
        "observe" => Some("observe"),
        "unobserve" => Some("unobserve"),
        "disconnect" => Some("disconnect"),
        _ => None,
    }
}

fn is_dynamic_observe_call(
    call: &JsCallExpression,
    callback_node: &biome_js_syntax::JsSyntaxNode,
) -> bool {
    for ancestor in call.syntax().ancestors().skip(1) {
        if ancestor == *callback_node {
            break;
        }

        if matches!(
            ancestor.kind(),
            JsSyntaxKind::JS_IF_STATEMENT
                | JsSyntaxKind::JS_SWITCH_STATEMENT
                | JsSyntaxKind::JS_FOR_STATEMENT
                | JsSyntaxKind::JS_FOR_IN_STATEMENT
                | JsSyntaxKind::JS_FOR_OF_STATEMENT
                | JsSyntaxKind::JS_WHILE_STATEMENT
                | JsSyntaxKind::JS_DO_WHILE_STATEMENT
                | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
                | JsSyntaxKind::JS_LOGICAL_EXPRESSION
        ) {
            return true;
        }

        if JsCallExpression::can_cast(ancestor.kind()) {
            return true;
        }

        if let Some(expression) = AnyJsExpression::cast_ref(&ancestor)
            && matches!(expression, AnyJsExpression::JsArrowFunctionExpression(_))
        {
            return true;
        }
    }

    false
}
