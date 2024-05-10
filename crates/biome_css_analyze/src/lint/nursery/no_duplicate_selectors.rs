use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::vec;

use biome_analyze::Ast;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssAtRule, AnyCssRelativeSelector, AnyCssRule, AnyCssSelector, CssComplexSelector,
    CssRelativeSelector, CssRelativeSelectorList, CssRoot, CssSelectorList, CssSyntaxNode,
};
use biome_deserialize_macros::Deserializable;
use biome_rowan::{declare_node_union, AstNode, SyntaxNodeCast};

use serde::{Deserialize, Serialize};

declare_rule! {
    /// Disallow duplicate selectors. This rule checks for two types of duplication:
    /// - Duplication of a selector list within a stylesheet, e.g. `a, b {} a, b {}`. Duplicates are found even if the selectors come in different orders or have different spacing, e.g. `a d, b > c {} b>c, a d {}`.
    /// - Duplication of a single selector with a rule's selector list, e.g. `a, b, a {}`.  (See options below, this is disabled by default)
    ///
    /// The same selector is allowed to repeat in the following circumstances:
    /// - It is used in different selector lists, e.g. `a {} a, b {}`.
    /// - The duplicates are in rules with different parent nodes, e.g. inside and outside of a media query.
    ///
    /// This rule resolves nested selectors. So `a b {} a { & b {} }` counts as a problem, because the resolved selectors end up with a duplicate.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .abc,
    /// .def,
    /// .abc { /* declaration */ }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```
    /// .foo { /* declaration */ }
    /// .bar { /* declaration */ }
    /// ```
    ///
    /// ## Options
    ///
    /// If true, disallow duplicate selectors within selector lists. The following settings:
    ///
    /// ```json5, ignore
    /// {
    ///     "noDuplicateSelectors": {
    ///         "options": {
    ///           "disallowInList": true
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// Will result in the following failing:
    ///
    /// ```css, ignore
    /// input, textarea {}; textarea {}
    /// ```
    ///
    pub NoDuplicateSelectors {
        version: "next",
        name: "noDuplicateSelectors",
        recommended: true,
        sources: &[RuleSource::Stylelint("no-duplicate-selectors")],
    }
}

#[derive(Debug, Default, Clone, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoDuplicateSelectorsOptions {
    /// If set to `true` this rule will check for duplicate selectors within selector lists.
    pub disallow_in_list: bool,
}

declare_node_union! {
    pub AnySelectorLike = AnyCssSelector | AnyCssRelativeSelector
}

/// Object containing the resolved selector and the relative node for that resolved selector.
///
/// This struct has a hash function which returns the hash of `selector_node`.
#[derive(Debug, Eq)]
struct ResolvedSelector {
    selector_text: String,
    selector_node: CssSyntaxNode,
}

impl PartialEq for ResolvedSelector {
    fn eq(&self, other: &ResolvedSelector) -> bool {
        self.selector_text == other.selector_text
    }
}
impl Hash for ResolvedSelector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.selector_text.hash(state);
    }
}
impl Borrow<String> for ResolvedSelector {
    fn borrow(&self) -> &String {
        &self.selector_text
    }
}

pub struct DuplicateSelector {
    first: CssSyntaxNode,
    duplicate: CssSyntaxNode,
}

impl Rule for NoDuplicateSelectors {
    type Query = Ast<CssRoot>;
    type State = DuplicateSelector;
    type Signals = Vec<Self::State>;
    type Options = NoDuplicateSelectorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let node = ctx.query();
        let options = ctx.options();

        let mut resolved_list: HashSet<ResolvedSelector> = HashSet::new();
        let mut output: Vec<DuplicateSelector> = vec![];

