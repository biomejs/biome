use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsArrowFunctionExpression, JsFunctionBody, JsReturnStatement};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::use_consistent_arrow_return::UseConsistentArrowReturnOptions;

declare_lint_rule! {
    /// Enforce consistent arrow function bodies.
    ///
    /// This rule enforces the use of arrow functions with no body block when the function body consists of a single return statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    ///```js,expect_diagnostic
    /// const bar = () => {
    ///     return {
    ///         bar: {
    ///             foo: 1,
    ///             bar: 2,
    ///         }
    ///     };
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = () => 0;
    /// ```
    ///
    pub UseConsistentArrowReturn {
        version: "next",
        name: "useConsistentArrowReturn",
        language: "js",
        sources: &[RuleSource::Eslint("arrow-body-style").same()],
        recommended: false,
    }
}

impl Rule for UseConsistentArrowReturn {
    type Query = Ast<JsArrowFunctionExpression>;
    type State = JsFunctionBody;
    type Signals = Option<Self::State>;
    type Options = UseConsistentArrowReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let arrow = ctx.query();
        let body = JsFunctionBody::cast(arrow.body().ok()?.into_syntax())?;

        if !body.directives().is_empty() || body.syntax().has_comments_descendants() {
            return None;
        }

        if body.statements().len() == 1 {
            let first_statement = body.statements().iter().next()?;
            if let Some(return_statement) = JsReturnStatement::cast(first_statement.into_syntax())
                && return_statement.argument().is_some()
            {
                return Some(body);
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This arrow function doesn't need a return statement."
                },
            )
            .note(markup! {
                "Consider changing the function body into the returned expression."
            }),
        )
    }
}
