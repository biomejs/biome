//! Tailwind CSS configuration declared in a stylesheet.
//!
//! Tailwind CSS v4 is configured in CSS: `@theme` blocks define theme values,
//! `@utility` rules define utilities, and `@custom-variant` rules define
//! variants. The module graph records them per file, and
//! [tailwind_stylesheet](crate::tailwind_stylesheet) merges a stylesheet with
//! the files it imports.

use biome_css_syntax::{
    AnyCssBracketedValueItem, AnyCssCustomIdentifier, AnyCssDashedIdentifier,
    AnyCssDeclarationName, AnyCssDeclarationOrRule, AnyCssDeclarationOrRuleBlock, AnyCssExpression,
    AnyCssFunction, AnyCssFunctionName, AnyCssProperty, AnyCssValue, AnyTwUtilityName,
    CssDeclaration, CssIdentifier, TwCustomVariantAtRule, TwThemeAtRule, TwUtilityAtRule,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, Text};

use super::CssImport;

/// The Tailwind CSS configuration a stylesheet declares, in source order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TailwindStylesheet {
    /// Theme variables from `@theme` blocks, such as `--color-brand: #00f`,
    /// and resets, such as `--color-*: initial`.
    pub theme: Vec<TailwindThemeEntry>,
    /// Utilities with a fixed name from `@utility` rules.
    pub utilities: Vec<TailwindUtility>,
    /// Utilities that take a value from `@utility` rules ending in `-*`, such
    /// as `@utility tab-* { ... }`.
    pub functional_utilities: Vec<TailwindFunctionalUtility>,
    /// Variant names from `@custom-variant` rules.
    pub custom_variants: Vec<Text>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TailwindThemeEntry {
    /// A theme variable, such as `--color-brand: #00f`.
    Variable { name: Text, value: Text },
    /// A namespace reset, such as `--color-*: initial`. `reference` is the part
    /// before `-*`: `--color`, or `--` for `--*: initial`.
    Reset { reference: Text },
}

/// A utility declared with `@utility`, such as `@utility tab-4 { tab-size: 4; }`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TailwindUtility {
    pub name: Text,
    /// The property names of the declarations in the rule, including nested
    /// rules.
    pub properties: Box<[Text]>,
    /// The number of declarations in the rule, including nested rules.
    pub declaration_count: usize,
}

/// A utility declared with `@utility <name>-*`, such as
/// `@utility tab-* { tab-size: --value(integer); }`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TailwindFunctionalUtility {
    /// The part before `-*`, such as `tab`. A negative utility keeps its
    /// sign, as in `-zoom-in`.
    pub name: Text,
    /// The declarations of the rule, including nested rules, in source order.
    pub declarations: Box<[TailwindUtilityDeclaration]>,
}

/// A declaration of a [TailwindFunctionalUtility].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TailwindUtilityDeclaration {
    /// The property name, such as `tab-size` or `--tw-enter-opacity`.
    pub property: Text,
    /// The `--value(…)` and `--modifier(…)` functions in the value, in source
    /// order.
    pub functions: Box<[TailwindValueFunction]>,
}

/// A `--value(…)` or `--modifier(…)` function, which Tailwind CSS replaces
/// with the candidate's value or modifier when one of the arguments accepts it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TailwindValueFunction {
    /// Whether the function is `--modifier(…)` rather than `--value(…)`.
    pub is_modifier: bool,
    pub arguments: Box<[TailwindValueArgument]>,
}

/// An argument of a [TailwindValueFunction].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TailwindValueArgument {
    /// A bare value type, such as `integer` in `--value(integer)`.
    Type(Text),
    /// An arbitrary value type, such as `length` in `--value([length])`, or
    /// `*` in `--value([*])`.
    ArbitraryType(Text),
    /// A theme namespace, such as `--tab-size` in `--value(--tab-size-*)`.
    Theme(Text),
    /// A literal value, such as `auto` in `--value("auto")`.
    Literal(Text),
}

