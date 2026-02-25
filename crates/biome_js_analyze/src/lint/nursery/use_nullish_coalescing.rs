use crate::{JsRuleAction, services::typed::Typed};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsConditionalExpression, JsDoWhileStatement, JsForStatement, JsIfStatement,
    JsLogicalExpression, JsLogicalOperator, JsParenthesizedExpression, JsSyntaxKind,
    JsWhileStatement, T,
};
use biome_js_type_info::ConditionalType;
use biome_rowan::{AstNode, BatchMutationExt, TextRange};
use biome_rule_options::use_nullish_coalescing::UseNullishCoalescingOptions;

declare_lint_rule! {
    /// Enforce using nullish coalescing operator (`??`) instead of logical or (`||`).
    ///
    /// The `??` operator only checks for `null` and `undefined`, while `||` checks
    /// for any falsy value including `0`, `''`, and `false`. This can prevent bugs
    /// where legitimate falsy values are incorrectly treated as missing.
    ///
    /// This rule triggers when the left operand of `||` is possibly nullish (contains
    /// `null` or `undefined` in its type). A safe fix is only offered when the type
    /// analysis confirms the left operand can only be truthy or nullish (not other
    /// falsy values like `0` or `''`).
    ///
    /// By default, `||` expressions in conditional test positions (if/while/for/ternary)
    /// are ignored, as the falsy-checking behavior is often intentional there. This can
    /// be disabled with the `ignoreConditionalTests` option.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default'; // should use ??
    /// ```
    ///
    /// ```ts
    /// declare const maybeNumber: number | undefined;
    /// const value = maybeNumber || 0; // should use ??
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// // Already using ??
    /// declare const maybeString: string | null;
    /// const value = maybeString ?? 'default';
    /// ```
    ///
    /// ```ts
    /// // Type is not nullish - no null or undefined in union
    /// declare const definiteString: string;
    /// const value = definiteString || 'fallback';
    /// ```
    ///
    /// ```ts
    /// // In conditional test position (ignored by default)
    /// declare const cond: string | null;
    /// if (cond || 'fallback') {
    ///   console.log('in if');
    /// }
    /// ```
    ///
    pub UseNullishCoalescing {
        version: "next",
        name: "useNullishCoalescing",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("prefer-nullish-coalescing").inspired()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Types],
        issue_number: Some("8043"),
    }
}

pub struct UseNullishCoalescingState {
    operator_range: TextRange,
    can_fix: bool,
}

impl Rule for UseNullishCoalescing {
    type Query = Typed<JsLogicalExpression>;
    type State = UseNullishCoalescingState;
    type Signals = Option<Self::State>;
    type Options = UseNullishCoalescingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let logical = ctx.query();
        let operator = logical.operator().ok()?;
        if operator != JsLogicalOperator::LogicalOr {
            return None;
        }

        let options = ctx.options();
        if options.ignore_conditional_tests() && is_in_test_position(logical) {
            return None;
        }

        let left = logical.left().ok()?;
        let left_ty = ctx.type_of_expression(&left);

        if !is_possibly_nullish(&left_ty) {
            return None;
        }

        let can_fix = is_safe_type_for_replacement(&left_ty)
            && is_safe_syntax_context_for_replacement(logical);

        Some(UseNullishCoalescingState {
            operator_range: logical.operator_token().ok()?.text_trimmed_range(),
            can_fix,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.operator_range,
                markup! {
                    "Use "<Emphasis>"??"</Emphasis>" instead of "<Emphasis>"||"</Emphasis>"."
                },
            )
            .note(markup! {
                "The "<Emphasis>"||"</Emphasis>" operator checks for all falsy values (including 0, '', and false), while "<Emphasis>"??"</Emphasis>" only checks for null and undefined."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if !state.can_fix {
            return None;
        }

        let node = ctx.query();
        let old_token = node.operator_token().ok()?;

        let new_token = make::token(T![??])
            .with_leading_trivia_pieces(old_token.leading_trivia().pieces())
            .with_trailing_trivia_pieces(old_token.trailing_trivia().pieces());

        let mut mutation = ctx.root().begin();
        mutation.replace_token(old_token, new_token);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"??"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn is_safe_type_for_replacement(ty: &biome_js_type_info::Type) -> bool {
    if ty.is_union() {
        ty.flattened_union_variants().all(|variant| {
            matches!(
                variant.conditional_semantics(),
                ConditionalType::Truthy | ConditionalType::Nullish
            )
        })
    } else {
        matches!(
            ty.conditional_semantics(),
            ConditionalType::Truthy | ConditionalType::Nullish
        )
    }
}

fn is_possibly_nullish(ty: &biome_js_type_info::Type) -> bool {
    if ty.is_union() {
        ty.flattened_union_variants()
            .any(|variant| variant.conditional_semantics().is_nullish())
    } else {
        ty.conditional_semantics().is_nullish()
    }
}

fn is_safe_syntax_context_for_replacement(logical: &JsLogicalExpression) -> bool {
    let is_parenthesized = logical
        .syntax()
        .parent()
        .is_some_and(|parent| JsParenthesizedExpression::can_cast(parent.kind()));

    if !is_parenthesized
        && logical
            .syntax()
            .parent()
            .is_some_and(|parent| JsLogicalExpression::can_cast(parent.kind()))
    {
        return false;
    }

    let left = logical.left().ok();
    let right = logical.right().ok();

    if left
        .as_ref()
        .is_some_and(is_unparenthesized_and_or_expression)
        || right
            .as_ref()
            .is_some_and(is_unparenthesized_and_or_expression)
    {
        return false;
    }

    true
}

fn is_unparenthesized_and_or_expression(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsParenthesizedExpression(_) => false,
        AnyJsExpression::JsLogicalExpression(logical) => {
            logical.operator().ok().is_some_and(|op| {
                matches!(
                    op,
                    JsLogicalOperator::LogicalAnd | JsLogicalOperator::LogicalOr
                )
            })
        }
        _ => false,
    }
}

fn is_in_test_position(logical: &JsLogicalExpression) -> bool {
    let logical_range = logical.syntax().text_trimmed_range();

    let test_contains_logical = |test: Option<AnyJsExpression>| -> bool {
        test.is_some_and(|t| t.syntax().text_trimmed_range().contains_range(logical_range))
    };

    for ancestor in logical.syntax().ancestors() {
        let is_in_test = match ancestor.kind() {
            JsSyntaxKind::JS_IF_STATEMENT => {
                test_contains_logical(JsIfStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()))
            }
            JsSyntaxKind::JS_WHILE_STATEMENT => {
                test_contains_logical(JsWhileStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()))
            }
            JsSyntaxKind::JS_FOR_STATEMENT => {
                test_contains_logical(JsForStatement::cast_ref(&ancestor).and_then(|s| s.test()))
            }
            JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
                test_contains_logical(JsDoWhileStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()))
            }
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                test_contains_logical(JsConditionalExpression::cast_ref(&ancestor).and_then(|s| s.test().ok()))
            }
            _ => false,
        };
        if is_in_test {
            return true;
        }
    }
    false
}
