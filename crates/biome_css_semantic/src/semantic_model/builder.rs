use biome_css_syntax::{CssRoot, CssSyntaxKind, CssSyntaxNode};
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;

use super::model::{Declaration, Rule, Selector, SemanticModel, SemanticModelData};
use crate::events::SemanticEvent;

pub struct SemanticModelBuilder {
    root: CssRoot,
    node_by_range: FxHashMap<TextRange, CssSyntaxNode>,
    rules: Vec<Rule>,
    current_rule_stack: Vec<Rule>,
}

impl SemanticModelBuilder {
    pub fn new(root: CssRoot) -> Self {
        Self {
            root,
            node_by_range: FxHashMap::default(),
            rules: Vec::new(),
            current_rule_stack: Vec::new(),
        }
    }

    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            root: self.root,
            node_by_range: self.node_by_range,
            rules: self.rules,
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
                property,
                value,
                property_range,
                value_range,
            } => {
                if let Some(current_rule) = self.current_rule_stack.last_mut() {
                    current_rule.declarations.push(Declaration {
                        property,
                        value,
                        property_range,
                        value_range,
                    });
                }
            }
        }
    }
}
