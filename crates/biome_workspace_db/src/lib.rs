//! This is the database used inside the biome Workspace, mainly the `biome_service` crate.
//!

#[cfg(feature = "html_embeds")]
pub mod embedded;

use biome_db::{ParsedSnippet, ParsedSource};
use biome_languages::DocumentFileSource;
use biome_languages::LanguageDb;
#[cfg(feature = "module_graph")]
use biome_module_graph::{ModuleDb, ModuleInfo, ModuleInfoKind, TypeDb};
use biome_parser::AnyParse;
use biome_rowan::SendNode;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use salsa::{Setter, Storage};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParsedSourceUpdateMode {
    Replace,
    Setters,
}

/// The database used by the `biome_service` crate.
///
/// All data stored in the database must be clonable and must support [Sync] and [Send].
#[salsa::db]
#[derive(Clone, Default)]
pub struct WorkspaceDb {
    /// It maps a file path to its corresponding parsed version.
    files: Arc<HashMap<Utf8PathBuf, ParsedSource>>,
    /// It maps a file path to its module graph representation
    #[cfg(feature = "module_graph")]
    pub modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,
    /// It stores the file sources across projects.
    file_sources: Arc<boxcar::Vec<DocumentFileSource>>,
    // NOTE: this must stay last as per salsa restrictions.
    storage: Storage<Self>,
}

/// Handles to the collections that a [WorkspaceDb] shares with all its
/// clones.
///
/// The database and its clones all point to the same underlying collections,
/// so an update made through this type is immediately visible to all of them,
/// and no lock is needed.
///
/// This matters when the database is updated through salsa setters: a setter
/// can only run once every clone of the database has been dropped. A thread
/// that still holds a clone must be able to finish its work on its own,
/// without waiting for the lock that protects the database while the setter
/// runs. This type is what makes that possible.
#[derive(Clone)]
pub struct WorkspaceDbData {
    #[cfg(feature = "module_graph")]
    modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,
    file_sources: Arc<boxcar::Vec<DocumentFileSource>>,
}

impl WorkspaceDbData {
    /// Inserts a file source so that it can be retrieved by index later.
    ///
    /// Returns the index at which the file source can be retrieved using
    /// `get_source()`.
    pub fn insert_source(&self, document_file_source: DocumentFileSource) -> usize {
        self.file_sources
            .iter()
            .find(|(_, file_source)| **file_source == document_file_source)
            .map_or_else(
                || self.file_sources.push(document_file_source),
                |(index, _)| index,
            )
    }

    #[cfg(feature = "module_graph")]
    pub fn insert_module(&self, path: Utf8PathBuf, module: ModuleInfo) {
        self.modules.pin().insert(path, module);
    }

    #[cfg(feature = "module_graph")]
    pub fn remove_module(&self, path: &Utf8Path) {
        self.modules.pin().remove(path);
    }

    /// Removes all modules that start with the given path. That's usually used
    /// when removing a library or a folder from the project.
    pub fn unload_path(&self, path: &Utf8Path) {
        #[cfg(feature = "module_graph")]
        {
            let modules = self.modules.pin();
            let to_remove: Vec<Utf8PathBuf> = modules
                .keys()
                .filter(|p| p.starts_with(path))
                .cloned()
                .collect();
            for p in to_remove {
                modules.remove(&p);
            }
        }
        #[cfg(not(feature = "module_graph"))]
        let _ = path;
    }
}

impl WorkspaceDb {
    /// Returns handles to the collections that this database shares with all
    /// its clones.
    pub fn data(&self) -> WorkspaceDbData {
        WorkspaceDbData {
            #[cfg(feature = "module_graph")]
            modules: self.modules.clone(),
            file_sources: self.file_sources.clone(),
        }
    }

    /// Inserts a file source so that it can be retrieved by index later.
    ///
    /// Returns the index at which the file source can be retrieved using
    /// `get_source()`.
    pub fn insert_source(&mut self, document_file_source: DocumentFileSource) -> usize {
        self.data().insert_source(document_file_source)
    }

    pub fn insert_file(&mut self, path: &Utf8Path, file: ParsedSource) {
        self.files.pin().insert(path.to_path_buf(), file);
    }

    pub fn update_file(&mut self, path: &Utf8Path, file: ParsedSource) {
        self.files.pin().update(path.to_path_buf(), |_| file);
    }

    pub fn update_file_with_mode(
        &mut self,
        path: &Utf8Path,
        file: ParsedSource,
        mode: ParsedSourceUpdateMode,
    ) -> ParsedSource {
        let parsed = file.parsed(self).clone();
        let document_source_index = file.document_source_index(self);
        let snippets = file.snippets(self).clone();
        self.update_or_insert_file(path, parsed, document_source_index, snippets, mode)
    }

