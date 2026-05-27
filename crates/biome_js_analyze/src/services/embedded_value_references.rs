use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_rowan::{TextRange, TokenText};

/// Service to track references from non-source snippets (templates).
/// This is the analyzer-side version that only uses biome_rowan primitives.
#[derive(Debug, Clone)]
pub struct EmbeddedValueReferences {
    /// Identifiers used as values.
    value: Vec<Vec<(TextRange, TokenText)>>,
    /// Identifiers used only as types (e.g. `icon: IconType`).
    type_only: Vec<Vec<(TextRange, TokenText)>>,
}

impl EmbeddedValueReferences {
    pub fn new(
        value: Vec<Vec<(TextRange, TokenText)>>,
        type_only: Vec<Vec<(TextRange, TokenText)>>,
    ) -> Self {
        Self { value, type_only }
    }

    /// Check if an identifier is used as a value in any tracked non-source snippet.
    pub(crate) fn is_used_as_value(&self, identifier: &str) -> bool {
        Self::contains(&self.value, identifier)
    }

    /// Check if an identifier is referenced at all (value or type) in any
    /// tracked non-source snippet. The unused-* rules use this: a type-only
    /// usage in a template (`icon: IconType`) still counts as a use.
    pub(crate) fn is_used(&self, identifier: &str) -> bool {
        Self::contains(&self.value, identifier) || Self::contains(&self.type_only, identifier)
    }

    fn contains(refs: &[Vec<(TextRange, TokenText)>], identifier: &str) -> bool {
        refs.iter()
            .any(|refs| refs.iter().any(|(_, token)| token.text() == identifier))
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
