use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{
    AnyJsExpression, BinaryExpressionNodePosition, JsAssignmentExpression, JsAssignmentOperator,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow shorthand assign when variable appears on both sides.
    ///
    /// This rule helps avoid potential bugs related to incorrect assignments or unintended
    /// side effects that may occur during refactoring.
    ///
    /// Source: https://rust-lang.github.io/rust-clippy/master/#/misrefactored_assign_op
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// a += a + b
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a -= a - b
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a *= a * b
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// a += b
    /// ```
    ///
    /// ```js
    /// a = a + b
    /// ```
    ///
    /// ```js
    /// a = a - b
    /// ```
    ///
    pub(crate) NoMisrefactoredShorthandAssign {
        version: "next",
        name: "noMisrefactoredShorthandAssign",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    replacement_expression: AnyJsExpression,
}

impl Rule for NoMisrefactoredShorthandAssign {
    type Query = Ast<JsAssignmentExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if matches!(node.operator(), Ok(JsAssignmentOperator::Assign)) {
            return None;
        }

        let right = node.right().ok()?;
        let operator_token = node
            .operator_token()
            .ok()?
            .kind()
            .to_string()?
            .split('=')
            .nth(0)?;

        let binary_expression = right.as_js_binary_expression()?;
        let bin_operator = binary_expression
            .operator_token()
            .ok()?
            .kind()
            .to_string()?;

        let not_same_operator_from_shorthand = operator_token != bin_operator;

        if not_same_operator_from_shorthand {
            return None;
        }

        let left = node.left().ok()?;
        let variable_position_in_expression = binary_expression.get_node_position(left.syntax())?;

        let replacement_expression = match variable_position_in_expression {
            BinaryExpressionNodePosition::Left => binary_expression.right().ok()?,
            BinaryExpressionNodePosition::Right => binary_expression.left().ok()?,
        };

        Some(RuleState {
            replacement_expression,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let replacement_text = node
            .clone()
            .with_right(state.replacement_expression.clone())
            .syntax()
            .text_trimmed()
            .to_string();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable appears on both sides of an assignment operation"
                },
            )
            .note(markup! {
                "This assignment might be result of a wrong refactoring, use "<Emphasis>""{replacement_text}""</Emphasis>" instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let replacement_node = node
            .clone()
            .with_right(state.replacement_expression.clone());

        let replacement_text = replacement_node.clone().syntax().text_trimmed().to_string();

        mutation.replace_node(node.clone(), replacement_node);

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            mutation,
            message: markup! { "Use "<Emphasis>""{replacement_text}""</Emphasis>" instead." }
                .to_owned(),
        })
    }
}
