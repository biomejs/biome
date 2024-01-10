use crate::utils::is_node_equal;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_js_syntax::{AnyJsExpression, AnyJsSwitchClause, JsSwitchStatement};
use biome_rowan::{AstNode, TextRange};

declare_rule! {
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
    pub(crate) NoDuplicateCase {
        version: "1.0.0",
        name: "noDuplicateCase",
        source: RuleSource::Eslint("no-duplicate-case"),
        recommended: true,
    }
}

impl Rule for NoDuplicateCase {
    type Query = Ast<JsSwitchStatement>;
    type State = (TextRange, TextRange);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut defined_tests: Vec<AnyJsExpression> = Vec::new();
        let mut signals = Vec::new();
        for case in node.cases() {
            if let AnyJsSwitchClause::JsCaseClause(case) = case {
                if let Ok(test) = case.test() {
                    let define_test = defined_tests
                        .iter()
                        .find(|define_test| is_node_equal(define_test.syntax(), test.syntax()));
                    if let Some(define_test) = define_test {
                        signals.push((define_test.range(), test.range()));
                    } else {
                        defined_tests.push(test);
                    }
                }
            }
        }
        signals
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (first_label_range, label_range) = state;
        Some(
            RuleDiagnostic::new(rule_category!(), label_range, "Duplicate case label.")
                .detail(first_label_range, "The first similar label is here:"),
        )
    }
}
