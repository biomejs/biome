use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_semantic::model::{AnyRuleStart, RuleId};
use biome_css_syntax::{AnyCssRoot, CssSyntaxKind};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::hash::{BuildHasher, Hasher};

use biome_rule_options::no_duplicate_selectors::NoDuplicateSelectorsOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow duplicate selectors.
    ///
    /// Two rules are duplicates when their selectors target the same elements within the same
    /// surrounding at-rules. The comparison treats equivalent spellings as equal:
    ///
    /// - whitespace differences are ignored;
    /// - HTML element names are compared without case (`DIV` equals `div`);
    /// - the order of combined parts is ignored (`.a.b` equals `.b.a`);
    /// - the order of a selector list is ignored (`.a, .b` equals `.b, .a`).
    ///
    /// Nested selectors are expanded before comparison. For example, `a { & b {} }` is compared as
    /// `a b`. A selector inside an at-rule such as `@media` is compared only with selectors inside
    /// the same at-rule, not with a matching selector at the top level.
    ///
    /// ## Sass limitations
    ///
    /// This rule does not evaluate Sass. It compares selectors within the same Sass block, but does
    /// not expand mixins, includes, or `@extend`. Selectors containing interpolation or placeholders
    /// are ignored because their emitted form cannot be determined statically.
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

fn scss_selector_context(rule: &AnyRuleStart) -> Option<TextRange> {
    rule.syntax()
        .ancestors()
        .find(|ancestor| {
            matches!(
                ancestor.kind(),
                CssSyntaxKind::SCSS_AT_ROOT_AT_RULE
                    | CssSyntaxKind::SCSS_EACH_AT_RULE
                    | CssSyntaxKind::SCSS_ELSE_CLAUSE
                    | CssSyntaxKind::SCSS_FOR_AT_RULE
                    | CssSyntaxKind::SCSS_FUNCTION_AT_RULE
                    | CssSyntaxKind::SCSS_IF_AT_RULE
                    | CssSyntaxKind::SCSS_INCLUDE_AT_RULE
                    | CssSyntaxKind::SCSS_MIXIN_AT_RULE
                    | CssSyntaxKind::SCSS_WHILE_AT_RULE
            )
        })
        .map(|ancestor| ancestor.text_trimmed_range())
}

impl Rule for NoDuplicateSelectors {
    type Query = Semantic<AnyCssRoot>;
    type State = DuplicateSelectorList;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateSelectorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let root = ctx.root();

        // Maps (context_hash, normalized_selector_list_key) → first occurrence range.
        let mut seen = FxHashMap::default();
        let mut duplicates = Vec::new();
        let mut visited = FxHashSet::default();

        // Iterative DFS using an explicit stack with sentinel frames for context pop.
        enum Frame {
            Visit(RuleId),
            /// Sentinel: pop one entry from the context hash stack after a subtree is done.
            PopAtRule,
        }

        // Stack of running hashes representing the at-rule nesting context.
        // Seed with 0 for top-level (no at-rule context).
        let mut context_hash_stack = vec![0u64];
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
                    context_hash_stack.pop();
                }
                Frame::Visit(rule_id) => {
                    let rule = match model.get_rule_by_id(&rule_id) {
                        Some(r) => r,
                        None => continue,
                    };

                    if !visited.insert(rule.id()) {
                        continue;
                    }

                    let rule_node = rule.node(&root);
                    let is_at_rule = matches!(
                        &rule_node,
                        AnyRuleStart::CssMediaAtRule(_)
                            | AnyRuleStart::CssSupportsAtRule(_)
                            | AnyRuleStart::CssContainerAtRule(_)
                            | AnyRuleStart::CssScopeAtRule(_)
                            | AnyRuleStart::CssStartingStyleAtRule(_)
                    );

                    if is_at_rule {
                        let parent_hash = *context_hash_stack.last().unwrap();
                        let mut hasher = FxBuildHasher.build_hasher();
                        hasher.write_u64(parent_hash);
                        hasher.write_u32(rule.id().index() as u32);
                        context_hash_stack.push(hasher.finish());
                        stack.push(Frame::PopAtRule);
                    }

                    // Push children in reverse order so they process left-to-right.
                    for child_id in rule.child_ids().iter().rev() {
                        stack.push(Frame::Visit(*child_id));
                    }

                    // For qualified rules with selectors, check for duplicates.
                    if !is_at_rule && !rule.selectors().is_empty() {
                        if rule.selectors().iter().any(|selector| {
                            let selector = selector.resolved().to_string();
                            selector.contains('%') || selector.contains("#{")
                        }) {
                            continue;
                        }
                        let mut normalized_selectors: Vec<String> = rule
                            .selectors()
                            .iter()
                            .map(|s| s.resolved().normalize())
                            .collect();
                        normalized_selectors.sort();

                        let list_key = normalized_selectors.join(", ");
                        let context_hash = *context_hash_stack.last().unwrap();
                        let context_key = (
                            context_hash,
                            scss_selector_context(&rule_node),
                            list_key.clone(),
                        );
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