    pub fn replace_file(
        &mut self,
        path: &Utf8Path,
        parsed: AnyParse,
        document_source_index: usize,
        snippets: Vec<ParsedSnippet>,
    ) -> ParsedSource {
        let file = ParsedSource::new(
            self,
            path.to_path_buf(),
            parsed,
            document_source_index,
            snippets,
        );
        self.files.pin().insert(path.to_path_buf(), file);
        file
    }

    pub fn upsert_file(
        &mut self,
        path: &Utf8Path,
        parsed: AnyParse,
        document_source_index: usize,
        snippets: Vec<ParsedSnippet>,
    ) -> ParsedSource {
        self.update_or_insert_file(
            path,
            parsed,
            document_source_index,
            snippets,
            ParsedSourceUpdateMode::Setters,
        )
    }

    pub fn update_or_insert_file(
        &mut self,
        path: &Utf8Path,
        parsed: AnyParse,
        document_source_index: usize,
        snippets: Vec<ParsedSnippet>,
        mode: ParsedSourceUpdateMode,
    ) -> ParsedSource {
        if mode == ParsedSourceUpdateMode::Replace {
            return self.replace_file(path, parsed, document_source_index, snippets);
        }

        let existing_file = { self.files.pin().get(path).copied() };

        if let Some(existing_file) = existing_file {
            existing_file.set_parsed(self).to(parsed);
            existing_file
                .set_document_source_index(self)
                .to(document_source_index);
            existing_file.set_snippets(self).to(snippets);
            existing_file
        } else {
            self.replace_file(path, parsed, document_source_index, snippets)
        }
    }

    #[cfg(feature = "module_graph")]
    pub fn get_module(&self, path: &Utf8Path) -> Option<ModuleInfo> {
        self.modules.pin().get(path).copied()
    }

    pub fn get_file(&self, path: &Utf8Path) -> Option<ParsedSource> {
        self.files.pin().get(path).copied()
    }

    /// Removes all modules that start with the given path. That's usually used when removing a library or a
    /// folder from the project.
    #[cfg(feature = "module_graph")]
    pub fn unload_path_from_module(&self, path: &Utf8Path) {
        let modules = self.modules.pin();
        let to_remove: Vec<Utf8PathBuf> = modules
            .keys()
            .filter(|p| p.starts_with(path))
            .cloned()
            .collect();
        for p in to_remove {
            modules.remove(&p);
        }
    }

    /// Returns a [Rc] to itself, cast to [ModuleDb]. This is used to send the service
    /// to the analyzer.
    #[cfg(feature = "module_graph")]
    pub fn rc_module_db(&self) -> Rc<dyn ModuleDb> {
        Rc::new(self.clone())
    }

    /// Returns a [Rc] to itself, cast to [LanguageDb]. This is used to send the service
    /// to the analyzer.
    pub fn rc_language_db(&self) -> Rc<dyn LanguageDb> {
        Rc::new(self.clone())
    }

    #[cfg(feature = "module_graph")]
    pub fn insert_module(&self, path: Utf8PathBuf, module: ModuleInfo) {
        self.data().insert_module(path, module);
    }

    /// It updates the CST of an existing parsed source
    pub fn update_parsed_root(&mut self, path: &Utf8Path, new_root: SendNode) {
        self.update_parsed_root_with_mode(path, new_root, ParsedSourceUpdateMode::Setters);
    }

    /// It updates the CST of an existing parsed source
    pub fn update_parsed_root_with_mode(
        &mut self,
        path: &Utf8Path,
        new_root: SendNode,
        mode: ParsedSourceUpdateMode,
    ) {
        if let Some(parsed_source) = self.get_file(path) {
            let mut any_parse = parsed_source.parsed(self).clone();
            any_parse.set_new_root(new_root);
            match mode {
                ParsedSourceUpdateMode::Replace => {
                    self.replace_file(
                        path,
                        any_parse,
                        parsed_source.document_source_index(self),
                        parsed_source.snippets(self).clone(),
                    );
                }
                ParsedSourceUpdateMode::Setters => {
                    parsed_source.set_parsed(self).to(any_parse);
                }
            }
        }
    }

    #[cfg(feature = "module_graph")]
    pub fn remove_module(&self, path: &Utf8Path) {
        self.data().remove_module(path);
    }

    pub fn unload_path(&self, path: &Utf8Path) {
        self.data().unload_path(path);
    }
}

