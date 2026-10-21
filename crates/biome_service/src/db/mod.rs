//! This is the database used inside the biome Workspace, mainly the `biome_service` crate.
//!

mod parse;
mod state;

pub(crate) use state::DbReadGuard;
pub use state::DbState;

use crate::WorkspaceError;
use crate::projects::{NestedPath, ProjectDb, ProjectInput, ProjectKey};
use crate::settings::{Settings, SettingsIdentity, VcsIgnoredPatterns};
pub(crate) use crate::workspace::views::FileView;
use crate::workspace::views::{ClientId, Views};
use biome_configuration::Configuration;
use biome_configuration::vcs::VcsClientKind;
use biome_db::Source;
use biome_languages::DocumentFileSource;
use biome_languages::LanguageDb;
#[cfg(feature = "module_graph")]
use biome_module_graph::{
    InferredLocalTypeId, InferredModuleKey, ModuleDb, ModuleGraphGeneration, ModuleInfo,
    ModuleInfoKind, TypeDb, module_for_key,
};
use biome_resolver::FsWithResolverProxy;
#[cfg(feature = "module_graph")]
use biome_resolver::{
    ResolverDb, ResolverFsProxy, ResolverPathChange, ResolverPathChanges, ResolverPaths,
    resolver_paths_need_sync, sync_resolver_paths,
};
use biome_rowan::NodeCache;
#[cfg(feature = "module_graph")]
use biome_rowan::Text;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::{Compute, HashMap, Operation};
use parking_lot::Mutex;
use salsa::{Setter, Storage};
use std::convert::Infallible;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{debug, instrument};

const SETTINGS_QUERY_CACHE_CAPACITY: usize = 256;

type SourceViews = Views<Source, Source>;

type NodeCaches = Arc<HashMap<Utf8PathBuf, Arc<Mutex<NodeCache>>>>;
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

/// The views of the content of a file. See [views](crate::workspace::views).
///
/// A query that read the views runs again when they change through a setter.
#[salsa::input]
#[derive(Debug)]
pub(crate) struct FileViews {
    #[returns(ref)]
    views: SourceViews,
}

/// The project of the texts outside every project, whose settings are the
/// default ones. Each Salsa storage creates it once.
#[salsa::input(singleton)]
pub(crate) struct DefaultProject {
    #[returns(copy)]
    project: ProjectInput,
}

/// Changes when the map of projects gains, loses, or replaces a project.
///
/// The map is read without Salsa tracking it, so a query that looks up the
/// project of a path reads the generation to run again when the lookup can
/// find another project. Each Salsa storage creates it once.
#[salsa::input(singleton)]
pub(crate) struct ProjectsGeneration {
    #[returns(copy)]
    value: u64,
}

/// The database used by the `biome_service` crate.
///
/// All data stored in the database must be clonable and must support [Sync] and [Send].
#[salsa::db]
#[derive(Clone)]
pub struct WorkspaceDb {
    /// It maps a file path to the views of its content.
    ///
    /// Salsa doesn't track this map. In Owned mode, outside a scanner epoch, a
    /// path keeps its [FileViews] input while it has a view, and its views
    /// change through a setter. Adding the first view of a path, removing its
    /// last one, and the updates of a scanner epoch change the map instead:
    /// of the tracked queries, only the ones of the resolver run again,
    /// because the database reports these changes to it.
    files: Arc<HashMap<Utf8PathBuf, FileViews>>,
    /// Node caches kept between the parses of a path. See
    /// [WorkspaceDbData::retain_node_cache].
    node_caches: NodeCaches,
    /// It maps a file path to its module graph representation
    #[cfg(feature = "module_graph")]
    pub modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,
    /// The path info observed by the resolver.
    #[cfg(feature = "module_graph")]
    resolver_paths: ResolverPaths,
    /// The filesystem used to read the path info of new paths.
    ///
    /// Only the resolver reads it, through [ResolverDb::resolver_fs].
    #[cfg(feature = "module_graph")]
    resolver_fs: Arc<dyn FsWithResolverProxy>,
    /// It stores the file sources across projects.
    file_sources: Arc<boxcar::Vec<DocumentFileSource>>,
    /// The inline configurations that clients apply on top of the settings of
    /// the texts they publish.
    inline_configurations: Arc<boxcar::Vec<Configuration>>,
    /// A map of projects loaded in the workspace.
    projects: Arc<HashMap<ProjectKey, ProjectInput>>,
    settings_queries: Arc<SettingsQueryCache>,
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
    files: Arc<HashMap<Utf8PathBuf, FileViews>>,
    node_caches: NodeCaches,
    #[cfg(feature = "module_graph")]
    modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,
    #[cfg(feature = "module_graph")]
    resolver_paths: ResolverPaths,
    file_sources: Arc<boxcar::Vec<DocumentFileSource>>,
    inline_configurations: Arc<boxcar::Vec<Configuration>>,
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

