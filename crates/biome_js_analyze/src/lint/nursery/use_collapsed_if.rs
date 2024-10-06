use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make::{js_binary_expression, parenthesized, token};
use biome_js_syntax::{AnyJsExpression, AnyJsStatement, JsIfStatement, JsLogicalOperator, T};
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

        let child_if_statement = match consequent {
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

        let binary_expression = js_binary_expression(
            parenthesized_if_needed(&parent_test),
            token(T![&&]),
            parenthesized_if_needed(&child_test),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(parent_test, binary_expression.into());
        mutation.replace_node(parent_consequent, child_consequent);

        // TODO: Insert semicolon before the next statement if needed

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Use collapsed "<Emphasis>"if"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn parenthesized_if_needed(expr: &AnyJsExpression) -> AnyJsExpression {
    if needs_parenthesis(expr) {
        parenthesized(expr.clone()).into()
    } else {
        expr.clone()
    }
}

/// If the test expression has an operator that has lower precedence than `&&`,
/// it needs to be wrapped with a parenthesis before concatenating expressions using `&&`.
/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table
fn needs_parenthesis(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsLogicalExpression(expr) => matches!(
            expr.operator().ok(),
            Some(JsLogicalOperator::LogicalOr | JsLogicalOperator::NullishCoalescing)
        ),
        AnyJsExpression::JsConditionalExpression(_)
        | AnyJsExpression::JsAssignmentExpression(_)
        | AnyJsExpression::JsYieldExpression(_)
        | AnyJsExpression::JsSequenceExpression(_) => true,
        _ => false,
    }
}