/// Shared state for creating operation-local [WorkspaceDb] forks.
///
/// This type contains no Salsa local state. Each call to [Self::fork] creates a
/// database value with fresh Salsa local state and shared workspace data.
#[derive(Clone, Default)]
pub struct SharedWorkspaceDb {
    files: Arc<HashMap<Utf8PathBuf, ParsedSource>>,
    #[cfg(feature = "module_graph")]
    modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,
    file_sources: Arc<boxcar::Vec<DocumentFileSource>>,
    storage: salsa::StorageHandle<WorkspaceDb>,
}

impl SharedWorkspaceDb {
    pub fn fork(&self) -> WorkspaceDb {
        WorkspaceDb {
            files: self.files.clone(),
            file_sources: self.file_sources.clone(),
            #[cfg(feature = "module_graph")]
            modules: self.modules.clone(),
            storage: self.storage.clone().into_storage(),
        }
    }
}

#[salsa::db]
impl salsa::Database for WorkspaceDb {}

#[salsa::db]
impl biome_db::Db for WorkspaceDb {
    fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<ParsedSource> {
        self.files.pin().get(path).copied()
    }
}

#[cfg(feature = "module_graph")]
#[salsa::db]
impl TypeDb for WorkspaceDb {}

#[cfg(feature = "module_graph")]
#[salsa::db]
impl ModuleDb for WorkspaceDb {
    fn module_for_path(&self, path: &Utf8Path) -> Option<ModuleInfo> {
        self.get_module(path)
    }

    fn for_each_module(&self, f: &mut dyn FnMut(&Utf8Path, &ModuleInfoKind)) {
        let modules = self.modules.pin();
        let iter = modules.iter();
        for (path, module_info) in iter {
            let kind = module_info.kind(self);
            f(path.as_path(), &kind);
        }
    }
}

#[salsa::db]
impl LanguageDb for WorkspaceDb {
    /// Returns a previously inserted file source by index.
    ///
    /// File sources can be inserted using `insert_source()`.
    fn source_from_index(&self, index: usize) -> Option<DocumentFileSource> {
        self.file_sources.get(index).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_db::Db;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_languages::JsFileSource;
    use salsa::plumbing::AsId;
    use std::sync::Barrier;
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    static SETTER_READER_STARTED: Barrier = Barrier::new(2);

    fn parse_js(source: &str) -> AnyParse {
        parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        )
        .into()
    }

    #[salsa::tracked]
    fn blocking_document_source_index(db: &dyn Db, file: ParsedSource) -> usize {
        SETTER_READER_STARTED.wait();

        let timeout = Instant::now() + Duration::from_secs(2);
        while Instant::now() < timeout {
            db.unwind_if_revision_cancelled();
            std::thread::sleep(Duration::from_millis(1));
        }

        file.document_source_index(db)
    }

    #[test]
    fn upsert_file_updates_existing_input() {
        let mut db = WorkspaceDb::default();
        let path = Utf8Path::new("test.js");

        let file = db.upsert_file(path, parse_js("let a = 1;"), 0, vec![]);
        let updated_file = db.upsert_file(path, parse_js("let b = 2;"), 0, vec![]);

        assert_eq!(file.as_id(), updated_file.as_id());
        assert_eq!(db.get_file(path).unwrap().as_id(), file.as_id());
    }

    #[test]
    fn replace_file_replaces_existing_input() {
        let mut db = WorkspaceDb::default();
        let path = Utf8Path::new("test.js");

        let file = db.replace_file(path, parse_js("let a = 1;"), 0, vec![]);
        let updated_file = db.replace_file(path, parse_js("let b = 2;"), 0, vec![]);

        assert_ne!(file.as_id(), updated_file.as_id());
        assert_eq!(db.get_file(path).unwrap().as_id(), updated_file.as_id());
    }

    #[test]
    fn setter_update_cancels_running_query_without_deadlock() {
        let mut db = WorkspaceDb::default();
        let path = Utf8PathBuf::from("test.js");
        let file = db.upsert_file(&path, parse_js("let a = 1;"), 0, vec![]);
        let (writer_finished_tx, writer_finished_rx) = mpsc::channel();

        std::thread::scope(|scope| {
            let reader_db = db.clone();
            let reader = scope.spawn(move || {
                salsa::Cancelled::catch(|| blocking_document_source_index(&reader_db, file))
            });

            let writer_path = path.clone();
            scope.spawn(move || {
                SETTER_READER_STARTED.wait();
                db.upsert_file(&writer_path, parse_js("let b = 2;"), 0, vec![]);
                writer_finished_tx.send(()).unwrap();
            });

            assert!(
                writer_finished_rx
                    .recv_timeout(Duration::from_secs(3))
                    .is_ok(),
                "setter update deadlocked while waiting for a running query"
            );

            let result = reader.join().unwrap();
            assert!(
                matches!(result, Err(salsa::Cancelled::PendingWrite)),
                "{result:?}"
            );
        });
    }
}
