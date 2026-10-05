use biome_css_syntax::{
    AnyCssDashedIdentifier, AnyCssDeclarationName, AnyCssFunction, AnyCssGenericComponentValue,
    AnyCssGenericPropertyValueOrExpression, AnyCssProperty, AnyCssRelativeSelector, AnyCssSelector,
    AnyCssValue, CssDashedIdentifier, CssDeclaration, CssLanguage, CssNestedQualifiedRule,
    CssPropertyAtRule, CssQualifiedRule, CssString,
    CssSyntaxKind::*,
    ScssAtRootSelector, ScssBinaryExpression, ScssExpression, ScssInterpolation,
    ScssUnaryExpression, ScssVariable, decode_css_identifier,
    property_syntax::{
        PropertySyntaxErrorKind, PropertySyntaxParseDiagnostic, PropertySyntaxResult, encode,
    },
    single_expression_item,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxKindSet, TextRange};
use std::collections::VecDeque;

use crate::model::{AnyCssSelectorLike, AnyRuleStart};
use crate::{
    model::{CssProperty, CssPropertyInitialValueKind},
    semantic_model::model::Specificity,
    specificity::{
        evaluate_complex_selector, evaluate_compound_selector, evaluate_partial_combinator_selector,
    },
};

const ROOT_SELECTOR: &str = ":root";

const RELATIVE_SELECTOR_LIST_PARENT_KINDS: SyntaxKindSet<CssLanguage> =
    CssQualifiedRule::KIND_SET.union(CssNestedQualifiedRule::KIND_SET);
const SELECTOR_LIST_PARENT_KINDS: SyntaxKindSet<CssLanguage> =
    RELATIVE_SELECTOR_LIST_PARENT_KINDS.union(ScssAtRootSelector::KIND_SET);
