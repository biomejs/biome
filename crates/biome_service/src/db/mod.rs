//! This is the database used inside the biome Workspace, mainly the `biome_service` crate.
//!

mod state;

pub(crate) use state::DbReadGuard;
pub use state::DbState;

use crate::WorkspaceError;
use crate::projects::{NestedPath, ProjectDb, ProjectInput, ProjectKey};
use crate::settings::{Settings, SettingsIdentity, VcsIgnoredPatterns};
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
use std::sync::RwLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{debug, instrument};

const SETTINGS_QUERY_CACHE_CAPACITY: usize = 256;
type SettingsQueryEventHandler =
    std::panic::AssertUnwindSafe<Arc<dyn Fn(salsa::Event) + Send + Sync>>;

#[salsa::db]
#[derive(Default)]
pub(crate) struct SettingsQueryDb {
    storage: Storage<Self>,
}

#[salsa::db]
impl salsa::Database for SettingsQueryDb {}

struct SettingsQueryCacheState {
    storage: salsa::StorageHandle<SettingsQueryDb>,
    interned_values: Arc<AtomicUsize>,
}

struct SettingsQueryCache {
    state: RwLock<SettingsQueryCacheState>,
    event_handler: Option<SettingsQueryEventHandler>,
}

impl Default for SettingsQueryCache {
    fn default() -> Self {
        Self {
            state: RwLock::new(SettingsQueryCacheState::new(None)),
            event_handler: None,
        }
    }
}

impl SettingsQueryCacheState {
    fn new(event_handler: Option<SettingsQueryEventHandler>) -> Self {
        let interned_values = Arc::new(AtomicUsize::new(0));
        let counter = interned_values.clone();
        let storage = Storage::new(Some(Box::new(move |event| {
            if matches!(&event.kind, salsa::EventKind::DidInternValue { .. }) {
                counter.fetch_add(1, Ordering::Relaxed);
            }
            if let Some(event_handler) = &event_handler {
                (event_handler.0)(event);
            }
        })));
        Self {
            storage: storage.into_zalsa_handle(),
            interned_values,
        }
    }

    fn database(&self) -> SettingsQueryDb {
        SettingsQueryDb {
            storage: self.storage.clone().into_storage(),
        }
    }
}

impl SettingsQueryCache {
    fn database(&self) -> SettingsQueryDb {
        {
            let state = self
                .state
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.interned_values.load(Ordering::Relaxed) < SETTINGS_QUERY_CACHE_CAPACITY {
                return state.database();
            }
        }
        let mut state = self
            .state
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.interned_values.load(Ordering::Relaxed) >= SETTINGS_QUERY_CACHE_CAPACITY {
            *state = SettingsQueryCacheState::new(
                self.event_handler
                    .as_ref()
                    .map(|handler| std::panic::AssertUnwindSafe(handler.0.clone())),
            );
        }
        state.database()
    }

