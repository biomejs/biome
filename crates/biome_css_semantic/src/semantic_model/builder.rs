use std::collections::BTreeMap;

use biome_css_syntax::CssRoot;
use biome_rowan::{AstNode, TextRange};
use rustc_hash::FxHashMap;

use super::model::{
    CssGlobalCustomVariable, CssModelDeclaration, Rule, RuleId, Selector, SemanticModel,
    SemanticModelData, Specificity,
};
use crate::events::SemanticEvent;
use crate::model::AnyRuleStart;

pub struct SemanticModelBuilder {
    root: CssRoot,
    /// List of all top-level rules in the CSS file
    rules: Vec<Rule>,
    global_custom_variables: FxHashMap<String, CssGlobalCustomVariable>,
    /// Stack of rule IDs to keep track of the current rule hierarchy
    current_rule_stack: Vec<RuleId>,
    next_rule_id: RuleId,
    /// Map to get the rule containing the given range of CST nodes
    range_to_rule: BTreeMap<TextRange, Rule>,
    rules_by_id: FxHashMap<RuleId, Rule>,
    /// Indicates if the current node is within a `:root` selector
    is_in_root_selector: bool,
}

impl SemanticModelBuilder {
    pub fn new(root: CssRoot) -> Self {
        Self {
            root,
            rules: Vec::new(),
            current_rule_stack: Vec::new(),
            global_custom_variables: FxHashMap::default(),
            range_to_rule: BTreeMap::default(),
            is_in_root_selector: false,
            next_rule_id: RuleId::default(),
            rules_by_id: FxHashMap::default(),
        }
    }

    pub fn get_last_parent_selector_rule(&self) -> Option<&Rule> {
        let mut iterator = self.current_rule_stack.iter().rev();
        let mut current_parent_id = iterator
            .next()
            .and_then(|rule_id| self.rules_by_id.get(rule_id))
            .and_then(|rule| rule.parent_id);

        loop {
            if let Some(parent_id) = &current_parent_id {
                let rule = self.rules_by_id.get(parent_id);
                if let Some(rule) = rule {
                    if matches!(
                        rule.node(),
                        AnyRuleStart::CssMediaAtRule(_) | AnyRuleStart::CssSupportsAtRule(_)
                    ) {
                        current_parent_id = iterator
                            .next()
                            .and_then(|rule_id| self.rules_by_id.get(rule_id))
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
            .and_then(|rule_id| self.rules_by_id.get(rule_id))
            .and_then(|rule| rule.parent_id);

        loop {
            if let Some(parent_id) = &current_parent_id {
                let rule = self.rules_by_id.get(parent_id);
                if let Some(rule) = rule {
                    if matches!(
                        rule.node(),
                        AnyRuleStart::CssMediaAtRule(_) | AnyRuleStart::CssSupportsAtRule(_)
                    ) {
                        current_parent_id = iterator
                            .next()
                            .and_then(|rule_id| self.rules_by_id.get(rule_id))
                            .and_then(|rule| rule.parent_id);
                    } else {
                        if current_index == index {
                            return Some(rule);
                        }

                        current_parent_id = iterator
                            .next()
                            .and_then(|rule_id| self.rules_by_id.get(rule_id))
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
            root: self.root,
            rules: self.rules,
            global_custom_variables: self.global_custom_variables,
            range_to_rule: self.range_to_rule,
            rules_by_id: self.rules_by_id,
        };
        SemanticModel::new(data)
    }

    #[inline]
    pub fn push_event(&mut self, event: SemanticEvent) {
        match event {
            SemanticEvent::RuleStart(node) => {
                let new_rule_id = self.next_rule_id;
                self.next_rule_id = RuleId::new(new_rule_id.index() + 1);

                let parent_id = self.current_rule_stack.last().copied();

                let new_rule = Rule {
                    id: new_rule_id,
                    node,
                    selectors: Vec::new(),
                    declarations: Vec::new(),
                    parent_id,
                    child_ids: Vec::new(),
                    specificity: Specificity::default(),
                };

                if let Some(&parent_id) = self.current_rule_stack.last()
                    && let Some(parent_rule) = self.rules_by_id.get_mut(&parent_id)
                {
                    parent_rule.child_ids.push(new_rule_id);
                }

                self.rules_by_id.insert(new_rule_id, new_rule);
                self.current_rule_stack.push(new_rule_id);
            }
            SemanticEvent::RuleEnd => {
                if let Some(completed_rule) = self.current_rule_stack.pop() {
                    let completed_rule = &self.rules_by_id[&completed_rule];
                    let has_parent = self.current_rule_stack.last().is_some();

                    if has_parent {
                        self.range_to_rule
                            .insert(completed_rule.range(), completed_rule.clone());
                    } else {
                        self.range_to_rule
                            .insert(completed_rule.range(), completed_rule.clone());
                        self.rules.push(completed_rule.clone());
                    }
                }
            }
            SemanticEvent::SelectorDeclaration { node, specificity } => {
                if let Some(current_rule) = self.current_rule_stack.last() {
                    let parent_specificity = if node.has_nesting_selectors() {
                        let nesting_level = node.nesting_level();
                        self.get_parent_selector_at(nesting_level)
                            .map(|rule| {
                                rule.selectors()
                                    .iter()
                                    .map(|s| s.specificity())
                                    .max()
                                    .unwrap_or_default()
                            })
                            .unwrap_or_default()
                    } else {
                        self.get_last_parent_selector_rule()
                            .map(|rule| {
                                rule.selectors()
                                    .iter()
                                    .map(|s| s.specificity())
                                    .max()
                                    .unwrap_or_default()
                            })
                            .unwrap_or_default()
                    };

                    let current_rule = self.rules_by_id.get_mut(current_rule).unwrap();
                    let combined = parent_specificity + specificity;
                    current_rule.selectors.push(Selector {
                        node,
                        specificity: combined,
                    });
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

                if let Some(current_rule) = self.current_rule_stack.last_mut() {
                    let current_rule = self.rules_by_id.get_mut(current_rule).unwrap();
                    if is_global_var {
                        let property_name = property.syntax().text_trimmed().to_string();
                        self.global_custom_variables.insert(
                            property_name,
                            CssGlobalCustomVariable::Root(CssModelDeclaration {
                                declaration: node.clone(),
                                property: property.clone(),
                                value: value.clone(),
                            }),
                        );
                    }
                    current_rule.declarations.push(CssModelDeclaration {
                        declaration: node,
                        property,
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
                        property: property.clone(),
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