const SCSS_REQUIRES_EVALUATION_KINDS: SyntaxKindSet<CssLanguage> = ScssVariable::KIND_SET
    .union(ScssInterpolation::KIND_SET)
    .union(ScssBinaryExpression::KIND_SET)
    .union(ScssUnaryExpression::KIND_SET)
    .union(AnyCssFunction::KIND_SET);

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
        value: CssPropertyInitialValueKind,
    },
    /// Indicates the start of a `:root` selector
    RootSelectorStart,
    /// Indicates the end of a `:root` selector
    RootSelectorEnd,
    /// Indicates the start of an `@property` rule
    AtProperty {
        property: CssDashedIdentifier,
        initial_value: Option<CssPropertyInitialValueKind>,
        syntax: PropertySyntaxResult,
        inherits: Option<bool>,
        inherits_requires_evaluation: bool,
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
            kind if AnyRuleStart::can_cast(kind) => {
                if let Some(start) = AnyRuleStart::cast(node.clone()) {
                    self.stash.push_back(SemanticEvent::RuleStart(start));
                }
            }
            CSS_SELECTOR_LIST => {
                if !node
                    .parent()
                    .is_some_and(|parent| SELECTOR_LIST_PARENT_KINDS.matches(parent.kind()))
                {
                    return;
                };
                node.children()
                    .filter_map(AnyCssSelector::cast)
                    .for_each(|s| self.process_selector(s));
            }
            CSS_RELATIVE_SELECTOR_LIST => {
                if !node.parent().is_some_and(|parent| {
                    RELATIVE_SELECTOR_LIST_PARENT_KINDS.matches(parent.kind())
                }) {
                    return;
                };
                node.children()
                    .filter_map(AnyCssRelativeSelector::cast)
                    .for_each(|selector| match selector {
                        AnyCssRelativeSelector::CssRelativeSelector(selector) => {
                            if let Ok(selector) = selector.selector() {
                                self.process_selector(selector);
                            }
                        }
                        AnyCssRelativeSelector::ScssPartialCombinatorSelector(selector) => {
                            self.process_selector(selector.into());
                        }
                        AnyCssRelativeSelector::CssBogusSelector(_) => {}
                    });
            }
            CSS_DECLARATION => {
                if node
                    .parent()
                    .is_some_and(|parent| parent.kind() == CSS_SUPPORTS_FEATURE_DECLARATION)
                {
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
                            for property_value in property.values().iter().filter_map(Result::ok) {
                                self.stash.push_back(SemanticEvent::PropertyDeclaration {
                                    node: declaration.clone(),
                                    property: property_name.clone().into(),
                                    value: CssPropertyInitialValueKind::from(property_value),
                                });
                            }
                        }
                        AnyCssProperty::CssGenericProperty(generic) => {
                            let Ok(name) = generic.name() else {
                                return;
                            };
                            let value = match generic.value() {
                                Ok(value) => match value {
                                    AnyCssGenericPropertyValueOrExpression::CssCustomPropertyValue(
                                        value,
                                    ) => CssPropertyInitialValueKind::from(value),
                                    AnyCssGenericPropertyValueOrExpression::CssLegacyFilterValue(
                                        value,
                                    ) => CssPropertyInitialValueKind::from(value),
                                    AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(
                                        list,
                                    ) => CssPropertyInitialValueKind::from(list),
                                    AnyCssGenericPropertyValueOrExpression::ScssExpression(expr) => {
                                        CssPropertyInitialValueKind::from(expr)
                                    }
                                },
                                Err(_) => return,
                            };

                            let property = match name {
                                AnyCssDeclarationName::AnyCssDashedIdentifier(
                                    AnyCssDashedIdentifier::CssDashedIdentifier(name),
                                ) => CssProperty::from(name),
                                AnyCssDeclarationName::AnyCssDashedIdentifier(
                                    AnyCssDashedIdentifier::ScssInterpolatedDashedIdentifier(_),
                                )
                                | AnyCssDeclarationName::ScssInterpolatedIdentifier(_) => {
                                    return;
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
            AnyCssSelector::ScssPartialCombinatorSelector(selector) => {
                let specificity = evaluate_partial_combinator_selector(&selector);
                self.add_selector_event(selector.into(), specificity);
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
        // SCSS-interpolated names like `--#{$name}` can't be resolved statically.
        let Some(property_name) = node
            .declarator()
            .and_then(|d| d.name())
            .ok()
            .and_then(|name| name.as_css_dashed_identifier().cloned())
        else {
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
        let mut syntax = PropertySyntaxResult::Missing;
        let mut inherits = None;
        let mut inherits_requires_evaluation = false;

        for declaration in decls.declarations().into_iter().filter_map(|d| {
            d.as_css_declaration_with_semicolon()
                .and_then(|d| d.declaration().ok())
        }) {
            if let Ok(biome_css_syntax::AnyCssProperty::CssGenericProperty(prop)) =
                declaration.property()
                && let Ok(prop_name) = prop.name()
            {
                let prop_name = prop_name.to_trimmed_string();
                let prop_name = decode_css_identifier(&prop_name);
                if prop_name.eq_ignore_ascii_case("initial-value") {
                    initial_value = prop.value().ok().map(|value| match value {
                        AnyCssGenericPropertyValueOrExpression::CssCustomPropertyValue(value) => {
                            CssPropertyInitialValueKind::from(value)
                        }
                        AnyCssGenericPropertyValueOrExpression::CssLegacyFilterValue(value) => {
                            CssPropertyInitialValueKind::from(value)
                        }
                        AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(
                            list,
                        ) => CssPropertyInitialValueKind::from(list),
                        AnyCssGenericPropertyValueOrExpression::ScssExpression(expr) => {
                            CssPropertyInitialValueKind::from(expr)
                        }
                    });
                } else if prop_name.eq_ignore_ascii_case("syntax") {
                    syntax = match prop.value() {
                        Ok(value) => parse_property_syntax(value),
                        Err(_) => invalid_property_syntax(prop.range()),
                    };
                } else if prop_name.eq_ignore_ascii_case("inherits") {
                    let Ok(value) = prop.value() else {
                        continue;
                    };
                    let value_text = value.to_trimmed_string();
                    inherits = if value_text.eq_ignore_ascii_case("true") {
                        Some(true)
                    } else if value_text.eq_ignore_ascii_case("false") {
                        Some(false)
                    } else {
                        None
                    };
                    inherits_requires_evaluation = inherits.is_none()
                        && value
                            .as_scss_expression()
                            .is_some_and(scss_expression_requires_evaluation);
                }
            }
        }

        self.stash.push_back(SemanticEvent::AtProperty {
            property: property_name,
            initial_value,
            syntax,
            inherits,
            inherits_requires_evaluation,
            range: node.range(),
        });
    }

    fn add_selector_event(&mut self, node: AnyCssSelectorLike, specificity: Specificity) {
        self.stash
            .push_back(SemanticEvent::SelectorDeclaration { node, specificity });
    }

    pub fn leave(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        if AnyRuleStart::can_cast(node.kind()) {
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

fn parse_property_syntax(value: AnyCssGenericPropertyValueOrExpression) -> PropertySyntaxResult {
    let range = value.range();
    let string = match value {
        AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(list) => {
            let mut components = list.iter();
            let Some(AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::CssString(string))) =
                components.next()
            else {
                return invalid_property_syntax(range);
            };
            if components.next().is_some() {
                return invalid_property_syntax(range);
            }
            string
        }
        AnyCssGenericPropertyValueOrExpression::ScssExpression(expression) => {
            let Some(string) = scss_expression_string(&expression) else {
                return if scss_expression_requires_evaluation(&expression) {
                    PropertySyntaxResult::Dynamic
                } else {
                    invalid_property_syntax(range)
                };
            };
            string
        }
        _ => return invalid_property_syntax(range),
    };
    encode(&string)
}

fn scss_expression_string(expression: &ScssExpression) -> Option<CssString> {
    single_expression_item(expression)?
        .as_any_css_value()?
        .as_css_string()
        .cloned()
}

fn scss_expression_requires_evaluation(expression: &ScssExpression) -> bool {
    expression
        .syntax()
        .descendants()
        .any(|node| SCSS_REQUIRES_EVALUATION_KINDS.matches(node.kind()))
}

fn invalid_property_syntax(range: TextRange) -> PropertySyntaxResult {
    PropertySyntaxResult::Error(PropertySyntaxParseDiagnostic::new(
        PropertySyntaxErrorKind::ExpectedString,
        range,
    ))
}
