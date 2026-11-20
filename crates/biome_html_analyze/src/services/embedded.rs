use biome_embeds::{
    EmbeddedData,
    astro::{AstroData, astro_data},
    imports::static_imports,
};
use biome_languages::LanguageDb;
use biome_rowan::TextRange;
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

    pub(crate) fn astro(&self) -> Option<&AstroData> {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => astro_data(db.as_ref(), path),
            EmbeddedSource::Interned(data) => Some(data.astro()),
        }
    }

    pub(crate) fn static_imports(&self, range: TextRange) -> &[TextRange] {
        match &self.source {
            EmbeddedSource::Workspace { db, path } => static_imports(db.as_ref(), path, range),
            EmbeddedSource::Interned(data) => data.static_imports(range),
        }
    }
}

impl std::fmt::Debug for EmbeddedService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmbeddedService").finish_non_exhaustive()
    }
}
