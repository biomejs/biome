use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::vec;

use biome_analyze::Ast;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssAtRule, AnyCssRelativeSelector, AnyCssRule, AnyCssSelector, CssComplexSelector,
    CssRelativeSelector, CssRelativeSelectorList, CssRoot, CssRuleList, CssSelectorList,
    CssSyntaxNode,
};
use biome_deserialize_macros::Deserializable;
use biome_rowan::{AstNode, SyntaxNodeCast};

use serde::{Deserialize, Serialize};

declare_rule! {
    /// Disallow duplicate selectors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .abc,
    /// .def,
    /// .abc {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```
    /// .foo {}
    /// .bar {}
    /// ```
    ///
    /// ## Options
    ///
    /// If true, disallow duplicate selectors within selector lists.
    ///
    /// ```json
    /// {
    ///     "noDuplicateSelectors": {
    ///         "options": {
    ///           "disallowInList": true
    ///         }
    ///     }
    /// }
    /// ```
    ///
    pub NoDuplicateSelectors {
        version: "next",
        name: "noDuplicateSelectors",
        recommended: true,
        sources: &[RuleSource::Stylelint("no-duplicate-selectors")],
    }
}

#[derive(Debug, Clone, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoDuplicateSelectorsOptions {
    pub disallow_in_list: bool,
}

impl Default for NoDuplicateSelectorsOptions {
    fn default() -> Self {
        Self {
            disallow_in_list: false,
        }
    }
}

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
            let selectors = node.rules().syntax().descendants().filter_map(|x| {
                if let Some(_selector) = x.clone().cast::<AnyCssSelector>() {
                    return Some(x);
                }
                if let Some(_relative_selector) = x.clone().cast::<AnyCssRelativeSelector>() {
                    return Some(x);
                }
                None
            });

            for selector in selectors {
                let this_list = selector.clone().parent().unwrap();

                // i.e not actually a list
                if let Some(_parent_sel) = CssComplexSelector::cast_ref(&this_list) {
                    // Don't handle the children of complex selectors
                    // this_selctor_list = parent_selector.into_syntax().parent().unwrap()
                    continue;
                } else if let Some(_parent_sel) = CssRelativeSelector::cast_ref(&this_list) {
                    // Don't handle the children of complex relative
                    // this_selctor_list = parent_selector.into_syntax().parent().unwrap();
                    continue;
                }

                let this_rule = this_list.parent().unwrap();

                let mut selector_text = String::new();
                if let Some(selector) = CssRelativeSelector::cast_ref(&selector) {
                    selector_text = selector.clone().text();
                }
                if let Some(selector) = AnyCssSelector::cast_ref(&selector) {
                    // TODO: test if this needs to be normalized
                    selector_text = selector.clone().text();
                }
                let resolved = resolve_nested_selectors(selector_text, this_rule);

                for r in resolved {
                    let split: Vec<&str> = r.split_whitespace().collect();
                    let normalized = split.join(" ").to_lowercase();
                    if !resolved_list.insert(ResolvedSelector {
                        selector_text: normalized.clone(),
                        selector_node: selector.clone(),
                    }) {
                        let first = resolved_list.get(&normalized);
                        if let Some(first) = first {
                            output.push(DuplicateSelector {
                                first: first.selector_node.clone(),
                                duplicate: selector.clone(),
                            });
                        }
                    }
                }
            }
        } else {
            let select_lists = node.rules().syntax().descendants().filter_map(|x| {
                if let Some(_selector) = x.clone().cast::<CssSelectorList>() {
                    return Some(x);
                }
                if let Some(_relative_selector) = x.clone().cast::<CssRelativeSelectorList>() {
                    return Some(x);
                }
                None
            });

            for selector_list in select_lists {
                let mut this_list_resolved_list: HashSet<ResolvedSelector> = HashSet::new();

                let this_rule = selector_list.parent().unwrap();
                let mut selector_list_mapped: Vec<String> = selector_list
                    .children()
                    .into_iter()
                    .filter_map(|child| {
                        if let Some(selector) = AnyCssSelector::cast_ref(&child) {
                            let selector_text = normalize_complex_selector(selector.clone());
                            if !this_list_resolved_list.insert(ResolvedSelector {
                                selector_text: selector_text.clone(),
                                selector_node: selector.clone().into(),
                            }) {
                                let first = this_list_resolved_list.get(&selector_text);
                                if let Some(first) = first {
                                    output.push(DuplicateSelector {
                                        first: first.selector_node.clone(),
                                        duplicate: selector.into(),
                                    });
                                    // Return a none, since we already addressed this case
                                    return None;
                                }
                            }
                            return Some(selector_text);
                        } else if let Some(selector) = AnyCssRelativeSelector::cast_ref(&child) {
                            if !this_list_resolved_list.insert(ResolvedSelector {
                                selector_text: selector.clone().text(),
                                selector_node: selector.clone().into(),
                            }) {
                                let first = this_list_resolved_list.get(&selector.text());
                                if let Some(first) = first {
                                    output.push(DuplicateSelector {
                                        first: first.selector_node.clone(),
                                        duplicate: selector.into(),
                                    });
                                    // Return a none, since we already addressed this case
                                    return None;
                                }
                            }
                            return Some(selector.text());
                        }
                        None
                    })
                    .collect();
                selector_list_mapped.sort();
                let selector_list_text = selector_list_mapped.join(",");

                let resolved = resolve_nested_selectors(selector_list_text, this_rule);

                for r in resolved {
                    let split: Vec<&str> = r.split_whitespace().collect();
                    let normalized = split.join(" ").to_lowercase();
                    if !resolved_list.insert(ResolvedSelector {
                        selector_text: normalized.clone(),
                        selector_node: selector_list.clone().into(),
                    }) {
                        let first = resolved_list.get(&normalized);
                        if let Some(first) = first {
                            output.push(DuplicateSelector {
                                first: first.selector_node.clone(),
                                duplicate: selector_list.clone(),
                            });
                        }
                    }
                }
            }
        }
        output
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let duplicate = node.duplicate.to_string();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.duplicate.text_range(),
                markup! {
                    "Duplicate selector \""<Emphasis>{duplicate}</Emphasis>"\","
                },
            )
            .detail(node.first.text_range(), "first occurence:"),
        )
    }
}

