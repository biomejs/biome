//! This is the database used inside the biome Workspace, mainly the `biome_service` crate.
//!

mod state;

pub(crate) use state::DbReadGuard;
pub use state::DbState;

use crate::WorkspaceError;
use crate::projects::{NestedPath, ProjectDb, ProjectInput, ProjectKey};
use crate::settings::{Settings, VcsIgnoredPatterns};
use biome_configuration::vcs::VcsClientKind;
use biome_db::{ParsedSnippet, ParsedSource};
use biome_languages::DocumentFileSource;
use biome_languages::LanguageDb;
#[cfg(feature = "module_graph")]
use biome_module_graph::{
    InferredLocalTypeId, InferredModuleKey, ModuleDb, ModuleGraphGeneration, ModuleInfo,
    ModuleInfoKind, TypeDb, module_for_key,
};
use biome_parser::AnyParse;
use biome_rowan::SendNode;
#[cfg(feature = "module_graph")]
use biome_rowan::Text;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::{Compute, HashMap, Operation};
use salsa::{Setter, Storage};
use std::convert::Infallible;
use std::rc::Rc;
use std::sync::Arc;
use tracing::{debug, instrument};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParsedSourceUpdateMode {
    Replace,
    Setters,
}

/// Selects how an existing [`ProjectInput`] is updated.
///
/// Choose this mode at the [`DbState`] storage boundary, where the
/// caller knows whether it is operating on a temporary Shared-mode fork or the
/// canonical Owned-mode database. Project operations inside this module accept
/// the mode but must not infer it from the [`WorkspaceDb`] itself because both
/// storage modes use the same database type.
///
/// Passing `Setters` from Shared mode can deadlock because Salsa cannot acquire
/// exclusive storage access while the retained shared handle is alive. Passing
/// `Replace` from Owned mode changes the project's Salsa identity and leaves
/// the previous input allocated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProjectUpdateMode {
    /// Allocates a replacement input and publishes its handle in the project
    /// map.
    ///
    /// Use this only from an operation-local Shared-mode fork.
    Replace,
    /// Preserves the existing input and changes its fields through Salsa
    /// setters.
    ///
    /// Use this only for the Owned-mode database inside `OwnedDb::with_setter`.
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
    /// A map of projects loaded in the workspace.
    projects: Arc<HashMap<ProjectKey, ProjectInput>>,
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
            projects: Arc::default(),
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
    #[cfg(feature = "module_graph")]
    modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,
    file_sources: Arc<boxcar::Vec<DocumentFileSource>>,
    projects: Arc<HashMap<ProjectKey, ProjectInput>>,
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

    pub(crate) fn remove_project(&self, project_key: ProjectKey) {
        self.projects.pin().remove(&project_key);
    }

    /// Checks whether the module data contains `path` without making Salsa
    /// track the read operation.
    ///
    /// Use this for decisions such as skipping a file the scanner has already
    /// indexed. Do not use it in a Salsa query or before reading module data.
    /// Use [`ModuleDb::module_for_path`] when Salsa must track the read.
    #[cfg(feature = "module_graph")]
    pub fn contains_module_untracked(&self, path: &Utf8Path) -> bool {
        self.modules.pin().contains_key(path)
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
    /// Runs a write operation on the module data.
    ///
    /// The Salsa write starts before the data changes, so a read operation
    /// cannot observe new data with the previous generation. The generation is
    /// updated after the write operation completes.
    #[cfg(feature = "module_graph")]
    fn write_module_data(&mut self, write: impl FnOnce(&HashMap<Utf8PathBuf, ModuleInfo>)) {
        let generation = ModuleGraphGeneration::get(self);
        let next = generation.value(self).wrapping_add(1);
        let modules = self.modules.clone();

        // Start the Salsa write before changing the module data, then update
        // the generation after the write operation completes.
        let pending_setter = generation.set_value(self);
        write(&modules);
        pending_setter.to(next);
    }

    /// Returns handles to the collections that this database shares with all
    /// its clones.
    pub fn data(&self) -> WorkspaceDbData {
        WorkspaceDbData {
            #[cfg(feature = "module_graph")]
            modules: self.modules.clone(),
            file_sources: self.file_sources.clone(),
            projects: self.projects.clone(),
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

    pub fn get_parsed_source(&self, path: &Utf8Path) -> Option<ParsedSource> {
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
        self.write_module_data(|modules| {
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
        if let Some(parsed_source) = self.get_parsed_source(path) {
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
            self.write_module_data(|modules| {
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
                self.write_module_data(|modules| {
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

    // #region Project operations

    /// Replaces a project only while the project map still contains the input
    /// from which the replacement was derived.
    fn replace_project_with(
        &self,
        project_key: ProjectKey,
        mut replace: impl FnMut(&Self, ProjectInput) -> ProjectInput,
    ) -> Option<ProjectInput> {
        match self.try_replace_project_with(project_key, |db, project| {
            Ok::<_, Infallible>(replace(db, project))
        }) {
            Ok(project) => project,
            Err(error) => match error {},
        }
    }

    /// Replaces a project only while the project map still contains the input
    /// from which the replacement was derived.
    fn try_replace_project_with<E>(
        &self,
        project_key: ProjectKey,
        mut replace: impl FnMut(&Self, ProjectInput) -> Result<ProjectInput, E>,
    ) -> Result<Option<ProjectInput>, E> {
        let projects = self.projects.pin();
        let Some(mut current) = projects.get(&project_key).copied() else {
            return Ok(None);
        };

        loop {
            // Input allocation stays outside Papaya's callback because that
            // callback may be replayed during a compare-and-swap retry.
            let replacement = replace(self, current)?;
            match projects.compute(project_key, |entry| match entry {
                Some((_, actual)) if *actual == current => Operation::Insert(replacement),
                Some((_, actual)) => Operation::Abort(Some(*actual)),
                None => Operation::Abort(None),
            }) {
                Compute::Updated { .. } => return Ok(Some(replacement)),
                Compute::Aborted(Some(actual)) => current = actual,
                Compute::Aborted(None) => return Ok(None),
                Compute::Inserted(..) | Compute::Removed(..) => unreachable!(),
            }
        }
    }

    pub(crate) fn insert_nested_setting_with_mode(
        &mut self,
        project_key: ProjectKey,
        path: Utf8PathBuf,
        settings: Settings,
        mode: ProjectUpdateMode,
    ) {
        debug!("Set nested settings for {path}");
        match mode {
            ProjectUpdateMode::Replace => {
                let path = NestedPath::from(path);
                let settings = Arc::new(settings);
                self.replace_project_with(project_key, |db, project| {
                    let mut nested_settings = project.nested_settings(db).clone();
                    nested_settings.insert(path.clone(), settings.clone());
                    ProjectInput::new(
                        db,
                        project_key,
                        project.path(db).to_path_buf(),
                        project.root_settings(db),
                        nested_settings,
                    )
                });
            }
            ProjectUpdateMode::Setters => {
                let Some(project) = self.get_project(&project_key) else {
                    return;
                };
                let mut nested_settings = project.nested_settings(self).clone();
                nested_settings.insert(path.into(), Arc::new(settings));
                project.set_nested_settings(self).to(nested_settings);
            }
        }
    }

    pub(crate) fn insert_root_settings_with_mode(
        &mut self,
        project_key: ProjectKey,
        settings: Settings,
        mode: ProjectUpdateMode,
    ) {
        let root_settings = Arc::new(settings);
        match mode {
            ProjectUpdateMode::Replace => {
                self.replace_project_with(project_key, |db, project| {
                    ProjectInput::new(
                        db,
                        project_key,
                        project.path(db).to_path_buf(),
                        root_settings.clone(),
                        project.nested_settings(db).clone(),
                    )
                });
            }
            ProjectUpdateMode::Setters => {
                let Some(project) = self.get_project(&project_key) else {
                    return;
                };
                project.set_root_settings(self).to(root_settings);
            }
        }
    }

    /// Inserts a new project with the given root path.
    ///
    /// Returns the key of the newly inserted project, or returns an existing
    /// project key if a project with the given path already existed.
    #[instrument(skip(self, path), fields(path))]
    pub fn insert_project(&mut self, path: Utf8PathBuf) -> ProjectKey {
        debug!("Insert workspace folder {}", path.as_str());

        let data = self.projects.pin();
        for (key, project_data) in data.iter() {
            if project_data.path(self) == path.as_path() {
                return *key;
            }
        }

        let key = ProjectKey::new();
        data.insert(
            key,
            ProjectInput::new(
                self,
                key,
                path,
                Arc::new(Settings::default()),
                Default::default(),
            ),
        );
        key
    }

    /// Removes the project with the given key.
    pub fn remove_project(&self, project_key: ProjectKey) {
        self.data().remove_project(project_key);
    }

    pub(crate) fn store_nested_ignore_patterns_with_mode(
        &mut self,
        project_key: ProjectKey,
        payload: Vec<(Utf8PathBuf, Vec<String>)>,
        mode: ProjectUpdateMode,
    ) -> Result<(), WorkspaceError> {
        let update_root_settings =
            |mut root_settings: Arc<Settings>| -> Result<Arc<Settings>, WorkspaceError> {
                let git_ignores = match root_settings.vcs_settings.client_kind {
                    Some(VcsClientKind::Git) => payload
                        .iter()
                        .map(|(path, patterns)| {
                            let patterns = patterns.iter().map(String::as_str).collect::<Vec<_>>();
                            VcsIgnoredPatterns::git_ignore(path.as_path(), patterns.as_slice())
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                    None => Vec::new(),
                };

                let settings = Arc::make_mut(&mut root_settings);
                if let Some(ignore_matches) = settings.vcs_settings.ignore_matches.as_mut() {
                    for git_ignore in git_ignores {
                        ignore_matches.insert_git_match(git_ignore);
                    }
                }

                Ok(root_settings)
            };

        match mode {
            ProjectUpdateMode::Replace => {
                self.try_replace_project_with(project_key, |db, project| {
                    Ok::<_, WorkspaceError>(ProjectInput::new(
                        db,
                        project_key,
                        project.path(db).to_path_buf(),
                        update_root_settings(project.root_settings(db))?,
                        project.nested_settings(db).clone(),
                    ))
                })?
                .ok_or_else(WorkspaceError::no_project)?;
            }
            ProjectUpdateMode::Setters => {
                let project = self
                    .get_project(&project_key)
                    .ok_or_else(WorkspaceError::no_project)?;
                let root_settings = update_root_settings(project.root_settings(self))?;
                project.set_root_settings(self).to(root_settings);
            }
        }

        Ok(())
    }

    // #endregion
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
    projects: Arc<HashMap<ProjectKey, ProjectInput>>,
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
            projects,
        } = WorkspaceDb::default();
        Self {
            files,
            #[cfg(feature = "module_graph")]
            modules,
            file_sources,
            projects,
            storage: storage.into_zalsa_handle(),
        }
    }
}

impl SharedWorkspaceDb {
    pub fn data(&self) -> WorkspaceDbData {
        WorkspaceDbData {
            #[cfg(feature = "module_graph")]
            modules: self.modules.clone(),
            file_sources: self.file_sources.clone(),
            projects: self.projects.clone(),
        }
    }

    pub fn fork(&self) -> WorkspaceDb {
        WorkspaceDb {
            files: self.files.clone(),
            file_sources: self.file_sources.clone(),
            #[cfg(feature = "module_graph")]
            modules: self.modules.clone(),
            projects: self.projects.clone(),
            storage: self.storage.clone().into_storage(),
        }
    }
}

// #region Salsa Database impls

#[salsa::db]
impl salsa::Database for WorkspaceDb {}

#[salsa::db]
impl biome_db::Db for WorkspaceDb {
    fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<ParsedSource> {
        self.files.pin().get(path).copied()
    }
}

#[salsa::db]
impl ProjectDb for WorkspaceDb {
    fn get_project(&self, project_key: &ProjectKey) -> Option<ProjectInput> {
        self.projects.pin().get(project_key).copied()
    }

    fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey> {
        self.projects.pin().iter().find_map(|(key, project_data)| {
            path.starts_with(project_data.path(self)).then_some(*key)
        })
    }

    fn for_each_project(&self, f: &mut dyn FnMut(ProjectInput)) {
        for project in self.projects.pin().values() {
            f(*project);
        }
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
        let current = module_for_key(self, module_key)?;

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
        // Read the generation before reading the module data.
        let _ = self.module_graph_generation();
        self.get_module(path)
    }

    fn for_each_module(&self, f: &mut dyn FnMut(ModuleInfo)) {
        // Read the generation before reading the module data.
        let _ = self.module_graph_generation();
        let modules = self.modules.pin();
        let iter = modules.iter();
        for (_, module) in iter {
            f(*module);
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

// #endregion

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
    use salsa::plumbing::{AsId, ZalsaDatabase};
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

    fn assert_single_project_in_sync(db: &WorkspaceDb, project_key: ProjectKey) -> ProjectInput {
        assert_eq!(db.projects.pin().len(), 1);
        let indexed_project = db.get_project(&project_key).unwrap();

        let ingredient = ProjectInput::ingredient(db);
        let mut salsa_projects = ingredient.entries(db.zalsa());
        let salsa_project = salsa_projects.next().unwrap().as_struct();
        assert!(salsa_projects.next().is_none());
        assert_eq!(indexed_project.as_id(), salsa_project.as_id());

        indexed_project
    }

    /// Creates a database that can pause a write operation when Salsa starts
    /// invalidating existing read operations.
    ///
    /// The barrier lets a test inspect an existing read operation after the
    /// write starts but before the module data changes. `armed` prevents setup
    /// writes from triggering the barrier.
    #[cfg(feature = "module_graph")]
    fn module_write_test_db(barrier: Arc<Barrier>, armed: Arc<AtomicBool>) -> WorkspaceDb {
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
            projects: Arc::default(),
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
        assert_eq!(db.get_parsed_source(path).unwrap().as_id(), file.as_id());
    }

    #[test]
    fn replace_file_replaces_existing_input() {
        let mut db = WorkspaceDb::default();
        let path = Utf8Path::new("test.js");

        let file = db.replace_file(path, parse_js("let a = 1;"), 0, vec![]);
        let updated_file = db.replace_file(path, parse_js("let b = 2;"), 0, vec![]);

        assert_ne!(file.as_id(), updated_file.as_id());
        assert_eq!(
            db.get_parsed_source(path).unwrap().as_id(),
            updated_file.as_id()
        );
    }

    #[test]
    fn insert_project_keeps_index_and_salsa_in_sync() {
        let mut db = WorkspaceDb::default();
        let path = Utf8PathBuf::from("project");

        let project_key = db.insert_project(path.clone());
        assert_eq!(db.insert_project(path.clone()), project_key);

        let project = assert_single_project_in_sync(&db, project_key);
        assert_eq!(project.path(&db), path.as_path());
    }

    #[test]
    fn remove_project_removes_project_from_index() {
        let mut db = WorkspaceDb::default();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = assert_single_project_in_sync(&db, project_key);

        db.remove_project(project_key);

        assert!(db.projects.pin().is_empty());
        assert!(db.get_project(&project_key).is_none());

        let ingredient = ProjectInput::ingredient(&db);
        let mut salsa_projects = ingredient.entries(db.zalsa());
        let salsa_project = salsa_projects.next().unwrap().as_struct();
        assert!(salsa_projects.next().is_none());
        assert_eq!(salsa_project.as_id(), project.as_id());
    }

    #[test]
    fn insert_root_settings_keeps_index_and_salsa_in_sync() {
        let mut db = WorkspaceDb::default();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = assert_single_project_in_sync(&db, project_key);
        let mut settings = Settings::default();
        settings.vcs_settings.client_kind = Some(VcsClientKind::Git);

        db.insert_root_settings_with_mode(
            project_key,
            settings.clone(),
            ProjectUpdateMode::Setters,
        );
        db.insert_root_settings_with_mode(project_key, settings, ProjectUpdateMode::Setters);

        let updated_project = assert_single_project_in_sync(&db, project_key);
        assert_eq!(updated_project.as_id(), project.as_id());
        assert_eq!(
            updated_project.root_settings(&db).vcs_settings.client_kind,
            Some(VcsClientKind::Git)
        );
    }

    #[test]
    fn insert_nested_setting_keeps_index_and_salsa_in_sync() {
        let mut db = WorkspaceDb::default();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = assert_single_project_in_sync(&db, project_key);
        let nested_path = Utf8PathBuf::from("project/package");
        let mut settings = Settings::default();
        settings.vcs_settings.client_kind = Some(VcsClientKind::Git);

        db.insert_nested_setting_with_mode(
            project_key,
            nested_path.clone(),
            settings,
            ProjectUpdateMode::Setters,
        );

        let updated_project = assert_single_project_in_sync(&db, project_key);
        assert_eq!(updated_project.as_id(), project.as_id());
        assert_eq!(updated_project.nested_settings(&db).len(), 1);
        assert_eq!(
            db.get_nested_settings(project_key, &nested_path)
                .unwrap()
                .vcs_settings
                .client_kind,
            Some(VcsClientKind::Git)
        );
    }

    #[test]
    fn replacement_updates_retry_after_project_changes() {
        let shared = SharedWorkspaceDb::default();
        let mut db = shared.fork();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        drop(db);
        let writers_ready = Arc::new(Barrier::new(2));

        std::thread::scope(|scope| {
            for path in ["project/a", "project/b"] {
                let db = shared.fork();
                let writers_ready = writers_ready.clone();
                scope.spawn(move || {
                    let path = NestedPath::new(path);
                    let settings = Arc::new(Settings::default());
                    let mut first_attempt = true;
                    db.replace_project_with(project_key, |db, project| {
                        if first_attempt {
                            first_attempt = false;
                            writers_ready.wait();
                        }
                        let mut nested_settings = project.nested_settings(db).clone();
                        nested_settings.insert(path.clone(), settings.clone());
                        ProjectInput::new(
                            db,
                            project_key,
                            project.path(db).to_path_buf(),
                            project.root_settings(db),
                            nested_settings,
                        )
                    })
                    .unwrap();
                });
            }
        });

        let db = shared.fork();
        assert_eq!(
            db.get_project(&project_key)
                .unwrap()
                .nested_settings(&db)
                .len(),
            2
        );
    }

    #[test]
    fn replacement_update_does_not_restore_removed_project() {
        let shared = SharedWorkspaceDb::default();
        let mut db = shared.fork();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        drop(db);
        let update_started = Arc::new(Barrier::new(2));
        let project_removed = Arc::new(Barrier::new(2));

        std::thread::scope(|scope| {
            let db = shared.fork();
            let writer = {
                let update_started = update_started.clone();
                let project_removed = project_removed.clone();
                scope.spawn(move || {
                    db.replace_project_with(project_key, |db, project| {
                        update_started.wait();
                        project_removed.wait();
                        ProjectInput::new(
                            db,
                            project_key,
                            project.path(db).to_path_buf(),
                            Arc::new(Settings::default()),
                            project.nested_settings(db).clone(),
                        )
                    })
                })
            };

            update_started.wait();
            shared.data().remove_project(project_key);
            project_removed.wait();
            assert!(writer.join().unwrap().is_none());
        });

        assert!(shared.fork().get_project(&project_key).is_none());
    }

    #[test]
    fn store_nested_ignore_patterns_keeps_index_and_salsa_in_sync() {
        let mut db = WorkspaceDb::default();
        let root_path = Utf8PathBuf::from("/project");
        let project_key = db.insert_project(root_path.clone());
        let project = assert_single_project_in_sync(&db, project_key);
        let mut settings = Settings::default();
        settings.vcs_settings.client_kind = Some(VcsClientKind::Git);
        settings.vcs_settings.use_ignore_file = Some(true.into());
        settings
            .vcs_settings
            .store_root_ignore_patterns(root_path.as_path(), &[])
            .unwrap();
        db.insert_root_settings_with_mode(project_key, settings, ProjectUpdateMode::Setters);
        let nested_path = root_path.join("package");
        let ignored_path = nested_path.join("generated.js");

        assert!(!project.root_settings(&db).vcs_settings.is_ignored(
            ignored_path.as_path(),
            false,
            None
        ));

        db.store_nested_ignore_patterns_with_mode(
            project_key,
            vec![(nested_path, vec!["generated.js".to_string()])],
            ProjectUpdateMode::Setters,
        )
        .unwrap();

        let updated_project = assert_single_project_in_sync(&db, project_key);
        assert_eq!(updated_project.as_id(), project.as_id());
        assert!(updated_project.root_settings(&db).vcs_settings.is_ignored(
            ignored_path.as_path(),
            false,
            None
        ));
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
    fn module_insertion_keeps_data_and_generation_in_sync() {
        let barrier = Arc::new(Barrier::new(2));
        let armed = Arc::new(AtomicBool::new(false));
        let mut db = module_write_test_db(barrier.clone(), armed.clone());
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
    fn module_removal_keeps_data_and_generation_in_sync() {
        let barrier = Arc::new(Barrier::new(2));
        let armed = Arc::new(AtomicBool::new(false));
        let mut db = module_write_test_db(barrier.clone(), armed.clone());
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
    fn module_unload_keeps_data_and_generation_in_sync() {
        let barrier = Arc::new(Barrier::new(2));
        let armed = Arc::new(AtomicBool::new(false));
        let mut db = module_write_test_db(barrier.clone(), armed.clone());
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
