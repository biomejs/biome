use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_embeds::EmbeddedData;
use biome_embeds::bindings::{
    InternedBindingText, InternedBindingTokenText, get_binding_by_name, get_binding_by_text,
};
use biome_embeds::references::{
    InternedReference, is_reference_used, is_svelte_store_reference_used, is_type_reference_used,
    is_value_reference_used,
};
use biome_languages::LanguageDb;
use biome_rowan::TokenText;
use camino::Utf8PathBuf;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct EmbeddedService {
    source: EmbeddedSource,
}

#[derive(Clone)]
enum EmbeddedSource {
    Workspace {
        db: Rc<dyn LanguageDb>,
        path: Utf8PathBuf,
    },
    Transient(Arc<EmbeddedData>),
}

impl EmbeddedService {
    pub(crate) fn new(db: Rc<dyn LanguageDb>, path: Utf8PathBuf) -> Self {
        Self {
            source: EmbeddedSource::Workspace { db, path },
        }
    }

    pub(crate) fn from_data(data: Arc<EmbeddedData>) -> Self {
        Self {
            source: EmbeddedSource::Transient(data),
        }
    }

    pub(crate) fn contains_binding(&self, binding: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => get_binding_by_name(
                db.as_ref(),
                InternedBindingTokenText::new(db.as_ref(), path.clone(), binding),
            )
            .is_some(),
            EmbeddedSource::Transient(data) => data.contains_binding(binding.text()),
        }
    }

    pub(crate) fn contains_binding_text(&self, binding: &str) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => get_binding_by_text(
                db.as_ref(),
                InternedBindingText::new(db.as_ref(), path.clone(), binding.to_string()),
            )
            .is_some(),
            EmbeddedSource::Transient(data) => data.contains_binding(binding),
        }
    }

    pub(crate) fn is_used_as_value(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => is_value_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), path.clone(), identifier),
            ),
            EmbeddedSource::Transient(data) => data.is_used_as_value(identifier.text()),
        }
    }

    pub(crate) fn is_used_as_type(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => is_type_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), path.clone(), identifier),
            ),
            EmbeddedSource::Transient(data) => data.is_used_as_type(identifier.text()),
        }
    }

    pub(crate) fn is_used(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => is_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), path.clone(), identifier),
            ),
            EmbeddedSource::Transient(data) => data.is_used(identifier.text()),
        }
    }

    /// Svelte stores are a special case. The `$` prefix is used to "dereference" the store and get its value.
    ///
    /// See also: https://svelte.dev/docs/svelte/stores
    pub(crate) fn is_svelte_store_used(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => is_svelte_store_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), path.clone(), identifier),
            ),
            EmbeddedSource::Transient(data) => data.is_svelte_store_used(identifier.text()),
        }
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
