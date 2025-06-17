use std::collections::BTreeMap;

use biome_css_syntax::CssRoot;
use biome_rowan::{AstNode, TextRange};
use rustc_hash::FxHashMap;

use super::model::{
    CssGlobalCustomVariable, CssModelDeclaration, Rule, RuleId, Selector, SemanticModel,
    SemanticModelData, Specificity,
};
use crate::events::SemanticEvent;
use crate::model::RuleNode;

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
                    // SAFETY: RuleStart event checks for the node kind
                    node: RuleNode::cast(node.clone()).unwrap(),
                    selectors: Vec::new(),
                    declarations: Vec::new(),
                    parent_id,
                    child_ids: Vec::new(),
                    specificity: Specificity::default(),
                };

                if let Some(&parent_id) = self.current_rule_stack.last() {
                    if let Some(parent_rule) = self.rules_by_id.get_mut(&parent_id) {
                        parent_rule.child_ids.push(new_rule_id);
                    }
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
                let parent_specificity = self
                    .current_rule_stack
                    .last()
                    .and_then(|rule_id| self.rules_by_id.get(rule_id))
                    .and_then(|rule| rule.parent_id)
                    .and_then(|parent_id| self.rules_by_id.get(&parent_id))
                    .map(|parent| parent.specificity)
                    .unwrap_or_default();

                if let Some(current_rule) = self.current_rule_stack.last() {
                    let current_rule = self.rules_by_id.get_mut(current_rule).unwrap();
                    current_rule.selectors.push(Selector {
                        node,
                        specificity: parent_specificity + specificity,
                    });

                    current_rule.specificity += specificity;
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
