use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsStatement, JsBlockStatement, JsElseClause, JsIfStatement};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

declare_lint_rule! {
    /// Enforce using `else if` instead of nested `if` in `else` clauses.
    ///
    /// If an `if` statement is the only statement in the `else` block, it is often clearer to use an `else if` form.
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
    /// ```js,expect_diagnostic
    /// if (condition) {
    ///     // ...
    /// } else {
    ///     // Comment
    ///     if (anotherCondition) {
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
    pub UseCollapsedElseIf {
        version: "1.1.0",
        name: "useCollapsedElseIf",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-lonely-if"),
            RuleSource::Clippy("collapsible_else_if")
        ],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

pub struct RuleState {
    block_statement: JsBlockStatement,
    if_statement: JsIfStatement,
}

impl Rule for UseCollapsedElseIf {
    type Query = Ast<JsElseClause>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let else_clause = ctx.query();
        let alternate = else_clause.alternate().ok()?;
        let AnyJsStatement::JsBlockStatement(block_statement) = alternate else {
            return None;
        };
        let statements = block_statement.statements();
        if statements.len() != 1 {
            return None;
        }
        if let AnyJsStatement::JsIfStatement(if_statement) = statements.first()? {
            Some(RuleState {
                block_statement,
                if_statement,
            })
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.if_statement.syntax().text_range_with_trivia(),
            markup! {
                "This "<Emphasis>"if"</Emphasis>" statement can be collapsed into an "<Emphasis>"else if"</Emphasis>" statement."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let RuleState {
            block_statement,
            if_statement,
        } = state;

        let has_comments = block_statement
            .l_curly_token()
            .ok()?
            .has_trailing_comments()
            || if_statement.syntax().has_comments_direct()
            || block_statement.r_curly_token().ok()?.has_leading_comments();
        if has_comments {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsStatement::from(block_statement.clone()),
            if_statement.clone().into(),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use collapsed "<Emphasis>"else if"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}
