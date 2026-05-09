use biome_css_syntax::{AnyCssRoot, CssSyntaxKind, CssSyntaxToken};
use biome_rowan::{AstNode, AstPtr, TextRange, TokenText};
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;

use super::model::{
    CssGlobalCustomVariable, CssModelDeclaration, ResolvedSelector, Rule, RuleId, Selector,
    SemanticModel, SemanticModelData, Specificity, selector_tokens,
};
use crate::events::SemanticEvent;
use crate::model::AnyRuleStart;

pub struct SemanticModelBuilder {
    root: AnyCssRoot,
    /// All rules, indexed by RuleId
    all_rules: Vec<Rule>,
    /// IDs of top-level rules only
    top_level_rule_ids: Vec<RuleId>,
    global_custom_variables: FxHashMap<String, CssGlobalCustomVariable>,
    /// Stack of rule IDs to keep track of the current rule hierarchy
    current_rule_stack: Vec<RuleId>,
    /// Map from text range to RuleId
    range_to_rule_id: BTreeMap<TextRange, RuleId>,
    /// Indicates if the current node is within a `:root` selector
    is_in_root_selector: bool,
}

impl SemanticModelBuilder {
    pub fn new(root: AnyCssRoot) -> Self {
        Self {
            root,
            all_rules: Vec::new(),
            top_level_rule_ids: Vec::new(),
            current_rule_stack: Vec::new(),
            global_custom_variables: FxHashMap::default(),
            range_to_rule_id: BTreeMap::default(),
            is_in_root_selector: false,
        }
    }

