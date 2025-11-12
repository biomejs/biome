use crate::JsRuleAction;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsAssignmentPattern, AnyJsExpression, JsAssignmentExpression};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Enforce using nullish coalescing assignment operator (`??=`) instead of logical OR assignment (`||=`).
    ///
    /// The nullish coalescing assignment operator (`??=`) only assigns if the left side is null or undefined,
    /// while the logical OR assignment operator (`||=`) assigns for any falsy value (`0`, `''`, `false`, `null`, `undefined`, `NaN`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// x ||= 'default';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// obj.prop ||= getValue();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// x ??= 'default';
    /// ```
    ///
    /// ```js
    /// x = x || 'default'; // Use useNullishCoalescing instead
    /// ```
    ///
    pub UseNullishCoalescingAssignment {
        version: "next",
        name: "useNullishCoalescingAssignment",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("prefer-nullish-coalescing").inspired()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

pub struct RuleState {
    left: AnyJsAssignmentPattern,
    right: AnyJsExpression,
}

impl Rule for UseNullishCoalescingAssignment {
    type Query = biome_analyze::Ast<JsAssignmentExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let assignment = ctx.query();

        // Get the operator
        let operator = assignment.operator().ok()?;

        // Only match ||= operator
        if !matches!(
            operator,
            biome_js_syntax::JsAssignmentOperator::LogicalOrAssign
        ) {
            return None;
        }

        // Get left and right sides
        let left = assignment.left().ok()?;
        let right = assignment.right().ok()?;

        Some(RuleState { left, right })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Use "<Emphasis>"??="</Emphasis>" instead of "<Emphasis>"||="</Emphasis>"."
                },
            )
            .note(markup! {
                "The nullish coalescing assignment operator "<Emphasis>"??="</Emphasis>" only assigns when the value is null or undefined, which is safer than "<Emphasis>"||="</Emphasis>" which assigns for all falsy values."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        use biome_js_factory::make;
        use biome_js_syntax::T;
        use biome_rowan::BatchMutationExt;

        let assignment = ctx.query();
        let mut mutation = ctx.root().begin();

        // Create new ??= assignment expression
        let new_assignment = make::js_assignment_expression(
            state.left.clone(),
            make::token(T![??=]),
            state.right.clone(),
        );

        // Replace the assignment
        mutation.replace_node(assignment.clone(), new_assignment);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>"||="</Emphasis>" with "<Emphasis>"??="</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}
