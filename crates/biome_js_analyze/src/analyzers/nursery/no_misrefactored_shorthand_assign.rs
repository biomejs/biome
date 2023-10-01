use biome_analyze::{context::RuleContext, declare_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignment, JsAssignmentExpression, JsAssignmentOperator, JsBinaryExpression,
};
use biome_rowan::AstNode;

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

pub struct RuleState {}

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
        let binary_expression = right.as_js_binary_expression()?;
        let bin_operator = binary_expression
            .operator_token()
            .ok()?
            .kind()
            .to_string()?;

        let operator_token = node
            .operator_token()
            .ok()?
            .kind()
            .to_string()?
            .split_once('=')?;

        let left = node.left().ok()?;
        let left_var_name = get_assignment_variable_name(left.as_any_js_assignment()?)?;
        let all_variables: Vec<String> = find_all_variables(node);

        if operator_token.0 == bin_operator && all_variables.contains(&left_var_name) {
            Some(RuleState {})
        } else {
            None
        }
    }

    fn diagnostic(node: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.query().range(),
            markup! {
                "Variable appears on both sides of an assignment operation"
            },
        ))
    }
}

fn get_assignment_variable_name(assignment: &AnyJsAssignment) -> Option<String> {
    match assignment {
        AnyJsAssignment::JsComputedMemberAssignment(assignment) => Some(assignment.text()),
        AnyJsAssignment::JsIdentifierAssignment(assignment) => Some(assignment.text()),
        AnyJsAssignment::JsStaticMemberAssignment(assignment) => Some(assignment.text()),
        _ => None,
    }
}

fn find_all_variables(node: &JsAssignmentExpression) -> Vec<String> {
    node.syntax()
        .children()
        .filter_map(JsBinaryExpression::cast)
        .fold(vec![], |mut acc, x| {
            if let (Some(left), Some(right)) = (x.left().ok(), x.right().ok()) {
                acc.push(left.text());
                acc.push(right.text());
            }
            acc
        })
}
