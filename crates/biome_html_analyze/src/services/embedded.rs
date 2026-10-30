use biome_db::Source;
use biome_embeds::{
    EmbeddedData, VueDirectiveResolution,
    vue_directives::{InternedVueDirective, resolve_vue_directive},
};
use biome_languages::LanguageDb;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct EmbeddedService {
    source: EmbeddedSource,
}

#[derive(Clone)]
enum EmbeddedSource {
    Workspace {
        db: Rc<dyn LanguageDb>,
        /// The analyzed document.
        document: Source,
    },
    Interned(Arc<EmbeddedData>),
}

impl EmbeddedService {
    pub(crate) fn new(db: Rc<dyn LanguageDb>, document: Source) -> Self {
        Self {
            source: EmbeddedSource::Workspace { db, document },
        }
    }

    pub(crate) fn from_data(data: Arc<EmbeddedData>) -> Self {
        Self {
            source: EmbeddedSource::Interned(data),
        }
    }

    pub(crate) fn resolve_vue_directive(&self, name: &str) -> VueDirectiveResolution {
        match &self.source {
            EmbeddedSource::Workspace { db, document } => resolve_vue_directive(
                db.as_ref(),
                InternedVueDirective::new(db.as_ref(), *document, name.to_string()),
            ),
            EmbeddedSource::Interned(data) => data.resolve_vue_directive(name),
        }
    }
}

impl std::fmt::Debug for EmbeddedService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmbeddedService").finish_non_exhaustive()
    }
}
