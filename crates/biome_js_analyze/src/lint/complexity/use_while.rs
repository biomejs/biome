use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsStatement, JsForStatement, T};
use biome_rowan::{trim_leading_trivia_pieces, AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce the use of `while` loops instead of `for` loops when the initializer and update expressions are not needed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (; x.running;) {
    ///     x.step();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// for(let x = 0; x < 10; i++) {}
    /// ```
    ///
    /// ```js
    /// let x = 0
    /// for(; x < 10; i++) {}
    /// ```
    ///
    /// ```js
    /// for(let x = 0; x < 10;) {
    ///     i++
    /// }
    /// ```
    pub UseWhile {
        version: "1.0.0",
        name: "useWhile",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintSonarJs("prefer-while")],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseWhile {
    type Query = Ast<JsForStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.initializer().is_some() || node.test().is_none() || node.update().is_some() {
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let for_range = node.for_token().ok()?.text_trimmed_range();
        let r_paren_range = node.r_paren_token().ok()?.text_trimmed_range();

        Some(RuleDiagnostic::new(
            rule_category!(),
            for_range.cover(r_paren_range),
            markup! {
                "Use a "<Emphasis>"while"</Emphasis>" loop instead of a "<Emphasis>"for"</Emphasis>" loop."
            },
        ).note(markup! {
            "Prefer a "<Emphasis>"while"</Emphasis>" loop over a "<Emphasis>"for"</Emphasis>" loop without initialization and update."
        }))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let test_leading_comments = node.first_semi_token().ok()?.trailing_trivia().pieces();
        let for_trailing_trivia = node.for_token().ok()?.trailing_trivia().pieces();
        mutation.replace_node(
            AnyJsStatement::from(node.clone()),
            AnyJsStatement::from(make::js_while_statement(
                make::token(T![while]).with_trailing_trivia_pieces(for_trailing_trivia),
                node.l_paren_token().ok()?.with_trailing_trivia([]),
                node.test()?
                    .prepend_trivia_pieces(trim_leading_trivia_pieces(test_leading_comments))?,
                node.r_paren_token().ok()?,
                node.body().ok()?,
            )),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a "<Emphasis>"while"</Emphasis>" loop." }.to_owned(),
            mutation,
        ))
    }
}
