use biome_analyze::{
    FromServices, Phase, Phases, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic,
};
use biome_rowan::{TextRange, TokenText};
use rustc_hash::FxHashMap;
use std::sync::Arc;

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
        dbg!(rule_key);
        let embedded_bindings: &EmbeddedBindings = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["EmbeddedBindings"]))?;

        dbg!(&embedded_bindings);
        Ok(embedded_bindings.clone())
    }
}
