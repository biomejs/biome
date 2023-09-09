use biome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, RefactorKind, Rule};
use biome_diagnostics::Applicability;
use rome_console::markup;
use rome_js_factory::make;
use rome_js_syntax::{
    JsBinaryExpression, JsBinaryExpressionFields, JsBinaryOperator, JsSyntaxKind, T,
};
use rome_rowan::BatchMutationExt;

use crate::JsRuleAction;

declare_rule! {
    /// Provides a refactor to invert the left and right hand side of a binary expression
    ///
    /// ## Examples
    ///
    /// ```js
    /// (a < b)
    /// ```
    pub(crate) FlipBinExp {
        version: "1.0.0",
        name: "flipBinExp",
        recommended: false,
    }
}

impl Rule for FlipBinExp {
    type Query = Ast<JsBinaryExpression>;
    type State = JsSyntaxKind;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        let JsBinaryExpressionFields {
            left,
            operator_token: _,
            right,
        } = node.as_fields();

        // Ensure the node doesn't have any syntax error
        left.ok()?;
        right.ok()?;

        invert_op(node.operator().ok()?)
    }

    fn action(ctx: &RuleContext<Self>, op: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let prev_left = node.left().ok()?;
        let new_left = node.right().ok()?;
        mutation.replace_node(prev_left, new_left);

        let prev_op = node.operator_token().ok()?;
        let new_op = make::token(*op);
        mutation.replace_token(prev_op, new_op);

        let prev_right = node.right().ok()?;
        let new_right = node.left().ok()?;
        mutation.replace_node(prev_right, new_right);

        Some(JsRuleAction {
            category: ActionCategory::Refactor(RefactorKind::None),
            applicability: Applicability::Always,
            message: markup! { "Flip Binary Expression" }.to_owned(),
            mutation,
        })
    }
}

fn invert_op(op: JsBinaryOperator) -> Option<JsSyntaxKind> {
    match op {
        JsBinaryOperator::LessThan => Some(T![>]),
        JsBinaryOperator::GreaterThan => Some(T![<]),
        JsBinaryOperator::LessThanOrEqual => Some(T![>=]),
        JsBinaryOperator::GreaterThanOrEqual => Some(T![<=]),
        JsBinaryOperator::Equality => Some(T![==]),
        JsBinaryOperator::StrictEquality => Some(T![===]),
        JsBinaryOperator::Inequality => Some(T![!=]),
        JsBinaryOperator::StrictInequality => Some(T![!==]),
        JsBinaryOperator::Plus => Some(T![+]),
        JsBinaryOperator::Minus => None,
        JsBinaryOperator::Times => Some(T![*]),
        JsBinaryOperator::Divide => None,
        JsBinaryOperator::Remainder => None,
        JsBinaryOperator::Exponent => None,
        JsBinaryOperator::LeftShift => None,
        JsBinaryOperator::RightShift => None,
        JsBinaryOperator::UnsignedRightShift => None,
        JsBinaryOperator::BitwiseAnd => Some(T![&]),
        JsBinaryOperator::BitwiseOr => Some(T![|]),
        JsBinaryOperator::BitwiseXor => Some(T![^]),
    }
}
