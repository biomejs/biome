use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_embeds::EmbeddedData;
use biome_embeds::bindings::{
    InternedBindingText, InternedBindingTokenText, get_binding_by_name, get_binding_by_text,
    get_bindings_by_name,
};
use biome_embeds::references::{
    InternedReference, is_reference_used, is_svelte_store_reference_used, is_type_reference_used,
    is_value_reference_used, is_vue_directive_reference_used,
};
use biome_languages::LanguageDb;
use biome_rowan::{TextSize, TokenText};
use camino::Utf8PathBuf;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct EmbeddedService {
    source: EmbeddedSource,
    /// Where the analyzed snippet starts in its host document, if any.
    embedded_offset: Option<TextSize>,
}

#[derive(Clone)]
enum EmbeddedSource {
    Workspace {
        db: Rc<dyn LanguageDb>,
        path: Utf8PathBuf,
    },
    Interned(Arc<EmbeddedData>),
}

impl EmbeddedService {
    pub(crate) fn new(
        db: Rc<dyn LanguageDb>,
        path: Utf8PathBuf,
        embedded_offset: Option<TextSize>,
    ) -> Self {
        Self {
            source: EmbeddedSource::Workspace { db, path },
            embedded_offset,
        }
    }

    pub(crate) fn from_data(data: Arc<EmbeddedData>, embedded_offset: Option<TextSize>) -> Self {
        Self {
            source: EmbeddedSource::Interned(data),
            embedded_offset,
        }
    }

    pub(crate) fn contains_binding(&self, binding: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => get_binding_by_name(
                db.as_ref(),
                InternedBindingTokenText::new(db.as_ref(), path.clone(), binding),
            )
            .is_some(),
            EmbeddedSource::Interned(data) => data.contains_binding(binding.text()),
        }
    }

    /// Returns whether a binding named `binding` is in scope for a reference at
    /// `offset`, which is relative to the start of the analyzed snippet.
    ///
    /// Scoped bindings (such as Vue slot props) are only visible inside the
    /// element that introduces them, so they need the reference position in the
    /// host document to be resolved.
    pub(crate) fn contains_binding_visible_at(&self, binding: TokenText, offset: TextSize) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => {
                let bindings = get_bindings_by_name(
                    db.as_ref(),
                    InternedBindingTokenText::new(db.as_ref(), path.clone(), binding),
                );
                let Some(embedded_offset) = self.embedded_offset else {
                    return !bindings.is_empty();
                };
                let offset = embedded_offset + offset;
                bindings.iter().any(|binding| binding.is_visible_at(offset))
            }
            EmbeddedSource::Interned(data) => match self.embedded_offset {
                Some(embedded_offset) => {
                    data.contains_binding_visible_at(binding.text(), embedded_offset + offset)
                }
                None => data.contains_binding(binding.text()),
            },
        }
    }

    pub(crate) fn contains_binding_text(&self, binding: &str) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => get_binding_by_text(
                db.as_ref(),
                InternedBindingText::new(db.as_ref(), path.clone(), binding.to_string()),
            )
            .is_some(),
            EmbeddedSource::Interned(data) => data.contains_binding(binding),
        }
    }

    pub(crate) fn is_used_as_value(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => is_value_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), path.clone(), identifier),
            ),
            EmbeddedSource::Interned(data) => data.is_used_as_value(identifier.text()),
        }
    }

    pub(crate) fn is_used_as_type(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => is_type_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), path.clone(), identifier),
            ),
            EmbeddedSource::Interned(data) => data.is_used_as_type(identifier.text()),
        }
    }

    pub(crate) fn is_used(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => is_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), path.clone(), identifier),
            ),
            EmbeddedSource::Interned(data) => data.is_used(identifier.text()),
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
            EmbeddedSource::Interned(data) => data.is_svelte_store_used(identifier.text()),
        }
    }

    /// Vue custom directives are a special case. The template spells them in
    /// kebab-case (e.g. `v-highlight`), while the JS binding they refer to is
    /// spelled in camelCase (e.g. `vHighlight`).
    ///
    /// See also: https://vuejs.org/guide/reusability/custom-directives.html
    pub(crate) fn is_vue_directive_used(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => is_vue_directive_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), path.clone(), identifier),
            ),
            EmbeddedSource::Interned(data) => data.is_vue_directive_used(identifier.text()),
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
