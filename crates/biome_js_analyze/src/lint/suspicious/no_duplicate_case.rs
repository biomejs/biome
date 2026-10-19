use crate::utils::{is_node_equal, node_hash};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, AnyJsSwitchClause, JsSwitchStatement};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_duplicate_case::NoDuplicateCaseOptions;
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Disallow duplicate case labels.
    ///
    /// If a switch statement has duplicate test expressions in case clauses, it is likely that a programmer copied a case clause but forgot to change the test expression.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (a) {
    ///     case 1:
    ///         break;
    ///     case 1:
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (a) {
    ///     case one:
    ///         break;
    ///     case one:
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (a) {
    ///     case "1":
    ///         break;
    ///     case "1":
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (a) {
    ///     case 1:
    ///         break;
    ///     case 2:
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js
    /// switch (a) {
    ///     case one:
    ///         break;
    ///     case two:
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js
    /// switch (a) {
    ///     case "1":
    ///         break;
    ///     case "2":
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    pub NoDuplicateCase {
        version: "1.0.0",
        name: "noDuplicateCase",
        language: "js",
        sources: &[RuleSource::Eslint("no-duplicate-case").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoDuplicateCase {
    type Query = Ast<JsSwitchStatement>;
    type State = (TextRange, TextRange);
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateCaseOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // Distinct tests grouped by `node_hash`, each group in source order, so the first
        // equal test of a group is the first equal test of the switch.
        let mut defined_tests: FxHashMap<_, Vec<AnyJsExpression>> = FxHashMap::default();
        let mut signals = Vec::new();
        for case in node.cases() {
            if let AnyJsSwitchClause::JsCaseClause(case) = case
                && let Ok(test) = case.test()
            {
                let same_hash_tests = defined_tests.entry(node_hash(test.syntax())).or_default();
                let define_test = same_hash_tests
                    .iter()
                    .find(|define_test| is_node_equal(define_test.syntax(), test.syntax()));
                if let Some(define_test) = define_test {
                    signals.push((define_test.range(), test.range()));
                } else {
                    same_hash_tests.push(test);
                }
            }
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (first_label_range, label_range) = state;
        Some(
            RuleDiagnostic::new(rule_category!(), label_range, "Duplicate case label.")
                .detail(first_label_range, "The first similar label is here:")
                .note(markup! {
                    "Only the first matching "<Emphasis>"case"</Emphasis>" is selected. If it does not break or return, execution can fall through to this duplicate's body. It is probably a copy-paste mistake."
                })
                .note(markup! {
                    "Remove the duplicate "<Emphasis>"case"</Emphasis>", or change its value to the one you meant."
                }),
        )
    }
}
