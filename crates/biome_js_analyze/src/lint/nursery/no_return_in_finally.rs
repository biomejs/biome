use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, AnyJsFunctionBody, JsCallExpression, JsReturnStatement};
use biome_rowan::{AstNode, WalkEvent};
use biome_rule_options::no_return_in_finally::NoReturnInFinallyOptions;

use crate::services::control_flow::AnyJsControlFlowRoot;

declare_lint_rule! {
    /// Disallow return statements in `Promise.prototype.finally()` callbacks.
    ///
    /// Returning a value from a `Promise.prototype.finally()` callback is ignored, which can be confusing.
    /// Returned promises and thenables are awaited, and their rejection rejects
    /// the resulting promise.
    ///
    /// Returns inside nested blocks, including conditional branches and loops,
    /// are also disallowed. Returns inside nested functions are ignored by the rule.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Promise.resolve(1).finally(() => { return 2 });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// myPromise.finally(() => { return 2 });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// myPromise.finally(() => {
    ///     if (condition) {
    ///         return 2;
    ///     }
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Promise.resolve(1).finally(() => { console.log(2) });
    /// myPromise.finally(() => {});
    ///
    /// myPromise.finally(function () {
    ///     function nested() {
    ///         return 2;
    ///     }
    ///     console.log(nested());
    /// });
    /// ```
    ///
    /// ## References
    ///
    /// - MDN: [`Promise.prototype.finally` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/finally)
    ///
    pub NoReturnInFinally {
        version: "next",
        name: "noReturnInFinally",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintPromise("no-return-in-finally").inspired()],
    }
}

impl Rule for NoReturnInFinally {
    type Query = Ast<JsCallExpression>;
    type State = JsReturnStatement;
    type Signals = Option<Self::State>;
    type Options = NoReturnInFinallyOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();

        // Check if this is a .finally() call
        let member_name = call_expr
            .callee()
            .ok()?
            .as_js_static_member_expression()?
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?;

        if member_name.text_trimmed() != "finally" {
            return None;
        }

        // Get the first argument (callback function)
        let args = call_expr.arguments().ok()?.args();
        let first_arg = args.into_iter().next()?.ok()?;

        let body = match first_arg.as_any_js_expression()? {
            AnyJsExpression::JsArrowFunctionExpression(arrow) => match arrow.body().ok()? {
                AnyJsFunctionBody::JsFunctionBody(body) => body,
                AnyJsFunctionBody::AnyJsExpression(_) => return None,
            },
            AnyJsExpression::JsFunctionExpression(func) => func.body().ok()?,
            _ => return None,
        };

        let mut preorder = body.syntax().preorder();
        while let Some(event) = preorder.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if AnyJsControlFlowRoot::can_cast(node.kind()) {
                preorder.skip_subtree();
            } else if let Some(ret) = JsReturnStatement::cast(node) {
                return Some(ret);
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "Returning a value from a "<Emphasis>"finally"</Emphasis>" callback is not allowed."
                },
            )
            .note(markup! {
                "The return value in a "<Emphasis>"finally"</Emphasis>" callback is ignored, making any return statement potentially confusing."
            }).note(markup! {
                "Remove the return statement from the "<Emphasis>"finally"</Emphasis>" callback to resolve this issue."
            }),
        )
    }
}