    #[cfg(test)]
    fn with_event_handler(handler: Box<dyn Fn(salsa::Event) + Send + Sync>) -> Self {
        let event_handler: Option<SettingsQueryEventHandler> =
            Some(std::panic::AssertUnwindSafe(Arc::from(handler)));
        Self {
            state: RwLock::new(SettingsQueryCacheState::new(
                event_handler
                    .as_ref()
                    .map(|handler| std::panic::AssertUnwindSafe(handler.0.clone())),
            )),
            event_handler,
        }
    }
}

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
    settings_queries: Arc<SettingsQueryCache>,
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
            settings_queries: Arc::default(),
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
    pub(crate) fn settings_query_db(&self) -> SettingsQueryDb {
        self.settings_queries.database()
    }

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
                let settings = SettingsIdentity::from(Arc::new(settings));
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
                nested_settings.insert(path.into(), Arc::new(settings).into());
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
        let root_settings = SettingsIdentity::from(Arc::new(settings));
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
                Arc::new(Settings::default()).into(),
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
            |mut root_settings: SettingsIdentity| -> Result<SettingsIdentity, WorkspaceError> {
                let git_ignores = match root_settings.as_ref().vcs_settings.client_kind {
                    Some(VcsClientKind::Git) => payload
                        .iter()
                        .map(|(path, patterns)| {
                            let patterns = patterns.iter().map(String::as_str).collect::<Vec<_>>();
                            VcsIgnoredPatterns::git_ignore(path.as_path(), patterns.as_slice())
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                    None => Vec::new(),
                };

                let settings = root_settings.make_mut();
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
    settings_queries: Arc<SettingsQueryCache>,
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
            settings_queries,
        } = WorkspaceDb::default();
        Self {
            files,
            #[cfg(feature = "module_graph")]
            modules,
            file_sources,
            projects,
            settings_queries,
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
            settings_queries: self.settings_queries.clone(),
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
    #[cfg(feature = "lang_js")]
    use crate::file_handlers::javascript::{
        resolve_analyzer_options as resolve_js_analyzer_options,
        resolve_format_options as resolve_js_format_options,
    };
    #[cfg(feature = "lang_js")]
    use crate::file_handlers::{
        AnalyzerVisitorBuilder, analyzer_input_count_for_test, resolved_manifest_visitor_for_test,
    };
    use crate::settings::{
        SettingsEditorState, SettingsHandle, SettingsQuery, SettingsQuerySelection,
        SettingsSelectionKey,
    };
    #[cfg(feature = "lang_js")]
    use biome_analyze::AnalyzerOptions;
    #[cfg(feature = "lang_js")]
    use biome_configuration::analyzer::AnalyzerSelector;
    #[cfg(feature = "lang_js")]
    use biome_configuration::bool::Bool;
    use biome_db::Db;
    use biome_db::testing::{Events, function_query_will_execute_count_by_name};
    #[cfg(any(feature = "lang_js", feature = "module_graph"))]
    use biome_fs::BiomePath;
    #[cfg(feature = "module_graph")]
    use biome_fs::MemoryFileSystem;
    #[cfg(feature = "module_graph")]
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_js_parser::{JsParserOptions, parse};
    #[cfg(feature = "lang_js")]
    use biome_js_syntax::JsLanguage;
    #[cfg(feature = "lang_js")]
    use biome_languages::JsFileSource;
    #[cfg(feature = "module_graph")]
    use biome_module_graph::{ModuleDb, PathInfoCache, resolve_html_module};
    #[cfg(feature = "lang_js")]
    use biome_package::{Dependencies, PackageJson};
    #[cfg(feature = "module_graph")]
    use biome_project_layout::ProjectLayout;
    use salsa::plumbing::{AsId, ZalsaDatabase};
    #[cfg(feature = "lang_js")]
    use std::str::FromStr;
    use std::sync::Barrier;
    #[cfg(feature = "module_graph")]
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    static SETTER_READER_STARTED: Barrier = Barrier::new(2);

    #[salsa::interned]
    struct TestSettingsQueryInput {
        project: ProjectInput,
        #[returns(ref)]
        selection: SettingsQuerySelection,
    }

    #[salsa::interned]
    struct TestSettingsCacheInput {
        value: usize,
    }

    #[salsa::tracked(lru = 256)]
    fn test_settings_query<'db>(
        db: &'db dyn ProjectDb,
        input: TestSettingsQueryInput<'db>,
    ) -> bool {
        input
            .selection(db)
            .selected_settings(db, input.project(db))
            .as_ref()
            .linter_recommended_enabled()
    }

    fn run_test_settings_query(db: &dyn ProjectDb, query: &SettingsQuery) -> bool {
        let input = TestSettingsQueryInput::new(db, query.project(), query.selection().clone());
        test_settings_query(db, input)
    }

    fn parse_js(source: &str) -> AnyParse {
        parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        )
        .into()
    }

    fn settings_query_test_db() -> (WorkspaceDb, Events) {
        let events = Events::default();
        let storage = Storage::new(Some(Box::new({
            let events = events.clone();
            move |event| events.0.lock().unwrap().push(event)
        })));
        let settings_queries = Arc::new(SettingsQueryCache::with_event_handler(Box::new({
            let events = events.clone();
            move |event| events.0.lock().unwrap().push(event)
        })));
        let db = WorkspaceDb {
            files: Arc::default(),
            #[cfg(feature = "module_graph")]
            modules: Arc::default(),
            file_sources: Arc::default(),
            projects: Arc::default(),
            settings_queries,
            storage,
        };
        #[cfg(feature = "module_graph")]
        ModuleGraphGeneration::new(&db, 0);
        (db, events)
    }

    #[test]
    fn settings_query_cache_rotates_storage_at_capacity() {
        let db = WorkspaceDb::default();
        let first = db.settings_query_db();

        for value in 0..SETTINGS_QUERY_CACHE_CAPACITY {
            let current = db.settings_query_db();
            assert!(std::ptr::eq(first.zalsa(), current.zalsa()));
            TestSettingsCacheInput::new(&current, value);
        }

        let next = db.settings_query_db();
        assert!(!std::ptr::eq(first.zalsa(), next.zalsa()));
    }

    #[test]
    fn settings_query_cache_recovers_poisoned_lock() {
        let cache = SettingsQueryCache::default();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _state = cache.state.write().unwrap();
            panic!("poison settings query cache");
        }));
        assert!(result.is_err());

        cache.database();
    }

    fn take_events(events: &Events) -> Vec<salsa::Event> {
        std::mem::take(&mut *events.0.lock().unwrap())
    }

    fn settings_query_execution_count(db: &WorkspaceDb, events: &Events) -> usize {
        function_query_will_execute_count_by_name(db, "test_settings_query", &take_events(events))
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
            settings_queries: Arc::default(),
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
    fn resolved_settings_query_tracks_owned_settings_updates() {
        let (mut db, events) = settings_query_test_db();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = db.get_project(&project_key).unwrap();
        let settings = project.root_settings(&db);
        let query = SettingsQuery::new(
            project,
            SettingsSelectionKey::Root,
            settings.as_ref(),
            None,
            Utf8Path::new("project/file.js"),
        );
        take_events(&events);

        run_test_settings_query(&db, &query);
        assert_eq!(settings_query_execution_count(&db, &events), 1);
        run_test_settings_query(&db, &query);
        assert_eq!(settings_query_execution_count(&db, &events), 0);

        db.upsert_file(
            Utf8Path::new("project/file.js"),
            parse_js("let a = 1;"),
            0,
            vec![],
        );
        take_events(&events);
        run_test_settings_query(&db, &query);
        assert_eq!(settings_query_execution_count(&db, &events), 0);

        db.insert_root_settings_with_mode(
            project_key,
            Settings::default(),
            ProjectUpdateMode::Setters,
        );
        take_events(&events);
        run_test_settings_query(&db, &query);
        assert_eq!(settings_query_execution_count(&db, &events), 1);
        run_test_settings_query(&db, &query);
        assert_eq!(settings_query_execution_count(&db, &events), 0);
    }

    #[cfg(feature = "lang_js")]
    #[test]
    fn resolved_formatter_query_tracks_only_selected_settings() {
        let (mut db, events) = settings_query_test_db();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = db.get_project(&project_key).unwrap();
        let settings = project.root_settings(&db);
        let query = SettingsQuery::new(
            project,
            SettingsSelectionKey::Root,
            settings.as_ref(),
            None,
            Utf8Path::new("project/file.js"),
        );
        let handle = SettingsHandle::new(settings.as_ref(), SettingsEditorState::new(query));
        let source = JsFileSource::js_module();
        take_events(&events);

        resolve_js_format_options(
            &BiomePath::new("project/file.js"),
            &DocumentFileSource::Js(source),
            &handle,
            &db,
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_js_format_options",
                &take_events(&events),
            ),
            1
        );
        resolve_js_format_options(
            &BiomePath::new("project/file.js"),
            &DocumentFileSource::Js(source),
            &handle,
            &db,
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_js_format_options",
                &take_events(&events),
            ),
            0
        );

        db.upsert_file(
            Utf8Path::new("project/file.js"),
            parse_js("let a = 1;"),
            0,
            vec![],
        );
        take_events(&events);
        resolve_js_format_options(
            &BiomePath::new("project/file.js"),
            &DocumentFileSource::Js(source),
            &handle,
            &db,
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_js_format_options",
                &take_events(&events),
            ),
            0
        );

        db.insert_root_settings_with_mode(
            project_key,
            Settings::default(),
            ProjectUpdateMode::Setters,
        );
        take_events(&events);
        resolve_js_format_options(
            &BiomePath::new("project/file.js"),
            &DocumentFileSource::Js(source),
            &handle,
            &db,
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_js_format_options",
                &take_events(&events),
            ),
            1
        );
    }

    #[cfg(feature = "lang_js")]
    #[test]
    fn resolved_analyzer_options_query_tracks_only_selected_settings_and_source() {
        let (mut db, events) = settings_query_test_db();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = db.get_project(&project_key).unwrap();
        let settings = project.root_settings(&db);
        let query = SettingsQuery::new(
            project,
            SettingsSelectionKey::Root,
            settings.as_ref(),
            None,
            Utf8Path::new("project/file.js"),
        );
        let handle = SettingsHandle::new(settings.as_ref(), SettingsEditorState::new(query));
        let source = DocumentFileSource::Js(JsFileSource::js_module());
        take_events(&events);

        let options = resolve_js_analyzer_options(
            &BiomePath::new("project/file.js"),
            None,
            &source,
            None,
            &handle,
            &db,
        );
        assert_eq!(options.file_path, Utf8Path::new("project/file.js"));
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_js_analyzer_options",
                &take_events(&events),
            ),
            1
        );

        let options = resolve_js_analyzer_options(
            &BiomePath::new("project/other.js"),
            Some(Utf8Path::new("project")),
            &source,
            Some("explanation"),
            &handle,
            &db,
        );
        assert_eq!(options.file_path, Utf8Path::new("project/other.js"));
        assert_eq!(
            options.working_directory.as_ref().as_deref(),
            Some(Utf8Path::new("project"))
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_js_analyzer_options",
                &take_events(&events),
            ),
            0
        );

        db.upsert_file(
            Utf8Path::new("project/file.js"),
            parse_js("let a = 1;"),
            0,
            vec![],
        );
        take_events(&events);
        resolve_js_analyzer_options(
            &BiomePath::new("project/file.js"),
            None,
            &source,
            None,
            &handle,
            &db,
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_js_analyzer_options",
                &take_events(&events),
            ),
            0
        );

        resolve_js_analyzer_options(
            &BiomePath::new("project/file.tsx"),
            None,
            &DocumentFileSource::Js(JsFileSource::tsx()),
            None,
            &handle,
            &db,
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_js_analyzer_options",
                &take_events(&events),
            ),
            1
        );

        db.insert_root_settings_with_mode(
            project_key,
            Settings::default(),
            ProjectUpdateMode::Setters,
        );
        take_events(&events);
        resolve_js_analyzer_options(
            &BiomePath::new("project/file.js"),
            None,
            &source,
            None,
            &handle,
            &db,
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_js_analyzer_options",
                &take_events(&events),
            ),
            1
        );
    }

    #[cfg(feature = "lang_js")]
    #[test]
    fn resolved_analyzer_query_ignores_request_filters() {
        let (mut db, events) = settings_query_test_db();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = db.get_project(&project_key).unwrap();
        let settings = project.root_settings(&db).clone_arc();
        let query = SettingsQuery::new(
            project,
            SettingsSelectionKey::Root,
            settings.as_ref(),
            None,
            Utf8Path::new("project/file.js"),
        );
        let handle = SettingsHandle::new(settings.as_ref(), SettingsEditorState::new(query));
        let no_debugger = AnalyzerSelector::from_str("lint/suspicious/noDebugger").unwrap();
        let no_console = AnalyzerSelector::from_str("lint/suspicious/noConsole").unwrap();
        let only_a = [no_debugger];
        let skip_a = [no_console];
        let enabled_a = [no_debugger];
        let only_b = [no_console];
        let skip_b = [no_debugger];
        let enabled_b = [no_console];
        let input_count = analyzer_input_count_for_test(&db);
        take_events(&events);

        let first = AnalyzerVisitorBuilder::new(&handle, &db, AnalyzerOptions::default())
            .with_only(&only_a)
            .with_skip(&skip_a)
            .with_enabled_selectors(&enabled_a)
            .finish();
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_analyzer_visitor",
                &take_events(&events),
            ),
            1
        );

        let second = AnalyzerVisitorBuilder::new(&handle, &db, AnalyzerOptions::default())
            .with_only(&only_b)
            .with_skip(&skip_b)
            .with_enabled_selectors(&enabled_b)
            .finish();
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_analyzer_visitor",
                &take_events(&events),
            ),
            0
        );
        assert_ne!(first.enabled_rules, second.enabled_rules);
        assert_ne!(first.disabled_rules, second.disabled_rules);
        assert_eq!(analyzer_input_count_for_test(&db), input_count + 1);
    }

    #[cfg(feature = "lang_js")]
    #[test]
    fn resolved_manifest_query_tracks_settings_and_manifest_dependencies() {
        let (mut db, events) = settings_query_test_db();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = db.get_project(&project_key).unwrap();
        let settings = project.root_settings(&db);
        let query = SettingsQuery::new(
            project,
            SettingsSelectionKey::Root,
            settings.as_ref(),
            None,
            Utf8Path::new("project/file.js"),
        );
        let manifest = PackageJson {
            dev_dependencies: Dependencies(
                vec![("vitest".into(), "^1.0.0".into())].into_boxed_slice(),
            ),
            ..PackageJson::default()
        };
        take_events(&events);

        resolved_manifest_visitor_for_test(
            &db,
            query.selection().selected_settings(&db, query.project()),
            query.override_indices().into(),
            manifest.clone(),
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_manifest_visitor",
                &take_events(&events),
            ),
            1
        );
        resolved_manifest_visitor_for_test(
            &db,
            query.selection().selected_settings(&db, query.project()),
            query.override_indices().into(),
            manifest.clone(),
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_manifest_visitor",
                &take_events(&events),
            ),
            0
        );

        db.upsert_file(
            Utf8Path::new("project/file.js"),
            parse_js("let a = 1;"),
            0,
            vec![],
        );
        take_events(&events);
        resolved_manifest_visitor_for_test(
            &db,
            query.selection().selected_settings(&db, query.project()),
            query.override_indices().into(),
            manifest.clone(),
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_manifest_visitor",
                &take_events(&events),
            ),
            0
        );

        db.insert_root_settings_with_mode(
            project_key,
            Settings::default(),
            ProjectUpdateMode::Setters,
        );
        take_events(&events);
        resolved_manifest_visitor_for_test(
            &db,
            query.selection().selected_settings(&db, query.project()),
            query.override_indices().into(),
            manifest,
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_manifest_visitor",
                &take_events(&events),
            ),
            1
        );

        let changed_manifest = PackageJson {
            dev_dependencies: Dependencies(
                vec![("jest".into(), "^30.0.0".into())].into_boxed_slice(),
            ),
            ..PackageJson::default()
        };
        resolved_manifest_visitor_for_test(
            &db,
            query.selection().selected_settings(&db, query.project()),
            query.override_indices().into(),
            changed_manifest,
        );
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "resolved_manifest_visitor",
                &take_events(&events),
            ),
            1
        );
    }

    #[test]
    fn resolved_settings_query_uses_replacement_project_identity() {
        let (mut db, events) = settings_query_test_db();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = db.get_project(&project_key).unwrap();
        let settings = project.root_settings(&db);
        let query = SettingsQuery::new(
            project,
            SettingsSelectionKey::Root,
            settings.as_ref(),
            None,
            Utf8Path::new("project/file.js"),
        );
        run_test_settings_query(&db, &query);
        take_events(&events);

        db.insert_root_settings_with_mode(
            project_key,
            Settings::default(),
            ProjectUpdateMode::Replace,
        );
        let replacement = db.get_project(&project_key).unwrap();
        assert_ne!(project.as_id(), replacement.as_id());
        let settings = replacement.root_settings(&db);
        let replacement_query = SettingsQuery::new(
            replacement,
            SettingsSelectionKey::Root,
            settings.as_ref(),
            None,
            Utf8Path::new("project/file.js"),
        );
        take_events(&events);

        run_test_settings_query(&db, &replacement_query);
        assert_eq!(settings_query_execution_count(&db, &events), 1);
        run_test_settings_query(&db, &replacement_query);
        assert_eq!(settings_query_execution_count(&db, &events), 0);
    }

    #[test]
    fn settings_context_records_root_and_deepest_nested_selection() {
        let mut db = WorkspaceDb::default();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        db.insert_nested_setting_with_mode(
            project_key,
            Utf8PathBuf::from("project/packages"),
            Settings::default(),
            ProjectUpdateMode::Setters,
        );
        db.insert_nested_setting_with_mode(
            project_key,
            Utf8PathBuf::from("project/packages/deep"),
            Settings::default(),
            ProjectUpdateMode::Setters,
        );

        let (project, selection, _, settings) = db
            .get_settings_context_for_path(project_key, Utf8Path::new("project/root.js"))
            .unwrap();
        let root_query = SettingsQuery::new(
            project,
            selection,
            settings.as_ref(),
            None,
            Utf8Path::new("project/root.js"),
        );
        assert_eq!(root_query.selection().key(), SettingsSelectionKey::Root);

        let (project, selection, working_directory, settings) = db
            .get_settings_context_for_path(
                project_key,
                Utf8Path::new("project/packages/deep/file.js"),
            )
            .unwrap();
        let nested_query = SettingsQuery::new(
            project,
            selection,
            settings.as_ref(),
            None,
            Utf8Path::new("project/packages/deep/file.js"),
        );
        assert_eq!(working_directory, Utf8Path::new("project/packages/deep"));
        assert_eq!(
            nested_query.selection().key(),
            SettingsSelectionKey::Nested(NestedPath::new("project/packages/deep"))
        );
        assert_ne!(root_query.selection(), nested_query.selection());

        project.set_nested_settings(&mut db).to(Default::default());
        assert_eq!(
            nested_query
                .selection()
                .selected_settings(&db, nested_query.project()),
            project.root_settings(&db)
        );
    }

    #[test]
    fn inline_settings_do_not_change_tracked_project_selection() {
        let mut db = WorkspaceDb::default();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = db.get_project(&project_key).unwrap();
        let settings = project.root_settings(&db);
        let mut inline_settings = Settings::default();
        inline_settings.linter.rules = Some(biome_configuration::Rules {
            recommended: Some(false),
            ..Default::default()
        });
        let query = SettingsQuery::new(
            project,
            SettingsSelectionKey::Root,
            settings.as_ref(),
            Some(inline_settings),
            Utf8Path::new("project/file.js"),
        );

        assert!(run_test_settings_query(&db, &query));
        assert!(
            !query
                .inline_settings()
                .unwrap()
                .as_ref()
                .linter_recommended_enabled()
        );
    }

    #[cfg(feature = "lang_js")]
    #[test]
    fn inline_settings_resolve_without_tracked_formatter_queries() {
        let (mut db, events) = settings_query_test_db();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = db.get_project(&project_key).unwrap();
        let settings = project.root_settings(&db);
        let project_count = ProjectInput::ingredient(&db).entries(db.zalsa()).count();
        let settings = settings.clone_arc();
        take_events(&events);

        for _ in 0..16 {
            let mut inline_settings = Settings::default();
            inline_settings
                .languages
                .javascript
                .parser
                .parse_class_parameter_decorators = Some(Bool(true));
            let query = SettingsQuery::new(
                project,
                SettingsSelectionKey::Root,
                settings.as_ref(),
                Some(inline_settings),
                Utf8Path::new("project/file.js"),
            );
            let handle = SettingsHandle::new(settings.as_ref(), SettingsEditorState::new(query));
            let options = handle.parse_options::<JsLanguage>(
                &BiomePath::new("project/file.js"),
                &DocumentFileSource::Js(JsFileSource::js_module()),
            );
            resolve_js_format_options(
                &BiomePath::new("project/file.js"),
                &DocumentFileSource::Js(JsFileSource::js_module()),
                &handle,
                &db,
            );
            resolve_js_analyzer_options(
                &BiomePath::new("project/file.js"),
                None,
                &DocumentFileSource::Js(JsFileSource::js_module()),
                None,
                &handle,
                &db,
            );

            assert!(options.parse_class_parameter_decorators);
        }

        assert_eq!(
            ProjectInput::ingredient(&db).entries(db.zalsa()).count(),
            project_count
        );
        let events = take_events(&events);
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "resolved_js_format_options", &events,),
            0
        );
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "resolved_js_analyzer_options", &events,),
            0
        );
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
            updated_project
                .root_settings(&db)
                .as_ref()
                .vcs_settings
                .client_kind,
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
                    let settings = SettingsIdentity::from(Arc::new(Settings::default()));
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
                            Arc::new(Settings::default()).into(),
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

        assert!(
            !project.root_settings(&db).as_ref().vcs_settings.is_ignored(
                ignored_path.as_path(),
                false,
                None
            )
        );

        db.store_nested_ignore_patterns_with_mode(
            project_key,
            vec![(nested_path, vec!["generated.js".to_string()])],
            ProjectUpdateMode::Setters,
        )
        .unwrap();

        let updated_project = assert_single_project_in_sync(&db, project_key);
        assert_eq!(updated_project.as_id(), project.as_id());
        assert!(
            updated_project
                .root_settings(&db)
                .as_ref()
                .vcs_settings
                .is_ignored(ignored_path.as_path(), false, None)
        );
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
