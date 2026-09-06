use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_embeds::EmbeddedData;
use biome_rowan::TokenText;
use std::sync::Arc;

#[derive(Clone)]
pub struct EmbeddedService {
    data: Arc<EmbeddedData>,
}

impl EmbeddedService {
    pub(crate) fn new(data: Arc<EmbeddedData>) -> Self {
        Self { data }
    }

    pub(crate) fn contains_binding(&self, binding: &TokenText) -> bool {
        self.data.binding(binding.text()).is_some()
    }

    pub(crate) fn contains_binding_text(&self, binding: &str) -> bool {
        self.data.binding(binding).is_some()
    }

    pub(crate) fn is_used_as_value(&self, identifier: &TokenText) -> bool {
        self.data.is_used_as_value(identifier.text())
    }

    pub(crate) fn is_used_as_type(&self, identifier: &TokenText) -> bool {
        self.data.is_used_as_type(identifier.text())
    }

    pub(crate) fn is_used(&self, identifier: &TokenText) -> bool {
        self.data.is_used(identifier.text())
    }

    /// Svelte stores are a special case. The `$` prefix is used to "dereference" the store and get its value.
    ///
    /// See also: https://svelte.dev/docs/svelte/stores
    pub(crate) fn is_svelte_store_used(&self, identifier: &TokenText) -> bool {
        self.data.is_svelte_store_used(identifier.text())
    }

    pub(crate) fn is_vue_directive_used(&self, identifier: &TokenText) -> bool {
        self.data.is_vue_directive_used(identifier.text())
    }
}

impl std::fmt::Debug for EmbeddedService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmbeddedService").finish_non_exhaustive()
    }
}

impl FromServices for EmbeddedService {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let service: &Self = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["EmbeddedService"]))?;

        Ok(service.clone())
    }
}