        if options.disallow_in_list {
            let selectors = node
                .rules()
                .syntax()
                .descendants()
                .filter_map(|x| AnySelectorLike::cast_ref(&x));

            for (selector, selector_list) in selectors.filter_map(|selector| {
                let parent = selector.clone().into_syntax().parent()?;
                if parent.clone().cast::<CssComplexSelector>().is_some()
                    || parent.clone().cast::<CssRelativeSelector>().is_some()
                {
                    return None;
                }
                Some((selector, parent))
            }) {
                let Some(this_rule) = selector_list.parent() else {
                    continue;
                };

                let selector_text = match selector.clone() {
                    AnySelectorLike::AnyCssSelector(selector) => {
                        normalize_complex_selector(selector)
                    }
                    AnySelectorLike::AnyCssRelativeSelector(selector) => selector.text(),
                };

                for r in resolve_nested_selectors(selector_text, this_rule) {
                    let split: Vec<&str> = r.split_whitespace().collect();
                    let normalized = split.join(" ").to_lowercase();

                    if let Some(first) = resolved_list.get(&normalized) {
                        output.push(DuplicateSelector {
                            first: first.selector_node.clone(),
                            duplicate: selector.clone().into_syntax(),
                        });
                    } else {
                        resolved_list.insert(ResolvedSelector {
                            selector_text: normalized.clone(),
                            selector_node: selector.clone().into_syntax(),
                        });
                    }
                }
            }
        } else {
            // Union node with CssSelectorList and CssRelativeSelectorList does not have overlapping From/Into
            let selector_lists = node.rules().syntax().descendants().filter(|x| {
                x.clone().cast::<CssSelectorList>().is_some()
                    || x.clone().cast::<CssRelativeSelectorList>().is_some()
            });

            for (selector_list, rule) in selector_lists.filter_map(|selector_list| {
                let parent = selector_list.clone().parent()?;
                Some((selector_list, parent))
            }) {
                let mut this_list_resolved_list: HashSet<ResolvedSelector> = HashSet::new();

                let mut selector_list_mapped: Vec<String> = selector_list
                    .clone()
                    .children()
                    .filter_map(|child| {
                        let selector_text = if let Some(selector) = AnyCssSelector::cast_ref(&child)
                        {
                            normalize_complex_selector(selector.clone())
                        } else {
                            child
                                .clone()
                                .cast::<AnyCssRelativeSelector>()
                                .unwrap()
                                .text()
                        };

                        if let Some(first) = this_list_resolved_list.get(&selector_text) {
                            output.push(DuplicateSelector {
                                first: first.selector_node.clone(),
                                duplicate: child.clone(),
                            });
                            return None;
                        }

                        this_list_resolved_list.insert(ResolvedSelector {
                            selector_text: selector_text.clone(),
                            selector_node: child,
                        });
                        Some(selector_text)
                    })
                    .collect();
                selector_list_mapped.sort();

                for r in resolve_nested_selectors(selector_list_mapped.join(","), rule) {
                    let split: Vec<&str> = r.split_whitespace().collect();
                    let normalized = split.join(" ").to_lowercase();
                    if let Some(first) = resolved_list.get(&normalized) {
                        output.push(DuplicateSelector {
                            first: first.selector_node.clone(),
                            duplicate: selector_list.clone(),
                        });
                    } else {
                        resolved_list.insert(ResolvedSelector {
                            selector_text: normalized.clone(),
                            selector_node: selector_list.clone(),
                        });
                    }
                }
            }
        }
        output
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let duplicate_text = if let Some(duplicate) = AnySelectorLike::cast_ref(&node.duplicate) {
            duplicate.text()
        } else if let Some(duplicate) = CssSelectorList::cast_ref(&node.duplicate) {
            duplicate.text()
        } else if let Some(duplicate) = CssRelativeSelectorList::cast_ref(&node.duplicate) {
            duplicate.text()
        } else {
            node.duplicate.to_string()
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.duplicate.text_trimmed_range(),
                markup! {
                    "Duplicate selectors may result in unintentionally overriding rules: "<Emphasis>{ duplicate_text }</Emphasis>
                },
            )
            .detail(node.first.text_trimmed_range(), "Please consider moving the rule's contents to the first occurence:")
            .note(markup! {
                "Remove duplicate selectors within the rule"
            }),
        )
    }
}

