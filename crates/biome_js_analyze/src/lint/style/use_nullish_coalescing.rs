use crate::JsRuleAction;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    JsConditionalExpression, JsForStatement, JsIfStatement, JsLogicalExpression, JsLogicalOperator,
    JsSyntaxKind, JsWhileStatement, T,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_nullish_coalescing::UseNullishCoalescingOptions;

declare_lint_rule! {
    /// Enforce using nullish coalescing operator (`??`) instead of logical or (`||`)
    /// when providing default values.
    ///
    /// The `??` operator only checks for `null` and `undefined`, while `||` checks
    /// for any falsy value including `0`, `''`, and `false`. This can prevent bugs
    /// where legitimate falsy values are incorrectly treated as missing.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const value = count || 0;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const value = count ?? 0;
    /// ```
    pub UseNullishCoalescing {
        version: "next",
        name: "useNullishCoalescing",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("prefer-nullish-coalescing").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

pub struct RuleState {}

impl Rule for UseNullishCoalescing {
    type Query = biome_analyze::Ast<JsLogicalExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseNullishCoalescingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let logical = ctx.query();

        // Only check || operators
        let operator = logical.operator().ok()?;
        if operator != JsLogicalOperator::LogicalOr {
            return None;
        }

        // Check if in conditional test position
        let options = ctx.options();
        if options.ignore_conditional_tests && is_in_test_position(logical) {
            return None;
        }

        Some(RuleState {})
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.operator_token().ok()?.text_trimmed_range(),
                markup! {
                    "Use "<Emphasis>"??"</Emphasis>" instead of "<Emphasis>"||"</Emphasis>"."
                },
            )
            .note(markup! {
                "The || operator checks for all falsy values (null, undefined, 0, '', false), "
                "while ?? only checks for null and undefined."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        // Replace || token with ??
        let old_token = node.operator_token().ok()?;
        let new_token = make::token(T![??])
            .with_leading_trivia_pieces(old_token.leading_trivia().pieces())
            .with_trailing_trivia_pieces(old_token.trailing_trivia().pieces());

        mutation.replace_token(old_token, new_token);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"??"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// Check if logical expression is in a test position (if/while/for/ternary)
fn is_in_test_position(logical: &JsLogicalExpression) -> bool {
    for ancestor in logical.syntax().ancestors() {
        match ancestor.kind() {
            JsSyntaxKind::JS_IF_STATEMENT => {
                if let Some(if_stmt) = JsIfStatement::cast_ref(&ancestor)
                    && let Ok(test) = if_stmt.test()
                {
                    let test_range = test.syntax().text_trimmed_range();
                    let logical_range = logical.syntax().text_trimmed_range();
                    if test_range.contains_range(logical_range) {
                        return true;
                    }
                }
            }
            JsSyntaxKind::JS_WHILE_STATEMENT => {
                if let Some(while_stmt) = JsWhileStatement::cast_ref(&ancestor)
                    && let Ok(test) = while_stmt.test()
                {
                    let test_range = test.syntax().text_trimmed_range();
                    let logical_range = logical.syntax().text_trimmed_range();
                    if test_range.contains_range(logical_range) {
                        return true;
                    }
                }
            }
            JsSyntaxKind::JS_FOR_STATEMENT => {
                if let Some(for_stmt) = JsForStatement::cast_ref(&ancestor)
                    && let Some(test) = for_stmt.test()
                {
                    let test_range = test.syntax().text_trimmed_range();
                    let logical_range = logical.syntax().text_trimmed_range();
                    if test_range.contains_range(logical_range) {
                        return true;
                    }
                }
            }
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                if let Some(ternary) = JsConditionalExpression::cast_ref(&ancestor)
                    && let Ok(test) = ternary.test()
                {
                    let test_range = test.syntax().text_trimmed_range();
                    let logical_range = logical.syntax().text_trimmed_range();
                    if test_range.contains_range(logical_range) {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}
