use crate::bindings::EmbeddedBinding;
use crate::references::{
    EmbeddedTypeReference, EmbeddedValueReference, is_potential_vue_directive_reference,
    svelte_store_reference_name, vue_directive_name_matches_reference_name,
};

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
}

impl EmbeddedData {
    pub(crate) fn new(
        bindings: Vec<EmbeddedBinding>,
        value_references: Vec<EmbeddedValueReference>,
        type_references: Vec<EmbeddedTypeReference>,
    ) -> Self {
        Self {
            bindings,
            value_references,
            type_references,
        }
    }

    /// Returns the first collected binding with the identifier text `name`.
    pub fn binding(&self, name: &str) -> Option<&EmbeddedBinding> {
        self.bindings
            .iter()
            .find(|binding| binding.text.text() == name)
    }

    /// Returns the first imported binding with the identifier text `name`.
    pub fn binding_with_source(&self, name: &str) -> Option<&EmbeddedBinding> {
        self.bindings
            .iter()
            .find(|binding| binding.text.text() == name && binding.source.is_some())
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
        );

        assert!(data.is_vue_directive_used("vClickOutside"));
    }
}
