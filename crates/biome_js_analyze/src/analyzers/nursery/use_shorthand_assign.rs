use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsAssignmentExpression, JsAssignmentOperator, JsBinaryExpression,
    JsBinaryOperator, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Require assignment operator shorthand where possible
    ///
    /// JavaScript provides shorthand operators that combine variable assignment and some simple mathematical operations
    ///
    /// Source: https://eslint.org/docs/latest/rules/operator-assignment/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = a + 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = a - 1;
    /// ```
    ///
    ///  ```js,expect_diagnostic
    /// var a = 1;
    /// a = a * 1;
    /// ```
    ///
    ///  ```js,expect_diagnostic
    /// var a = 1;
    /// a = a / 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = a % 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = a << 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = a >> 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = a >>> 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = a & 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = a ^ 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = a | 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = a ** 1;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = 1;
    /// a += 1;
    /// ```
    ///
    /// ```js
    /// var a = 1;
    /// a -= 1;
    /// ```
    ///
    ///  ```js
    /// var a = 1;
    /// a *= 1;
    /// ```
    ///
    ///  ```js
    /// var a = 1;
    /// a /= 1;
    /// ```
    ///
    /// ```js
    /// var a = 1;
    /// a %= 1;
    /// ```
    ///
    /// ```js
    /// var a = 1;
    /// a <<= 1;
    /// ```
    ///
    /// ```js
    /// var a = 1;
    /// a >>= 1;
    /// ```
    ///
    /// ```js
    /// var a = 1;
    /// a >>>= 1;
    /// ```
    ///
    /// ```js
    /// var a = 1;
    /// a &= 1;
    /// ```
    ///
    /// ```js
    /// var a = 1;
    /// a ^= 1;
    /// ```
    ///
    /// ```js
    /// var a = 1;
    /// a |= 1;
    /// ```
    ///
    /// ```js
    /// var a = 1;
    /// a **= 1;
    /// ```
    ///
    pub(crate) UseShorthandAssign {
        version: "next",
        name: "useShorthandAssign",
        recommended: false,
    }
}

pub struct RuleState {
    shorthand_operator: JsSyntaxKind,
    right: AnyJsExpression
}

impl Rule for UseShorthandAssign {
    type Query = Ast<JsAssignmentExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let operator = node.operator().ok()?;
        let right = node.right().ok();
        let left = node.left().ok();
        let left_var_name = left?.text();

        if !matches!(operator, JsAssignmentOperator::Assign) {
            return None;
        }

        let binary_expression = match right {
            Some(AnyJsExpression::JsBinaryExpression(binary_expression)) => Some(binary_expression),
            Some(AnyJsExpression::JsParenthesizedExpression(param)) => Some(
                JsBinaryExpression::cast_ref(param.expression().ok()?.syntax())?,
            ),
            _ => None,
        }?;

        let has_same_reference =
            has_same_reference_in_expression(left_var_name, &binary_expression)?;
        let operator = binary_expression.operator().ok()?;
        let right = binary_expression.right().ok()?;
        let shorhand = get_shorthand(&operator)?;

        if has_same_reference {
            Some(RuleState {
                shorthand_operator: shorhand,
                right
            })
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let shorthand_operator = state.shorthand_operator.to_string()?;

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Assignment (=) can be replaced with operator assignment "{shorthand_operator}""
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let shorthand_operator = state.shorthand_operator;

        let token_leading_trivia = node.operator_token().ok()?.leading_trivia().pieces();
        let token_trailing_trivia = node.operator_token().ok()?.trailing_trivia().pieces();
        let token = make::token(shorthand_operator)
            .with_leading_trivia_pieces(token_leading_trivia)
            .with_trailing_trivia_pieces(token_trailing_trivia);

        let next_node = node.clone()
        .with_operator_token_token(token)
        .with_right(state.right.clone());

        mutation.replace_node(
            AnyJsExpression::JsAssignmentExpression(node.clone()),
            AnyJsExpression::JsAssignmentExpression(next_node),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use "<Emphasis>""{shorthand_operator.to_string()?}""</Emphasis>" instead." }
                .to_owned(),
            mutation,
        })
    }
}

fn has_same_reference_in_expression(
    variable_name: String,
    binary_expression: &JsBinaryExpression,
) -> Option<bool> {
    Some(variable_name == binary_expression.left().ok()?.omit_parentheses().text())
}

fn get_shorthand(operator: &JsBinaryOperator) -> Option<JsSyntaxKind> {
    match operator {
        JsBinaryOperator::Plus => Some(T![+=]),
        JsBinaryOperator::Minus => Some(T![-=]),
        JsBinaryOperator::Times => Some(T![*=]),
        JsBinaryOperator::Divide => Some(T![/=]),
        JsBinaryOperator::Remainder => Some(T![%=]),
        JsBinaryOperator::Exponent => Some(T![**=]),
        JsBinaryOperator::BitwiseAnd => Some(T![&=]),
        JsBinaryOperator::BitwiseOr => Some(T![|=]),
        JsBinaryOperator::BitwiseXor => Some(T![^=]),
        JsBinaryOperator::LeftShift => Some(T![<<=]),
        JsBinaryOperator::RightShift => Some(T![>>=]),
        JsBinaryOperator::UnsignedRightShift => Some(T![>>>=]),
        _ => None,
    }
}
