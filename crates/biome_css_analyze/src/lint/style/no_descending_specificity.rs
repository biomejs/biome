use std::collections::BTreeMap;

use rustc_hash::{FxHashMap, FxHashSet};

use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_semantic::model::{ResolvedSelector, Rule as CssSemanticRule, RuleId, Specificity};
use biome_css_syntax::{
    AnyCssRoot, CssContainerAtRule, CssLanguage, CssLayerAtRule, CssMediaAtRule, CssScopeAtRule,
    CssStartingStyleAtRule, CssSupportsAtRule, CssSyntaxKind, ScssAtRootAtRule, ScssEachAtRule,
    ScssElseClause, ScssForAtRule, ScssFunctionAtRule, ScssIfAtRule, ScssIncludeAtRule,
    ScssMixinAtRule, ScssWhileAtRule,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, SyntaxKindSet, TextRange, TokenText};
use biome_rule_options::no_descending_specificity::NoDescendingSpecificityOptions;

use crate::services::semantic::Semantic;

const INDEPENDENT_AT_RULE_KINDS: SyntaxKindSet<CssLanguage> = CssContainerAtRule::KIND_SET
    .union(CssMediaAtRule::KIND_SET)
    .union(CssScopeAtRule::KIND_SET)
    .union(CssStartingStyleAtRule::KIND_SET)
    .union(CssSupportsAtRule::KIND_SET);

const SCSS_SELECTOR_CONTEXT_KINDS: SyntaxKindSet<CssLanguage> = ScssAtRootAtRule::KIND_SET
    .union(ScssEachAtRule::KIND_SET)
    .union(ScssElseClause::KIND_SET)
    .union(ScssForAtRule::KIND_SET)
    .union(ScssFunctionAtRule::KIND_SET)
    .union(ScssIfAtRule::KIND_SET)
    .union(ScssIncludeAtRule::KIND_SET)
    .union(ScssMixinAtRule::KIND_SET)
    .union(ScssWhileAtRule::KIND_SET);

declare_lint_rule! {
    /// Disallow lower-specificity selectors after higher-specificity selectors.
    ///
    /// Specificity is the priority score CSS calculates from a selector. When two selectors have
    /// the same specificity, the later declaration wins. A selector with higher specificity wins
    /// regardless of source order.
    ///
    /// A lower-specificity selector placed later can therefore look like an override even though it
    /// cannot replace the earlier style. Ordering selectors from lower to higher specificity makes
    /// the cascade easier to read.
    ///
    /// The rule reports likely conflicts between selectors that end with the same target under the
    /// same surrounding rules, such as `@media` or `@layer`. It cannot determine every case where
    /// two selectors match the same element.
    ///
    /// ## SCSS limitations
    ///
    /// This rule does not evaluate SCSS. It compares statically written selectors within the same
    /// SCSS block, but not selectors across mixin or include boundaries, or across mutually exclusive
    /// control-flow branches. Selectors containing interpolation or placeholders are ignored because
    /// their emitted selector and specificity depend on SCSS evaluation.
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
    /// /* The rule cannot determine that these selectors target the same elements. */
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
    /// ```css
    /// @layer one {
    ///   b a { color: green; }
    /// }
    ///
    /// @layer two {
    ///   a { color: blue; }
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
        let mut descending_selectors = BTreeMap::new();

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

            let rule_node = rule.node(&root);
            find_descending_selector(
                &rule,
                SelectorContext {
                    at_rule: at_rule_context,
                    layer: rule_node
                        .syntax()
                        .ancestors()
                        .find_map(CssLayerAtRule::cast)
                        .map(|layer| layer.range()),
                    scss: rule_node
                        .syntax()
                        .ancestors()
                        .find(|ancestor| SCSS_SELECTOR_CONTEXT_KINDS.matches(ancestor.kind()))
                        .map(|ancestor| ancestor.text_trimmed_range()),
                },
                &mut visited_selectors,
                &mut descending_selectors,
            );

            let child_at_rule_context = if INDEPENDENT_AT_RULE_KINDS
                .matches(rule_node.syntax().kind())
            {
                Some(rule.id())
            } else {
                at_rule_context
            };
            for child_id in rule.child_ids().iter().rev() {
                if let Some(child_rule) = model.get_rule_by_id(child_id) {
                    rules.push((child_rule, child_at_rule_context));
                }
            }
        }
        descending_selectors
            .into_values()
            .collect::<Vec<_>>()
            .into_boxed_slice()
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

