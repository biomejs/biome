pub mod embedded;

use crate::embedded::{EmbeddedBinding, EmbeddedDb, EmbeddedValueReference};
use biome_css_semantic::db::CssSemanticDb;
use biome_db::{ParsedSnippet, ParsedSource};
use biome_js_semantic::JsSemanticDb;
use biome_languages::DocumentFileSource;
use biome_languages::LanguageDb;
use biome_module_graph::{ModuleDb, ModuleInfo, ModuleInfoKind};
use biome_rowan::SendNode;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use salsa::Storage;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FileTooLarge {
    pub size: usize,
    pub limit: usize,
}

#[salsa::db]
#[derive(Clone, Default)]
pub struct WorkspaceDb {
    files: Arc<HashMap<Utf8PathBuf, ParsedSource>>,
    modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,
    file_sources: Arc<boxcar::Vec<DocumentFileSource>>,
    bindings: Arc<Vec<Vec<EmbeddedBinding>>>,
    references: Arc<Vec<Vec<EmbeddedValueReference>>>,

    // LAST
    storage: Storage<Self>,
}

impl WorkspaceDb {
    /// Inserts a new snippet. `range` should be the `content_range`
    pub fn insert_snippets(&mut self, path: &Utf8Path, snippets: Vec<ParsedSnippet>) {
        let file = self.get_file(path);
        if let Some(file) = file {
            let snippets = salsa::Setter::to(file.set_snippets(self), snippets);
            let new_file = ParsedSource::new(
                self,
                file.path(self).to_path_buf(),
                file.parsed(self).clone(),
                file.document_source_index(self),
                snippets,
            );
            self.update_file(path, new_file);
        }
    }

    pub fn insert_bindings(&mut self, bindings: Vec<Vec<EmbeddedBinding>>) {
        self.bindings = Arc::new(bindings);
    }

    pub fn insert_references(&mut self, references: Vec<Vec<EmbeddedValueReference>>) {
        self.references = Arc::new(references);
    }

    /// Inserts a file source so that it can be retrieved by index later.
    ///
    /// Returns the index at which the file source can be retrieved using
    /// `get_source()`.
    pub fn insert_source(&mut self, document_file_source: DocumentFileSource) -> usize {
        self.file_sources
            .iter()
            .position(|(_, file_source)| *file_source == document_file_source)
            .unwrap_or_else(|| self.file_sources.push(document_file_source))
    }

    pub fn insert_file(&mut self, path: &Utf8Path, file: ParsedSource) {
        self.files.pin().insert(path.to_path_buf(), file);
    }

    pub fn update_file(&mut self, path: &Utf8Path, file: ParsedSource) {
        self.files.pin().update(path.to_path_buf(), |_| file);
    }

    pub fn get_module(&self, path: &Utf8Path) -> Option<ModuleInfo> {
        self.modules.pin().get(path).copied()
    }

    pub fn get_file(&self, path: &Utf8Path) -> Option<ParsedSource> {
        self.files.pin().get(path).copied()
    }

    /// Removes all modules that start with the given path. That's usually used when removing a library or a
    /// folder from the project.
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

    /// Returns an [Arc] to itself, cast to [ModuleDb]. This is used to send the service
    /// to the analyzer.
    pub fn boxed_module_db(&self) -> Box<dyn ModuleDb> {
        Box::new(self.clone())
    }

    /// Returns an [Rc] to itself, cast to [ModuleDb]. This is used to send the service
    /// to the analyzer.
    pub fn rc_module_db(&self) -> Rc<dyn ModuleDb> {
        Rc::new(self.clone())
    }

    pub fn insert_module(&self, path: Utf8PathBuf, module: ModuleInfo) {
        self.modules.pin().insert(path, module);
    }

    /// It updates the CST of an existing parsed source
    pub fn update_parsed_root(&self, path: &Utf8Path, new_root: SendNode) {
        self.files
            .pin()
            .update(path.to_path_buf(), |parsed_source| {
                let mut any_parse = parsed_source.parsed(self).clone();
                any_parse.set_new_root(new_root.clone());

                ParsedSource::new(
                    self,
                    path.to_path_buf(),
                    any_parse,
                    parsed_source.document_source_index(self),
                    parsed_source.snippets(self).clone(),
                )
            });
    }

    pub fn remove_module(&self, path: &Utf8Path) {
        self.modules.pin().remove(path);
    }

    pub fn unload_path(&self, path: &Utf8Path) {
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
}

/// This handler is exclusively used for cloning operations (reading operations).
/// Writing operations still go through [ProjectDatabase].
#[derive(Clone, Default)]
pub struct WorkspaceDbHandle {
    files: Arc<HashMap<Utf8PathBuf, ParsedSource>>,
    modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,
    file_sources: Arc<boxcar::Vec<DocumentFileSource>>,
    bindings: Arc<Vec<Vec<EmbeddedBinding>>>,
    references: Arc<Vec<Vec<EmbeddedValueReference>>>,
    storage: salsa::StorageHandle<WorkspaceDb>,
}

impl WorkspaceDbHandle {
    pub fn to_db(&self) -> WorkspaceDb {
        WorkspaceDb {
            files: self.files.clone(),
            // snippets: Arc<HashMap<TextRange, ParsedSnippet>>,
            file_sources: self.file_sources.clone(),
            bindings: self.bindings.clone(),
            modules: self.modules.clone(),
            references: self.references.clone(),
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

#[salsa::db]
impl CssSemanticDb for WorkspaceDb {}

#[salsa::db]
impl JsSemanticDb for WorkspaceDb {}

#[salsa::db]
impl EmbeddedDb for WorkspaceDb {
    fn bindings(&self) -> &[Vec<EmbeddedBinding>] {
        self.bindings.as_slice()
    }

    fn references(&self) -> &[Vec<EmbeddedValueReference>] {
        self.references.as_slice()
    }
}

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