    pub fn get_last_parent_selector_rule(&self) -> Option<&Rule> {
        let mut iterator = self.current_rule_stack.iter().rev();
        let mut current_parent_id = iterator
            .next()
            .and_then(|rule_id| self.all_rules.get(rule_id.index()))
            .and_then(|rule| rule.parent_id);

        loop {
            if let Some(parent_id) = &current_parent_id {
                let rule = self.all_rules.get(parent_id.index());
                if let Some(rule) = rule {
                    let typed_node = rule.node.to_node(self.root.syntax());
                    if matches!(
                        typed_node,
                        AnyRuleStart::CssMediaAtRule(_)
                            | AnyRuleStart::CssScopeAtRule(_)
                            | AnyRuleStart::CssSupportsAtRule(_)
                    ) {
                        current_parent_id = iterator
                            .next()
                            .and_then(|rule_id| self.all_rules.get(rule_id.index()))
                            .and_then(|rule| rule.parent_id);
                    } else {
                        return Some(rule);
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }

    pub fn get_parent_selector_at(&self, index: usize) -> Option<&Rule> {
        let mut iterator = self.current_rule_stack.iter().rev();
        let mut current_index = 1;
        let mut current_parent_id = iterator
            .next()
            .and_then(|rule_id| self.all_rules.get(rule_id.index()))
            .and_then(|rule| rule.parent_id);

        loop {
            if let Some(parent_id) = &current_parent_id {
                let rule = self.all_rules.get(parent_id.index());
                if let Some(rule) = rule {
                    let typed_node = rule.node.to_node(self.root.syntax());
                    if matches!(
                        typed_node,
                        AnyRuleStart::CssMediaAtRule(_)
                            | AnyRuleStart::CssScopeAtRule(_)
                            | AnyRuleStart::CssSupportsAtRule(_)
                    ) {
                        current_parent_id = iterator
                            .next()
                            .and_then(|rule_id| self.all_rules.get(rule_id.index()))
                            .and_then(|rule| rule.parent_id);
                    } else {
                        if current_index == index {
                            return Some(rule);
                        }

                        current_parent_id = iterator
                            .next()
                            .and_then(|rule_id| self.all_rules.get(rule_id.index()))
                            .and_then(|rule| rule.parent_id);
                        current_index += 1;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }

    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            all_rules: self.all_rules,
            top_level_rule_ids: self.top_level_rule_ids,
            global_custom_variables: self.global_custom_variables,
            range_to_rule_id: self.range_to_rule_id,
        };
        SemanticModel::new(
            data,
            self.root.syntax().as_send().expect("To be a root node"),
        )
    }

    #[inline]
    pub fn push_event(&mut self, event: SemanticEvent) {
        match event {
            SemanticEvent::RuleStart(node) => {
                let new_rule_id = RuleId::new(self.all_rules.len());

                let parent_id = self.current_rule_stack.last().copied();

                let new_rule = Rule {
                    id: new_rule_id,
                    node: AstPtr::new(&node),
                    selectors: Vec::new(),
                    declarations: Vec::new(),
                    parent_id,
                    child_ids: Vec::new(),
                    specificity: Specificity::default(),
                };

                if let Some(&parent_id) = self.current_rule_stack.last() {
                    self.all_rules[parent_id.index()]
                        .child_ids
                        .push(new_rule_id);
                }

                self.all_rules.push(new_rule);
                self.current_rule_stack.push(new_rule_id);
            }
            SemanticEvent::RuleEnd => {
                if let Some(completed_rule_id) = self.current_rule_stack.pop() {
                    let range = self.all_rules[completed_rule_id.index()].range(&self.root);
                    self.range_to_rule_id.insert(range, completed_rule_id);

                    if self.current_rule_stack.is_empty() {
                        self.top_level_rule_ids.push(completed_rule_id);
                    }
                }
            }
            SemanticEvent::SelectorDeclaration { node, specificity } => {
                if let Some(&current_rule_id) = self.current_rule_stack.last() {
                    let parent_specificity = if node.has_nesting_selectors() {
                        let nesting_level = node.nesting_level();
                        self.get_parent_selector_at(nesting_level)
                            .map(|rule| {
                                rule.selectors
                                    .iter()
                                    .map(|s| s.specificity)
                                    .max()
                                    .unwrap_or_default()
                            })
                            .unwrap_or_default()
                    } else {
                        self.get_last_parent_selector_rule()
                            .map(|rule| {
                                rule.selectors
                                    .iter()
                                    .map(|s| s.specificity)
                                    .max()
                                    .unwrap_or_default()
                            })
                            .unwrap_or_default()
                    };

                    let current_tokens = selector_tokens(&node);

                    let parent_rule = self.get_last_parent_selector_rule();
                    let resolved_selectors: Vec<ResolvedSelector> =
                        if let Some(parent_rule) = parent_rule {
                            resolve_selector(&current_tokens, &parent_rule.selectors)
                        } else {
                            vec![ResolvedSelector(
                                current_tokens
                                    .iter()
                                    .map(|t| (t.kind(), t.token_text_trimmed()))
                                    .collect(),
                            )]
                        };

                    let current_rule = &mut self.all_rules[current_rule_id.index()];
                    let combined = parent_specificity + specificity;

                    for resolved in resolved_selectors {
                        current_rule.selectors.push(Selector {
                            node: AstPtr::new(&node),
                            resolved,
                            specificity: combined,
                        });
                    }

                    if combined > current_rule.specificity {
                        current_rule.specificity = combined;
                    }
                }
            }
            SemanticEvent::PropertyDeclaration {
                node,
                property,
                value,
            } => {
                let is_global_var =
                    self.is_in_root_selector && property.syntax().text_trimmed().starts_with("--");

                if let Some(&current_rule_id) = self.current_rule_stack.last() {
                    if is_global_var {
                        let property_name = property.syntax().text_trimmed().to_string();
                        self.global_custom_variables.insert(
                            property_name,
                            CssGlobalCustomVariable::Root(CssModelDeclaration {
                                declaration: AstPtr::new(&node),
                                property: AstPtr::new(&property),
                                value: value.clone(),
                            }),
                        );
                    }
                    let current_rule = &mut self.all_rules[current_rule_id.index()];
                    current_rule.declarations.push(CssModelDeclaration {
                        declaration: AstPtr::new(&node),
                        property: AstPtr::new(&property),
                        value,
                    });
                }
            }
            SemanticEvent::RootSelectorStart => {
                self.is_in_root_selector = true;
            }
            SemanticEvent::RootSelectorEnd => {
                self.is_in_root_selector = false;
            }
            SemanticEvent::AtProperty {
                property,
                initial_value,
                syntax,
                inherits,
                range,
            } => {
                let property_name = property.to_trimmed_string();
                self.global_custom_variables.insert(
                    property_name,
                    CssGlobalCustomVariable::AtProperty {
                        property: AstPtr::new(&property),
                        initial_value,
                        syntax,
                        inherits,
                        range,
                    },
                );
            }
        }
    }
}

/// Synthetic space-literal `(kind, TokenText)` pair used as the implicit descendant
/// combinator when a nested selector contains no `&` reference.
fn space_combinator() -> (CssSyntaxKind, TokenText) {
    use biome_rowan::SyntaxKind;
    (
        CssSyntaxKind::CSS_SPACE_LITERAL,
        TokenText::new_raw(CssSyntaxKind::CSS_SPACE_LITERAL.to_raw(), " "),
    )
}

/// Resolves the `current` token sequence against each parent [`Selector`],
/// producing one [`ResolvedSelector`] per parent selector.
///
/// Resolution rules (per the CSS nesting spec):
/// - If any token in `current` is an `AMP` (`&`), every such occurrence is
///   replaced in-place by the full token sequence of the parent selector.
/// - If there is no `&`, the parent token sequence is prepended and a
///   synthetic space-literal token is inserted as the descendant combinator.
///
/// Tokens are stored as `(CssSyntaxKind, TokenText)` pairs so that the
/// `Display` impl can reconstruct canonical whitespace around combinators.
fn resolve_selector(current: &[CssSyntaxToken], parents: &[Selector]) -> Vec<ResolvedSelector> {
    let has_amp = current.iter().any(|t| t.kind() == CssSyntaxKind::AMP);

    parents
        .iter()
        .map(|parent| {
            let parent_tokens = &parent.resolved.0;
            if has_amp {
                let amp_count = current
                    .iter()
                    .filter(|t| t.kind() == CssSyntaxKind::AMP)
                    .count();
                let non_amp_count = current.len() - amp_count;
                let capacity = amp_count * parent_tokens.len() + non_amp_count;

                let mut tokens = Vec::with_capacity(capacity);
                for t in current {
                    if t.kind() == CssSyntaxKind::AMP {
                        tokens.extend(parent_tokens.iter().cloned());
                    } else {
                        tokens.push((t.kind(), t.token_text_trimmed()));
                    }
                }
                ResolvedSelector(tokens)
            } else {
                // Prepend parent tokens + implicit descendant combinator.
                let mut tokens = Vec::with_capacity(parent_tokens.len() + 1 + current.len());
                tokens.extend(parent_tokens.iter().cloned());
                tokens.push(space_combinator());
                tokens.extend(current.iter().map(|t| (t.kind(), t.token_text_trimmed())));
                ResolvedSelector(tokens)
            }
        })
        .collect()
}
