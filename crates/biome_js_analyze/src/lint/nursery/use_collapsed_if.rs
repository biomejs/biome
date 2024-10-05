use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make::{js_binary_expression, token};
use biome_js_syntax::{AnyJsExpression, AnyJsStatement, JsBlockStatement, JsIfStatement, T};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce using single `if` instead of nested `if` clauses.
    ///
    /// If an `if (b)` statement is the only statement in an `if (a)` block, it is often clearer to use an `if (a && b)` form.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (condition) {
    ///     if (anotherCondition) {
    ///         // ...
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (condition) {
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
    /// if (condition && anotherCondition) {
    ///     // ...
    /// }
    /// ```
    ///
    /// ```js
    /// if (condition) {
    ///     if (anotherCondition) {
    ///         // ...
    ///     }
    ///     doSomething();
    /// }
    /// ```
    ///
    /// ```js
    /// if (condition) {
    ///     if (anotherCondition) {
    ///         // ...
    ///     } else {
    ///         // ...
    ///     }
    /// }
    /// ```
    ///
    pub UseCollapsedIf {
        version: "next",
        name: "useCollapsedIf",
        language: "js",
        sources: &[
            RuleSource::EslintUnicorn("no-lonely-if"),
            RuleSource::Clippy("collapsible_if")
        ],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

pub struct RuleState {
    parent_if_statement: JsIfStatement,
    parent_block_statement: JsBlockStatement,
    child_if_statement: JsIfStatement,
}

impl Rule for UseCollapsedIf {
    type Query = Ast<JsIfStatement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let if_stmt = ctx.query();
        let consequent = if_stmt.consequent().ok()?;
        let AnyJsStatement::JsBlockStatement(parent_block_statement) = consequent else {
            return None;
        };
        let statements = parent_block_statement.statements();
        if statements.len() != 1 {
            return None;
        }
        let AnyJsStatement::JsIfStatement(child_if_statement) = statements.first()? else {
            return None;
        };
        if child_if_statement.else_clause().is_some() {
            return None;
        }
        Some(RuleState {
            parent_if_statement: if_stmt.clone(),
            parent_block_statement,
            child_if_statement,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.child_if_statement.syntax().text_range(),
            markup! {
                "This "<Emphasis>"if"</Emphasis>" statement can be collapsed into another "<Emphasis>"if"</Emphasis>" statement."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let RuleState {
            parent_if_statement,
            parent_block_statement,
            child_if_statement,
        } = state;

        let has_comments = parent_block_statement
            .l_curly_token()
            .ok()?
            .has_trailing_comments()
            || child_if_statement.syntax().has_comments_direct()
            || parent_block_statement
                .r_curly_token()
                .ok()?
                .has_leading_comments();
        if has_comments {
            return None;
        }

        let parent_condition = parent_if_statement.test().ok()?;
        let child_condition = child_if_statement.test().ok()?;
        let binary_expression = js_binary_expression(
            parent_condition.clone(),
            token(T![&&]),
            child_condition.clone(),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            parent_condition.clone(),
            AnyJsExpression::from(binary_expression),
        );
        mutation.replace_node(
            AnyJsStatement::from(parent_block_statement.clone()),
            child_if_statement.consequent().ok()?,
        );

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Use collapsed "<Emphasis>"if"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}