fn resolve_nested_selectors(selector: String, this_rule: CssSyntaxNode) -> Vec<String> {
    let mut parent_selectors: Vec<String> = vec![];

    let parent_rule = get_parent_rule(this_rule);

    match &parent_rule {
        None => return vec![selector],
        Some(parent_rule) => {
            if let Some(parent_rule) = AnyCssAtRule::cast_ref(&parent_rule) {
                // resolve
                match parent_rule {
                    AnyCssAtRule::CssContainerAtRule(_rule) => todo!(),
                    AnyCssAtRule::CssKeyframesAtRule(_rule) => todo!(),
                    AnyCssAtRule::CssLayerAtRule(_rule) => todo!(),
                    AnyCssAtRule::CssMediaAtRule(rule) => {
                        let mut resolved = "@".to_string();
                        resolved.push_str(&rule.media_token().unwrap().text());
                        resolved.push_str(&rule.queries().text());
                        // Replace the spaces with something that is not valid in CSS
                        let resolved = resolved.replace(char::is_whitespace, "-");
                        parent_selectors.push(resolved);
                    }
                    AnyCssAtRule::CssPageAtRule(_rule) => todo!(),
                    AnyCssAtRule::CssScopeAtRule(_rule) => todo!(),
                    AnyCssAtRule::CssStartingStyleAtRule(_rule) => todo!(),
                    AnyCssAtRule::CssSupportsAtRule(_rule) => todo!(),
                    _ => {}
                }
            }
            if let Some(parent_rule) = AnyCssRule::cast_ref(&parent_rule) {
                match parent_rule {
                    AnyCssRule::CssBogusRule(_) => todo!(),
                    AnyCssRule::CssAtRule(parent_rule) => {
                        // Treat the AtRule as a selector
                        let rule = parent_rule.rule().unwrap();
                        parent_selectors.push(rule.text());
                    }
                    AnyCssRule::CssNestedQualifiedRule(parent_rule) => {
                        for selector in parent_rule.prelude() {
                            if let Ok(selector) = selector {
                                parent_selectors.push(selector.text());
                            }
                        }
                    }
                    AnyCssRule::CssQualifiedRule(parent_rule) => {
                        for selector in parent_rule.prelude() {
                            if let Ok(selector) = selector {
                                parent_selectors.push(selector.text());
                            }
                        }
                    }
                }
            }

            let resolved_selectors: Vec<String> =
                parent_selectors
                    .iter()
                    .fold(vec![], |result: Vec<String>, parent_selector| {
                        if selector.contains("&") {
                            let resolved_parent_selectors = resolve_nested_selectors(
                                parent_selector.to_string(),
                                parent_rule.clone(),
                            );
                            let resolved = resolved_parent_selectors
                                .into_iter()
                                .map(|newly_resolved| return selector.replace("&", &newly_resolved))
                                .collect();
                            return [result, resolved].concat();
                        } else {
                            let combined_selectors = parent_selector.to_owned() + " " + &selector;
                            let resolved =
                                resolve_nested_selectors(combined_selectors, parent_rule.clone());
                            return [result, resolved].concat();
                        }
                    });
            if resolved_selectors.len() > 0 {
                return resolved_selectors;
            }
            return vec![selector];
        }
    }
}

// This does not handle the highest level rules
fn get_parent_rule(this_rule: CssSyntaxNode) -> Option<CssSyntaxNode> {
    if let Some(parent_list) = this_rule.parent() {
        return parent_list.grand_parent();
    }
    return None;
}

fn normalize_complex_selector(selector: AnyCssSelector) -> String {
    let mut selector_text = String::new();

    if let Some(complex_selector) = CssComplexSelector::cast_ref(&selector.clone().into_syntax()) {
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
    return selector.text();
}