/// Resolves nested selectors into an equivalent flat selector.
/// If there is no parent rule, return the selector string originally passed.
/// E.g.
/// ```css, ignore
/// a { b { c { /* declaration */ } } }
/// ```
///
/// is resolved to:
/// ```css, ignore
/// a b c {
///     /* declaration */
/// }
/// ```
///
/// When trying to resolve this_rule of type [AnyCssAtRule], this function will generate a hash to replace the selector name:
/// ```css, ignore
/// @media print {
///     selector { /* declaration */}
/// }
/// ```
///
/// is resolved to:
/// ```css, ignore
/// <hash> selector { /* declaration */}
/// ```
///
/// [AnyCssAtRule] is resolved based on the text range.
/// The same rule will not be resolved to the same hash because these are considered to belong to a separate context.
///
/// Returns the resolved selector as a string.
fn resolve_nested_selectors(selector: String, this_rule: CssSyntaxNode) -> Vec<String> {
    let mut parent_selectors: Vec<String> = vec![];
    let parent_rule = this_rule.parent().and_then(|parent| parent.grand_parent());

    match &parent_rule {
        None => vec![selector],
        Some(parent_rule) => {
            if let Some(parent_rule) = AnyCssAtRule::cast_ref(parent_rule) {
                let mut hasher = DefaultHasher::new();
                parent_rule.range().hash(&mut hasher);
                // Each @rule is unique scope
                // Use a hash to create the comparable scope
                parent_selectors.push(hasher.finish().to_string());
            } else if let Some(parent_rule) = AnyCssRule::cast_ref(parent_rule) {
                match parent_rule {
                    AnyCssRule::CssNestedQualifiedRule(parent_rule) => {
                        parent_selectors.extend(
                            parent_rule
                                .prelude()
                                .into_iter()
                                .filter_map(|selector| selector.ok())
                                .map(|selector| selector.text()),
                        );
                    }
                    AnyCssRule::CssQualifiedRule(parent_rule) => {
                        parent_selectors.extend(
                            parent_rule
                                .prelude()
                                .into_iter()
                                .filter_map(|selector| selector.ok())
                                .map(|selector| selector.text()),
                        );
                    }
                    _ => {
                        // Bogus rules are not handled
                        // AtRule is handled by AnyCssAtRule above
                    }
                }
            }

            let resolved_selectors: Vec<String> =
                parent_selectors
                    .iter()
                    .fold(vec![], |result: Vec<String>, parent_selector| {
                        if selector.contains('&') {
                            let resolved_parent_selectors = resolve_nested_selectors(
                                parent_selector.to_string(),
                                parent_rule.clone(),
                            );
                            let resolved = resolved_parent_selectors
                                .into_iter()
                                .map(|newly_resolved| selector.replace('&', &newly_resolved))
                                .collect();
                            [result, resolved].concat()
                        } else {
                            let combined_selectors = parent_selector.to_owned() + " " + &selector;
                            let resolved =
                                resolve_nested_selectors(combined_selectors, parent_rule.clone());
                            [result, resolved].concat()
                        }
                    });
            if !resolved_selectors.is_empty() {
                return resolved_selectors;
            }
            vec![selector]
        }
    }
}

/// Checks if [AnyCssSelector] can be cast to [CssComplexSelector].
/// If it is able to cast, trim the combinator, e.g.
///
/// ``` css, ignore
/// a > b, c  > d { /* declaration */ }
/// ```
///
/// is normalized to:
/// ``` css, ignore
/// a>b, c>d { /* declaration */ }
/// ```
///
/// It will return `selector.text()` if it is unable to cast.
/// Returns the selector as a string.
fn normalize_complex_selector(selector: AnyCssSelector) -> String {
    let mut selector_text = String::new();

    if let AnyCssSelector::CssComplexSelector(complex_selector) = selector {
        if let Ok(left) = complex_selector.left() {
            selector_text.push_str(&left.text());
        }
        if let Ok(combinator) = complex_selector.combinator() {
            let combinator = combinator.text_trimmed();
            selector_text.push_str(combinator);
        }
        if let Ok(right) = complex_selector.right() {
            selector_text.push_str(&right.text());
        }
        return selector_text;
    }
    selector.text()
}
