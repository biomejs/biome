use std::collections::VecDeque;

use biome_css_syntax::{
    AnyCssSelector, CssDeclarationBlock, CssRelativeSelector, CssSyntaxKind::*,
};
use biome_rowan::{AstNode, SyntaxNodeCast, TextRange};

use crate::{
    model::{CssProperty, CssValue},
    semantic_model::model::Specificity,
};

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
        property: CssProperty,
        value: CssValue,
        range: TextRange,
    },
    /// Indicates the start of a `:root` selector
    RootSelectorStart,
    /// Indicates the end of a `:root` selector
    RootSelectorEnd,
    /// Indicates the start of an `@property` rule
    AtProperty {
        property: CssProperty,
        value: CssValue,
        range: TextRange,
    },
}

#[derive(Default, Debug)]
pub struct SemanticEventExtractor {
    stash: VecDeque<SemanticEvent>,
    current_rule_stack: Vec<TextRange>,
    in_root_selector: bool,
}

impl SemanticEventExtractor {
    pub fn enter(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        match node.kind() {
            // Begin a new CSS rule context
            // This tracks the hierarchical structure of rules, including:
            // 1. Standard rulesets
            //    Example: p { color: red; }
            // 2. Nested selectors
            //    Example: .parent { .child { font-size: 14px; } }
            // 3. At-rules like media queries
            //    Example: @media (min-width: 600px) { header { padding: 20px; } }
            //
            // Each rule start is pushed onto a stack to maintain parent-child relationships,
            // allowing for proper scoping and inheritance of styles.
            kind if kind == CSS_QUALIFIED_RULE
                || kind == CSS_NESTED_QUALIFIED_RULE
                || kind == CSS_MEDIA_AT_RULE =>
            {
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
                            property: CssProperty {
                                name: property_name.text_trimmed().to_string(),
                                range: property_name.text_range(),
                            },
                            value: CssValue {
                                value: value.text_trimmed().to_string(),
                                range: value.text_range(),
                            },
                            range: node.text_range(),
                        });
                    }
                }
            }
            CSS_PROPERTY_AT_RULE => {
                self.process_at_property(node);
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
                if selector.text() == ":root" {
                    self.stash.push_back(SemanticEvent::RootSelectorStart);
                    self.in_root_selector = true;
                }
                self.add_selector_event(selector.text().to_string(), selector.range())
            }
            _ => {}
        }
    }

    fn process_at_property(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        let property_name = match node.first_child() {
            Some(name) => name,
            None => return,
        };

        let value = match property_name.next_sibling() {
            Some(val) => val,
            None => return,
        };

        let decls = match value.cast::<CssDeclarationBlock>() {
            Some(d) => d,
            None => return,
        };

        for d in decls.declarations() {
            if let Ok(declaration) = d.declaration() {
                if let Ok(biome_css_syntax::AnyCssProperty::CssGenericProperty(p)) =
                    declaration.property()
                {
                    if let Ok(name) = p.name() {
                        if name.text() == "initial-value" {
                            self.stash.push_back(SemanticEvent::AtProperty {
                                property: CssProperty {
                                    name: property_name.text_trimmed().to_string(),
                                    range: property_name.text_range(),
                                },
                                value: CssValue {
                                    value: p.value().text().to_string(),
                                    range: p.value().range(),
                                },
                                range: node.text_range(),
                            });
                        }
                    }
                }
            }
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
            CSS_QUALIFIED_RULE | CSS_NESTED_QUALIFIED_RULE | CSS_MEDIA_AT_RULE
        ) {
            self.current_rule_stack.pop();
            self.stash.push_back(SemanticEvent::RuleEnd);
            if self.in_root_selector {
                self.stash.push_back(SemanticEvent::RootSelectorEnd);
                self.in_root_selector = false;
            }
        }
    }

    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }
}
