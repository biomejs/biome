use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_semantic::model::{AnyRuleStart, RuleId};
use biome_css_syntax::AnyCssRoot;
use biome_diagnostics::Severity;
use biome_rowan::TextRange;
use rustc_hash::{FxHashMap, FxHashSet};

use biome_rule_options::no_duplicate_selectors::NoDuplicateSelectorsOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow duplicate selectors.
    ///
    /// This rule checks for duplicate selectors across a stylesheet. A duplicate
    /// occurs when two rules have the same fully-resolved selector list (after
    /// sorting for order-independence), within the same at-rule context.
    ///
    /// Selectors are **normalized** before comparison so that equivalent selectors
    /// written differently are still caught:
    /// - Whitespace differences are ignored.
    /// - HTML type selectors are compared case-insensitively (`DIV` == `div`).
    /// - Compound selector parts are order-normalized, so `.a.b` and `.b.a` are
    ///   considered equal.
    /// - Selector list order is ignored: `.a, .b` and `.b, .a` are equal.
    ///
    /// CSS nesting is fully resolved before comparison:
    /// `a { & b {} }` produces the resolved selector `a b`, which is compared
    /// against all other resolved selectors in the same context.
    ///
    /// Selectors are only compared within the same at-rule context. A selector
    /// inside `@media` is not considered a duplicate of the same selector at the
    /// top level.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .foo {}
    /// .foo {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// .foo, .bar {}
    /// .bar, .foo {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a b {}
    /// a {
    ///   & b {}
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// .foo {}
    /// .bar {}
    /// ```
    ///
    /// ```css
    /// .foo {}
    /// @media (min-width: 600px) {
    ///   .foo {}
    /// }
    /// ```
    ///
    /// ```css
    /// .foo {
    ///   .foo {}
    /// }
    /// ```
    ///
    pub NoDuplicateSelectors {
        version: "2.4.9",
        name: "noDuplicateSelectors",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::Stylelint("no-duplicate-selectors").same()],
    }
}

/// A single diagnostic: the duplicate rule's range and the first occurrence's range.
#[derive(Debug)]
pub struct DuplicateSelectorList {
    /// Range of the rule whose selector list is a duplicate.
    duplicate_range: TextRange,
    /// Range of the first rule with this selector list.
    first_range: TextRange,
    /// Normalized selector list (for the diagnostic message).
    normalized_list: String,
}

impl Rule for NoDuplicateSelectors {
    type Query = Semantic<AnyCssRoot>;
    type State = DuplicateSelectorList;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateSelectorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let root = ctx.root();

        // Maps (at_rule_context, normalized_selector_list_key) → first occurrence range.
        let mut seen: FxHashMap<(Vec<RuleId>, String), TextRange> = FxHashMap::default();
        let mut duplicates: Vec<DuplicateSelectorList> = Vec::new();
        let mut visited: FxHashSet<RuleId> = FxHashSet::default();

        // Iterative DFS using an explicit stack with sentinel frames for context pop.
        enum Frame {
            Visit(RuleId),
            /// Sentinel: pop one entry from `at_rule_context` after a subtree is done.
            PopAtRule,
        }

        let mut at_rule_context: Vec<RuleId> = Vec::new();
        // Seed the stack with all top-level rules (reversed so they process in order).
        let mut stack: Vec<Frame> = model
            .rules()
            .iter()
            .rev()
            .map(|r| Frame::Visit(r.id()))
            .collect();

        while let Some(frame) = stack.pop() {
            match frame {
                Frame::PopAtRule => {
                    at_rule_context.pop();
                }
                Frame::Visit(rule_id) => {
                    let rule = match model.get_rule_by_id(&rule_id) {
                        Some(r) => r,
                        None => continue,
                    };

                    if !visited.insert(rule.id()) {
                        continue;
                    }

                    let is_at_rule = matches!(
                        rule.node(&root),
                        AnyRuleStart::CssMediaAtRule(_)
                            | AnyRuleStart::CssSupportsAtRule(_)
                            | AnyRuleStart::CssContainerAtRule(_)
                            | AnyRuleStart::CssStartingStyleAtRule(_)
                    );

                    if is_at_rule {
                        at_rule_context.push(rule.id());
                        // Push a sentinel so we pop after this subtree is processed.
                        stack.push(Frame::PopAtRule);
                    }

                    // Push children in reverse order so they process left-to-right.
                    for child_id in rule.child_ids().iter().rev() {
                        stack.push(Frame::Visit(*child_id));
                    }

                    // For qualified rules with selectors, check for duplicates.
                    if !is_at_rule && !rule.selectors().is_empty() {
                        let mut normalized_selectors: Vec<String> = rule
                            .selectors()
                            .iter()
                            .map(|s| s.resolved().normalize())
                            .collect();
                        normalized_selectors.sort();

                        let list_key = normalized_selectors.join(", ");
                        let context_key = (at_rule_context.clone(), list_key.clone());
                        let rule_range = rule.range(&root);

                        match seen.get(&context_key) {
                            Some(&first_range) => {
                                duplicates.push(DuplicateSelectorList {
                                    duplicate_range: rule_range,
                                    first_range,
                                    normalized_list: list_key,
                                });
                            }
                            None => {
                                seen.insert(context_key, rule_range);
                            }
                        }
                    }
                }
            }
        }

        duplicates.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.duplicate_range,
                markup! {
                    "Duplicate selector list \""<Emphasis>{state.normalized_list.as_str()}</Emphasis>"\"."
                },
            )
            .detail(
                state.first_range,
                markup! {
                    "It was first defined here."
                },
            )
            .note(markup! {
                "Remove or merge the duplicate rule to keep the stylesheet clean."
            }),
        )
    }
}
