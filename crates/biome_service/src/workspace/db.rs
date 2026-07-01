use crate::module_graph::PathInfoCache;
use biome_db::{ParsedSnippet, ParsedSource};
use biome_languages::DocumentFileSource;
use biome_parser::AnyParse;
use biome_rowan::SendNode;
use biome_workspace_db::{ParsedSourceUpdateMode, SharedWorkspaceDb, WorkspaceDb};
use camino::{Utf8Path, Utf8PathBuf};
use parking_lot::Mutex;

use crate::embed::EmbedContent;

/// Represents the state of the database in the workspace.
pub struct DbState {
    storage: DbStorage,
    pub(crate) path_info_cache: PathInfoCache,
}

enum DbStorage {
    Shared(SharedWorkspaceDb),
    Owned(Mutex<WorkspaceDb>),
}

impl Default for DbState {
    fn default() -> Self {
        Self {
            storage: DbStorage::Shared(SharedWorkspaceDb::default()),
            path_info_cache: PathInfoCache::default(),
        }
    }
}

impl DbState {
    pub fn lsp() -> Self {
        Self {
            storage: DbStorage::Owned(Mutex::new(WorkspaceDb::default())),
            path_info_cache: PathInfoCache::default(),
        }
    }

    pub(crate) fn fork(&self) -> WorkspaceDb {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork(),
            DbStorage::Owned(db) => db.lock().clone(),
        }
    }

    pub(crate) fn insert_source(&self, document_file_source: DocumentFileSource) -> usize {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().insert_source(document_file_source),
            DbStorage::Owned(db) => db.lock().insert_source(document_file_source),
        }
    }

    pub(crate) fn update_parsed_root(&self, path: &Utf8Path, new_root: SendNode) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().update_parsed_root_with_mode(
                path,
                new_root,
                ParsedSourceUpdateMode::Replace,
            ),
            DbStorage::Owned(db) => db.lock().update_parsed_root_with_mode(
                path,
                new_root,
                ParsedSourceUpdateMode::Setters,
            ),
        }
    }

    pub(crate) fn update_parsed_file(
        &self,
        path: &Utf8Path,
        parsed: AnyParse,
        language_index: usize,
        snippets: Vec<(AnyParse, EmbedContent, usize)>,
    ) -> ParsedSource {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                let mut db = shared_db.fork();
                let parsed_snippets = create_parsed_snippets(&db, snippets);
                db.update_or_insert_file(
                    path,
                    parsed,
                    language_index,
                    parsed_snippets,
                    ParsedSourceUpdateMode::Replace,
                )
            }
            DbStorage::Owned(db) => {
                let mut db = db.lock();
                let parsed_snippets = create_parsed_snippets(&db, snippets);
                db.update_or_insert_file(
                    path,
                    parsed,
                    language_index,
                    parsed_snippets,
                    ParsedSourceUpdateMode::Setters,
                )
            }
        }
    }

    pub(crate) fn unload_path(&self, path: &Utf8Path) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().unload_path(path),
            DbStorage::Owned(db) => db.lock().unload_path(path),
        }
    }

    #[cfg(feature = "module_graph")]
    pub(crate) fn insert_module(&self, path: Utf8PathBuf, module: biome_module_graph::ModuleInfo) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().insert_module(path, module),
            DbStorage::Owned(db) => db.lock().insert_module(path, module),
        }
    }

    #[cfg(feature = "module_graph")]
    pub(crate) fn remove_module(&self, path: &Utf8Path) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().remove_module(path),
            DbStorage::Owned(db) => db.lock().remove_module(path),
        }
    }
}

fn create_parsed_snippets(
    db: &WorkspaceDb,
    snippets: Vec<(AnyParse, EmbedContent, usize)>,
) -> Vec<ParsedSnippet> {
    snippets
        .into_iter()
        .map(|(parse, content, index)| {
            ParsedSnippet::new(
                db,
                parse,
                content.element_range,
                content.content_range,
                content.content_offset,
                index,
            )
        })
        .collect()
}
