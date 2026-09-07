use crate::bindings::EmbeddedBinding;
use crate::references::{
    EmbeddedTypeReference, EmbeddedValueReference, is_potential_vue_directive_reference,
    svelte_store_reference_name, vue_directive_name_matches_reference_name,
};
use biome_rowan::TokenText;

/// The result of resolving a Vue custom directive in an embedded document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VueDirectiveResolution {
    /// A local declaration resolves the directive.
    Declared,
    /// No local declaration resolves the directive.
    Undeclared,
    /// The component may declare the directive through syntax that cannot be
    /// resolved statically.
    Unknown,
}

/// Vue custom-directive declarations collected from an embedded document.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct VueDirectiveDeclarations {
    setup_bindings: Vec<TokenText>,
    option_names: Vec<TokenText>,
    has_unknown_options: bool,
}

impl VueDirectiveDeclarations {
    pub(crate) fn new(
        setup_bindings: Vec<TokenText>,
        option_names: Vec<TokenText>,
        has_unknown_options: bool,
    ) -> Self {
        Self {
            setup_bindings,
            option_names,
            has_unknown_options,
        }
    }

    /// Declarations for a document we could not read, so that every directive
    /// resolves to [`VueDirectiveResolution::Unknown`] instead of being reported.
    pub(crate) fn unknown() -> Self {
        Self {
            has_unknown_options: true,
            ..Self::default()
        }
    }

    /// Resolves a template directive name against statically known declarations.
    pub fn resolve(&self, directive_name: &str) -> VueDirectiveResolution {
        if !is_potential_vue_directive_name(directive_name) {
            return VueDirectiveResolution::Undeclared;
        }

        if self.setup_bindings.iter().any(|binding| {
            vue_directive_name_matches_reference_name(directive_name, binding.text())
        }) || self
            .option_names
            .iter()
            .any(|name| vue_directive_name_matches_option_name(directive_name, name.text()))
        {
            VueDirectiveResolution::Declared
        } else if self.has_unknown_options {
            VueDirectiveResolution::Unknown
        } else {
            VueDirectiveResolution::Undeclared
        }
    }
}

/// Bindings and references collected across a host document and its embedded
/// language snippets.
///
/// Binding and ordinary reference queries compare identifier text exactly.
/// Framework-specific queries apply Svelte store and Vue custom-directive
/// naming rules.
#[derive(Debug, Default)]
pub struct EmbeddedData {
    /// Bindings created in embedded documents
    bindings: Vec<EmbeddedBinding>,
    /// Refernces that are used as values
    value_references: Vec<EmbeddedValueReference>,
    /// References that are only used in type contexts
    type_references: Vec<EmbeddedTypeReference>,
    /// Vue custom-directive declarations available to the host template.
    vue_directive_declarations: VueDirectiveDeclarations,
}

impl EmbeddedData {
    pub(crate) fn new(
        bindings: Vec<EmbeddedBinding>,
        value_references: Vec<EmbeddedValueReference>,
        type_references: Vec<EmbeddedTypeReference>,
        vue_directive_declarations: VueDirectiveDeclarations,
    ) -> Self {
        Self {
            bindings,
            value_references,
            type_references,
            vue_directive_declarations,
        }
    }

    /// Returns whether a collected binding has the identifier text `name`.
    pub fn contains_binding(&self, name: &str) -> bool {
        self.bindings
            .iter()
            .any(|binding| binding.text.text() == name)
    }

    /// Returns whether a collected value reference has the identifier text
    /// `name`.
    pub fn is_used_as_value(&self, name: &str) -> bool {
        self.value_references
            .iter()
            .any(|reference| reference.text.text() == name)
    }

    /// Returns whether a collected type reference has the identifier text
    /// `name`.
    pub fn is_used_as_type(&self, name: &str) -> bool {
        self.type_references
            .iter()
            .any(|reference| reference.text.text() == name)
    }

    /// Returns whether a collected value or type reference has the identifier
    /// text `name`.
    pub fn is_used(&self, name: &str) -> bool {
        self.is_used_as_value(name) || self.is_used_as_type(name)
    }

    /// Returns whether a Svelte value reference uses the store named `name`
    /// through its `$`-prefixed auto-subscription.
    pub fn is_svelte_store_used(&self, name: &str) -> bool {
        self.value_references.iter().any(|reference| {
            svelte_store_reference_name(reference.text.text())
                .is_some_and(|store_name| store_name == name)
        })
    }

    /// Returns whether a collected Vue custom-directive reference resolves to
    /// the JavaScript binding named `name`.
    pub fn is_vue_directive_used(&self, name: &str) -> bool {
        is_potential_vue_directive_reference(name)
            && self.value_references.iter().any(|reference| {
                vue_directive_name_matches_reference_name(reference.text.text(), name)
            })
    }

    /// Resolves a Vue custom directive against declarations in the host component.
    pub fn resolve_vue_directive(&self, name: &str) -> VueDirectiveResolution {
        self.vue_directive_declarations.resolve(name)
    }
}

