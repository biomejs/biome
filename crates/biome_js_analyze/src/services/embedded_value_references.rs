use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_rowan::{TextRange, TokenText};

/// Service to track references from non-source snippets (templates).
/// This is the analyzer-side version that only uses biome_rowan primitives.
#[derive(Debug, Clone)]
pub struct EmbeddedValueReferences {
    /// Identifiers used as values.
    values: Vec<Vec<(TextRange, TokenText)>>,
    /// Identifiers used only as types (e.g. `icon: IconType`).
    types: Vec<Vec<(TextRange, TokenText)>>,
}

impl EmbeddedValueReferences {
    pub fn new(
        values: Vec<Vec<(TextRange, TokenText)>>,
        types: Vec<Vec<(TextRange, TokenText)>>,
    ) -> Self {
        Self { values, types }
    }

    /// Check if an identifier is used as a value in any tracked non-source snippet.
    pub(crate) fn is_used_as_value(&self, identifier: &str) -> bool {
        self.values
            .iter()
            .flatten()
            .any(|(_, token)| token.text() == identifier)
    }

    /// Check if an identifier is used only as a type in any tracked non-source snippet.
    pub(crate) fn is_used_as_type(&self, identifier: &str) -> bool {
        self.types
            .iter()
            .flatten()
            .any(|(_, token)| token.text() == identifier)
    }

    /// Check if an identifier is referenced as value or type in any
    /// tracked non-source snippet.
    pub(crate) fn is_used(&self, identifier: &str) -> bool {
        self.values
            .iter()
            .chain(&self.types)
            .flatten()
            .any(|(_, token)| token.text() == identifier)
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
