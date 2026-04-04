use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_rowan::{TextRange, TokenText};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct EmbeddedBindings(pub Vec<FxHashMap<TextRange, TokenText>>);

impl EmbeddedBindings {
    pub(crate) fn contains_binding(&self, binding: &str) -> bool {
        for bindings in self.0.iter() {
            if bindings.values().any(|token| token.text() == binding) {
                return true;
            }
        }
        false
    }
}

impl FromServices for EmbeddedBindings {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let embedded_bindings: &Self = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["EmbeddedBindings"]))?;

        Ok(embedded_bindings.clone())
    }
}
