use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsFunction, JsFunctionBody, JsReturnStatement};
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
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     return 0;
    /// };
    /// ```
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
    type Query = Ast<AnyJsFunction>;
    type State = JsFunctionBody;
    type Signals = Option<Self::State>;
    type Options = UseConsistentArrowReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let func = ctx.query();
        let body = match func {
            AnyJsFunction::JsArrowFunctionExpression(arrow) => arrow
                .body()
                .ok()
                .and_then(|body| JsFunctionBody::cast(body.into_syntax()))?,
            AnyJsFunction::JsFunctionDeclaration(func) => func.body().ok()?,
            AnyJsFunction::JsFunctionExpression(func) => func.body().ok()?,
            AnyJsFunction::JsFunctionExportDefaultDeclaration(func) => func.body().ok()?,
        };

        if body.statements().len() == 1 {
            let first_statement = body.statements().iter().next()?;
            if JsReturnStatement::can_cast(first_statement.syntax().kind()) {
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
                    "This function can be written as an arrow function."
                },
            )
            .note(markup! {
                "Consider using an arrow function here."
            }),
        )
    }
}
