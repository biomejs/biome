use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsExpression, JsSyntaxKind, TsNonNullAssertionAssignment,
    TsNonNullAssertionExpression,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Prevents the wrong usage of the non-null assertion operator (`!`) in TypeScript files.
    ///
    /// > The `!` non-null assertion operator in TypeScript is used to assert that a value's type does not include `null` or `undefined`. Using the operator any more than once on a single value does nothing.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    /// ```ts,expect_diagnostic
    /// const bar = foo!!.bar;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function fn(bar?: { n: number }) {
    ///   return bar!?.n;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function fn(bar?: { n: number }) {
    ///   return ((bar!))?.();
    /// }
    /// ```
    ///
    /// ### Valid
    /// ```ts
    /// const bar = foo!.bar;
    ///
    /// obj?.string!.trim();
    ///
    /// function fn(key: string | null) {
    ///   const obj = {};
    ///   return obj?.[key!];
    /// }
    /// ```
    ///
    pub NoExtraNonNullAssertion {
        version: "1.0.0",
        name: "noExtraNonNullAssertion",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-extra-non-null-assertion")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnyTsNonNullAssertion = TsNonNullAssertionAssignment | TsNonNullAssertionExpression
}

impl Rule for NoExtraNonNullAssertion {
    type Query = Ast<AnyTsNonNullAssertion>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            AnyTsNonNullAssertion::TsNonNullAssertionAssignment(_) => {
                let parent = node.parent::<AnyJsAssignment>()?;

                // Cases considered as invalid:
                // - TsNonNullAssertionAssignment > TsNonNullAssertionAssignment
                if matches!(parent, AnyJsAssignment::TsNonNullAssertionAssignment(_)) {
                    return Some(());
                }
            }
            AnyTsNonNullAssertion::TsNonNullAssertionExpression(_) => {
                let parent = node
                    .syntax()
                    .ancestors()
                    .skip(1)
                    .find(|ancestor| ancestor.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION)
                    .and_then(AnyJsExpression::cast)?;

                // Cases considered as invalid:
                // - TsNonNullAssertionAssignment > TsNonNullAssertionExpression
                // - TsNonNullAssertionExpression > TsNonNullAssertionExpression
                // - JsCallExpression[optional] > TsNonNullAssertionExpression
                // - JsStaticMemberExpression[optional] > TsNonNullAssertionExpression
                let has_extra_non_assertion = match parent {
                    AnyJsExpression::JsAssignmentExpression(expr) => expr
                        .left()
                        .ok()?
                        .as_any_js_assignment()?
                        .as_ts_non_null_assertion_assignment()
                        .is_some(),
                    AnyJsExpression::TsNonNullAssertionExpression(_) => true,
                    AnyJsExpression::JsStaticMemberExpression(expr) => expr.is_optional(),
                    AnyJsExpression::JsCallExpression(expr) => expr.is_optional(),
                    _ => false,
                };

                if has_extra_non_assertion {
                    return Some(());
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            "Forbidden extra non-null assertion.",
        );

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();

        let excl_token = match node {
            AnyTsNonNullAssertion::TsNonNullAssertionAssignment(assignment) => {
                assignment.excl_token().ok()?
            }
            AnyTsNonNullAssertion::TsNonNullAssertionExpression(expression) => {
                expression.excl_token().ok()?
            }
        };

        mutation.remove_token(excl_token);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove extra non-null assertion." }.to_owned(),
            mutation,
        ))
    }
}
