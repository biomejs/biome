use std::collections::VecDeque;

use biome_css_syntax::{AnyCssSelector, CssQualifiedRule};
use biome_rowan::{AstNode, TextRange};

#[derive(Debug)]
pub enum SemanticEvent {
    SelectorDeclaration {
        range: TextRange,
        name: String,
        selector_range: TextRange,
    },
    PropertyDeclaration {
        selector_range: TextRange,
        property: String,
        property_range: TextRange,
        value: String,
        value_range: TextRange,
    },
}

#[derive(Default, Debug)]
pub struct SemanticEventExtractor {
    stash: VecDeque<SemanticEvent>,
    current_selector_range: Option<TextRange>,
}

impl SemanticEventExtractor {
    pub fn enter(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        match node.kind() {
            biome_css_syntax::CssSyntaxKind::CSS_QUALIFIED_RULE => {
                if let Some(qualified_rule) = CssQualifiedRule::cast(node.clone()) {
                    self.current_selector_range =
                        Some(qualified_rule.prelude().syntax().text_range());
                }
            }
            biome_css_syntax::CssSyntaxKind::CSS_SELECTOR_LIST => {
                for selector in node.children() {
                    if let Some(s) = AnyCssSelector::cast(selector) {
                        match s {
                            AnyCssSelector::CssComplexSelector(s) => {
                                if let Ok(l) = s.left() {
                                    self.stash.push_back(SemanticEvent::SelectorDeclaration {
                                        range: node.text_range(),
                                        selector_range: l.range(),
                                        name: l.text().to_string(),
                                    });
                                }

                                if let Ok(r) = s.right() {
                                    self.stash.push_back(SemanticEvent::SelectorDeclaration {
                                        range: node.text_range(),
                                        selector_range: r.range(),
                                        name: r.text().to_string(),
                                    });
                                }
                            }
                            AnyCssSelector::CssCompoundSelector(selector) => {
                                self.stash.push_back(SemanticEvent::SelectorDeclaration {
                                    range: node.text_range(),
                                    name: selector.text().to_string(),
                                    selector_range: selector.range(),
                                });
                            }
                            AnyCssSelector::CssBogusSelector(_)
                            | AnyCssSelector::CssGritMetavariable(_) => {}
                        }
                    }
                }
            }
            biome_css_syntax::CssSyntaxKind::CSS_DECLARATION_OR_RULE_LIST => {
                for block in node.children() {
                    if let Some(selector_range) = self.current_selector_range {
                        if let Some(decl) = block.first_child() {
                            if let Some(property) = decl.first_child() {
                                if let Some(property_name) = property.first_child() {
                                    if let Some(value) = property_name.next_sibling() {
                                        self.stash.push_back(SemanticEvent::PropertyDeclaration {
                                            selector_range,
                                            property: property_name.text().to_string(),
                                            property_range: property_name.text_range(),
                                            value: value.text().to_string(),
                                            value_range: value.text_range(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn leave(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        if node.kind() == biome_css_syntax::CssSyntaxKind::CSS_QUALIFIED_RULE {
            self.current_selector_range = None;
        }
    }

    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }
}