/// Returns whether `directive_name` can be a custom Vue directive.
pub fn is_potential_vue_directive_name(directive_name: &str) -> bool {
    let Some(name) = directive_name.strip_prefix("v-") else {
        return false;
    };

    is_valid_vue_directive_suffix(name)
}

fn is_valid_vue_directive_suffix(name: &str) -> bool {
    !name.is_empty()
        && !name.ends_with('-')
        && !name.as_bytes().windows(2).any(|pair| pair == b"--")
}

/// Returns the canonical `<script setup>` binding name for a custom Vue directive.
pub fn vue_directive_binding_name(directive_name: &str) -> Option<String> {
    if !is_potential_vue_directive_name(directive_name) {
        return None;
    }

    let mut binding_name = String::with_capacity(directive_name.len());
    binding_name.push('v');

    let mut capitalize = true;
    for character in directive_name.strip_prefix("v-")?.chars() {
        if character == '-' {
            capitalize = true;
        } else if capitalize {
            binding_name.extend(character.to_uppercase());
            capitalize = false;
        } else {
            binding_name.push(character);
        }
    }

    Some(binding_name)
}

fn vue_directive_name_matches_option_name(directive_name: &str, option_name: &str) -> bool {
    let Some(directive_name) = directive_name.strip_prefix("v-") else {
        return false;
    };
    if !is_valid_vue_directive_suffix(directive_name) {
        return false;
    }

    if directive_name == option_name {
        return true;
    }

    let directive_characters = directive_name.chars();
    let mut option_characters = option_name.chars();
    let mut capitalize_next = false;
    let mut first_character = true;

    for directive_character in directive_characters {
        if directive_character == '-' {
            capitalize_next = true;
            continue;
        }

        let Some(option_character) = option_characters.next() else {
            return false;
        };
        let matches = if first_character && directive_character.is_ascii_lowercase() {
            option_character == directive_character
                || (option_character.is_ascii_uppercase()
                    && option_character.eq_ignore_ascii_case(&directive_character))
        } else if capitalize_next && directive_character.is_ascii_alphabetic() {
            option_character.is_ascii_uppercase()
                && option_character.eq_ignore_ascii_case(&directive_character)
        } else {
            option_character == directive_character
        };
        if !matches {
            return false;
        }

        capitalize_next = false;
        first_character = false;
    }

    option_characters.next().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_rowan::{RawSyntaxKind, TextRange, TokenText};

    #[test]
    fn is_vue_directive_used_finds_custom_directive() {
        let data = EmbeddedData::new(
            Vec::new(),
            vec![EmbeddedValueReference {
                range: TextRange::default(),
                text: TokenText::new_raw(RawSyntaxKind(0), "v-click-outside"),
            }],
            Vec::new(),
            VueDirectiveDeclarations::default(),
        );

        assert!(data.is_vue_directive_used("vClickOutside"));
    }

    #[test]
    fn resolve_vue_directive_matches_setup_bindings_and_option_names() {
        let declarations = VueDirectiveDeclarations::new(
            vec![TokenText::new_raw(RawSyntaxKind(0), "vSetupDirective")],
            vec![
                TokenText::new_raw(RawSyntaxKind(0), "kebab-directive"),
                TokenText::new_raw(RawSyntaxKind(0), "camelDirective"),
                TokenText::new_raw(RawSyntaxKind(0), "PascalDirective"),
            ],
            false,
        );

        assert_eq!(
            declarations.resolve("v-setup-directive"),
            VueDirectiveResolution::Declared
        );
        assert_eq!(
            declarations.resolve("v-kebab-directive"),
            VueDirectiveResolution::Declared
        );
        assert_eq!(
            declarations.resolve("v-camel-directive"),
            VueDirectiveResolution::Declared
        );
        assert_eq!(
            declarations.resolve("v-pascal-directive"),
            VueDirectiveResolution::Declared
        );
        assert_eq!(
            declarations.resolve("v-missing-directive"),
            VueDirectiveResolution::Undeclared
        );
    }

    #[test]
    fn resolve_vue_directive_keeps_known_names_when_options_are_unknown() {
        let declarations = VueDirectiveDeclarations::new(
            Vec::new(),
            vec![TokenText::new_raw(RawSyntaxKind(0), "knownDirective")],
            true,
        );

        assert_eq!(
            declarations.resolve("v-known-directive"),
            VueDirectiveResolution::Declared
        );
        assert_eq!(
            declarations.resolve("v-may-be-declared"),
            VueDirectiveResolution::Unknown
        );
    }

    #[test]
    fn vue_directive_binding_name_rejects_malformed_names() {
        assert_eq!(
            vue_directive_binding_name("v-click-outside").as_deref(),
            Some("vClickOutside")
        );
        assert_eq!(vue_directive_binding_name("v-"), None);
        assert_eq!(vue_directive_binding_name("v-trailing-"), None);
        assert_eq!(vue_directive_binding_name("v-double--hyphen"), None);
    }
}
