use std::{borrow::Cow, collections::VecDeque};

use biome_css_syntax::{
    AnyCssSelector, CssDeclarationBlock, CssRelativeSelector, CssSyntaxKind::*,
};
use biome_rowan::{AstNode, SyntaxNodeCast, SyntaxNodeOptionExt, TextRange};

use crate::{
    model::{CssProperty, CssValue},
    semantic_model::model::Specificity,
    specificity::{evaluate_complex_selector, evaluate_compound_selector},
};

const ROOT_SELECTOR: &str = ":root";

#[derive(Debug)]
pub enum SemanticEvent {
    RuleStart(TextRange),
    RuleEnd,
    SelectorDeclaration {
        name: String,
        range: TextRange,
        original: AnyCssSelector,
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
        initial_value: Option<CssValue>,
        syntax: Option<String>,
        inherits: Option<bool>,
        range: TextRange,
    },
}

#[derive(Default, Debug)]
pub struct SemanticEventExtractor {
    stash: VecDeque<SemanticEvent>,
    current_rule_stack: Vec<TextRange>,
    is_in_root_selector: bool,
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
                || kind == CSS_MEDIA_AT_RULE
                || kind == CSS_SUPPORTS_AT_RULE =>
            {
                let range = node.text_range_with_trivia();
                self.stash.push_back(SemanticEvent::RuleStart(range));
                self.current_rule_stack.push(range);
            }
            CSS_SELECTOR_LIST => {
                if !matches!(
                    node.parent().kind(),
                    Some(CSS_QUALIFIED_RULE | CSS_NESTED_QUALIFIED_RULE)
                ) {
                    return;
                };
                node.children()
                    .filter_map(AnyCssSelector::cast)
                    .for_each(|s| self.process_selector(s));
            }
            CSS_RELATIVE_SELECTOR_LIST => {
                if !matches!(
                    node.parent().kind(),
                    Some(CSS_QUALIFIED_RULE | CSS_NESTED_QUALIFIED_RULE)
                ) {
                    return;
                };
                node.children()
                    .filter_map(CssRelativeSelector::cast)
                    .filter_map(|s| s.selector().ok())
                    .for_each(|s| self.process_selector(s));
            }
            CSS_DECLARATION => {
                if matches!(node.parent().kind(), Some(CSS_SUPPORTS_FEATURE_DECLARATION)) {
                    return;
                }

                if let Some(property_name) = node.first_child().and_then(|p| p.first_child()) {
                    if let Some(value) = property_name.next_sibling() {
                        self.stash.push_back(SemanticEvent::PropertyDeclaration {
                            property: CssProperty {
                                name: property_name.text_trimmed().to_string(),
                                range: property_name.text_trimmed_range(),
                            },
                            value: CssValue {
                                text: value.text_trimmed().to_string(),
                                range: value.text_trimmed_range(),
                            },
                            range: node.text_range_with_trivia(),
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

    #[inline]
    fn process_selector(&mut self, selector: AnyCssSelector) {
        match selector {
            AnyCssSelector::CssComplexSelector(s) => {
                let specificity = evaluate_complex_selector(&s);
                self.add_selector_event(
                    Cow::Borrowed(&s.to_trimmed_string()),
                    s.range(),
                    AnyCssSelector::CssComplexSelector(s),
                    specificity,
                );
            }

            AnyCssSelector::CssCompoundSelector(selector) => {
                let selector_text = selector.to_trimmed_string();
                if selector_text == ROOT_SELECTOR {
                    self.stash.push_back(SemanticEvent::RootSelectorStart);
                    self.is_in_root_selector = true;
                }
                let specificity = evaluate_compound_selector(&selector);
                self.add_selector_event(
                    Cow::Borrowed(&selector_text),
                    selector.range(),
                    AnyCssSelector::CssCompoundSelector(selector),
                    specificity,
                )
            }
            _ => {}
        }
    }

    /// Handles the `@property` rule, which defines custom CSS properties.
    ///
    /// ```css
    /// @property --my-property {
    ///   syntax: "<length>";
    ///   inherits: true;
    ///   initial-value: 0;
    /// }
    ///
    /// @property --my-other-property {}
    /// ```
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

        let mut initial_value = None;
        let mut syntax = None;
        let mut inherits = None;

        for declaration in decls
            .declarations()
            .into_iter()
            .filter_map(|d| d.declaration().ok())
        {
            if let Ok(biome_css_syntax::AnyCssProperty::CssGenericProperty(prop)) =
                declaration.property()
            {
                if let Ok(prop_name) = prop.name() {
                    match prop_name.to_trimmed_string().as_str() {
                        "initial-value" => {
                            initial_value = Some(CssValue {
                                text: prop.value().to_trimmed_string().to_string(),
                                range: prop.value().range(),
                            });
                        }
                        "syntax" => {
                            syntax = Some(prop.value().to_trimmed_string().to_string());
                        }
                        "inherits" => {
                            inherits = Some(prop.value().to_trimmed_string() == "true");
                        }
                        _ => {}
                    }
                }
            }
        }

        self.stash.push_back(SemanticEvent::AtProperty {
            property: CssProperty {
                name: property_name.text_trimmed().to_string(),
                range: property_name.text_range_with_trivia(),
            },
            initial_value,
            syntax,
            inherits,
            range: node.text_range_with_trivia(),
        });
    }

    fn add_selector_event(
        &mut self,
        name: Cow<str>,
        range: TextRange,
        original: AnyCssSelector,
        specificity: Specificity,
    ) {
        self.stash.push_back(SemanticEvent::SelectorDeclaration {
            name: name.into_owned(),
            range,
            original,
            specificity,
        });
    }

    pub fn leave(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        if matches!(
            node.kind(),
            CSS_QUALIFIED_RULE | CSS_NESTED_QUALIFIED_RULE | CSS_MEDIA_AT_RULE
        ) {
            self.current_rule_stack.pop();
            self.stash.push_back(SemanticEvent::RuleEnd);
            if self.is_in_root_selector {
                self.stash.push_back(SemanticEvent::RootSelectorEnd);
                self.is_in_root_selector = false;
            }
        }
    }

    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }
}
