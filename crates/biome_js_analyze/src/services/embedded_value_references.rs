use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_rowan::{TextRange, TokenText};
use rustc_hash::FxHashMap;

/// Service to track value references from non-source snippets (templates).
/// This is the analyzer-side version that only uses biome_rowan primitives.
#[derive(Debug, Clone)]
pub struct EmbeddedValueReferences(pub Vec<FxHashMap<TextRange, TokenText>>);

impl EmbeddedValueReferences {
    /// Check if an identifier is used as a value in any tracked non-source snippet
    pub(crate) fn is_used_as_value(&self, identifier: &str) -> bool {
        for refs in self.0.iter() {
            if refs.values().any(|token| token.text() == identifier) {
                return true;
            }
        }
        false
    }
}

impl FromServices for EmbeddedValueReferences {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let value_refs: &Self = services.get_service().ok_or_else(|| {
            ServicesDiagnostic::new(rule_key.rule_name(), &["EmbeddedValueReferences"])
        })?;

        Ok(value_refs.clone())
    }
}
