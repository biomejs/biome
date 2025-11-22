use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression,
    JsSyntaxKind::{EQ2, NEQ},
    JsSyntaxToken, T,
};
use biome_rowan::{BatchMutationExt, SyntaxResult};
use biome_rule_options::no_equals_to_null::NoEqualsToNullOptions;

declare_lint_rule! {
    /// Require the use of `===` or `!==` for comparison with `null`.
    ///
    /// Comparing to `null` with `==` or `!=` may have unintended results as the
    /// expression evaluates to `true` when comparing `null` to `undefined`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foo == null;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo != null;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// foo === null;
    /// ```
    ///
    /// ```js
    /// foo !== null;
    /// ```
    pub NoEqualsToNull {
        version: "2.3.8",
        name: "noEqualsToNull",
        language: "js",
        sources: &[RuleSource::Eslint("no-eq-null").same()],
        recommended: false,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoEqualsToNull {
    type Query = Ast<JsBinaryExpression>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoEqualsToNullOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let op = node.operator_token().ok()?;

        if matches!(op.kind(), EQ2 | NEQ)
            && (is_null_literal(&node.left()) || is_null_literal(&node.right()))
        {
            return Some(op);
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, op: &Self::State) -> Option<RuleDiagnostic> {
        let text_trimmed = op.text_trimmed();
        Some(RuleDiagnostic::new(
            rule_category!(),
            op.text_trimmed_range(),
            markup! {
                ""<Emphasis>"null"</Emphasis>" comparison with "<Emphasis>{text_trimmed}</Emphasis>" is disallowed."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, op: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let suggestion = if op.kind() == EQ2 { T![===] } else { T![!==] };
        mutation.replace_token(op.clone(), make::token(suggestion));

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{suggestion.to_string()?}</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn is_null_literal(res: &SyntaxResult<AnyJsExpression>) -> bool {
    matches!(
        res,
        Ok(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNullLiteralExpression(_)
        ))
    )
}
