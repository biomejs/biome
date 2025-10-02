use biome_css_syntax::{
    AnyCssDeclarationName, AnyCssProperty, AnyCssSelector, CssDeclaration, CssPropertyAtRule,
    CssRelativeSelector, CssSyntaxKind::*,
};
use biome_rowan::{AstNode, SyntaxNodeOptionExt, TextRange};
use std::collections::VecDeque;

use crate::model::{AnyCssSelectorLike, AnyRuleStart};
use crate::{
    model::{CssProperty, CssPropertyInitialValue},
    semantic_model::model::Specificity,
    specificity::{evaluate_complex_selector, evaluate_compound_selector},
};

const ROOT_SELECTOR: &str = ":root";

#[derive(Debug)]
pub enum SemanticEvent {
    RuleStart(AnyRuleStart),
    RuleEnd,
    SelectorDeclaration {
        node: AnyCssSelectorLike,
        specificity: Specificity,
    },
    PropertyDeclaration {
        node: CssDeclaration,
        property: CssProperty,
        value: CssPropertyInitialValue,
    },
    /// Indicates the start of a `:root` selector
    RootSelectorStart,
    /// Indicates the end of a `:root` selector
    RootSelectorEnd,
    /// Indicates the start of an `@property` rule
    AtProperty {
        property: CssProperty,
        initial_value: Option<CssPropertyInitialValue>,
        syntax: Option<String>,
        inherits: Option<bool>,
        range: TextRange,
    },
}

#[derive(Default, Debug)]
pub struct SemanticEventExtractor {
    stash: VecDeque<SemanticEvent>,
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
                || kind == CSS_CONTAINER_AT_RULE
                || kind == CSS_MEDIA_AT_RULE
                || kind == CSS_STARTING_STYLE_AT_RULE
                || kind == CSS_SUPPORTS_AT_RULE =>
            {
                if let Some(start) = AnyRuleStart::cast(node.clone()) {
                    self.stash.push_back(SemanticEvent::RuleStart(start));
                }
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
                // SAFETY: checked by the previous match
                let declaration = CssDeclaration::cast_ref(node).unwrap();

                if let Ok(property) = declaration.property() {
                    match property {
                        AnyCssProperty::CssComposesProperty(property) => {
                            let Ok(property_name) = property.name() else {
                                return;
                            };
                            let Ok(property_value) = property.value() else {
                                return;
                            };
                            self.stash.push_back(SemanticEvent::PropertyDeclaration {
                                node: declaration,
                                property: property_name.into(),
                                value: CssPropertyInitialValue::from(property_value),
                            });
                        }
                        AnyCssProperty::CssGenericProperty(generic) => {
                            let Ok(name) = generic.name() else {
                                return;
                            };
                            let value = CssPropertyInitialValue::from(generic.value());

                            let property = match name {
                                AnyCssDeclarationName::CssDashedIdentifier(name) => {
                                    CssProperty::from(name)
                                }
                                AnyCssDeclarationName::CssIdentifier(name) => {
                                    CssProperty::from(name)
                                }
                                AnyCssDeclarationName::TwValueThemeReference(name) => {
                                    let Ok(ident) = name.reference() else {
                                        return;
                                    };
                                    CssProperty::from(ident)
                                }
                            };

                            self.stash.push_back(SemanticEvent::PropertyDeclaration {
                                node: declaration,
                                property,
                                value,
                            });
                        }
                        AnyCssProperty::CssBogusProperty(_) => {}
                    }
                }
            }
            CSS_PROPERTY_AT_RULE => {
                // SAFETY: the match checks for its kind already.
                let property = CssPropertyAtRule::cast_ref(node).unwrap();
                self.process_at_property(property);
            }
            _ => {}
        }
    }

    #[inline]
    fn process_selector(&mut self, selector: AnyCssSelector) {
        match selector {
            AnyCssSelector::CssComplexSelector(s) => {
                let specificity = evaluate_complex_selector(&s);
                self.add_selector_event(s.into(), specificity);
            }

            AnyCssSelector::CssCompoundSelector(selector) => {
                let selector_text = selector.to_trimmed_string();
                if selector_text == ROOT_SELECTOR {
                    self.stash.push_back(SemanticEvent::RootSelectorStart);
                    self.is_in_root_selector = true;
                }
                let specificity = evaluate_compound_selector(&selector);
                self.add_selector_event(selector.into(), specificity)
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
    fn process_at_property(&mut self, node: CssPropertyAtRule) {
        let Ok(property_name) = node.name() else {
            return;
        };
        let Some(decls) = node
            .block()
            .ok()
            .and_then(|block| block.as_css_declaration_block().cloned())
        else {
            return;
        };

        let mut initial_value = None;
        let mut syntax = None;
        let mut inherits = None;

        for declaration in decls.declarations().into_iter().filter_map(|d| {
            d.as_css_declaration_with_semicolon()
                .and_then(|d| d.declaration().ok())
        }) {
            if let Ok(biome_css_syntax::AnyCssProperty::CssGenericProperty(prop)) =
                declaration.property()
                && let Ok(prop_name) = prop.name()
            {
                match prop_name.to_trimmed_string().as_str() {
                    "initial-value" => {
                        initial_value = Some(CssPropertyInitialValue::from(prop.value()));
                    }
                    "syntax" => {
                        syntax = Some(prop.value().to_trimmed_string().to_string());
                    }
                    "inherits" => {
                        inherits = Some(
                            prop.value()
                                .to_trimmed_string()
                                .eq_ignore_ascii_case("true"),
                        );
                    }
                    _ => {}
                }
            }
        }

        self.stash.push_back(SemanticEvent::AtProperty {
            property: CssProperty::from(property_name),
            initial_value,
            syntax,
            inherits,
            range: node.range(),
        });
    }

    fn add_selector_event(&mut self, node: AnyCssSelectorLike, specificity: Specificity) {
        self.stash
            .push_back(SemanticEvent::SelectorDeclaration { node, specificity });
    }

    pub fn leave(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        if matches!(
            node.kind(),
            CSS_QUALIFIED_RULE | CSS_NESTED_QUALIFIED_RULE | CSS_MEDIA_AT_RULE
        ) {
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
