use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsExpression, JsAssignmentExpression, JsAssignmentOperator,
    JsBinaryExpression, JsBinaryOperator, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::{
    utils::{find_variable_position, VariablePosition},
    JsRuleAction,
};

declare_lint_rule! {
    /// Require assignment operator shorthand where possible.
    ///
    /// JavaScript provides shorthand operators combining a variable assignment and simple mathematical operation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// a = a + 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a = a - 1;
    /// ```
    ///
    ///  ```js,expect_diagnostic
    /// a = a * 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// a += 1;
    /// ```
    ///
    /// ```js
    /// a -= 1;
    /// ```
    ///
    ///  ```js
    /// a *= 1;
    /// ```
    pub UseShorthandAssign {
        version: "1.3.0",
        name: "useShorthandAssign",
        language: "js",
        sources: &[RuleSource::Eslint("operator-assignment")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    shorthand_operator: JsSyntaxKind,
    replacement_expression: AnyJsExpression,
}

impl Rule for UseShorthandAssign {
    type Query = Ast<JsAssignmentExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !matches!(node.operator(), Ok(JsAssignmentOperator::Assign)) {
            return None;
        }

        let left = node.left().ok()?;
        let right = node.right().ok()?;

        let left_var_name = match left.as_any_js_assignment()? {
            AnyJsAssignment::JsComputedMemberAssignment(assignment) => {
                assignment.to_trimmed_string()
            }
            AnyJsAssignment::JsIdentifierAssignment(assignment) => assignment.to_trimmed_string(),
            AnyJsAssignment::JsStaticMemberAssignment(assignment) => assignment.to_trimmed_string(),
            _ => return None,
        };

        let binary_expression = match right {
            AnyJsExpression::JsBinaryExpression(binary_expression) => binary_expression,
            AnyJsExpression::JsParenthesizedExpression(param) => {
                JsBinaryExpression::cast(param.expression().ok()?.into_syntax())?
            }
            _ => return None,
        };

        let operator = binary_expression.operator().ok()?;
        let shorthand_operator = get_shorthand(operator)?;

        let variable_position_in_expression =
            find_variable_position(&binary_expression, &left_var_name)?;

        let replacement_expression = match variable_position_in_expression {
            VariablePosition::Left => binary_expression.right().ok()?,
            VariablePosition::Right => binary_expression.left().ok()?,
        };

        if !operator.is_commutative()
            && matches!(variable_position_in_expression, VariablePosition::Right)
        {
            return None;
        }

        Some(RuleState {
            shorthand_operator,
            replacement_expression,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let shorthand_operator = state.shorthand_operator.to_string()?;

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Assignment "<Emphasis>"(=)"</Emphasis>" can be replaced with operator assignment "<Emphasis>""{shorthand_operator}""</Emphasis>"."
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

        let shorthand_node = node
            .clone()
            .with_operator_token_token(token)
            .with_right(state.replacement_expression.clone());

        mutation.replace_node(node.clone(), shorthand_node);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>""{shorthand_operator.to_string()?}""</Emphasis>" instead." }
                .to_owned(),
            mutation,
        ))
    }
}

fn get_shorthand(operator: JsBinaryOperator) -> Option<JsSyntaxKind> {
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