    /// Inserts an inline configuration so that it can be retrieved by index
    /// later, and returns its index.
    pub(crate) fn insert_inline_configuration(&self, configuration: Configuration) -> usize {
        self.inline_configurations
            .iter()
            .find(|(_, inserted)| **inserted == configuration)
            .map_or_else(
                || self.inline_configurations.push(configuration),
                |(index, _)| index,
            )
    }

    pub(crate) fn remove_project(&self, project_key: ProjectKey) {
        self.projects.pin().remove(&project_key);
    }

    /// Removes the parsed source cached for `path`.
    pub fn remove_file(&self, path: &Utf8Path) {
        self.files.pin().remove(path);
    }

    /// Keeps a node cache for `path`, which the parses of its text then
    /// share.
    ///
    /// A node cache lets a parse reuse the syntax nodes of the previous parse,
    /// which makes reparsing a modified text faster. It's worth it for
    /// documents that are expected to change, such as the ones open in an
    /// editor. Sharing a cache doesn't change the parse result.
    pub fn retain_node_cache(&self, path: &Utf8Path) {
        self.node_caches
            .pin()
            .get_or_insert_with(path.to_path_buf(), Arc::default);
    }

    /// Drops the node cache of `path`, if there is one.
    pub fn remove_node_cache(&self, path: &Utf8Path) {
        self.node_caches.pin().remove(path);
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

    /// Returns whether the resolver reads the content of `path`.
    ///
    /// This is the case for the manifests the resolver has read, such as
    /// package manifests and TypeScript configurations, including the
    /// configurations they reference.
    #[cfg(feature = "module_graph")]
    pub fn is_resolver_manifest(&self, path: &Utf8Path) -> bool {
        self.resolver_paths.is_manifest(path)
    }

    /// Returns the change to report when the filesystem reports that `path`
    /// was created or modified.
    #[cfg(feature = "module_graph")]
    pub(crate) fn resolver_changes_for_modified_path(
        &self,
        path: &Utf8Path,
    ) -> ResolverPathChanges {
        self.resolver_paths.changes_for_modified_path(path)
    }

    /// Returns the resolver changes to report for the known paths inside the
    /// `node_modules` directories that `directory` can resolve packages from.
    ///
    /// These are the `node_modules` directories of `directory`, of its
    /// ancestors, and of its descendants.
    #[cfg(feature = "module_graph")]
    pub(crate) fn resolver_changes_in_visible_node_modules(
        &self,
        directory: &Utf8Path,
    ) -> Vec<(Utf8PathBuf, ResolverPathChanges)> {
        self.resolver_paths
            .paths_matching(|path| {
                // The package owning the outermost `node_modules` directory of
                // the path.
                path.ancestors()
                    .filter(|ancestor| ancestor.file_name() == Some("node_modules"))
                    .last()
                    .and_then(Utf8Path::parent)
                    .is_some_and(|package_path| {
                        directory.starts_with(package_path) || package_path.starts_with(directory)
                    })
            })
            .into_iter()
            .map(|path| {
                let changes = self.resolver_paths.changes_for_modified_path(&path);
                (path, changes)
            })
            .collect()
    }

    /// Returns the resolver change to report when the parsed source of `path`
    /// is published or removed, if the resolver reads its content.
    #[cfg(feature = "module_graph")]
    pub(crate) fn resolver_source_change(
        &self,
        path: &Utf8Path,
    ) -> Option<(Utf8PathBuf, ResolverPathChanges)> {
        self.is_resolver_manifest(path)
            .then(|| (path.to_path_buf(), ResolverPathChange::Content.into()))
    }

    /// Returns the resolver changes to report for the known paths equal to or
    /// inside `path`.
    #[cfg(feature = "module_graph")]
    pub(crate) fn resolver_changes_within(
        &self,
        path: &Utf8Path,
    ) -> Vec<(Utf8PathBuf, ResolverPathChanges)> {
        self.resolver_paths
            .paths_within(path)
            .into_iter()
            .map(|path| {
                let changes = self.resolver_paths.changes_for_modified_path(&path);
                (path, changes)
            })
            .collect()
    }

    #[cfg(feature = "module_graph")]
    pub fn remove_module(&self, path: &Utf8Path) {
        self.modules.pin().remove(path);
    }

    /// Removes all files and modules that start with the given path. That's
    /// usually used when removing a library or a folder from the project.
    pub fn unload_path(&self, path: &Utf8Path) {
        let files = self.files.pin();
        let to_remove: Vec<Utf8PathBuf> = files
            .keys()
            .filter(|p| p.starts_with(path))
            .cloned()
            .collect();
        for p in to_remove {
            files.remove(&p);
        }
        self.node_caches.pin().retain(|p, _| !p.starts_with(path));
        self.unload_modules(path);
    }

    /// Removes the modules within `path`.
    pub(crate) fn unload_modules(&self, path: &Utf8Path) {
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
    /// Creates a database whose resolver reads path info from `fs`.
    pub fn new(fs: Arc<dyn FsWithResolverProxy>) -> Self {
        Self::with_storage(fs, Storage::default(), Arc::default())
    }

    fn with_storage(
        #[cfg_attr(not(feature = "module_graph"), expect(unused_variables))] fs: Arc<
            dyn FsWithResolverProxy,
        >,
        storage: Storage<Self>,
        settings_queries: Arc<SettingsQueryCache>,
    ) -> Self {
        let db = Self {
            files: Arc::default(),
            node_caches: Arc::default(),
            #[cfg(feature = "module_graph")]
            modules: Arc::default(),
            #[cfg(feature = "module_graph")]
            resolver_paths: ResolverPaths::default(),
            #[cfg(feature = "module_graph")]
            resolver_fs: fs,
            file_sources: Arc::default(),
            inline_configurations: Arc::default(),
            projects: Arc::default(),
            settings_queries,
            storage,
        };
        #[cfg(feature = "module_graph")]
        ModuleGraphGeneration::new(&db, 0);
        let default_project = ProjectInput::new(
            &db,
            ProjectKey::new(),
            Utf8PathBuf::new(),
            Arc::new(Settings::default()).into(),
            Default::default(),
        );
        DefaultProject::new(&db, default_project);
        ProjectsGeneration::new(&db, 0);
        db
    }

    /// Advances the [ProjectsGeneration], so that the queries that looked up
    /// the project of a path run again.
    ///
    /// Use this from the Owned-mode database inside `OwnedDb::with_setter`.
    pub(crate) fn invalidate_projects(&mut self) {
        if let Some(generation) = ProjectsGeneration::try_get(self) {
            let next = generation.value(self).wrapping_add(1);
            generation.set_value(self).to(next);
        }
    }

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

    /// Advances the module graph generation without changing its entries.
    ///
    /// This publishes module changes already made through shared collections to
    /// Salsa-tracked queries.
    #[cfg(feature = "module_graph")]
    pub(crate) fn invalidate_module_graph(&mut self) {
        self.write_module_data(|_| {});
    }

    /// Refreshes the resolver path info of the given paths from the filesystem.
    ///
    /// Only paths the resolver has already looked up are refreshed, and only
    /// the path info that changed is updated.
    #[cfg(feature = "module_graph")]
    pub fn sync_resolver_paths(
        &mut self,
        changes: impl IntoIterator<Item = (Utf8PathBuf, ResolverPathChanges)>,
    ) {
        sync_resolver_paths(self, changes);
    }

    /// Returns whether [Self::sync_resolver_paths] would update any path info.
    #[cfg(feature = "module_graph")]
    pub fn resolver_paths_need_sync(&self, changes: &[(Utf8PathBuf, ResolverPathChanges)]) -> bool {
        resolver_paths_need_sync(
            self,
            changes
                .iter()
                .map(|(path, change)| (path.as_path(), *change)),
        )
    }

    /// Returns handles to the collections that this database shares with all
    /// its clones.
    pub fn data(&self) -> WorkspaceDbData {
        WorkspaceDbData {
            files: self.files.clone(),
            node_caches: self.node_caches.clone(),
            #[cfg(feature = "module_graph")]
            modules: self.modules.clone(),
            #[cfg(feature = "module_graph")]
            resolver_paths: self.resolver_paths.clone(),
            file_sources: self.file_sources.clone(),
            inline_configurations: self.inline_configurations.clone(),
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

    /// Returns a previously inserted inline configuration by index. See
    /// [WorkspaceDbData::insert_inline_configuration].
    pub(crate) fn inline_configuration(&self, index: usize) -> Option<&Configuration> {
        self.inline_configurations.get(index)
    }

    /// Publishes `file` as the disk view of `path`.
    pub fn insert_file(&mut self, path: &Utf8Path, file: Source) {
        let views = FileViews::new(self, Views::from_disk(file));
        self.files.pin().insert(path.to_path_buf(), views);
    }

    /// Publishes `text` as the disk view of `path`, parsed as `file_source`,
    /// with a new parsed source.
    pub fn replace_file(
        &mut self,
        path: &Utf8Path,
        text: impl Into<Arc<str>>,
        file_source: DocumentFileSource,
    ) -> Source {
        self.update_or_insert_view(
            path,
            FileView::Disk,
            text.into(),
            file_source,
            None,
            ParsedSourceUpdateMode::Replace,
        )
    }

    /// Publishes `text` as the disk view of `path`, parsed as `file_source`,
    /// updating its parsed source through Salsa setters.
    pub fn upsert_file(
        &mut self,
        path: &Utf8Path,
        text: impl Into<Arc<str>>,
        file_source: DocumentFileSource,
    ) -> Source {
        self.update_or_insert_view(
            path,
            FileView::Disk,
            text.into(),
            file_source,
            None,
            ParsedSourceUpdateMode::Setters,
        )
    }

    /// Publishes `text`, parsed as `file_source` with the
    /// `inline_configuration` of the client, as `view` of `path`, and returns
    /// the current view of `path`.
    ///
    /// The text isn't parsed here: the parse runs the first time a query reads
    /// it.
    pub(crate) fn update_or_insert_view(
        &mut self,
        path: &Utf8Path,
        view: FileView,
        text: Arc<str>,
        file_source: DocumentFileSource,
        inline_configuration: Option<Configuration>,
        mode: ParsedSourceUpdateMode,
    ) -> Source {
        let file_source_index = self.insert_source(file_source);
        let configuration_index = inline_configuration
            .map(|configuration| self.data().insert_inline_configuration(configuration));
        let new_source = |db: &Self, text| {
            Source::builder(path.to_path_buf(), text, file_source_index)
                .configuration_index(configuration_index)
                .new(db)
        };
        if mode == ParsedSourceUpdateMode::Replace {
            let source = new_source(self, text);
            let views = self.replace_views(path, |views| {
                Some(views.map_or(Views::from_view(view, source), |views| {
                    views.with(view, source)
                }))
            });
            return views.map_or(source, |views| *views.current().value());
        }

        let existing_views = { self.files.pin().get(path).copied() };
        if let Some(file_views) = existing_views {
            let views = file_views.views(self).clone();
            let views = match views.get(view).copied() {
                Some(source) => {
                    source.set_text(self).to(text);
                    if source.file_source_index(self) != file_source_index {
                        source.set_file_source_index(self).to(file_source_index);
                    }
                    if source.configuration_index(self) != configuration_index {
                        source.set_configuration_index(self).to(configuration_index);
                    }
                    views
                }
                None => {
                    let source = new_source(self, text);
                    let views = views.with(view, source);
                    file_views.set_views(self).to(views.clone());
                    views
                }
            };
            *views.current().value()
        } else {
            let source = new_source(self, text);
            let file_views = FileViews::new(self, Views::from_view(view, source));
            self.files.pin().insert(path.to_path_buf(), file_views);
            // Manifest queries look up the parsed source of a path outside
            // Salsa. Publishing a new parsed source must invalidate them.
            #[cfg(feature = "module_graph")]
            {
                let change = self.data().resolver_source_change(path);
                self.sync_resolver_paths(change);
            }
            source
        }
    }

    /// Removes `view` of `path`. Removes the file when it has no view left.
    pub(crate) fn remove_view(
        &mut self,
        path: &Utf8Path,
        view: FileView,
        mode: ParsedSourceUpdateMode,
    ) {
        self.update_views(path, mode, |views| views.without(view));
    }

    /// Removes the editor view of `client` of `path`. When no client has the
    /// file open anymore, the file is removed too, unless `keep_disk_view`.
    pub(crate) fn remove_editor_view(
        &mut self,
        path: &Utf8Path,
        client: ClientId,
        keep_disk_view: bool,
        mode: ParsedSourceUpdateMode,
    ) {
        self.update_views(path, mode, |views| {
            views
                .without_editor(client)
                .filter(|views| keep_disk_view || views.has_editors())
        });
    }

    /// Replaces the views of `path` with the ones `update` derives from them,
    /// and removes the file when `update` returns `None`. Does nothing when
    /// `path` has no views.
    fn update_views(
        &mut self,
        path: &Utf8Path,
        mode: ParsedSourceUpdateMode,
        update: impl Fn(SourceViews) -> Option<SourceViews>,
    ) {
        if mode == ParsedSourceUpdateMode::Replace {
            self.replace_views(path, |views| views.and_then(&update));
            return;
        }

        let Some(file_views) = self.files.pin().get(path).copied() else {
            return;
        };
        match update(file_views.views(self).clone()) {
            Some(views) => {
                file_views.set_views(self).to(views);
            }
            None => self.remove_file(path),
        }
    }

    /// Removes the disk views of the files within `path`. The files that a
    /// client has open keep their editor views.
    pub(crate) fn remove_disk_views_within(
        &mut self,
        path: &Utf8Path,
        mode: ParsedSourceUpdateMode,
    ) {
        let paths: Vec<_> = self
            .files
            .pin()
            .keys()
            .filter(|file_path| file_path.starts_with(path))
            .cloned()
            .collect();
        for file_path in paths {
            self.remove_view(&file_path, FileView::Disk, mode);
        }
    }

    /// Replaces the views of `path` with the ones `replace` derives from the
    /// current ones, and removes the file when `replace` returns `None`.
    /// Returns the views that were published.
    ///
    /// The change is published only while the map still holds the views it
    /// was derived from. Otherwise, it's derived again from the latest views.
    fn replace_views(
        &self,
        path: &Utf8Path,
        replace: impl Fn(Option<SourceViews>) -> Option<SourceViews>,
    ) -> Option<SourceViews> {
        let files = self.files.pin();
        let mut current = files.get(path).copied();
        loop {
            let views = replace(current.map(|file_views| file_views.views(self).clone()));
            // Input allocation stays outside Papaya's callback because that
            // callback may be replayed during a compare-and-swap retry.
            let replacement = views.clone().map(|views| FileViews::new(self, views));
            let result = files.compute(path.to_path_buf(), |entry| {
                let actual = entry.map(|(_, file_views)| *file_views);
                match (actual, replacement) {
                    (actual, _) if actual != current => Operation::Abort(actual),
                    (_, Some(replacement)) => Operation::Insert(replacement),
                    (Some(_), None) => Operation::Remove,
                    (None, None) => Operation::Abort(None),
                }
            });
            match result {
                Compute::Aborted(actual) if actual != current => current = actual,
                _ => return views,
            }
        }
    }

    /// Removes the parsed source cached for `path`.
    ///
    /// Use this from the Owned-mode database inside `OwnedDb::with_setter`,
    /// because the resolver is notified through Salsa setters.
    pub fn remove_file(&mut self, path: &Utf8Path) {
        if self.files.pin().remove(path).is_some() {
            #[cfg(feature = "module_graph")]
            {
                let change = self.data().resolver_source_change(path);
                self.sync_resolver_paths(change);
            }
        }
    }

    #[cfg(feature = "module_graph")]
    pub fn get_module(&self, path: &Utf8Path) -> Option<ModuleInfo> {
        self.modules.pin().get(path).copied()
    }

    /// Returns the view of `path` that the services shared by the clients
    /// read: the editor view of the client that opened the file first, or the
    /// disk view when no client has it open.
    pub fn get_parsed_source(&self, path: &Utf8Path) -> Option<Source> {
        let file_views = self.files.pin().get(path).copied()?;
        Some(*file_views.views(self).current().value())
    }

    /// Returns the paths of the files that `client` has open.
    pub(crate) fn paths_open_in(&self, client: ClientId) -> Vec<Utf8PathBuf> {
        self.files
            .pin()
            .iter()
            .filter(|(_, file_views)| file_views.views(self).editor(client).is_some())
            .map(|(path, _)| path.clone())
            .collect()
    }

    /// Returns the view of `path` that `client` reads. See
    /// [Views::for_client].
    pub(crate) fn get_client_parsed_source(
        &self,
        path: &Utf8Path,
        client: ClientId,
    ) -> Option<Source> {
        let file_views = self.files.pin().get(path).copied()?;
        file_views
            .views(self)
            .for_client(client)
            .map(|view| *view.value())
    }

    /// Returns the number of cached parsed sources.
    ///
    /// Only used by tests to check that eviction empties the collection,
    /// rather than only the specific paths a test looks up.
    #[cfg(test)]
    pub(crate) fn parsed_sources_len(&self) -> usize {
        self.files.pin().len()
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
        {
            let files = self.files.pin();
            let to_remove: Vec<Utf8PathBuf> = files
                .keys()
                .filter(|p| p.starts_with(path))
                .cloned()
                .collect();
            for p in to_remove {
                files.remove(&p);
            }
        }
        self.node_caches.pin().retain(|p, _| !p.starts_with(path));
        self.unload_modules(path);
    }

    /// Removes the modules within `path`.
    pub(crate) fn unload_modules(&mut self, path: &Utf8Path) {
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

            let changes = self.data().resolver_changes_within(path);
            self.sync_resolver_paths(changes);
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
    files: Arc<HashMap<Utf8PathBuf, FileViews>>,
    node_caches: NodeCaches,
    #[cfg(feature = "module_graph")]
    modules: Arc<HashMap<Utf8PathBuf, ModuleInfo>>,
    #[cfg(feature = "module_graph")]
    resolver_paths: ResolverPaths,
    #[cfg(feature = "module_graph")]
    resolver_fs: Arc<dyn FsWithResolverProxy>,
    file_sources: Arc<boxcar::Vec<DocumentFileSource>>,
    inline_configurations: Arc<boxcar::Vec<Configuration>>,
    projects: Arc<HashMap<ProjectKey, ProjectInput>>,
    settings_queries: Arc<SettingsQueryCache>,
    storage: salsa::StorageHandle<WorkspaceDb>,
}

impl SharedWorkspaceDb {
    /// Converts `db` into handles for creating operation-local forks.
    ///
    /// The resulting forks share Salsa storage and workspace collections with
    /// `db`, but each fork has fresh Salsa local state.
    /// Creates handles to a new database whose resolver reads path info from
    /// `fs`.
    pub fn new(fs: Arc<dyn FsWithResolverProxy>) -> Self {
        Self::from_workspace_db(WorkspaceDb::new(fs))
    }

    pub(crate) fn from_workspace_db(db: WorkspaceDb) -> Self {
        let WorkspaceDb {
            files,
            node_caches,
            #[cfg(feature = "module_graph")]
            modules,
            #[cfg(feature = "module_graph")]
            resolver_paths,
            #[cfg(feature = "module_graph")]
            resolver_fs,
            file_sources,
            inline_configurations,
            storage,
            projects,
            settings_queries,
        } = db;
        Self {
            files,
            node_caches,
            #[cfg(feature = "module_graph")]
            modules,
            #[cfg(feature = "module_graph")]
            resolver_paths,
            #[cfg(feature = "module_graph")]
            resolver_fs,
            file_sources,
            inline_configurations,
            projects,
            settings_queries,
            storage: storage.into_zalsa_handle(),
        }
    }

    pub fn data(&self) -> WorkspaceDbData {
        WorkspaceDbData {
            files: self.files.clone(),
            node_caches: self.node_caches.clone(),
            #[cfg(feature = "module_graph")]
            modules: self.modules.clone(),
            #[cfg(feature = "module_graph")]
            resolver_paths: self.resolver_paths.clone(),
            file_sources: self.file_sources.clone(),
            inline_configurations: self.inline_configurations.clone(),
            projects: self.projects.clone(),
        }
    }

    pub fn fork(&self) -> WorkspaceDb {
        WorkspaceDb {
            files: self.files.clone(),
            node_caches: self.node_caches.clone(),
            file_sources: self.file_sources.clone(),
            inline_configurations: self.inline_configurations.clone(),
            #[cfg(feature = "module_graph")]
            modules: self.modules.clone(),
            #[cfg(feature = "module_graph")]
            resolver_paths: self.resolver_paths.clone(),
            #[cfg(feature = "module_graph")]
            resolver_fs: self.resolver_fs.clone(),
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
    fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<Source> {
        self.get_parsed_source(path)
    }
}

#[cfg(feature = "module_graph")]
#[salsa::db]
impl ResolverDb for WorkspaceDb {
    fn resolver_fs(&self) -> &dyn ResolverFsProxy {
        self.resolver_fs.as_ref()
    }

    fn resolver_paths(&self) -> &ResolverPaths {
        &self.resolver_paths
    }
}

#[salsa::db]
impl ProjectDb for WorkspaceDb {
    fn get_project(&self, project_key: &ProjectKey) -> Option<ProjectInput> {
        self.projects.pin().get(project_key).copied()
    }

    fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey> {
        self.projects
            .pin()
            .iter()
            .filter(|(_, project_data)| path.starts_with(project_data.path(self)))
            .max_by_key(|(_, project_data)| project_data.path(self).components().count())
            .map(|(key, _)| *key)
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
    use biome_db::testing::{Events, function_query_will_execute_count_by_name};
    #[cfg(any(feature = "lang_js", feature = "module_graph"))]
    use biome_fs::BiomePath;
    use biome_fs::MemoryFileSystem;
    #[cfg(feature = "lang_js")]
    use biome_js_syntax::JsLanguage;
    #[cfg(any(feature = "module_graph", feature = "html_embeds"))]
    use biome_languages::HtmlFileSource;
    #[cfg(feature = "lang_js")]
    use biome_languages::JsFileSource;
    #[cfg(feature = "module_graph")]
    use biome_module_graph::{ModuleDb, resolve_html_module};
    #[cfg(feature = "lang_js")]
    use biome_package::{Dependencies, PackageJson};
    use biome_parser::db::{ParseDb, ParsedSourceExt};
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
        #[returns(copy)]
        project: ProjectInput,
        selection: SettingsQuerySelection,
    }

    #[salsa::interned]
    struct TestSettingsCacheInput {
        #[returns(copy)]
        value: usize,
    }

    #[salsa::tracked(returns(copy), lru = 256)]
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

    fn js() -> DocumentFileSource {
        DocumentFileSource::Js(biome_languages::JsFileSource::js_module())
    }

    fn test_fs() -> Arc<dyn FsWithResolverProxy> {
        Arc::new(MemoryFileSystem::default())
    }

    fn test_db() -> WorkspaceDb {
        WorkspaceDb::new(test_fs())
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
        let db = WorkspaceDb::with_storage(test_fs(), storage, settings_queries);
        (db, events)
    }

    #[test]
    fn settings_query_cache_rotates_storage_at_capacity() {
        let db = test_db();
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

    /// Returns the project inputs that Salsa allocated, except the default
    /// project of the database.
    fn salsa_projects(db: &WorkspaceDb) -> Vec<ProjectInput> {
        let default_project = DefaultProject::get(db).project(db);
        ProjectInput::ingredient(db)
            .entries(db.zalsa())
            .map(|entry| entry.as_struct())
            .filter(|project| project.as_id() != default_project.as_id())
            .collect()
    }

    fn assert_single_project_in_sync(db: &WorkspaceDb, project_key: ProjectKey) -> ProjectInput {
        assert_eq!(db.projects.pin().len(), 1);
        let indexed_project = db.get_project(&project_key).unwrap();

        let salsa_projects = salsa_projects(db);
        assert_eq!(salsa_projects.len(), 1);
        assert_eq!(indexed_project.as_id(), salsa_projects[0].as_id());

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
        WorkspaceDb::with_storage(test_fs(), storage, Arc::default())
    }

    #[cfg(feature = "module_graph")]
    fn test_module(db: &mut WorkspaceDb, path: &str) -> ModuleInfo {
        let path = BiomePath::new(path);
        db.replace_file(
            path.as_path(),
            "",
            DocumentFileSource::Html(HtmlFileSource::html()),
        );
        let (module, _, _) =
            resolve_html_module(db, &path).expect("the parsed HTML source was just inserted");
        ModuleInfo::new(
            db,
            path.as_path().to_path_buf(),
            ModuleInfoKind::Html(module),
        )
    }

    #[salsa::tracked(returns(copy))]
    fn blocking_document_source_index(db: &dyn ParseDb, file: Source) -> usize {
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
        let mut db = test_db();
        let path = Utf8Path::new("test.js");

        let file = db.upsert_file(path, "let a = 1;", js());
        let updated_file = db.upsert_file(path, "let b = 2;", js());

        assert_eq!(file.as_id(), updated_file.as_id());
        assert_eq!(db.get_parsed_source(path).unwrap().as_id(), file.as_id());
    }

    #[cfg(feature = "lang_js")]
    #[test]
    fn published_text_is_parsed_once_when_read() {
        let (mut db, events) = settings_query_test_db();
        let path = Utf8Path::new("/test.js");
        let file = db.update_or_insert_view(
            path,
            FileView::Disk,
            "let a = 1;".into(),
            js(),
            None,
            ParsedSourceUpdateMode::Setters,
        );
        let parse_count = |events: &Events| {
            function_query_will_execute_count_by_name(&db, "parsed_root", &take_events(events))
        };
        assert_eq!(parse_count(&events), 0);

        let root = file.parsed(&db).syntax::<JsLanguage>();
        assert_eq!(root.text_with_trivia().to_string(), "let a = 1;");
        assert_eq!(parse_count(&events), 1);

        file.parsed(&db);
        assert_eq!(parse_count(&events), 0);
    }

    #[cfg(feature = "html_embeds")]
    #[test]
    fn updated_text_keeps_the_identity_of_its_snippets() {
        let mut db = test_db();
        let path = Utf8Path::new("/test.html");
        let html = DocumentFileSource::Html(HtmlFileSource::html());
        let file = db.update_or_insert_view(
            path,
            FileView::Disk,
            "<script>let a = 1;</script>".into(),
            html,
            None,
            ParsedSourceUpdateMode::Setters,
        );
        let snippet = file.snippets(&db)[0].as_id();

        db.update_or_insert_view(
            path,
            FileView::Disk,
            "<script>let b = 2;</script>".into(),
            html,
            None,
            ParsedSourceUpdateMode::Setters,
        );

        let snippets = file.snippets(&db);
        assert_eq!(snippets.len(), 1);
        assert_eq!(snippets[0].as_id(), snippet);
        let script = snippets[0]
            .parsed(&db)
            .clone()
            .embedded_syntax::<JsLanguage>();
        assert_eq!(script.node.text_with_trivia().to_string(), "let b = 2;");
    }

    /// A replacement derived from views that a concurrent update removed is
    /// derived again from the latest views, so it can't bring back the
    /// removed editor view.
    #[test]
    fn replaced_views_are_derived_from_the_latest_views() {
        let mut db = test_db();
        let path = Utf8Path::new("test.js");
        db.update_or_insert_view(
            path,
            FileView::Disk,
            "disk".into(),
            js(),
            None,
            ParsedSourceUpdateMode::Replace,
        );
        db.update_or_insert_view(
            path,
            FileView::Editor(ClientId::default()),
            "editor".into(),
            js(),
            None,
            ParsedSourceUpdateMode::Replace,
        );
        let disk = Source::new(&db, path.to_path_buf(), "new_disk".into(), 0);

        let derived_views = std::cell::Cell::new(0);
        let views = db.replace_views(path, |views| {
            if derived_views.replace(derived_views.get() + 1) == 0 {
                // The client closes the file after its views were read.
                db.data().remove_file(path);
            }
            Some(views.map_or(Views::from_disk(disk), |views| {
                views.with(FileView::Disk, disk)
            }))
        });

        assert_eq!(derived_views.get(), 2);
        assert_eq!(views, Some(Views::from_disk(disk)));
        assert_eq!(db.get_parsed_source(path), Some(disk));
    }

    #[test]
    fn replace_file_replaces_existing_input() {
        let mut db = test_db();
        let path = Utf8Path::new("test.js");

        let file = db.replace_file(path, "let a = 1;", js());
        let updated_file = db.replace_file(path, "let b = 2;", js());

        assert_ne!(file.as_id(), updated_file.as_id());
        assert_eq!(
            db.get_parsed_source(path).unwrap().as_id(),
            updated_file.as_id()
        );
    }

    #[test]
    fn insert_project_keeps_index_and_salsa_in_sync() {
        let mut db = test_db();
        let path = Utf8PathBuf::from("project");

        let project_key = db.insert_project(path.clone());
        assert_eq!(db.insert_project(path.clone()), project_key);

        let project = assert_single_project_in_sync(&db, project_key);
        assert_eq!(project.path(&db), path.as_path());
    }

    #[test]
    fn remove_project_removes_project_from_index() {
        let mut db = test_db();
        let project_key = db.insert_project(Utf8PathBuf::from("project"));
        let project = assert_single_project_in_sync(&db, project_key);

        db.remove_project(project_key);

        assert!(db.projects.pin().is_empty());
        assert!(db.get_project(&project_key).is_none());

        let salsa_projects = salsa_projects(&db);
        assert_eq!(salsa_projects.len(), 1);
        assert_eq!(salsa_projects[0].as_id(), project.as_id());
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

        db.upsert_file(Utf8Path::new("project/file.js"), "let a = 1;", js());
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

        db.upsert_file(Utf8Path::new("project/file.js"), "let a = 1;", js());
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

        db.upsert_file(Utf8Path::new("project/file.js"), "let a = 1;", js());
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

        db.upsert_file(Utf8Path::new("project/file.js"), "let a = 1;", js());
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
        let mut db = test_db();
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
        let mut db = test_db();
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
        let mut db = test_db();
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
        let mut db = test_db();
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
        let shared = SharedWorkspaceDb::new(test_fs());
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
        let shared = SharedWorkspaceDb::new(test_fs());
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
        let mut db = test_db();
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
        let mut db = test_db();
        let path = Utf8PathBuf::from("test.js");
        let file = db.upsert_file(&path, "let a = 1;", js());
        let (writer_finished_tx, writer_finished_rx) = mpsc::channel();

        std::thread::scope(|scope| {
            let reader_db = db.clone();
            let reader = scope.spawn(move || {
                salsa::Cancelled::catch(|| blocking_document_source_index(&reader_db, file))
            });

            let writer_path = path.clone();
            scope.spawn(move || {
                SETTER_READER_STARTED.wait();
                db.upsert_file(&writer_path, "let b = 2;", js());
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
        let module = test_module(&mut db, path.as_str());
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
        let module = test_module(&mut db, path.as_str());
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
        let root_module = test_module(&mut db, root.as_str());
        let nested_module = test_module(&mut db, nested.as_str());
        let outside_module = test_module(&mut db, outside.as_str());
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
