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
use biome_rowan::{
    AstNode, AstNodeExt, BatchMutationExt, Language, SyntaxTriviaPiece, declare_node_union,
    trim_trailing_trivia_pieces,
};
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
                let test_leading_trivia: Vec<_> = negated_test
                    .syntax()
                    .first_token()?
                    .leading_trivia()
                    .pieces()
                    .collect();
                let consequent = node.consequent().ok()?;
                let alternate = node.alternate().ok()?;
                let question_mark_token = node.question_mark_token().ok()?;
                let colon_token = node.colon_token().ok()?;
                let (consequent_trailing, consequent_suffix) = split_trailing_trivia(
                    consequent.syntax().last_token()?.trailing_trivia().pieces(),
                );
                let (alternate_trailing, alternate_suffix) = split_trailing_trivia(
                    alternate.syntax().last_token()?.trailing_trivia().pieces(),
                );

                // Update the ternary in-place so comments stored on `?` / `:`
                // stay attached to the branch they describe after the swap.
                let mut new_consequent_trailing = alternate_trailing;
                new_consequent_trailing.extend(consequent_suffix);
                let mut new_alternate_trailing = consequent_trailing;
                new_alternate_trailing.extend(alternate_suffix);
                let new_question_mark_token = question_mark_token
                    .clone()
                    .with_trailing_trivia_pieces(colon_token.trailing_trivia().pieces());
                let new_colon_token = colon_token
                    .clone()
                    .with_trailing_trivia_pieces(question_mark_token.trailing_trivia().pieces());

                let new_node = node
                    .clone()
                    .with_test(negated_test)
                    .with_question_mark_token(new_question_mark_token)
                    .with_consequent(with_trailing_trivia_pieces(
                        alternate.clone(),
                        new_consequent_trailing,
                    )?)
                    .with_colon_token(new_colon_token)
                    .with_alternate(with_trailing_trivia_pieces(
                        consequent,
                        new_alternate_trailing.clone(),
                    )?)
                    .with_leading_trivia_pieces(test_leading_trivia)?
                    .with_trailing_trivia_pieces(new_alternate_trailing)?;

                mutation.replace_node_discard_trivia(node, new_node);
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
        AnyJsExpression::JsUnaryExpression(unary_expr) => {
            let argument = unary_expr.argument().ok()?;
            let operator_token = unary_expr.operator_token().ok()?;
            let mut leading_trivia: Vec<_> = operator_token.leading_trivia().pieces().collect();
            leading_trivia.extend(operator_token.trailing_trivia().pieces());
            leading_trivia.extend(argument.syntax().first_token()?.leading_trivia().pieces());
            argument.with_leading_trivia_pieces(leading_trivia)
        }
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

fn split_trailing_trivia<L: Language>(
    trivia: impl ExactSizeIterator<Item = SyntaxTriviaPiece<L>> + DoubleEndedIterator,
) -> (Vec<SyntaxTriviaPiece<L>>, Vec<SyntaxTriviaPiece<L>>) {
    let pieces: Vec<_> = trivia.collect();
    let trimmed_len = trim_trailing_trivia_pieces(pieces.clone().into_iter()).len();
    (
        pieces[..trimmed_len].to_vec(),
        pieces[trimmed_len..].to_vec(),
    )
}

fn with_trailing_trivia_pieces<N>(
    node: N,
    trailing_trivia: Vec<SyntaxTriviaPiece<N::Language>>,
) -> Option<N>
where
    N: AstNode,
{
    let last_token = node.syntax().last_token()?;
    node.replace_token_discard_trivia(
        last_token.clone(),
        last_token.with_trailing_trivia_pieces(trailing_trivia),
    )
}
