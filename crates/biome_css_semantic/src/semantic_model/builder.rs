use biome_css_syntax::{CssRoot, CssSyntaxKind, CssSyntaxNode};
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;

use super::model::{CssVariable, Declaration, Rule, Selector, SemanticModel, SemanticModelData};
use crate::events::SemanticEvent;

pub struct SemanticModelBuilder {
    root: CssRoot,
    node_by_range: FxHashMap<TextRange, CssSyntaxNode>,
    rules: Vec<Rule>,
    global_css_variables: FxHashMap<String, CssVariable>,
    current_rule_stack: Vec<Rule>,
    in_root_selector: bool,
}

impl SemanticModelBuilder {
    pub fn new(root: CssRoot) -> Self {
        Self {
            root,
            node_by_range: FxHashMap::default(),
            rules: Vec::new(),
            current_rule_stack: Vec::new(),
            global_css_variables: FxHashMap::default(),
            in_root_selector: false,
        }
    }

    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            root: self.root,
            node_by_range: self.node_by_range,
            rules: self.rules,
            global_css_variables: self.global_css_variables,
        };
        SemanticModel::new(data)
    }

    #[inline]
    pub fn push_node(&mut self, node: &CssSyntaxNode) {
        use CssSyntaxKind::*;
        if matches!(
            node.kind(),
            CSS_SELECTOR_LIST | CSS_DECLARATION | CSS_DECLARATION_OR_RULE_LIST | CSS_QUALIFIED_RULE
        ) {
            self.node_by_range.insert(node.text_range(), node.clone());
        }
    }

    #[inline]
    pub fn push_event(&mut self, event: SemanticEvent) {
        match event {
            SemanticEvent::RuleStart(range) => {
                let new_rule = Rule {
                    selectors: Vec::new(),
                    declarations: Vec::new(),
                    children: Vec::new(),
                    range,
                };
                self.current_rule_stack.push(new_rule);
            }
            SemanticEvent::RuleEnd => {
                if let Some(completed_rule) = self.current_rule_stack.pop() {
                    if let Some(parent_rule) = self.current_rule_stack.last_mut() {
                        parent_rule.children.push(completed_rule);
                    } else {
                        self.rules.push(completed_rule);
                    }
                }
            }
            SemanticEvent::SelectorDeclaration {
                name,
                range,
                specificity,
            } => {
                if let Some(current_rule) = self.current_rule_stack.last_mut() {
                    current_rule.selectors.push(Selector {
                        name,
                        range,
                        specificity,
                    });
                }
            }
            SemanticEvent::PropertyDeclaration {
                ref property,
                ref value,
                range,
            } => {
                if let Some(current_rule) = self.current_rule_stack.last_mut() {
                    if self.in_root_selector {
                        let key = &property.name;
                        if key.starts_with("--") {
                            self.global_css_variables.insert(
                                key.to_string(),
                                CssVariable {
                                    name: property.clone(),
                                    value: value.clone(),
                                    range,
                                },
                            );
                        }
                    }
                    current_rule.declarations.push(Declaration {
                        property: property.clone(),
                        value: value.clone(),
                    });
                }
            }
            SemanticEvent::RootSelectorStart => {
                self.in_root_selector = true;
            }
            SemanticEvent::RootSelectorEnd => {
                self.in_root_selector = false;
            }
            SemanticEvent::AtProperty {
                property,
                value,
                range,
            } => {
                self.global_css_variables.insert(
                    property.name.to_string(),
                    CssVariable {
                        name: property,
                        value,
                        range,
                    },
                );
            }
        }
    }
}
