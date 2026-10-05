//! Tailwind CSS configuration declared in a stylesheet.
//!
//! Tailwind CSS v4 is configured in CSS: `@theme` blocks define theme values,
//! `@utility` rules define utilities, and `@custom-variant` rules define
//! variants. The module graph records them per file, and
//! [tailwind_stylesheet](crate::tailwind_stylesheet) merges a stylesheet with
//! the files it imports.

use biome_css_syntax::{
    AnyCssDeclarationName, AnyCssDeclarationOrRule, AnyCssDeclarationOrRuleBlock, AnyCssProperty,
    AnyTwUtilityName, CssDeclaration, TwCustomVariantAtRule, TwThemeAtRule, TwUtilityAtRule,
};
use biome_rowan::{AstNode, Text};

/// The Tailwind CSS configuration a stylesheet declares, in source order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TailwindStylesheet {
    /// Theme variables from `@theme` blocks, such as `--color-brand: #00f`,
    /// and resets, such as `--color-*: initial`.
    pub theme: Vec<TailwindThemeEntry>,
    /// Utilities with a fixed name from `@utility` rules.
    pub utilities: Vec<TailwindUtility>,
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

impl TailwindStylesheet {
    pub fn is_empty(&self) -> bool {
        self.theme.is_empty() && self.utilities.is_empty() && self.custom_variants.is_empty()
    }

    pub(crate) fn extend(&mut self, other: &Self) {
        self.theme.extend(other.theme.iter().cloned());
        self.utilities.extend(other.utilities.iter().cloned());
        self.custom_variants
            .extend(other.custom_variants.iter().cloned());
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
        // Utilities ending in `-*` take values, which aren't supported yet.
        let Ok(AnyTwUtilityName::CssIdentifier(name)) = utility.name() else {
            return;
        };
        let Ok(block) = utility.block() else {
            return;
        };
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
        self.utilities.push(TailwindUtility {
            name: name.syntax().text_trimmed().to_string().into(),
            properties: properties.into_boxed_slice(),
            declaration_count,
        });
    }

    pub(crate) fn visit_custom_variant(&mut self, variant: &TwCustomVariantAtRule) {
        if let Ok(name) = variant.name() {
            self.custom_variants
                .push(name.syntax().text_trimmed().to_string().into());
        }
    }
}
