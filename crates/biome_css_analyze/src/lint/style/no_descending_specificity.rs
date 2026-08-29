use rustc_hash::{FxHashMap, FxHashSet};

use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_semantic::model::{
    AnyRuleStart, Rule as CssSemanticRule, RuleId, Specificity,
};
use biome_css_syntax::{AnyCssRoot, AnyCssSelector};
use biome_diagnostics::Severity;
use biome_rowan::TextRange;

use biome_rowan::AstNode;
use biome_rule_options::no_descending_specificity::NoDescendingSpecificityOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow a lower specificity selector from coming after a higher specificity selector.
    ///
    /// Source order is important in CSS, and when two selectors have the same specificity, the one that occurs last will take priority.
    /// However, the situation is different when one of the selectors has a higher specificity.
    /// In that case, source order does not matter: the selector with higher specificity will win out even if it comes first.
    ///
    /// The clashes of these two mechanisms for prioritization, source order and specificity, can cause some confusion when reading stylesheets.
    /// If a selector with higher specificity comes before the selector it overrides, we have to think harder to understand it, because it violates the source order expectation.
    /// **Stylesheets are most legible when overriding selectors always come after the selectors they override.**
    /// That way both mechanisms, source order and specificity, work together nicely.
    ///
    /// This rule enforces that practice as best it can, reporting fewer errors than it should.
    /// It cannot catch every actual overriding selector, but it can catch certain common mistakes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// b a { color: red; }
    /// a { color: red; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   & > b { color: red; }
    /// }
    /// b { color: red; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// :root input {
    ///     color: red;
    /// }
    /// html input {
    ///     color: red;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// .a th {
    ///   color: red;
    /// }
    ///
    /// .a .b .c th {
    ///   color: green;
    /// }
    ///
    /// .a .b th {
    ///   color: blue;
    /// }
    /// ```
    ///
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { color: red; }
    /// b a { color: red; }
    /// ```
    ///
    /// ```css
    /// b { color: red; }
    /// a {
    ///   & > b { color: red; }
    /// }
    /// ```
    ///
    /// ```css
    /// a:hover { color: red; }
    /// a { color: red; }
    /// ```
    ///
    /// ```css
    /// a b {
    ///     color: red;
    /// }
    /// /* This selector is overwritten by the one above it, but this is not an error because the rule only evaluates it as a compound selector */
    /// :where(a) :is(b) {
    ///     color: blue;
    /// }
    /// ```
    ///
    /// ```css
    /// .a th {
    ///   color: red;
    /// }
    ///
    /// @media print {
    ///   .a .b .c th {
    ///     color: green;
    ///   }
    /// }
    /// ```
    ///
    pub NoDescendingSpecificity {
        version: "1.9.3",
        name: "noDescendingSpecificity",
        language: "css",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::Stylelint("no-descending-specificity").same()],
    }
}

#[derive(Debug)]
pub struct DescendingSelector {
    high: (TextRange, Specificity),
    low: (TextRange, Specificity),
}

// `None` represents the top-level comparison context, which has no enclosing at-rule.
type SelectorContexts = FxHashMap<Option<RuleId>, FxHashMap<String, (TextRange, Specificity)>>;

impl Rule for NoDescendingSpecificity {
    type Query = Semantic<AnyCssRoot>;
    type State = DescendingSelector;
    type Signals = Box<[Self::State]>;
    type Options = NoDescendingSpecificityOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let root = ctx.root();
        let mut visited_rules = FxHashSet::default();
        let mut visited_selectors = SelectorContexts::default();
        let mut descending_selectors = Vec::new();

        let mut rules = model
            .rules()
            .into_iter()
            .rev()
            .map(|rule| (rule, None))
            .collect::<Vec<_>>();
        while let Some((rule, at_rule_context)) = rules.pop() {
            if !visited_rules.insert(rule.id()) {
                continue;
            }

            find_descending_selector(
                &root,
                &rule,
                at_rule_context,
                &mut visited_selectors,
                &mut descending_selectors,
            );

            let child_at_rule_context = match rule.node(&root) {
                AnyRuleStart::CssContainerAtRule(_)
                | AnyRuleStart::CssMediaAtRule(_)
                | AnyRuleStart::CssScopeAtRule(_)
                | AnyRuleStart::CssStartingStyleAtRule(_)
                | AnyRuleStart::CssSupportsAtRule(_) => Some(rule.id()),
                _ => at_rule_context,
            };
            for child_id in rule.child_ids().iter().rev() {
                if let Some(child_rule) = model.get_rule_by_id(child_id) {
                    rules.push((child_rule, child_at_rule_context));
                }
            }
        }
        descending_selectors.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
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
                    "Descending specificity selector may not be applied. Consider rearranging the order of the selectors. See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/Specificity">"MDN web docs"</Hyperlink>" for more details."
            }),
        )
    }
}

/// find tail selector
/// ```css
/// a b:hover {
///   ^^^^^^^
/// }
/// ```
fn find_tail_selector_str(selector: &AnyCssSelector) -> Option<String> {
    match selector {
        AnyCssSelector::CssCompoundSelector(s) => {
            let mut result = String::new();
            if let Some(simple) = s.simple_selector() {
                simple.syntax().text_trimmed().for_each_chunk(|chunk| {
                    result.push_str(chunk);
                });
            }

            s.sub_selectors()
                .syntax()
                .text_trimmed()
                .for_each_chunk(|chunk| result.push_str(chunk));

            Some(result)
        }
        AnyCssSelector::CssComplexSelector(s) => {
            // negligible recursion
            s.right().as_ref().ok().and_then(find_tail_selector_str)
        }
        _ => None,
    }
}

/// Checks selectors against the highest preceding specificity with the same tail selector in the same at-rule context.
/// If a lower specificity selector is found after a higher specificity selector with the same tail selector, it records this as a descending selector.
fn find_descending_selector(
    root: &AnyCssRoot,
    rule: &CssSemanticRule,
    at_rule_context: Option<RuleId>,
    visited_selectors: &mut SelectorContexts,
    descending_selectors: &mut Vec<DescendingSelector>,
) {
    let visited_selectors = visited_selectors.entry(at_rule_context).or_default();

    for selector in rule.selectors() {
        let Some(casted_selector) = AnyCssSelector::cast(selector.node(root).syntax().clone())
        else {
            continue;
        };
        let Some(tail_selector_str) = find_tail_selector_str(&casted_selector) else {
            continue;
        };

        if let Some(seen) = visited_selectors.get_mut(&tail_selector_str) {
            let (last_text_range, last_specificity) = *seen;
            let specificity = selector.specificity();
            if last_specificity > specificity {
                descending_selectors.push(DescendingSelector {
                    high: (last_text_range, last_specificity),
                    low: (selector.range(root), specificity),
                });
            } else if specificity > last_specificity {
                *seen = (selector.range(root), specificity);
            }
        } else {
            visited_selectors.insert(
                tail_selector_str,
                (selector.range(root), selector.specificity()),
            );
        }
    }
}
