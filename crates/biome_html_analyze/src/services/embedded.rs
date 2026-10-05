use biome_embeds::{
    EmbeddedData, VueDirectiveResolution,
    vue_directives::{InternedVueDirective, resolve_vue_directive},
};
use biome_languages::LanguageDb;
use camino::Utf8PathBuf;
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
        path: Utf8PathBuf,
    },
    Interned(Arc<EmbeddedData>),
}

impl EmbeddedService {
    pub(crate) fn new(db: Rc<dyn LanguageDb>, path: Utf8PathBuf) -> Self {
        Self {
            source: EmbeddedSource::Workspace { db, path },
        }
    }

    pub(crate) fn from_data(data: Arc<EmbeddedData>) -> Self {
        Self {
            source: EmbeddedSource::Interned(data),
        }
    }

    pub(crate) fn resolve_vue_directive(&self, name: &str) -> VueDirectiveResolution {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => resolve_vue_directive(
                db.as_ref(),
                InternedVueDirective::new(db.as_ref(), path.clone(), name.to_string()),
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
