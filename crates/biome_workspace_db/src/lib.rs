//! This is the database used inside the biome Workspace, mainly the `biome_service` crate.
//!

#[cfg(feature = "html_embeds")]
pub mod embedded;

use biome_db::{ParsedSnippet, ParsedSource};
use biome_languages::DocumentFileSource;
use biome_languages::LanguageDb;
#[cfg(feature = "module_graph")]
use biome_module_graph::{
    InferredLocalTypeId, InferredModuleKey, ModuleDb, ModuleGraphGeneration, ModuleInfo,
    ModuleInfoKind, TypeDb,
};
use biome_parser::AnyParse;
use biome_rowan::SendNode;
#[cfg(feature = "module_graph")]
use biome_rowan::Text;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
#[cfg(feature = "module_graph")]
use salsa::plumbing::{AsId, FromId};
use salsa::{Setter, Storage};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParsedSourceUpdateMode {
    /// Mint a new Salsa input and replace the path mapping. Existing handles no
    /// longer identify the file stored at that path.
    Replace,
    /// Mutate an existing Salsa input in place. The handle remains stable, but
    /// setters wait until no live database clone is reading the old revision.
    Setters,
}

/// The database used by the `biome_service` crate.
///
/// All data stored in the database must be clonable and must support [Sync] and [Send].
#[salsa::db]
#[derive(Clone)]
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

impl Default for WorkspaceDb {
    fn default() -> Self {
        let db = Self {
            files: Arc::default(),
            #[cfg(feature = "module_graph")]
            modules: Arc::default(),
            file_sources: Arc::default(),
            storage: Storage::default(),
        };
        #[cfg(feature = "module_graph")]
        ModuleGraphGeneration::new(&db, 0);
        db
    }
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
}

impl WorkspaceDb {
    #[cfg(feature = "module_graph")]
    fn mutate_modules(&mut self, mutate: impl FnOnce(&HashMap<Utf8PathBuf, ModuleInfo>)) {
        let generation = ModuleGraphGeneration::get(self);
        let next = generation.value(self).wrapping_add(1);
        let modules = self.modules.clone();

        // Begin the Salsa write before mutating the shared registry, then publish
        // the new generation only after the registry mutation is complete.
        let pending_setter = generation.set_value(self);
        mutate(&modules);
        pending_setter.to(next);
    }

