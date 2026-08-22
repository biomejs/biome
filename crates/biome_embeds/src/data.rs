use crate::bindings::EmbeddedBinding;
use crate::references::{
    EmbeddedTypeReference, EmbeddedValueReference, svelte_store_reference_name,
};

#[derive(Debug, Default)]
pub struct EmbeddedData {
    bindings: Vec<EmbeddedBinding>,
    value_references: Vec<EmbeddedValueReference>,
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

    pub fn contains_binding(&self, name: &str) -> bool {
        self.bindings
            .iter()
            .any(|binding| binding.text.text() == name)
    }

    pub fn is_used_as_value(&self, name: &str) -> bool {
        self.value_references
            .iter()
            .any(|reference| reference.text.text() == name)
    }

    pub fn is_used_as_type(&self, name: &str) -> bool {
        self.type_references
            .iter()
            .any(|reference| reference.text.text() == name)
    }

    pub fn is_used(&self, name: &str) -> bool {
        self.is_used_as_value(name) || self.is_used_as_type(name)
    }

    pub fn is_svelte_store_used(&self, name: &str) -> bool {
        self.value_references.iter().any(|reference| {
            svelte_store_reference_name(reference.text.text())
                .is_some_and(|store_name| store_name == name)
        })
    }
}
