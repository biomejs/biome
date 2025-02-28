use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsAssignmentExpression, JsAssignmentOperator, JsBinaryExpression,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::{
    utils::{find_variable_position, VariablePosition},
    JsRuleAction,
};

declare_lint_rule! {
    /// Disallow shorthand assign when variable appears on both sides.
    ///
    /// This rule helps to avoid potential bugs related to incorrect assignments or unintended
    /// side effects that may occur during refactoring.
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
    /// ### Valid
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
    pub NoMisrefactoredShorthandAssign {
        version: "1.3.0",
        name: "noMisrefactoredShorthandAssign",
        language: "js",
        sources: &[RuleSource::Clippy("misrefactored_assign_op")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoMisrefactoredShorthandAssign {
    type Query = Ast<JsAssignmentExpression>;
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if matches!(node.operator(), Ok(JsAssignmentOperator::Assign)) {
            return None;
        }

        let right = node.right().ok()?;
        let operator = node.operator_token().ok()?;
        let operator = operator.text_trimmed();
        let operator = &operator[0..operator.len() - 1];

        let binary_expression = match right {
            AnyJsExpression::JsBinaryExpression(binary_expression) => binary_expression,
            AnyJsExpression::JsParenthesizedExpression(param) => {
                JsBinaryExpression::cast(param.expression().ok()?.into_syntax())?
            }
            _ => return None,
        };

        let bin_operator = binary_expression.operator_token().ok()?;
        let bin_operator = bin_operator.text_trimmed();

        let not_same_operator_from_shorthand = operator != bin_operator;

        if not_same_operator_from_shorthand {
            return None;
        }

        let left = node.left().ok()?;
        let left = left.as_any_js_assignment()?;
        let left_text = left.to_trimmed_string();

        let variable_position_in_expression =
            find_variable_position(&binary_expression, &left_text)?;

        if !binary_expression.operator().ok()?.is_commutative()
            && matches!(variable_position_in_expression, VariablePosition::Right)
        {
            return None;
        }

        match variable_position_in_expression {
            VariablePosition::Left => binary_expression.right(),
            VariablePosition::Right => binary_expression.left(),
        }
        .ok()
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable appears on both sides of an assignment operation."
                },
            )
            .note(markup! {
                "This assignment might be the result of a wrong refactoring."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let replacement_node = node.clone().with_right(state.clone());
        let replacement_text = replacement_node.clone().syntax().text_trimmed().to_string();

        mutation.replace_node(node.clone(), replacement_node);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>""{replacement_text}""</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}
