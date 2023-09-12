use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsConditionalExpression, JsIfStatement, JsUnaryExpression,
    JsUnaryOperator,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeExt, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow negation in the condition of an `if` statement if it has an `else` clause
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (!true) {consequent;} else {alternate;}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// !true ? consequent : alternate
    ///```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (!true) {consequent;}
    ///```
    ///
    /// ```js
    /// true ? consequent : alternate
    ///```
    pub(crate) NoNegationElse {
        version: "1.0.0",
        name: "noNegationElse",
        recommended: false,
    }
}

impl Rule for NoNegationElse {
    type Query = Ast<AnyJsCondition>;
    type State = JsUnaryExpression;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();

        match n {
            AnyJsCondition::JsConditionalExpression(expr) => {
                if is_negation(&expr.test().ok()?).unwrap_or(false) {
                    Some(expr.test().ok()?.as_js_unary_expression().unwrap().clone())
                } else {
                    None
                }
            }
            AnyJsCondition::JsIfStatement(stmt) => {
                if is_negation(&stmt.test().ok()?).unwrap_or(false)
                    && !matches!(
                        stmt.else_clause()?.alternate().ok()?,
                        AnyJsStatement::JsIfStatement(_)
                    )
                {
                    Some(stmt.test().ok()?.as_js_unary_expression().unwrap().clone())
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Invert blocks when performing a negation test."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match node {
            AnyJsCondition::JsConditionalExpression(expr) => {
                let mut next_expr = expr
                    .clone()
                    .replace_node(expr.test().ok()?, state.argument().ok()?)?;
                next_expr = next_expr
                    .clone()
                    .replace_node(next_expr.alternate().ok()?, expr.consequent().ok()?)?;
                next_expr = next_expr
                    .clone()
                    .replace_node(next_expr.consequent().ok()?, expr.alternate().ok()?)?;
                mutation.replace_node(
                    node.clone(),
                    AnyJsCondition::JsConditionalExpression(next_expr),
                );
            }
            AnyJsCondition::JsIfStatement(stmt) => {
                let next_stmt = stmt
                    .clone()
                    .replace_node(stmt.test().ok()?, state.argument().ok()?)?;
                let next_stmt = next_stmt.clone().replace_node(
                    next_stmt.else_clause()?,
                    make::js_else_clause(
                        stmt.else_clause()?.else_token().ok()?,
                        stmt.consequent().ok()?,
                    ),
                )?;
                let next_stmt = next_stmt.clone().replace_node(
                    next_stmt.consequent().ok()?,
                    stmt.else_clause()?.alternate().ok()?,
                )?;
                mutation.replace_node(node.clone(), AnyJsCondition::JsIfStatement(next_stmt));
            }
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Exchange alternate and consequent of the node" }.to_owned(),
            mutation,
        })
    }
}

fn is_negation(node: &AnyJsExpression) -> Option<bool> {
    match node {
        AnyJsExpression::JsUnaryExpression(expr) => {
            match (expr.operator().ok(), expr.argument().ok()) {
                (
                    Some(JsUnaryOperator::LogicalNot),
                    Some(AnyJsExpression::JsUnaryExpression(inner_unary)),
                ) => Some(inner_unary.operator().ok()? != JsUnaryOperator::LogicalNot),
                _ => Some(true),
            }
        }

        _ => Some(false),
    }
}

declare_node_union! {
    pub AnyJsCondition = JsConditionalExpression | JsIfStatement
}
