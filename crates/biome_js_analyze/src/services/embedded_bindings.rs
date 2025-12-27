use biome_analyze::{
    FromServices, Phase, Phases, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic,
};
use biome_rowan::{TextRange, TokenText};
use rustc_hash::FxHashMap;
use std::sync::Arc;

pub(crate) struct EmbeddedBindingsService(pub(crate) Vec<FxHashMap<TextRange, TokenText>>);

impl EmbeddedBindingsService {
    pub(crate) fn contains_binding(&self, binding: &str) -> bool {
        for bindings in self.0.iter() {
            if bindings.values().any(|token| token.text() == binding) {
                return true;
            }
        }
        false
    }
}

impl FromServices for EmbeddedBindingsService {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let embedded_bindings: &Arc<Self> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["EmbeddedBindings"]))?;

        Ok(Self(embedded_bindings.0.clone()))
    }
}

impl Phase for EmbeddedBindingsService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}
