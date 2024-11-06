use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsConditionalExpression, JsIfStatement, JsUnaryOperator, T,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt};

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
            RuleSource::Eslint("no-negated-condition"),
            RuleSource::Clippy("if_not_else"),
        ],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoNegationElse {
    type Query = Ast<AnyJsCondition>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

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
                let negated_test = test.as_js_unary_expression()?.argument().ok()?;
                let new_node = node
                    .clone()
                    .with_test(negated_test)
                    .with_consequent(node.alternate().ok()?)
                    .with_colon_token(make::token_decorated_with_space(T![:]))
                    .with_alternate(node.consequent().ok()?);
                mutation.replace_node(node, new_node);
            }
            AnyJsCondition::JsIfStatement(node) => {
                let test = node.test().ok()?;
                let negated_test = test.as_js_unary_expression()?.argument().ok()?;
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
    if let AnyJsExpression::JsUnaryExpression(node) = node {
        if node.operator() == Ok(JsUnaryOperator::LogicalNot) {
            if let Ok(AnyJsExpression::JsUnaryExpression(inner_unary)) = node.argument() {
                // Some users use double exclamation to convert a value to a boolean
                // e.g. `!!0`
                return inner_unary.operator() != Ok(JsUnaryOperator::LogicalNot);
            }
            return true;
        }
    }
    false
}
