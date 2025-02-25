use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, QueryMatch, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{AnyJsStatement, JsIfStatement, T};
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
        version: "1.9.4",
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
    child_if_statement: JsIfStatement,
}

impl Rule for UseCollapsedIf {
    type Query = Ast<JsIfStatement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let if_stmt = ctx.query();

        // Ignore `if` with an `else` clause
        if if_stmt.else_clause().is_some() {
            return None;
        }

        let child_if_statement = match if_stmt.consequent().ok()? {
            // If `consequent` is a `JsBlockStatement` and the block contains only one
            // `JsIfStatement`, the child `if` statement should be merged.
            AnyJsStatement::JsBlockStatement(parent_block_statement) => {
                let statements = parent_block_statement.statements();
                if statements.len() != 1 {
                    return None;
                }

                let AnyJsStatement::JsIfStatement(child_if_statement) = statements.first()? else {
                    return None;
                };

                Some(child_if_statement)
            }
            // If `consequent` is a `JsIfStatement` without any block, it should be merged.
            AnyJsStatement::JsIfStatement(child_if_statement) => Some(child_if_statement),
            _ => None,
        }?;

        // It cannot be merged if the child `if` statement has any else clause(s).
        if child_if_statement.else_clause().is_some() {
            return None;
        }

        Some(RuleState {
            parent_if_statement: if_stmt.clone(),
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
            child_if_statement,
        } = state;

        let parent_consequent = parent_if_statement.consequent().ok()?;
        let parent_test = parent_if_statement.test().ok()?;
        let child_consequent = child_if_statement.consequent().ok()?;
        let child_test = child_if_statement.test().ok()?;

        let parent_has_comments = match &parent_consequent {
            AnyJsStatement::JsBlockStatement(block_stmt) => {
                block_stmt.l_curly_token().ok()?.has_trailing_comments()
                    || block_stmt.r_curly_token().ok()?.has_leading_comments()
            }
            _ => false,
        };

        let has_comments = parent_has_comments
            || child_if_statement.syntax().has_comments_direct()
            || child_if_statement
                .r_paren_token()
                .ok()?
                .has_trailing_comments();
        if has_comments {
            return None;
        }
        let operator = make::token_decorated_with_space(T![&&]);
        let mut expr =
            make::js_logical_expression(parent_test.clone(), operator, child_test.clone());

        // Parenthesize arms of the `&&` expression if needed
        let left = expr.left().ok()?;
        if left.needs_parentheses() {
            expr = expr.with_left(make::parenthesized(left).into());
        }

        let right = expr.right().ok()?;
        if right.needs_parentheses() {
            expr = expr.with_right(make::parenthesized(right).into());
        }

        // If the inner `if` statement has no block and the statement does not end with semicolon,
        // it cannot be fixed automatically because that will break the ASI rule.
        if !matches!(&child_consequent, AnyJsStatement::JsBlockStatement(_)) {
            let last_token = child_consequent.syntax().last_token()?;
            if last_token.kind() != T![;] {
                return None;
            }
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(parent_test, expr.into());
        mutation.replace_node(parent_consequent, child_consequent);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use collapsed "<Emphasis>"if"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}
