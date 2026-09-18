use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_languages::LanguageDb;
use biome_rowan::TokenText;
use biome_workspace_db::embedded::bindings::{
    InternedBindingText, InternedBindingTokenText, get_binding_by_name, get_binding_by_text,
};
use biome_workspace_db::embedded::references::{
    InternedReference, is_reference_used, is_type_reference_used, is_value_reference_used,
};
use camino::Utf8PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct EmbeddedService {
    db: Rc<dyn LanguageDb>,
    path: Utf8PathBuf,
}

impl EmbeddedService {
    pub(crate) fn new(db: Rc<dyn LanguageDb>, path: Utf8PathBuf) -> Self {
        Self { db, path }
    }

    pub(crate) fn contains_binding(&self, binding: TokenText) -> bool {
        get_binding_by_name(
            self.db.as_ref(),
            InternedBindingTokenText::new(self.db.as_ref(), self.path.clone(), binding),
        )
        .is_some()
    }

    pub(crate) fn contains_binding_text(&self, binding: &str) -> bool {
        get_binding_by_text(
            self.db.as_ref(),
            InternedBindingText::new(self.db.as_ref(), self.path.clone(), binding.to_string()),
        )
        .is_some()
    }

    pub(crate) fn is_used_as_value(&self, identifier: TokenText) -> bool {
        is_value_reference_used(
            self.db.as_ref(),
            InternedReference::new(self.db.as_ref(), self.path.clone(), identifier),
        )
    }

    pub(crate) fn is_used_as_type(&self, identifier: TokenText) -> bool {
        is_type_reference_used(
            self.db.as_ref(),
            InternedReference::new(self.db.as_ref(), self.path.clone(), identifier),
        )
    }

    pub(crate) fn is_used(&self, identifier: TokenText) -> bool {
        is_reference_used(
            self.db.as_ref(),
            InternedReference::new(self.db.as_ref(), self.path.clone(), identifier),
        )
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