#[derive(Debug)]
pub struct DescendingSelector {
    high: (TextRange, Specificity),
    low: (TextRange, Specificity),
}

/// Identifies the at-rule and cascade-layer scopes in which selectors can be compared.
#[derive(Eq, Hash, PartialEq)]
struct SelectorContext {
    /// The nearest enclosing at-rule tracked as an independent comparison scope, or `None` at the top level.
    at_rule: Option<RuleId>,
    /// The range of the nearest enclosing `@layer` block, or `None` for unlayered selectors.
    layer: Option<TextRange>,
    /// The nearest enclosing SCSS block whose emitted position requires evaluation.
    scss: Option<TextRange>,
}

type TailSelector = Vec<(CssSyntaxKind, TokenText)>;
type SelectorContexts =
    FxHashMap<SelectorContext, FxHashMap<TailSelector, (TextRange, Specificity)>>;

/// Returns the last top-level compound selector from a resolved selector.
///
/// Combinators inside functional pseudo-classes, such as `>` in `:has(.a > .b)`, remain part of
/// the compound selector.
fn tail_selector(selector: &ResolvedSelector) -> Option<TailSelector> {
    let mut delimiter_depth = 0usize;
    let mut tail_start = 0;

    for (index, (kind, _)) in selector.tokens().enumerate() {
        match kind {
            CssSyntaxKind::L_PAREN | CssSyntaxKind::L_BRACK => delimiter_depth += 1,
            CssSyntaxKind::R_PAREN | CssSyntaxKind::R_BRACK => {
                delimiter_depth = delimiter_depth.saturating_sub(1);
            }
            _ if delimiter_depth == 0 && ResolvedSelector::is_combinator(*kind) => {
                tail_start = index + 1;
            }
            _ => {}
        }
    }

    let tail = selector
        .tokens()
        .skip(tail_start)
        .cloned()
        .collect::<Vec<_>>();
    (!tail.is_empty()).then_some(tail)
}

/// Checks selectors against the highest preceding specificity with the same tail selector in the same at-rule context.
/// If a lower specificity selector is found after a higher specificity selector with the same tail selector, it records this as a descending selector.
fn find_descending_selector(
    rule: &CssSemanticRule,
    context: SelectorContext,
    visited_selectors: &mut SelectorContexts,
    descending_selectors: &mut BTreeMap<TextRange, DescendingSelector>,
) {
    let visited_selectors = visited_selectors.entry(context).or_default();

    for selector in rule.selectors() {
        let resolved_selector = selector.resolved();
        let resolved_selector_text = resolved_selector.to_string();
        // SCSS placeholders may not emit CSS, and interpolation prevents static specificity.
        if resolved_selector_text.contains('%') || resolved_selector_text.contains("#{") {
            continue;
        }
        let Some(tail_selector) = tail_selector(resolved_selector) else {
            continue;
        };

        if let Some(seen) = visited_selectors.get_mut(&tail_selector) {
            let (last_text_range, last_specificity) = *seen;
            let specificity = selector.specificity();
            if last_specificity > specificity {
                let selector_range = selector.range();
                if let Some(descending_selector) = descending_selectors.get_mut(&selector_range) {
                    if last_specificity > descending_selector.high.1 {
                        descending_selector.high = (last_text_range, last_specificity);
                    }
                } else {
                    descending_selectors.insert(
                        selector_range,
                        DescendingSelector {
                            high: (last_text_range, last_specificity),
                            low: (selector_range, specificity),
                        },
                    );
                }
            } else if specificity > last_specificity {
                *seen = (selector.range(), specificity);
            }
        } else {
            visited_selectors.insert(
                tail_selector,
                (selector.range(), selector.specificity()),
            );
        }
    }
}
