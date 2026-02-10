use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsStatement, JsCallExpression, JsReturnStatement,
};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_return_in_finally::NoReturnInFinallyOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow return statements in `finally()`.
    ///
    /// Using return in a `finally()` callback can make the promise resolution
    /// value ambiguous and is generally not recommended. The return value from
    /// the `finally()` callback is ignored, making any return statement in this
    /// context potentially confusing.
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
    /// ### Valid
    ///
    /// ```js
    /// Promise.resolve(1).finally(() => { console.log(2) });
    /// myPromise.finally(() => {});
    /// ```
    ///
    pub NoReturnInFinally {
        version: "next",
        name: "noReturnInFinally",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintPromise("no-return-in-finally").same()],
    }
}

impl Rule for NoReturnInFinally {
    type Query = Semantic<JsCallExpression>;
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

        // Check if the argument is a function and find return statement

        match first_arg.as_any_js_expression()? {
            AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                // Check arrow function body
                match arrow.body().ok()? {
                    AnyJsFunctionBody::JsFunctionBody(body) => {
                        body.statements().iter().find_map(|stmt| match stmt {
                            AnyJsStatement::JsReturnStatement(ret) => Some(ret.clone()),
                            _ => None,
                        })
                    }
                    AnyJsFunctionBody::AnyJsExpression(_) => {
                        // Expression body doesn't have return statements
                        None
                    }
                }
            }
            AnyJsExpression::JsFunctionExpression(func) => {
                // Check function expression body
                if let Ok(body) = func.body() {
                    body.statements().iter().find_map(|stmt| match stmt {
                        AnyJsStatement::JsReturnStatement(ret) => Some(ret.clone()),
                        _ => None,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "No return in finally"
                },
            )
            .note(markup! {
                "The return value in a 'finally' callback is ignored, making any return statement potentially confusing."
            }),
        )
    }
}
