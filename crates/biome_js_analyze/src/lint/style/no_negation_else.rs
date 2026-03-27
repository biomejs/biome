use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsBinaryOperator, JsConditionalExpression, JsIfStatement,
    JsUnaryOperator, T,
};
use biome_rowan::{AstNode, BatchMutationExt, declare_node_union};
use biome_rule_options::no_negation_else::NoNegationElseOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow negation in the condition of an `if` statement if it has an `else` clause.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (!cond) { f();} else { g();}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// !cond ? 0 : 1
    ///```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (!cond) { f(); }
    ///```
    ///
    /// ```js
    /// cond ? 1 : 0
    ///```
    ///
    /// ```js
    /// if (!cond) { f(); }
    ///```
    ///
    /// ```js
    /// if (!!val) { f(); } else { g(); }
    ///```
    pub NoNegationElse {
        version: "1.0.0",
        name: "noNegationElse",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-negated-condition").same(),
            RuleSource::Clippy("if_not_else").same(),
        ],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoNegationElse {
    type Query = Ast<AnyJsCondition>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoNegationElseOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            AnyJsCondition::JsConditionalExpression(expr) => {
                is_negation(&expr.test().ok()?).then_some(())
            }
            AnyJsCondition::JsIfStatement(stmt) => (!matches!(
                stmt.else_clause()?.alternate().ok()?,
                AnyJsStatement::JsIfStatement(_)
            ) && is_negation(&stmt.test().ok()?))
            .then_some(()),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Invert blocks when performing a negation test."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match node.clone() {
            AnyJsCondition::JsConditionalExpression(node) => {
                let test = node.test().ok()?;
                let negated_test = replace_negation(&test)?;
                let consequent = node.consequent().ok()?;
                let alternate = node.alternate().ok()?;
                let trimmed_consequent = trim_branch_trailing_whitespace(consequent.clone())?;
                let question_mark_token = node.question_mark_token().ok()?;
                let colon_token = node.colon_token().ok()?;

                // Update the ternary in-place so comments stored on `?` / `:`
                // stay attached to the branch they describe after the swap.
                let new_question_mark_token = question_mark_token
                    .clone()
                    .with_trailing_trivia_pieces(colon_token.trailing_trivia().pieces());
                let new_colon_token = make::token_decorated_with_space(T![:])
                    .with_trailing_trivia_pieces(question_mark_token.trailing_trivia().pieces());

                mutation.replace_node(test, negated_test);
                mutation.replace_token_discard_trivia(question_mark_token, new_question_mark_token);
                mutation.replace_node_discard_trivia(consequent.clone(), alternate.clone());
                mutation.replace_token_discard_trivia(colon_token, new_colon_token);
                mutation.replace_node_discard_trivia(alternate, trimmed_consequent);
            }
            AnyJsCondition::JsIfStatement(node) => {
                let test = node.test().ok()?;

                let negated_test = replace_negation(&test)?;

                let else_clause = node.else_clause()?;
                let new_node = node
                    .clone()
                    .with_test(negated_test)
                    .with_consequent(else_clause.alternate().ok()?)
                    .with_else_clause(Some(
                        else_clause
                            .with_else_token(make::token_decorated_with_space(T![else]))
                            .with_alternate(node.consequent().ok()?),
                    ));
                mutation.replace_node(node, new_node);
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Invert the condition and the blocks." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    pub AnyJsCondition = JsConditionalExpression | JsIfStatement
}

fn is_negation(node: &AnyJsExpression) -> bool {
    if let AnyJsExpression::JsUnaryExpression(node) = node
        && node.operator() == Ok(JsUnaryOperator::LogicalNot)
    {
        if let Ok(AnyJsExpression::JsUnaryExpression(inner_unary)) = node.argument() {
            // Some users use double exclamation to convert a value to a boolean
            // e.g. `!!0`
            return inner_unary.operator() != Ok(JsUnaryOperator::LogicalNot);
        }
        return true;
    }
    if let AnyJsExpression::JsBinaryExpression(node) = node
        && (node.operator() == Ok(JsBinaryOperator::Inequality)
            || node.operator() == Ok(JsBinaryOperator::StrictInequality))
    {
        return true;
    }
    false
}

fn replace_negation(node: &AnyJsExpression) -> Option<AnyJsExpression> {
    match node {
        AnyJsExpression::JsUnaryExpression(unary_expr) => unary_expr.argument().ok(),
        AnyJsExpression::JsBinaryExpression(binary_expr) => {
            let operator = binary_expr.operator().ok()?;

            let token_leading_trivia = binary_expr.operator_token().ok()?.leading_trivia().pieces();
            let token_trailing_trivia = binary_expr
                .operator_token()
                .ok()?
                .trailing_trivia()
                .pieces();

            match operator {
                JsBinaryOperator::Inequality => Some(AnyJsExpression::JsBinaryExpression(
                    binary_expr.clone().with_operator_token_token(
                        make::token(T![==])
                            .with_leading_trivia_pieces(token_leading_trivia)
                            .with_trailing_trivia_pieces(token_trailing_trivia),
                    ),
                )),
                JsBinaryOperator::StrictInequality => Some(AnyJsExpression::JsBinaryExpression(
                    binary_expr.clone().with_operator_token_token(
                        make::token(T![===])
                            .with_leading_trivia_pieces(token_leading_trivia)
                            .with_trailing_trivia_pieces(token_trailing_trivia),
                    ),
                )),
                _ => None,
            }
        }
        _ => None,
    }
}

fn trim_branch_trailing_whitespace(node: AnyJsExpression) -> Option<AnyJsExpression> {
    let trailing: Vec<_> = node
        .syntax()
        .last_token()?
        .trailing_trivia()
        .pieces()
        .collect();

    if trailing.iter().any(|piece| piece.is_newline()) {
        return Some(node);
    }

    let keep_count = trailing
        .iter()
        .rposition(|piece| !piece.is_whitespace())
        .map_or(0, |index| index + 1);

    node.with_trailing_trivia_pieces(trailing.into_iter().take(keep_count))
}
