use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_languages::LanguageDb;
use biome_rowan::TokenText;
use biome_workspace_db::embedded::EmbeddedSourceData;
use biome_workspace_db::embedded::bindings::{
    InternedBindingText, InternedBindingTokenText, get_binding_by_name, get_binding_by_text,
};
use biome_workspace_db::embedded::references::{
    InternedReference, is_reference_used, is_svelte_store_reference_used, is_type_reference_used,
    is_value_reference_used,
};
use camino::Utf8PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct EmbeddedService {
    db: Option<Rc<dyn LanguageDb>>,
    path: Utf8PathBuf,
    source: Option<EmbeddedSourceData>,
}

impl EmbeddedService {
    pub(crate) fn new(
        db: Option<Rc<dyn LanguageDb>>,
        path: Utf8PathBuf,
        source: Option<EmbeddedSourceData>,
    ) -> Self {
        Self { db, path, source }
    }

    pub(crate) fn contains_binding(&self, binding: TokenText) -> bool {
        let (Some(db), Some(source)) = (&self.db, &self.source) else {
            return false;
        };
        get_binding_by_name(
            db.as_ref(),
            source.intern(db.as_ref()),
            InternedBindingTokenText::new(db.as_ref(), self.path.clone(), binding),
        )
        .is_some()
    }

    pub(crate) fn contains_binding_text(&self, binding: &str) -> bool {
        let (Some(db), Some(source)) = (&self.db, &self.source) else {
            return false;
        };
        get_binding_by_text(
            db.as_ref(),
            source.intern(db.as_ref()),
            InternedBindingText::new(db.as_ref(), self.path.clone(), binding.to_string()),
        )
        .is_some()
    }

    pub(crate) fn is_used_as_value(&self, identifier: TokenText) -> bool {
        let (Some(db), Some(source)) = (&self.db, &self.source) else {
            return false;
        };
        is_value_reference_used(
            db.as_ref(),
            source.intern(db.as_ref()),
            InternedReference::new(db.as_ref(), self.path.clone(), identifier),
        )
    }

    pub(crate) fn is_used_as_type(&self, identifier: TokenText) -> bool {
        let (Some(db), Some(source)) = (&self.db, &self.source) else {
            return false;
        };
        is_type_reference_used(
            db.as_ref(),
            source.intern(db.as_ref()),
            InternedReference::new(db.as_ref(), self.path.clone(), identifier),
        )
    }

    pub(crate) fn is_used(&self, identifier: TokenText) -> bool {
        let (Some(db), Some(source)) = (&self.db, &self.source) else {
            return false;
        };
        is_reference_used(
            db.as_ref(),
            source.intern(db.as_ref()),
            InternedReference::new(db.as_ref(), self.path.clone(), identifier),
        )
    }

    /// Svelte stores are a special case. The `$` prefix is used to "dereference" the store and get its value.
    ///
    /// See also: https://svelte.dev/docs/svelte/stores
    pub(crate) fn is_svelte_store_used(&self, identifier: TokenText) -> bool {
        let (Some(db), Some(source)) = (&self.db, &self.source) else {
            return false;
        };
        is_svelte_store_reference_used(
            db.as_ref(),
            source.intern(db.as_ref()),
            InternedReference::new(db.as_ref(), self.path.clone(), identifier),
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
