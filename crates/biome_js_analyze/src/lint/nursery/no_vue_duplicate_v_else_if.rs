use crate::services::vue::{EarlierCondition, VueElseIf};
use crate::utils::condition::split_by_logical_operator;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsLogicalOperator};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_vue_duplicate_v_else_if::NoVueDuplicateVElseIfOptions;

declare_lint_rule! {
    /// Disallow duplicate conditions in `v-if` / `v-else-if` chains.
    ///
    /// Vue renders only the first element of a `v-if` / `v-else-if` chain whose condition is
    /// true. When a `v-else-if` repeats a condition from earlier in the same chain, its element
    /// can never be rendered: whenever its condition is true, the earlier one is true too. This
    /// is almost always a copy-paste mistake.
    ///
    /// The rule also understands `||` and `&&`, so it reports conditions that earlier
    /// conditions already cover, even when they aren't written the same way. For example,
    /// `v-else-if="a"` can never be reached after `v-if="a || b"`, and `v-else-if="a && b"`
    /// can never be reached after `v-if="a"`.
    ///
    /// The rule assumes that evaluating a condition twice gives the same result both times.
    /// A condition that calls a function returning a different value on every call, such as
    /// `Math.random() > 0.5`, can be reported even though its element can be rendered.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-if="isSomething(x)" />
    ///   <div v-else-if="isSomething(x)" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-if="n === 1" />
    ///   <div v-else-if="n === 2" />
    ///   <div v-else-if="n === 1" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-if="a || b" />
    ///   <div v-else-if="a" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-if="a" />
    ///   <div v-else-if="a && b" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div v-if="isSomething(x)" />
    ///   <div v-else-if="isSomethingElse(x)" />
    ///
    ///   <div v-if="n === 1" />
    ///   <div v-else-if="n === 2" />
    ///   <div v-else-if="n === 3" />
    ///
    ///   <div v-if="a && b" />
    ///   <div v-else-if="a" />
    /// </template>
    /// ```
    ///
    /// ## See Also
    ///
    /// - For the same check on `if` statements in JavaScript, see [`noDuplicateElseIf`](https://biomejs.dev/linter/rules/no-duplicate-else-if/).
    ///
    pub NoVueDuplicateVElseIf {
        version: "next",
        name: "noVueDuplicateVElseIf",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-dupe-v-else-if").same()],
    }
}

impl Rule for NoVueDuplicateVElseIf {
    type Query = VueElseIf;
    /// The range of the condition, or of one of its `&&` operands, that the
    /// earlier conditions of its chain cover.
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoVueDuplicateVElseIfOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let condition = query.condition();

        // The whole condition is checked, and when it is an `&&`, each of its
        // operands is checked too: `d && (a || b)` can't be true after
        // `v-if="a"` and `v-else-if="b"`, because its operand `a || b` can't.
        let mut parts: Vec<_> =
            split_by_logical_operator(JsLogicalOperator::LogicalAnd, condition.clone()).collect();
        if parts.len() == 1 {
            parts.clear();
        }
        parts.push(condition.clone());
        let mut remaining: Vec<_> = parts.iter().map(|part| alternatives(part.clone())).collect();

        for earlier in query.earlier_conditions() {
            let covering = earlier_alternatives(earlier);
            for (part, alternatives) in parts.iter().zip(&mut remaining) {
                alternatives.retain(|alternative| {
                    !covering
                        .iter()
                        .any(|covering| is_covered_by(alternative, covering))
                });
                if alternatives.is_empty() {
                    return Some(part.range());
                }
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This condition is a duplicate of, or is covered by, earlier conditions in the "<Emphasis>"v-if"</Emphasis>" / "<Emphasis>"v-else-if"</Emphasis>" chain."
                },
            )
            .note(markup! {
                "Whenever this condition is true, an earlier condition of the chain is true too, so this element is never rendered."
            })
            .note(markup! {
                "Remove this element, or change its condition to the one you meant."
            }),
        )
    }
}

/// Splits `condition` into its `||` operands, then each of them into its
/// `&&` operands. The condition is true when all the `&&` operands of one of
/// its `||` operands are.
fn alternatives(condition: AnyJsExpression) -> Vec<Vec<AnyJsExpression>> {
    split_by_logical_operator(JsLogicalOperator::LogicalOr, condition)
        .map(|alternative| {
            split_by_logical_operator(JsLogicalOperator::LogicalAnd, alternative).collect()
        })
        .collect()
}

/// Splits an earlier condition the way [`alternatives`] splits a condition.
fn earlier_alternatives(condition: &EarlierCondition) -> Vec<Vec<EarlierCondition>> {
    condition
        .split(JsLogicalOperator::LogicalOr)
        .map(|alternative| alternative.split(JsLogicalOperator::LogicalAnd).collect())
        .collect()
}

/// Returns whether `alternative` contains every operand of `covering`, so
/// that `covering` is true whenever `alternative` is.
fn is_covered_by(alternative: &[AnyJsExpression], covering: &[EarlierCondition]) -> bool {
    covering.iter().all(|operand| {
        alternative
            .iter()
            .any(|expression| operand.is_same_condition(expression))
    })
}