/// A point in the source of a [TailwindStylesheet], as the number of theme
/// entries, utilities, and custom variants declared before it.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct TailwindPosition {
    theme: usize,
    utilities: usize,
    functional_utilities: usize,
    custom_variants: usize,
}

/// An `@import` that Tailwind CSS inlines, such as `@import "./theme.css"`.
/// Tailwind CSS leaves `@import url("./theme.css")` to the browser.
#[derive(Clone, Debug)]
pub(crate) struct TailwindImport {
    pub(crate) import: CssImport,
    /// Where the import appears among the configuration of its file.
    pub(crate) position: TailwindPosition,
}

impl TailwindStylesheet {
    pub fn is_empty(&self) -> bool {
        self.theme.is_empty()
            && self.utilities.is_empty()
            && self.functional_utilities.is_empty()
            && self.custom_variants.is_empty()
    }

    /// The position after everything declared so far.
    pub(crate) fn end(&self) -> TailwindPosition {
        TailwindPosition {
            theme: self.theme.len(),
            utilities: self.utilities.len(),
            functional_utilities: self.functional_utilities.len(),
            custom_variants: self.custom_variants.len(),
        }
    }

    /// Appends what `other` declares between `start` and `end`.
    pub(crate) fn extend_between(
        &mut self,
        other: &Self,
        start: TailwindPosition,
        end: TailwindPosition,
    ) {
        self.theme
            .extend_from_slice(&other.theme[start.theme..end.theme]);
        self.utilities
            .extend_from_slice(&other.utilities[start.utilities..end.utilities]);
        self.functional_utilities.extend_from_slice(
            &other.functional_utilities[start.functional_utilities..end.functional_utilities],
        );
        self.custom_variants
            .extend_from_slice(&other.custom_variants[start.custom_variants..end.custom_variants]);
    }

    pub(crate) fn visit_theme(&mut self, theme: &TwThemeAtRule) {
        let Ok(AnyCssDeclarationOrRuleBlock::CssDeclarationOrRuleBlock(block)) = theme.block()
        else {
            return;
        };
        // Only the block's own declarations define theme values. Nested rules,
        // such as `@keyframes`, don't.
        for item in block.items() {
            let AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(declaration) = item else {
                continue;
            };
            let Ok(AnyCssProperty::CssGenericProperty(property)) = declaration
                .declaration()
                .and_then(|declaration| declaration.property())
            else {
                continue;
            };
            let Ok(value) = property.value() else {
                continue;
            };
            match property.name() {
                Ok(AnyCssDeclarationName::AnyCssDashedIdentifier(name)) => {
                    self.theme.push(TailwindThemeEntry::Variable {
                        name: name.syntax().text_trimmed().to_string().into(),
                        value: value.syntax().text_trimmed().to_string().into(),
                    });
                }
                Ok(AnyCssDeclarationName::TwValueThemeReference(reference)) => {
                    if let Ok(reference) = reference.reference() {
                        self.theme.push(TailwindThemeEntry::Reset {
                            reference: reference.syntax().text_trimmed().to_string().into(),
                        });
                    }
                }
                _ => {}
            }
        }
    }

    pub(crate) fn visit_utility(&mut self, utility: &TwUtilityAtRule) {
        let Ok(block) = utility.block() else {
            return;
        };
        match utility.name() {
            Ok(AnyTwUtilityName::CssIdentifier(name)) => {
                self.utilities.push(TailwindUtility::new(&name, &block));
            }
            Ok(AnyTwUtilityName::TwFunctionalUtilityName(name)) => {
                if let Ok(name) = name.identifier() {
                    self.functional_utilities
                        .push(TailwindFunctionalUtility::new(&name, &block));
                }
            }
            Err(_) => {}
        }
    }

    pub(crate) fn visit_custom_variant(&mut self, variant: &TwCustomVariantAtRule) {
        if let Ok(name) = variant.name() {
            self.custom_variants
                .push(name.syntax().text_trimmed().to_string().into());
        }
    }
}

