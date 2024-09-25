use std::collections::{BTreeMap, BTreeSet};

use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_semantic::model::{Rule as CssSemanticRule, RuleId, Specificity};
use biome_css_syntax::CssRoot;
use biome_rowan::TextRange;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow a lower specificity selector from coming after a higher specificity selector.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding stylelint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   color: red;
    /// }
    /// ```
    ///
    pub NoDescendingSpecificity {
        version: "next",
        name: "noDescendingSpecificity",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("no-descending-specificity")],
    }
}

#[derive(Debug)]
pub struct DescendingSelector {
    high: (TextRange, Specificity),
    low: (TextRange, Specificity),
}

fn found_descending_selector(
    rule: &CssSemanticRule,
    visited_rules: &mut BTreeSet<RuleId>,
    visited_selectors: &mut BTreeMap<String, (TextRange, Specificity)>,
    descending_selectors: &mut Vec<DescendingSelector>,
) {
    if visited_rules.contains(&rule.id) {
        return;
    } else {
        visited_rules.insert(rule.id);
    };

    for selector in &rule.selectors {
        if let Some((last_textrange, last_specificity)) = visited_selectors.get(&selector.name) {
            if last_specificity > &selector.specificity {
                descending_selectors.push(DescendingSelector {
                    high: (*last_textrange, last_specificity.clone()),
                    low: (selector.range, selector.specificity.clone()),
                });
            } else {
                visited_selectors.insert(
                    selector.name.clone(),
                    (selector.range, selector.specificity.clone()),
                );
            }
        }
    }
}

impl Rule for NoDescendingSpecificity {
    type Query = Semantic<CssRoot>;
    type State = DescendingSelector;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let mut visited_rules = BTreeSet::new();
        let mut visited_selectors = BTreeMap::new();
        let mut descending_selectors = Vec::new();
        for rule in model.rules() {
            found_descending_selector(
                rule,
                &mut visited_rules,
                &mut visited_selectors,
                &mut descending_selectors,
            );
        }

        descending_selectors
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.low.0,
                markup! {
                    "Descending specificity selector found. This selector specificity is "{node.low.1.to_string()}
                },
            ).detail(node.high.0, markup!(
                "This selector specificity is "{node.high.1.to_string()}
            ))
            .note(markup! {
                    "Consider rearranging the order of the selectors."
            }),
        )
    }
}
