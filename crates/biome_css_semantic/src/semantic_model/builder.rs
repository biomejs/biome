use biome_css_syntax::{CssRoot, CssSyntaxKind, CssSyntaxNode};
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;

use super::model::{
    CssDeclaration, CssGlobalCustomVariable, Rule, Selector, SemanticModel, SemanticModelData,
};
use crate::events::SemanticEvent;

pub struct SemanticModelBuilder {
    root: CssRoot,
    node_by_range: FxHashMap<TextRange, CssSyntaxNode>,
    rules: Vec<Rule>,
    global_custom_variables: FxHashMap<String, CssGlobalCustomVariable>,
    current_rule_stack: Vec<Rule>,
    range_to_rule: FxHashMap<TextRange, Rule>,
    is_in_root_selector: bool,
}

impl SemanticModelBuilder {
    pub fn new(root: CssRoot) -> Self {
        Self {
            root,
            node_by_range: FxHashMap::default(),
            rules: Vec::new(),
            current_rule_stack: Vec::new(),
            global_custom_variables: FxHashMap::default(),
            range_to_rule: FxHashMap::default(),
            is_in_root_selector: false,
        }
    }

    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            root: self.root,
            node_by_range: self.node_by_range,
            rules: self.rules,
            global_custom_variables: self.global_custom_variables,
            range_to_rule: self.range_to_rule,
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
                        self.range_to_rule
                            .insert(completed_rule.range, completed_rule.clone());
                        parent_rule.children.push(completed_rule);
                    } else {
                        self.range_to_rule
                            .insert(completed_rule.range, completed_rule.clone());
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
                property,
                value,
                range,
            } => {
                let is_global_var = self.is_in_root_selector && property.name.starts_with("--");

                if let Some(current_rule) = self.current_rule_stack.last_mut() {
                    if is_global_var {
                        self.global_custom_variables.insert(
                            property.name.clone(),
                            CssGlobalCustomVariable::Root(CssDeclaration {
                                property: property.clone(),
                                value: value.clone(),
                                range,
                            }),
                        );
                    }
                    current_rule.declarations.push(CssDeclaration {
                        property,
                        value,
                        range,
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
                self.global_custom_variables.insert(
                    property.name.to_string(),
                    CssGlobalCustomVariable::AtProperty {
                        property,
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
