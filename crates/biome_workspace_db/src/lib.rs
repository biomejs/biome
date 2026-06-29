//! This is the database used inside the biome Workspace, mainly the `biome_service` crate.
//!

#[cfg(feature = "html_embeds")]
pub mod embedded;

use biome_db::{ParsedSnippet, ParsedSource};
use biome_languages::DocumentFileSource;
use biome_languages::LanguageDb;
#[cfg(feature = "module_graph")]
use biome_module_graph::{ModuleDb, ModuleInfo, ModuleInfoKind};
use biome_parser::AnyParse;
use biome_rowan::SendNode;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use salsa::Storage;
use std::rc::Rc;
use std::sync::Arc;

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

impl WorkspaceDb {
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

    pub fn insert_or_update_file(
        &mut self,
        path: &Utf8Path,
        parsed: AnyParse,
        document_source_index: usize,
        snippets: Vec<ParsedSnippet>,
    ) -> ParsedSource {
        if let Some(file) = self.get_file(path) {
            salsa::Setter::to(file.set_parsed(self), parsed);
            salsa::Setter::to(file.set_document_source_index(self), document_source_index);
            salsa::Setter::to(file.set_snippets(self), snippets);
            file
        } else {
            let file = ParsedSource::new(
                self,
                path.to_path_buf(),
                parsed,
                document_source_index,
                snippets,
            );
            self.insert_file(path, file);
            file
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
        self.modules.pin().insert(path, module);
    }

    /// It updates the CST of an existing parsed source
    pub fn update_parsed_root(
        &mut self,
        path: &Utf8Path,
        new_root: SendNode,
    ) -> Option<ParsedSource> {
        let parsed_source = self.get_file(path)?;
        let mut any_parse = parsed_source.parsed(self).clone();
        any_parse.set_new_root(new_root);
        salsa::Setter::to(parsed_source.set_parsed(self), any_parse);
        Some(parsed_source)
    }

    #[cfg(feature = "module_graph")]
    pub fn remove_module(&self, path: &Utf8Path) {
        self.modules.pin().remove(path);
    }

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
