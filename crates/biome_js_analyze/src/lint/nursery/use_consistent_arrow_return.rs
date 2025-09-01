use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, JsArrowFunctionExpression, JsFunctionBody,
    JsReturnStatement,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::use_consistent_arrow_return::UseConsistentArrowReturnOptions;

declare_lint_rule! {
    /// Enforce consistent arrow function bodies.
    ///
    /// This rule enforces the use of arrow functions with no body block when the function body consists of a single return statement.
    /// This rule does not report when:
    /// - the function body contains directives (e.g. `"use strict"`), or
    /// - the body (or its descendants) contain comments, or
    /// - the single `return` has no argument (`return;`).
    ///
    /// The fix wraps expressions in parentheses when required for correctness (e.g. object literals and sequence expressions).
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
    /// const bar = () => { "use strict"; return 1 }
    /// const baz = () => { /* intentional */ return x }
    /// const qux = () => ({ a: 1 })   // already concise with parens
    /// ```
    ///
    pub UseConsistentArrowReturn {
        version: "next",
        name: "useConsistentArrowReturn",
        language: "js",
        sources: &[RuleSource::Eslint("arrow-body-style").same()],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
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
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The body of this arrow function contains a single "<Emphasis>"return"</Emphasis> " statement."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, body: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let return_statement = body.statements().iter().next()?;
        let return_statement = JsReturnStatement::cast(return_statement.into_syntax())?;
        let return_argument = return_statement.argument()?;

        let new_body = if needs_parens_in_concise_body(&return_argument) {
            AnyJsExpression::from(make::parenthesized(return_argument))
        } else {
            return_argument
        };

        mutation.replace_node(
            AnyJsFunctionBody::from(body.clone()),
            AnyJsFunctionBody::AnyJsExpression(new_body),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the return statement" }.to_owned(),
            mutation,
        ))
    }
}

fn needs_parens_in_concise_body(expr: &AnyJsExpression) -> bool {
    use AnyJsExpression::*;
    matches!(
        expr,
        JsObjectExpression(_)
            | JsSequenceExpression(_)
            | JsParenthesizedExpression(_)
            | TsAsExpression(_)
            | TsSatisfiesExpression(_)
            | TsTypeAssertionExpression(_)
    )
}
