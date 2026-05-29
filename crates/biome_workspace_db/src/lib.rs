mod embeded_bindings;

use crate::embeded_bindings::{BindingsDb, EmbeddedBinding};
use biome_css_semantic::db::CssSemanticDb;
use biome_db::{ParsedSnippet, ParsedSource};
use biome_js_semantic::JsSemanticDb;
use biome_languages::AnyFileSource;
use biome_languages::db::LanguageDb;
use biome_module_graph::{ModuleDb, ModuleInfo, ModuleInfoKind};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use salsa::Storage;
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
    // snippets: Arc<HashMap<TextRange, ParsedSnippet>>,
    file_sources: boxcar::Vec<AnyFileSource>,
    bindings: Vec<Vec<EmbeddedBinding>>,
    modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,

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

    /// Inserts a file source so that it can be retrieved by index later.
    ///
    /// Returns the index at which the file source can be retrieved using
    /// `get_source()`.
    fn insert_source(&mut self, document_file_source: AnyFileSource) -> usize {
        self.file_sources
            .iter()
            .position(|(_, file_source)| *file_source == document_file_source)
            .unwrap_or_else(|| self.file_sources.push(document_file_source))
    }

    pub fn insert_module(&mut self, path: &Utf8Path, module: ModuleInfo) {
        self.modules.pin().insert(path.to_path_buf(), module);
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

    pub fn remove_module(&self, path: &Utf8Path) {
        self.modules.pin().remove(path);
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
impl BindingsDb for WorkspaceDb {
    fn bindings(&self) -> Vec<Vec<EmbeddedBinding>> {
        self.bindings.clone()
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
    fn source_from_index(&self, index: usize) -> Option<AnyFileSource> {
        self.file_sources.get(index).copied()
    }
}
