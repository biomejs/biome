use std::collections::VecDeque;

use biome_css_syntax::{AnyCssSelector, CssRelativeSelector, CssSyntaxKind::*};
use biome_rowan::{AstNode, TextRange};

use crate::semantic_model::model::Specificity;

#[derive(Debug)]
pub enum SemanticEvent {
    RuleStart(TextRange),
    RuleEnd,
    SelectorDeclaration {
        name: String,
        range: TextRange,
        specificity: Specificity,
    },
    PropertyDeclaration {
        property: String,
        value: String,
        property_range: TextRange,
        value_range: TextRange,
    },
}

#[derive(Default, Debug)]
pub struct SemanticEventExtractor {
    stash: VecDeque<SemanticEvent>,
    current_rule_stack: Vec<TextRange>,
}

impl SemanticEventExtractor {
    pub fn enter(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        match node.kind() {
            kind if kind == CSS_QUALIFIED_RULE || kind == CSS_NESTED_QUALIFIED_RULE => {
                let range = node.text_range();
                self.stash.push_back(SemanticEvent::RuleStart(range));
                self.current_rule_stack.push(range);
            }
            CSS_SELECTOR_LIST => {
                node.children()
                    .filter_map(AnyCssSelector::cast)
                    .for_each(|s| self.process_selector(s));
            }
            CSS_RELATIVE_SELECTOR_LIST => {
                node.children()
                    .filter_map(CssRelativeSelector::cast)
                    .filter_map(|s| s.selector().ok())
                    .for_each(|s| self.process_selector(s));
            }
            CSS_DECLARATION => {
                if let Some(property_name) = node.first_child().and_then(|p| p.first_child()) {
                    if let Some(value) = property_name.next_sibling() {
                        self.stash.push_back(SemanticEvent::PropertyDeclaration {
                            property: property_name.text_trimmed().to_string(),
                            value: value.text_trimmed().to_string(),
                            property_range: property_name.text_range(),
                            value_range: value.text_range(),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    fn process_selector(&mut self, selector: AnyCssSelector) {
        match selector {
            AnyCssSelector::CssComplexSelector(s) => {
                if let Ok(l) = s.left() {
                    self.add_selector_event(l.text(), l.range());
                }
                if let Ok(r) = s.right() {
                    self.add_selector_event(r.text(), r.range());
                }
            }
            AnyCssSelector::CssCompoundSelector(selector) => {
                self.add_selector_event(selector.text().to_string(), selector.range());
            }
            _ => {}
        }
    }

    fn add_selector_event(&mut self, name: String, range: TextRange) {
        self.stash.push_back(SemanticEvent::SelectorDeclaration {
            name,
            range,
            specificity: Specificity(0, 0, 0), // TODO: Implement this
        });
    }

    pub fn leave(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        if matches!(
            node.kind(),
            biome_css_syntax::CssSyntaxKind::CSS_QUALIFIED_RULE
                | biome_css_syntax::CssSyntaxKind::CSS_NESTED_QUALIFIED_RULE
        ) {
            self.current_rule_stack.pop();
            self.stash.push_back(SemanticEvent::RuleEnd);
        }
    }

    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }
}
