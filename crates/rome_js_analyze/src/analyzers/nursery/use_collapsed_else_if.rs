use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{AnyJsStatement, JsBlockStatement, JsElseClause, JsIfStatement};
use rome_rowan::{AstNode, AstNodeList, BatchMutationExt};

declare_rule! {
    /// Enforce using `else if` instead of `else { if ... }`.
    ///
    /// If an `if` statement is the only statement in the `else` block, it is often clearer to use an `else if` form.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-lonely-if
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (condition) {
    ///     // ...
    /// } else {
    ///     if (anotherCondition) {
    ///         // ...
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (condition) {
    ///     // ...
    /// } else {
    ///     if (anotherCondition) {
    ///         // ...
    ///     } else {
    ///         // ...
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (condition) {
    ///     // ...
    /// } else if (anotherCondition) {
    ///     // ...
    /// }
    /// ```
    ///
    /// ```js
    /// if (condition) {
    ///     // ...
    /// } else if (anotherCondition) {
    ///     // ...
    /// } else {
    ///     // ...
    /// }
    /// ```
    ///
    /// ```js
    /// if (condition) {
    ///     // ...
    /// } else {
    ///     if (anotherCondition) {
    ///         // ...
    ///     }
    ///     doSomething();
    /// }
    /// ```
    ///
    pub(crate) UseCollapsedElseIf {
        version: "next",
        name: "useCollapsedElseIf",
        recommended: false,
    }
}

pub(crate) struct RuleState {
    block_statement: JsBlockStatement,
    if_statement: JsIfStatement,
    provide_fix: bool,
}

impl Rule for UseCollapsedElseIf {
    type Query = Ast<JsElseClause>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let else_clause = ctx.query();
        let alternate = else_clause.alternate().ok()?;
        let block = alternate.as_js_block_statement()?;
        let block_statements = block.statements();
        if block_statements.len() != 1 {
            return None;
        }
        if let AnyJsStatement::JsIfStatement(if_statement) = block_statements.first()? {
            let has_comments = if_statement.syntax().has_leading_comments()
                || block
                    .r_curly_token()
                    .is_ok_and(|token| token.has_leading_comments());
            Some(RuleState {
                block_statement: block.to_owned(),
                if_statement,
                provide_fix: !has_comments,
            })
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.if_statement.syntax().text_range(),
            markup! {
                "This "<Emphasis>"if"</Emphasis>" statement can be collapsed into an "<Emphasis>"else if"</Emphasis>" statement."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if !state.provide_fix {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_element(
            state.block_statement.clone().into_syntax().into(),
            state.if_statement.clone().into_syntax().into(),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Use collapsed "<Emphasis>"else if"</Emphasis>" instead." }
                .to_owned(),
            mutation,
        })
    }
}
