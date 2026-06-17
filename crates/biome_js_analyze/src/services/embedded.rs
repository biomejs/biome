use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_rowan::TokenText;
use biome_workspace_db::embedded::EmbeddedDb;
use biome_workspace_db::embedded::bindings::{InternedBinding, get_binding_by_name};
use biome_workspace_db::embedded::references::{InternedReference, is_value_reference_used};
use camino::Utf8PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct EmbeddedService {
    db: Option<Rc<dyn EmbeddedDb>>,
    path: Utf8PathBuf,
}

impl EmbeddedService {
    pub(crate) fn new(db: Option<Rc<dyn EmbeddedDb>>, path: Utf8PathBuf) -> Self {
        Self { db, path }
    }

    pub(crate) fn contains_binding(&self, binding: TokenText) -> bool {
        if let Some(db) = &self.db {
            return get_binding_by_name(
                db.as_ref(),
                InternedBinding::new(db.as_ref(), &self.path.clone(), binding),
            )
            .is_some();
        }

        false
    }

    pub(crate) fn contains_binding_text(&self, binding: &str) -> bool {
        if let Some(db) = &self.db {
            return db.binding_by_name(&self.path, binding).is_some();
        }

        false
    }

    pub(crate) fn is_used_as_value(&self, identifier: TokenText) -> bool {
        if let Some(db) = &self.db {
            return is_value_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), &self.path.clone(), identifier),
            );
        }

        false
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