impl TailwindUtility {
    fn new(name: &CssIdentifier, block: &AnyCssDeclarationOrRuleBlock) -> Self {
        let mut properties = Vec::new();
        let mut declaration_count = 0;
        for declaration in block
            .syntax()
            .descendants()
            .filter_map(CssDeclaration::cast)
        {
            declaration_count += 1;
            if let Ok(AnyCssProperty::CssGenericProperty(property)) = declaration.property()
                && let Ok(AnyCssDeclarationName::CssIdentifier(property_name)) = property.name()
            {
                properties.push(property_name.syntax().text_trimmed().to_string().into());
            }
        }
        Self {
            name: name.syntax().text_trimmed().to_string().into(),
            properties: properties.into_boxed_slice(),
            declaration_count,
        }
    }
}

impl TailwindFunctionalUtility {
    fn new(name: &CssIdentifier, block: &AnyCssDeclarationOrRuleBlock) -> Self {
        let declarations = block
            .syntax()
            .descendants()
            .filter_map(CssDeclaration::cast)
            .filter_map(|declaration| {
                let AnyCssProperty::CssGenericProperty(property) = declaration.property().ok()?
                else {
                    return None;
                };
                let functions = property
                    .value()
                    .ok()?
                    .syntax()
                    .descendants()
                    .filter_map(AnyCssFunction::cast)
                    .filter_map(|function| TailwindValueFunction::new(&function))
                    .collect();
                Some(TailwindUtilityDeclaration {
                    property: property
                        .name()
                        .ok()?
                        .syntax()
                        .text_trimmed()
                        .to_string()
                        .into(),
                    functions,
                })
            })
            .collect();
        Self {
            name: name.syntax().text_trimmed().to_string().into(),
            declarations,
        }
    }
}

impl TailwindValueFunction {
    fn new(function: &AnyCssFunction) -> Option<Self> {
        let AnyCssFunction::CssFunction(function) = function else {
            return None;
        };
        let Ok(AnyCssFunctionName::CssIdentifier(name)) = function.name() else {
            return None;
        };
        let is_modifier = match name.syntax().text_trimmed().to_string().as_str() {
            "--value" => false,
            "--modifier" => true,
            _ => return None,
        };
        let arguments = function
            .items()
            .iter()
            .filter_map(|argument| {
                let Ok(AnyCssExpression::CssListOfComponentValuesExpression(argument)) = argument
                else {
                    return None;
                };
                let mut values = argument.css_component_value_list().iter();
                let value = values.next()?;
                if values.next().is_some() {
                    return None;
                }
                TailwindValueArgument::new(&value)
            })
            .collect();
        Some(Self {
            is_modifier,
            arguments,
        })
    }
}

impl TailwindValueArgument {
    fn new(value: &AnyCssValue) -> Option<Self> {
        match value {
            AnyCssValue::CssIdentifier(identifier) => Some(Self::Type(node_text(identifier))),
            AnyCssValue::CssBracketedValue(bracketed) => {
                let mut items = bracketed.items().iter();
                let Some(AnyCssBracketedValueItem::AnyCssCustomIdentifier(
                    AnyCssCustomIdentifier::CssCustomIdentifier(data_type),
                )) = items.next()
                else {
                    return None;
                };
                items
                    .next()
                    .is_none()
                    .then(|| Self::ArbitraryType(node_text(&data_type)))
            }
            AnyCssValue::TwValueThemeReference(reference) => {
                Some(Self::Theme(node_text(&reference.reference().ok()?)))
            }
            // Tailwind CSS reads `--value(--tab-size)` as `--value(--tab-size-*)`.
            AnyCssValue::AnyCssDashedIdentifier(AnyCssDashedIdentifier::CssDashedIdentifier(
                reference,
            )) => Some(Self::Theme(node_text(reference))),
            AnyCssValue::CssString(literal) => {
                Some(Self::Literal(literal.inner_string_text().ok()?.into()))
            }
            _ => None,
        }
    }
}

fn node_text(node: &impl AstNode) -> Text {
    node.syntax().text_trimmed().to_string().into()
}