    /// Returns handles to the collections that this database shares with all
    /// its clones.
    pub fn data(&self) -> WorkspaceDbData {
        WorkspaceDbData {
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

    /// Replaces the path mapping with the provided input handle.
    pub fn insert_file(&mut self, path: &Utf8Path, file: ParsedSource) {
        self.files.pin().insert(path.to_path_buf(), file);
    }

    /// Replaces the path mapping with the provided input handle.
    pub fn update_file(&mut self, path: &Utf8Path, file: ParsedSource) {
        self.files.pin().update(path.to_path_buf(), |_| file);
    }

    /// Updates a file according to `mode`, preserving the handle only for
    /// [`ParsedSourceUpdateMode::Setters`].
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

    /// Mints a new input and replaces the path mapping (`Replace` semantics).
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

    /// Mutates an existing input in place, or mints one if absent (`Setters`
    /// semantics).
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

    /// Applies the selected replacement or setter semantics and returns the
    /// handle now mapped to `path`.
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
    pub fn insert_module(&mut self, path: Utf8PathBuf, module: ModuleInfo) {
        self.mutate_modules(|modules| {
            modules.pin().insert(path, module);
        });
    }

    #[cfg(feature = "module_graph")]
    pub fn update_or_insert_module(
        &mut self,
        path: Utf8PathBuf,
        kind: ModuleInfoKind,
    ) -> ModuleInfo {
        let existing_module = { self.modules.pin().get(&path).copied() };

        if let Some(existing_module) = existing_module {
            existing_module.set_kind(self).to(kind);
            existing_module
        } else {
            let module = ModuleInfo::new(self, path.clone(), kind);
            self.insert_module(path, module);
            module
        }
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
    pub fn remove_module(&mut self, path: &Utf8Path) {
        if self.modules.pin().contains_key(path) {
            self.mutate_modules(|modules| {
                let modules = modules.pin();
                let removed = modules.remove(path);
                debug_assert!(removed.is_some());
            });
        }
    }

    pub fn unload_path(&mut self, path: &Utf8Path) {
        #[cfg(feature = "module_graph")]
        {
            let to_remove = self
                .modules
                .pin()
                .keys()
                .filter(|module_path| module_path.starts_with(path))
                .cloned()
                .collect::<Vec<_>>();
            if !to_remove.is_empty() {
                self.mutate_modules(|modules| {
                    let modules = modules.pin();
                    for module_path in &to_remove {
                        modules.remove(module_path);
                    }
                });
            }
        }
        #[cfg(not(feature = "module_graph"))]
        let _ = path;
    }
}

/// Shared state for creating operation-local [WorkspaceDb] forks.
///
/// This type contains no Salsa local state. Each call to [Self::fork] creates a
/// database value with fresh Salsa local state and shared workspace data.
#[derive(Clone)]
pub struct SharedWorkspaceDb {
    files: Arc<HashMap<Utf8PathBuf, ParsedSource>>,
    #[cfg(feature = "module_graph")]
    modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,
    file_sources: Arc<boxcar::Vec<DocumentFileSource>>,
    storage: salsa::StorageHandle<WorkspaceDb>,
}

impl Default for SharedWorkspaceDb {
    fn default() -> Self {
        let WorkspaceDb {
            files,
            #[cfg(feature = "module_graph")]
            modules,
            file_sources,
            storage,
        } = WorkspaceDb::default();
        Self {
            files,
            #[cfg(feature = "module_graph")]
            modules,
            file_sources,
            storage: storage.into_zalsa_handle(),
        }
    }
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
impl TypeDb for WorkspaceDb {
    fn local_type_name(
        &self,
        module_key: InferredModuleKey,
        type_id: InferredLocalTypeId,
    ) -> Option<Text> {
        let module = ModuleInfo::from_id(module_key.as_id());
        let current = self.module_for_path(module.path(self))?;
        if InferredModuleKey::new(current.as_id()) != module_key {
            return None;
        }

        let ModuleInfoKind::Js(info) = current.kind(self) else {
            return None;
        };
        info.local_type_name(type_id)
    }
}

#[cfg(feature = "module_graph")]
#[salsa::db]
impl ModuleDb for WorkspaceDb {
    fn module_graph_generation(&self) -> u64 {
        ModuleGraphGeneration::get(self).value(self)
    }

    fn module_for_path(&self, path: &Utf8Path) -> Option<ModuleInfo> {
        let _ = self.module_graph_generation();
        self.get_module(path)
    }

    fn for_each_module(&self, f: &mut dyn FnMut(&Utf8Path, &ModuleInfoKind)) {
        let _ = self.module_graph_generation();
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
    #[cfg(feature = "module_graph")]
    use biome_fs::{BiomePath, MemoryFileSystem};
    #[cfg(feature = "module_graph")]
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_js_parser::{JsParserOptions, parse};
    use biome_languages::JsFileSource;
    #[cfg(feature = "module_graph")]
    use biome_module_graph::{ModuleDb, PathInfoCache, resolve_html_module};
    #[cfg(feature = "module_graph")]
    use biome_project_layout::ProjectLayout;
    use salsa::plumbing::AsId;
    use std::sync::Barrier;
    #[cfg(feature = "module_graph")]
    use std::sync::atomic::{AtomicBool, Ordering};
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

    #[cfg(feature = "module_graph")]
    fn module_transaction_db(barrier: Arc<Barrier>, armed: Arc<AtomicBool>) -> WorkspaceDb {
        let storage = Storage::new(Some(Box::new(move |event| {
            if armed.load(Ordering::Acquire)
                && matches!(event.kind, salsa::EventKind::DidSetCancellationFlag)
            {
                barrier.wait();
            }
        })));
        let db = WorkspaceDb {
            files: Arc::default(),
            modules: Arc::default(),
            file_sources: Arc::default(),
            storage,
        };
        ModuleGraphGeneration::new(&db, 0);
        db
    }

    #[cfg(feature = "module_graph")]
    fn test_module(db: &WorkspaceDb, path: &str) -> ModuleInfo {
        let path = BiomePath::new(path);
        let fs = MemoryFileSystem::default();
        let root = parse_html("", HtmlParserOptions::default()).tree();
        let (module, _, _) = resolve_html_module(
            root,
            &[],
            &path,
            &fs,
            &ProjectLayout::default(),
            &PathInfoCache::default(),
        );
        ModuleInfo::new(
            db,
            path.as_path().to_path_buf(),
            ModuleInfoKind::Html(module),
        )
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

    #[cfg(feature = "module_graph")]
    #[test]
    fn module_insertion_publishes_after_generation_invalidation() {
        let barrier = Arc::new(Barrier::new(2));
        let armed = Arc::new(AtomicBool::new(false));
        let mut db = module_transaction_db(barrier.clone(), armed.clone());
        let path = Utf8PathBuf::from("inserted.html");
        let module = test_module(&db, path.as_str());
        let old_generation = db.module_graph_generation();
        let reader_db = db.clone();
        armed.store(true, Ordering::Release);

        let db = std::thread::scope(|scope| {
            let writer_path = path.clone();
            let writer = scope.spawn(move || {
                db.insert_module(writer_path, module);
                db
            });

            barrier.wait();
            assert_eq!(reader_db.module_graph_generation(), old_generation);
            assert!(reader_db.module_for_path(&path).is_none());
            drop(reader_db);

            writer.join().unwrap()
        });

        assert_eq!(db.module_graph_generation(), old_generation.wrapping_add(1));
        assert_eq!(
            db.module_for_path(&path).map(|module| module.as_id()),
            Some(module.as_id())
        );
    }

    #[cfg(feature = "module_graph")]
    #[test]
    fn module_removal_publishes_after_generation_invalidation() {
        let barrier = Arc::new(Barrier::new(2));
        let armed = Arc::new(AtomicBool::new(false));
        let mut db = module_transaction_db(barrier.clone(), armed.clone());
        let path = Utf8PathBuf::from("removed.html");
        let module = test_module(&db, path.as_str());
        db.insert_module(path.clone(), module);
        let old_generation = db.module_graph_generation();
        let reader_db = db.clone();
        armed.store(true, Ordering::Release);

        let db = std::thread::scope(|scope| {
            let writer_path = path.clone();
            let writer = scope.spawn(move || {
                db.remove_module(&writer_path);
                db
            });

            barrier.wait();
            assert_eq!(reader_db.module_graph_generation(), old_generation);
            assert_eq!(
                reader_db
                    .module_for_path(&path)
                    .map(|module| module.as_id()),
                Some(module.as_id())
            );
            drop(reader_db);

            writer.join().unwrap()
        });

        assert_eq!(db.module_graph_generation(), old_generation.wrapping_add(1));
        assert!(db.module_for_path(&path).is_none());
    }

    #[cfg(feature = "module_graph")]
    #[test]
    fn module_unload_publishes_after_generation_invalidation() {
        let barrier = Arc::new(Barrier::new(2));
        let armed = Arc::new(AtomicBool::new(false));
        let mut db = module_transaction_db(barrier.clone(), armed.clone());
        let root = Utf8PathBuf::from("root/a.html");
        let nested = Utf8PathBuf::from("root/nested/b.html");
        let outside = Utf8PathBuf::from("other/c.html");
        let root_module = test_module(&db, root.as_str());
        let nested_module = test_module(&db, nested.as_str());
        let outside_module = test_module(&db, outside.as_str());
        db.insert_module(root.clone(), root_module);
        db.insert_module(nested.clone(), nested_module);
        db.insert_module(outside.clone(), outside_module);
        let old_generation = db.module_graph_generation();
        let reader_db = db.clone();
        armed.store(true, Ordering::Release);

        let db = std::thread::scope(|scope| {
            let writer = scope.spawn(move || {
                db.unload_path(Utf8Path::new("root"));
                db
            });

            barrier.wait();
            assert_eq!(reader_db.module_graph_generation(), old_generation);
            assert_eq!(
                reader_db
                    .module_for_path(&root)
                    .map(|module| module.as_id()),
                Some(root_module.as_id())
            );
            assert_eq!(
                reader_db
                    .module_for_path(&nested)
                    .map(|module| module.as_id()),
                Some(nested_module.as_id())
            );
            assert_eq!(
                reader_db
                    .module_for_path(&outside)
                    .map(|module| module.as_id()),
                Some(outside_module.as_id())
            );
            drop(reader_db);

            writer.join().unwrap()
        });

        assert_eq!(db.module_graph_generation(), old_generation.wrapping_add(1));
        assert!(db.module_for_path(&root).is_none());
        assert!(db.module_for_path(&nested).is_none());
        assert_eq!(
            db.module_for_path(&outside).map(|module| module.as_id()),
            Some(outside_module.as_id())
        );
    }
}
