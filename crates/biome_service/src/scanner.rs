//! Scanner for crawling the file system and indexing documents within projects.
//!
//! In addition to scanning and indexing, the scanner also has a
//! [`watcher`](watcher::Watcher) that can watch scanned folders to keep the
//! index in sync with the filesystem.
//!
//! ## Scanner behaviour
//!
//! An initial project scan runs as an epoch. Database setters must not interrupt
//! it with [`salsa::Cancelled::PendingWrite`].
//!
//! The workspace starts the scan with
//! [`DbState::scan_session`](crate::db::DbState::scan_session), which runs it
//! in four phases:
//!
//! 1. The ignore-file phase discovers ignore files and publishes their matchers.
//! 2. The setup phase reads configuration files and manifests using those matchers.
//! 3. The files phase indexes again the files that the setup phase deferred,
//!    then the other files and their dependencies.
//! 4. The deferred files phase indexes again the files that the files phase
//!    deferred.
//!
//! When the LSP owns the database, the scanner works on a view that can't run
//! Salsa setters. The view records the changes that need setters, and the
//! session publishes them at the end of each phase. Regular writes wait until
//! the session ends, while reads continue.
//!
//! The view can't change the inputs that a file already has. When the content
//! of such a file changed, the setup and files phases defer the file, and the
//! next phase indexes it again with the published content. The other files
//! read the manifests and the JSON files that the resolver reads, so the files
//! phase indexes the files that the setup phase deferred first, and never
//! defers manifests and JSON files itself. The files that the deferred files
//! phase defers are published when the scan ends, and their next indexing
//! updates their services.
//!
//! Watcher-driven updates run outside the scan. If a setter interrupts file
//! indexing, the scanner retries that file instead of restarting a project
//! scan.
//!
//! A file changed during traversal may leave the completed index temporarily
//! stale. A subsequent watcher update indexes the latest file state. This is
//! an intended limitation, because:
//! - Dependencies rarely change during a scan phase
//! - Files opened by the editor still fire `changeFile`, so they are indexed again.
//!
//! A possible desynchronization is negligible.

mod watcher;
mod workspace_bridges;

#[cfg(test)]
mod test_utils;

use crate::diagnostics::Panic;
use crate::module_graph::ModuleDependencies;
use crate::projects::ProjectKey;
use crate::workspace::{ScanProjectResult, ServiceNotification, WorkspaceError};
use biome_diagnostics::serde::Diagnostic;
use biome_diagnostics::{Diagnostic as _, DiagnosticExt, Error, Severity};
use biome_fs::{
    BiomePath, FileSystem, PathInterner, PathKind, PhysicalFileOwners, TraversalContext,
    TraversalScope, normalize_path,
};
use camino::{Utf8Path, Utf8PathBuf};
use crossbeam::channel::{Receiver, Sender, unbounded};
use papaya::{HashMap, HashSet, HashSetRef, LocalGuard};
use parking_lot::Mutex as ParkingLotMutex;
use rayon::Scope;
use rustc_hash::{FxHashMap, FxHashSet};
use std::hash::RandomState;
use std::mem;
use std::panic::catch_unwind;
use std::sync::Mutex;
#[cfg(not(target_family = "wasm"))]
use std::thread;
use std::time::Duration;
use tracing::instrument;

pub use watcher::{Watcher, WatcherInstruction, WatcherKind, WatcherOptions, watcher_options};
pub(crate) use workspace_bridges::{
    ScannerWatcherBridge, WorkspaceScannerBridge, WorkspaceWatcherBridge,
};

pub(crate) struct ScanOptions {
    /// The kind of scan to perform (targeted, project, etc.).
    pub scan_kind: ScanKind,

    /// Forces scanning of the folder, even if it is already being watched.
    pub force: bool,

    /// Whether diagnostics should be reported for scanned files.
    ///
    /// If `false`, only diagnostics related to Biome configuration files are
    /// reported.
    pub verbose: bool,

    /// Whether scanned directories should be watched for filesystem changes.
    pub watch: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScanFolderKind {
    Project,
    IgnoreFiles,
}

pub(crate) struct ScanResult {
    /// Diagnostics reported while scanning the project.
    ///
    /// If [`ScanOptions::verbose`] is `false`, only diagnostics related to
    /// Biome configuration files are reported.
    pub diagnostics: Vec<Diagnostic>,

    /// Duration of the full scan.
    pub duration: Duration,

    /// List of nested configuration files found inside the project.
    ///
    /// The root configuration is not included in this.
    pub configuration_files: Vec<BiomePath>,
}

/// The reason a traversal considers a path for indexing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexRequestKind {
    /// There is an explicit request for the path to be indexed, due to being
    /// included in the configuration.
    Explicit(IndexTrigger),

    /// The file is being indexed because it is a dependency of one or more
    /// files that were explicitly indexed.
    Dependency(IndexTrigger),

    /// Ignore files are being discovered before a scan uses their matchers.
    IgnoreFileDiscovery,
}

impl IndexRequestKind {
    pub fn trigger(self) -> IndexTrigger {
        match self {
            Self::Explicit(trigger) | Self::Dependency(trigger) => trigger,
            Self::IgnoreFileDiscovery => IndexTrigger::Update,
        }
    }
}

/// Indicates what triggered the index request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexTrigger {
    /// The file is being indexed as part of an initial scanner run.
    InitialScan,

    /// The file is being (re-)indexed as part of an update, either by client
    /// request or a watcher update.
    Update,
}

pub(crate) struct Scanner {
    /// Open projects that have been scanned.
    projects: HashMap<ProjectKey, ScannedProject>,

    /// All the physical folders that are being watched by the watcher.
    watched_folders: HashSet<Utf8PathBuf>,

    /// Projects physical watcher roots back to every logical path alias.
    watch_aliases: ParkingLotMutex<FxHashMap<Utf8PathBuf, FxHashSet<Utf8PathBuf>>>,

    /// The physical target of every symbolic link registered as a logical watch folder.
    ///
    /// Locked only while [`Self::watch_aliases`] is locked. A link that resolves to a new target
    /// was retargeted, so the aliases derived through its previous target are stale.
    watched_symlink_targets: ParkingLotMutex<FxHashMap<Utf8PathBuf, Utf8PathBuf>>,

    /// The logical folders watched for the VCS ignore files of each project.
    ///
    /// These folders can be outside the project folder, so they are released separately when the
    /// project is unloaded.
    vcs_watch_folders: ParkingLotMutex<FxHashMap<ProjectKey, FxHashSet<Utf8PathBuf>>>,

    /// Channel sender for instructions to the [`Watcher`].
    watcher_tx: Sender<WatcherInstruction>,
}

struct ScannedProject {
    /// The folder of the project.
    path: Utf8PathBuf,

    /// The scan kind used for this project.
    scan_kind: ScanKind,

    /// Whether the project is being watched as well.
    watched: bool,
}

impl Scanner {
    /// Creates an instance of the scanner.
    pub fn new(watcher_tx: Sender<WatcherInstruction>) -> Self {
        Self {
            projects: Default::default(),
            watched_folders: Default::default(),
            watch_aliases: Default::default(),
            watched_symlink_targets: Default::default(),
            vcs_watch_folders: Default::default(),
            watcher_tx,
        }
    }

    /// Returns the [`ScanKind`] for the project with the given `project_key`,
    /// if any.
    #[inline]
    pub fn get_scan_kind_for_project(&self, project_key: ProjectKey) -> Option<ScanKind> {
        self.projects
            .pin()
            .get(&project_key)
            .map(|project| project.scan_kind.clone_without_targeting_info())
    }

    /// Runs the setup phase of the scan of the project `project_key`: reads
    /// its configuration files, manifests and ignore files.
    ///
    /// Returns `None` when the project is watched and `scan_options` doesn't
    /// force the scan, because the watcher keeps the index up to date.
    /// Otherwise, publish the changes of `workspace`, then pass the result to
    /// [`Self::finish_project_scan()`], with the paths of the files that the
    /// setup phase deferred.
    pub(crate) fn set_up_project_scan<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        scan_options: ScanOptions,
    ) -> Result<Option<ProjectScan>, WorkspaceError> {
        let project_path = workspace
            .get_project_path(project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        if !scan_options.force && self.is_project_watched(project_key, &project_path) {
            return Ok(None);
        }

        let additional_ignore_roots = self.targeted_vcs_ignore_roots(
            workspace,
            project_key,
            &project_path,
            &scan_options.scan_kind,
        );
        let setup = self.set_up_scan(
            workspace,
            project_key,
            &project_path,
            IndexTrigger::InitialScan,
            &scan_options,
            &additional_ignore_roots,
        );
        Ok(Some(ProjectScan {
            project_key,
            project_path,
            scan_options,
            setup,
        }))
    }

    fn targeted_vcs_ignore_roots<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        project_path: &Utf8Path,
        scan_kind: &ScanKind,
    ) -> Vec<Utf8PathBuf> {
        let ScanKind::TargetedKnownFiles { target_paths, .. } = scan_kind else {
            return Vec::new();
        };
        let Some(vcs_root) = workspace.get_vcs_root(project_key) else {
            return Vec::new();
        };

        let mut roots = FxHashSet::default();
        for target_path in target_paths {
            if target_path.starts_with(project_path) || !target_path.starts_with(&vcs_root) {
                continue;
            }
            let Ok(relative) = target_path.as_path().strip_prefix(&vcs_root) else {
                continue;
            };
            let root = relative.components().next().map_or_else(
                || vcs_root.clone(),
                |component| vcs_root.join(component.as_str()),
            );
            if workspace.fs().path_is_dir(&root) {
                roots.insert(root);
            }
        }

        let mut roots: Vec<_> = roots.into_iter().collect();
        roots.sort_unstable();
        roots
    }

    /// Runs the files phase of `scan`: indexes again the files that the setup
    /// phase deferred, at `deferred_paths`, then the other files of the
    /// project and their dependencies.
    ///
    /// If [`ScanOptions::watch`] is `true`, this will install a watcher on the
    /// project folder.
    pub(crate) fn finish_project_scan<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        scan: ProjectScan,
        deferred_paths: Vec<Utf8PathBuf>,
    ) -> ScanProjectResult {
        let ProjectScan {
            project_key,
            project_path,
            scan_options,
            mut setup,
        } = scan;
        setup.deferred_paths = deferred_paths;
        let result = self.scan_files(
            workspace,
            project_key,
            &project_path,
            IndexTrigger::InitialScan,
            &scan_options,
            setup,
        );

        let scanned_project = ScannedProject {
            path: project_path,
            scan_kind: scan_options.scan_kind.clone_without_targeting_info(),
            watched: scan_options.watch,
        };
        self.projects.pin().insert(project_key, scanned_project);

        workspace.notify(ServiceNotification::IndexUpdated);

        ScanProjectResult {
            diagnostics: result.diagnostics,
            duration: result.duration,
            configuration_files: result.configuration_files,
        }
    }

    /// Indexes the folder with given `path` within the project with the given
    /// `project_key`.
    ///
    /// The project must already have been scanned using
    /// [`Self::finish_project_scan()`]. The folder will be scanned with the
    /// same scan kind as the project.
    pub fn index_folder<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        path: &Utf8Path,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let (scan_kind, watch) = self
            .projects
            .pin()
            .get(&project_key)
            .map(|project| {
                (
                    project.scan_kind.clone_without_targeting_info(),
                    project.watched,
                )
            })
            .ok_or_else(WorkspaceError::no_project)?;

        let scan_options = ScanOptions {
            scan_kind,
            force: true,
            verbose: false,
            watch,
        };

        let result = self.scan(
            workspace,
            project_key,
            path,
            IndexTrigger::Update,
            &scan_options,
        );

        // No need to notify after this scan, because the individual file
        // updates will already trigger notifications.
        Ok(result.diagnostics)
    }

    /// Loads ignore matchers before the setup phase of a project scan.
    ///
    /// Returns diagnostics from discovery and the time spent loading the matchers.
    pub(crate) fn prepare_project_ignore_files<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        scan_options: &ScanOptions,
    ) -> Result<(Vec<Diagnostic>, Duration), WorkspaceError> {
        let start = web_time::Instant::now();
        let project_path = workspace
            .get_project_path(project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        if scan_options.scan_kind.is_none()
            || (!scan_options.force && self.is_project_watched(project_key, &project_path))
            || workspace.get_vcs_root(project_key).is_none()
        {
            return Ok((Vec::new(), start.elapsed()));
        }

        if scan_options.watch {
            self.watch_project_vcs_paths(workspace, project_key);
        }

        let additional_ignore_roots = self.targeted_vcs_ignore_roots(
            workspace,
            project_key,
            &project_path,
            &scan_options.scan_kind,
        );
        let mut diagnostics = Vec::new();
        for path in std::iter::once(&project_path).chain(&additional_ignore_roots) {
            let (ignore_paths, discovery_diagnostics) = self.discover_ignore_files(
                workspace,
                project_key,
                path,
                IndexTrigger::InitialScan,
                scan_options,
            );
            diagnostics.extend(discovery_diagnostics);
            // A failed update keeps the last usable matchers of `path`, so the rest of the
            // project is still indexed. The error is reported like any other scan diagnostic.
            if let Err(error) =
                workspace.update_project_ignore_files(project_key, path, &ignore_paths)
            {
                let diagnostic = Diagnostic::new(error);
                if DiagnosticsCollector::new(scan_options.verbose).should_collect(&diagnostic) {
                    diagnostics.push(diagnostic);
                }
            }
        }
        Ok((diagnostics, start.elapsed()))
    }

    /// Refreshes ignore matchers before rescanning a folder after an ignore-file event.
    pub fn refresh_ignore_files<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        path: &Utf8Path,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        if workspace.get_vcs_root(project_key).is_none() {
            return Ok(Vec::new());
        }
        let scan_kind = self
            .get_scan_kind_for_project(project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        let scan_options = ScanOptions {
            scan_kind,
            force: true,
            verbose: false,
            watch: false,
        };
        let (ignore_paths, diagnostics) = self.discover_ignore_files(
            workspace,
            project_key,
            path,
            IndexTrigger::Update,
            &scan_options,
        );
        workspace.update_project_ignore_files(project_key, path, &ignore_paths)?;
        Ok(diagnostics)
    }

    fn discover_ignore_files<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        path: &Utf8Path,
        trigger: IndexTrigger,
        scan_options: &ScanOptions,
    ) -> (Vec<BiomePath>, Vec<Diagnostic>) {
        let (mut ignore_paths, diagnostics) = self.run_scan_phase(
            workspace,
            project_key,
            trigger,
            IndexRequestKind::IgnoreFileDiscovery,
            scan_options,
            |ctx| {
                workspace
                    .fs()
                    .traversal(Box::new(|scope: &dyn TraversalScope| {
                        scope.evaluate(ctx, path.to_path_buf());
                    }));
                ctx.evaluated_paths
                    .pin()
                    .iter()
                    .filter(|path| path.is_ignore())
                    .cloned()
                    .collect::<Vec<_>>()
            },
        );
        ignore_paths.sort_unstable();
        (ignore_paths, diagnostics)
    }

    pub fn index_dependencies<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        project_path: &Utf8Path,
        dependencies: ModuleDependencies,
        trigger: IndexTrigger,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.index_with_context(workspace, project_key, trigger, true, |ctx| {
            scan_dependencies(project_path, dependencies, ctx).1
        })
    }

    /// Indexes the files at `paths` in parallel, and then the dependencies
    /// that they make reachable.
    ///
    /// Use this for files that a project scan accepted already, such as the
    /// files that it couldn't update. The folders of the dependencies are
    /// watched only when `watch` is set.
    pub(crate) fn index_files<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        project_path: &Utf8Path,
        paths: Vec<Utf8PathBuf>,
        trigger: IndexTrigger,
        watch: bool,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.index_with_context(workspace, project_key, trigger, watch, |ctx| {
            ctx.index_files_in_parallel(paths);
            let dependencies = mem::take(&mut ctx.dependencies)
                .into_inner()
                .unwrap()
                .into_iter()
                .collect();
            scan_dependencies(project_path, dependencies, ctx).1
        })
    }

    /// Runs `index` with the context of a scan of the project, and returns the
    /// diagnostics it reported. The folders that `index` returns are watched
    /// when `watch` is set.
    fn index_with_context<'a, W: WorkspaceScannerBridge>(
        &self,
        workspace: &'a W,
        project_key: ProjectKey,
        trigger: IndexTrigger,
        watch: bool,
        index: impl FnOnce(&mut ScanContext<'a, W>) -> FxHashSet<Utf8PathBuf>,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let scan_kind = self
            .get_scan_kind_for_project(project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        let scan_options = ScanOptions {
            scan_kind,
            force: false,
            verbose: false,
            watch,
        };
        let (folders_to_watch, diagnostics) = self.run_scan_phase(
            workspace,
            project_key,
            trigger,
            IndexRequestKind::Explicit(trigger),
            &scan_options,
            index,
        );
        if watch {
            self.watch_folders(workspace.fs(), folders_to_watch);
        }
        Ok(diagnostics)
    }

    /// Runs `phase` with a new scan context. Returns the result of `phase` and
    /// the diagnostics it reported.
    fn run_scan_phase<'a, W: WorkspaceScannerBridge, R>(
        &self,
        workspace: &'a W,
        project_key: ProjectKey,
        trigger: IndexTrigger,
        request_kind: IndexRequestKind,
        scan_options: &ScanOptions,
        phase: impl FnOnce(&mut ScanContext<'a, W>) -> R,
    ) -> (R, Vec<Diagnostic>) {
        let (interner, _path_receiver) = PathInterner::new();
        let (diagnostics_sender, diagnostics_receiver) = unbounded();
        let collector = DiagnosticsCollector::new(scan_options.verbose);

        // The traversal context is scoped to ensure all the channels it
        // contains are properly closed once the phase finishes.
        let mut ctx = ScanContext {
            workspace,
            project_key,
            interner,
            physical_files: PhysicalFileOwners::default(),
            diagnostics_sender,
            evaluated_paths: Default::default(),
            scan_kind: scan_options.scan_kind.clone(),
            trigger,
            request_kind,
            watch: scan_options.watch,
            dependencies: Default::default(),
        };

        #[cfg(not(target_family = "wasm"))]
        let result = thread::scope(|scope| {
            let handler = thread::Builder::new()
                .name("biome::scanner".to_string())
                .spawn_scoped(scope, || collector.run(diagnostics_receiver))
                .expect("failed to spawn scanner thread");

            let result = phase(&mut ctx);

            // Make sure the diagnostics sender is dropped.
            drop(ctx);

            // Wait for the collector thread to finish.
            (result, handler.join().unwrap())
        });

        // WASM can't spawn threads, so the phase and the collector run on the
        // current thread.
        #[cfg(target_family = "wasm")]
        let result = {
            let result = phase(&mut ctx);

            // Close the diagnostics channel before collecting to avoid a deadlock on WASM.
            drop(ctx);
            (result, collector.run(diagnostics_receiver))
        };

        result
    }

    /// Updates the index of the file with the given `path`.
    ///
    /// This is done through an instruction instead of calling
    /// [`WorkspaceScannerBridge::index_file()`] directly to ensure it is only
    /// done if the watcher is active.
    pub fn reindex_file(&self, path: Utf8PathBuf) {
        let _ = self.watcher_tx.send(WatcherInstruction::ReindexFile(path));
    }

    /// Returns whether the given logical folder is watched.
    ///
    /// Passing the path to a file always returns `false`.
    fn is_watched(&self, path: &Utf8Path) -> bool {
        let aliases = self.watch_aliases.lock();
        aliases.iter().any(|(physical_path, logical_paths)| {
            logical_paths.contains(path) && self.watched_folders.pin().contains(physical_path)
        })
    }

    /// Returns whether the project was scanned with a watcher that still watches its folder.
    ///
    /// A folder can also be watched for the ignore files of another project, so a watched folder
    /// alone doesn't mean that the watcher keeps the index of the project up to date.
    fn is_project_watched(&self, project_key: ProjectKey, project_path: &Utf8Path) -> bool {
        self.projects
            .pin()
            .get(&project_key)
            .is_some_and(|project| project.watched)
            && self.is_watched(project_path)
    }

    /// Registers logical folders and asks the watcher to watch their physical locations.
    ///
    /// Physical folders that lose their last alias because a symbolic link now resolves to a new
    /// target are unwatched.
    fn watch_folders(&self, fs: &dyn FileSystem, folders: FxHashSet<Utf8PathBuf>) {
        if folders.is_empty() {
            return;
        }

        let mut folders: Vec<_> = folders.into_iter().collect();
        folders.sort_by_key(|path| path.components().count());

        let (physical_folders, unused_folders) = {
            let mut aliases = self.watch_aliases.lock();
            let mut symlink_targets = self.watched_symlink_targets.lock();
            let mut physical_folders = FxHashSet::default();
            let mut unused_folders = FxHashSet::default();

            for logical_path in folders {
                let is_symlink = fs.path_is_symlink(&logical_path);
                let physical_path = physical_watch_path(fs, &logical_path, is_symlink, &aliases);
                if is_symlink
                    && symlink_targets
                        .insert(logical_path.clone(), physical_path.clone())
                        .is_some_and(|previous_target| previous_target != physical_path)
                {
                    remove_retargeted_aliases(
                        &mut aliases,
                        &logical_path,
                        &physical_path,
                        &mut unused_folders,
                    );
                }
                let mut logical_paths = logical_watch_paths(&physical_path, &aliases);
                logical_paths.insert(logical_path);
                aliases
                    .entry(physical_path.clone())
                    .or_default()
                    .extend(logical_paths);
                physical_folders.insert(physical_path);
            }

            unused_folders.retain(|physical_path| !aliases.contains_key(physical_path));
            (physical_folders, unused_folders)
        };

        let _ = self
            .watcher_tx
            .try_send(WatcherInstruction::WatchFolders(physical_folders));
        self.unwatch_unused_folders(unused_folders);
    }

    /// Asks the watcher to unwatch physical folders that no longer have a logical alias.
    fn unwatch_unused_folders(&self, physical_folders: FxHashSet<Utf8PathBuf>) {
        if !physical_folders.is_empty() {
            let _ = self
                .watcher_tx
                .try_send(WatcherInstruction::UnwatchUnusedFolders(physical_folders));
        }
    }

    /// Projects a physical watcher event to every registered logical alias.
    fn logical_paths_for_watcher_event(&self, path: &Utf8Path) -> Vec<Utf8PathBuf> {
        let aliases = self.watch_aliases.lock();
        let mut paths: Vec<_> = aliases
            .iter()
            .flat_map(|(root, logical_paths)| {
                path.strip_prefix(root)
                    .ok()
                    .into_iter()
                    .flat_map(move |relative_path| {
                        logical_paths
                            .iter()
                            .map(move |logical_path| logical_path.join(relative_path))
                    })
            })
            .collect();
        if paths.is_empty() {
            return vec![path.to_path_buf()];
        }
        paths.sort();
        paths.dedup();
        paths
    }

    /// Informs the workspace the given physical folder is watched.
    ///
    /// Returns `true` if the folder was not yet watched, `false` otherwise.
    fn insert_watched_folder(&self, path: Utf8PathBuf) -> bool {
        self.watched_folders.pin().insert(path)
    }

    /// Stops tracking the physical folder `path` as watched when no logical alias maps to it.
    ///
    /// Returns `true` if the folder was watched and must be unwatched, `false` otherwise.
    fn remove_unused_watched_folder(&self, path: &Utf8Path) -> bool {
        // Holding the alias lock orders this check after any registration of a new alias.
        let aliases = self.watch_aliases.lock();
        !aliases.contains_key(path) && self.watched_folders.pin().remove(path)
    }

    /// Removes the logical aliases `logical_paths` and unwatches physical folders left without
    /// an alias.
    ///
    /// Unlike [`Self::remove_watch_aliases()`], aliases below `logical_paths` are kept.
    fn remove_exact_watch_aliases(&self, logical_paths: &FxHashSet<Utf8PathBuf>) {
        if logical_paths.is_empty() {
            return;
        }
        let unused_folders = {
            let mut aliases = self.watch_aliases.lock();
            let mut unused_folders = FxHashSet::default();
            aliases.retain(|physical_path, aliases_of_folder| {
                aliases_of_folder.retain(|logical_path| !logical_paths.contains(logical_path));
                if aliases_of_folder.is_empty() {
                    unused_folders.insert(physical_path.clone());
                    false
                } else {
                    true
                }
            });
            self.watched_symlink_targets
                .lock()
                .retain(|logical_path, _| !logical_paths.contains(logical_path));
            unused_folders
        };
        self.unwatch_unused_folders(unused_folders);
    }

    /// Removes logical aliases under `path` and unwatches unused physical folders.
    fn remove_watch_aliases(&self, path: &Utf8Path, mut callback: impl FnMut(&Utf8Path)) {
        let unused_physical_paths = {
            let mut aliases = self.watch_aliases.lock();
            self.watched_symlink_targets
                .lock()
                .retain(|logical_path, _| !logical_path.starts_with(path));
            let mut unused_physical_paths = Vec::new();
            aliases.retain(|physical_path, logical_paths| {
                logical_paths.retain(|logical_path| !logical_path.starts_with(path));
                if logical_paths.is_empty() {
                    unused_physical_paths.push(physical_path.clone());
                    false
                } else {
                    true
                }
            });
            unused_physical_paths
        };

        for physical_path in unused_physical_paths {
            if self.watched_folders.pin().remove(&physical_path) {
                callback(&physical_path);
            }
        }
    }

    fn watch_project_vcs_paths<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
    ) {
        let folders: FxHashSet<_> = workspace
            .get_vcs_watch_paths(project_key)
            .into_iter()
            .filter_map(|path| path.parent().map(Utf8Path::to_path_buf))
            .collect();
        self.vcs_watch_folders
            .lock()
            .entry(project_key)
            .or_default()
            .extend(folders.iter().cloned());
        self.watch_folders(workspace.fs(), folders);
    }

    /// Performs a file system scan and indexes all supported documents that
    /// qualify for the given `scan_kind`.
    #[instrument(level = "debug", skip(self, workspace, scan_options))]
    fn scan<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        folder: &Utf8Path,
        trigger: IndexTrigger,
        scan_options: &ScanOptions,
    ) -> ScanResult {
        if scan_options.watch {
            self.watch_project_vcs_paths(workspace, project_key);
        }
        let setup = self.set_up_scan(workspace, project_key, folder, trigger, scan_options, &[]);
        self.scan_files(workspace, project_key, folder, trigger, scan_options, setup)
    }

    /// Runs the setup phase of a scan of `folder`: finds its files and reads
    /// its configuration files, manifests and ignore files.
    fn set_up_scan<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        folder: &Utf8Path,
        trigger: IndexTrigger,
        scan_options: &ScanOptions,
        additional_ignore_roots: &[Utf8PathBuf],
    ) -> ScanSetupResult {
        let ((folder_result, dependencies), diagnostics) = self.run_scan_phase(
            workspace,
            project_key,
            trigger,
            IndexRequestKind::Explicit(trigger),
            scan_options,
            |ctx| {
                let folder_result = self.scan_folder(folder, ctx, ScanFolderKind::Project);
                for ignore_root in additional_ignore_roots {
                    self.scan_folder(ignore_root, ctx, ScanFolderKind::IgnoreFiles);
                }
                let dependencies = mem::take(&mut ctx.dependencies).into_inner().unwrap();
                (folder_result, dependencies)
            },
        );
        let ScanFolderResult {
            duration,
            configuration_files,
            handleable_paths,
        } = folder_result;
        ScanSetupResult {
            deferred_paths: Vec::new(),
            handleable_paths,
            configuration_files,
            dependencies,
            diagnostics,
            duration,
        }
    }

    /// Runs the files phase of a scan of `folder`: indexes again the files
    /// that `setup` deferred, then the other files that it found and their
    /// dependencies.
    fn scan_files<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        folder: &Utf8Path,
        trigger: IndexTrigger,
        scan_options: &ScanOptions,
        setup: ScanSetupResult,
    ) -> ScanResult {
        let ScanSetupResult {
            deferred_paths,
            handleable_paths,
            configuration_files,
            dependencies,
            mut diagnostics,
            duration: setup_duration,
        } = setup;
        let (duration, files_diagnostics) = self.run_scan_phase(
            workspace,
            project_key,
            trigger,
            IndexRequestKind::Explicit(trigger),
            scan_options,
            |ctx| {
                let start = web_time::Instant::now();
                // The other files read the manifests that the setup phase
                // deferred, so they're indexed first.
                ctx.index_files_in_parallel(deferred_paths);
                let traversal_ctx = &*ctx;
                ctx.workspace
                    .fs()
                    .traversal(Box::new(|scope: &dyn TraversalScope| {
                        for path in &handleable_paths {
                            scope.handle(traversal_ctx, path.clone());
                        }
                    }));
                #[cfg_attr(target_family = "wasm", expect(unused_mut))]
                let mut duration = start.elapsed();

                // WASM has no watcher, and scans don't index dependencies
                // there.
                #[cfg(not(target_family = "wasm"))]
                {
                    let mut all_dependencies =
                        mem::take(&mut ctx.dependencies).into_inner().unwrap();
                    all_dependencies.extend(dependencies);
                    if !all_dependencies.is_empty() {
                        let (dependencies_duration, folders_to_watch) =
                            scan_dependencies(folder, all_dependencies.into_iter().collect(), ctx);
                        duration += dependencies_duration;
                        self.watch_folders(workspace.fs(), folders_to_watch);
                    }
                }
                #[cfg(target_family = "wasm")]
                let _ = (folder, dependencies);

                duration
            },
        );
        diagnostics.extend(files_diagnostics);

        ScanResult {
            diagnostics,
            duration: setup_duration + duration,
            configuration_files,
        }
    }

    /// Finds the files of `folder` and indexes its configuration files,
    /// manifests and ignore files.
    ///
    /// Returns the other files, for the files phase of the scan to index.
    #[instrument(level = "debug", skip(self, ctx))]
    fn scan_folder<W: WorkspaceScannerBridge>(
        &self,
        folder: &Utf8Path,
        ctx: &ScanContext<W>,
        kind: ScanFolderKind,
    ) -> ScanFolderResult {
        let start = web_time::Instant::now();

        let workspace = ctx.workspace;
        let fs = workspace.fs();
        fs.traversal(Box::new(move |scope: &dyn TraversalScope| {
            scope.evaluate(ctx, folder.to_path_buf());
        }));

        let mut evaluated_paths: Vec<_> = ctx
            .evaluated_paths()
            .iter()
            .filter(|path| path.starts_with(folder))
            .cloned()
            .collect();
        let mut configs = Vec::new();
        let mut manifests = Vec::new();
        let mut handleable_paths = Vec::with_capacity(evaluated_paths.len());
        let mut ignore_paths = Vec::new();
        let mut folders_to_watch = FxHashSet::<Utf8PathBuf>::default();

        evaluated_paths.sort_unstable_by(BiomePath::cmp_processing_priority);

        // We want to process files that closest to the project root first. For
        // example, we must process first the `.gitignore` at the root of the
        // project.
        for path in evaluated_paths {
            if path.is_ignore() {
                ignore_paths.push(path);
            } else if ctx.watch && fs.path_kind(path.as_path()).is_ok_and(PathKind::is_dir) {
                folders_to_watch.insert(path.into());
            } else if kind == ScanFolderKind::Project {
                if path.is_config() {
                    configs.push(path);
                } else if path.is_manifest() {
                    manifests.push(path);
                } else {
                    handleable_paths.push(path);
                }
            }
        }
        self.watch_folders(fs, folders_to_watch);
        if kind == ScanFolderKind::Project {
            fs.traversal(Box::new(|scope: &dyn TraversalScope| {
                for path in &configs {
                    scope.handle(ctx, path.clone());
                }
            }));
            fs.traversal(Box::new(|scope: &dyn TraversalScope| {
                for path in &manifests {
                    scope.handle(ctx, path.clone());
                }
            }));

            let result = workspace.update_project_config_files(ctx.project_key, &configs);
            match result {
                Ok(diagnostics) => {
                    for diagnostic in diagnostics {
                        ctx.send_diagnostic(diagnostic)
                    }
                }
                Err(error) => {
                    ctx.send_diagnostic(error);
                }
            }
        }

        let result = workspace.update_project_ignore_files(ctx.project_key, folder, &ignore_paths);
        if let Err(error) = result {
            ctx.send_diagnostic(error);
        }

        ScanFolderResult {
            duration: start.elapsed(),
            configuration_files: configs,
            handleable_paths,
        }
    }

    /// Instructs the watcher to stop watching the given folder with the given
    /// `path`.
    ///
    /// This will remove the folder from [`Self::watched_folders`].
    pub fn unload_folder(&self, path: Utf8PathBuf) {
        let _ = self
            .watcher_tx
            .try_send(WatcherInstruction::UnwatchFolder(path));
    }

    /// Unloads the project from the scanner and instructs the watcher to stop
    /// watching it.
    ///
    /// This does _not_ take care of unloading the documents inside the folder.
    pub fn unload_project(&self, project_key: ProjectKey, project_path: Utf8PathBuf) {
        self.projects.pin().remove(&project_key);
        let vcs_folders = self.release_vcs_watch_folders(project_key, &project_path);
        self.unload_folder(project_path);
        self.remove_exact_watch_aliases(&vcs_folders);
    }

    /// Returns the VCS watch folders of `project_key` that no other open project uses.
    ///
    /// Folders inside `project_path` are excluded, because unloading the project folder removes
    /// them. Folders inside another watched project are kept, because its scan may watch them.
    fn release_vcs_watch_folders(
        &self,
        project_key: ProjectKey,
        project_path: &Utf8Path,
    ) -> FxHashSet<Utf8PathBuf> {
        let mut vcs_watch_folders = self.vcs_watch_folders.lock();
        let Some(mut folders) = vcs_watch_folders.remove(&project_key) else {
            return FxHashSet::default();
        };
        let projects = self.projects.pin();
        folders.retain(|folder| {
            !folder.starts_with(project_path)
                && !vcs_watch_folders
                    .values()
                    .any(|other_folders| other_folders.contains(folder))
                && !projects
                    .values()
                    .any(|project| project.watched && folder.starts_with(&project.path))
        });
        folders
    }
}

struct ScanFolderResult {
    /// Duration of the scan.
    pub duration: Duration,

    /// List of nested configuration files found inside the folder.
    ///
    /// The root configuration is not included in this.
    pub configuration_files: Vec<BiomePath>,

    /// The files for the files phase of the scan to index.
    pub handleable_paths: Vec<BiomePath>,
}

/// The result of the setup phase of a folder scan, for its files phase.
struct ScanSetupResult {
    /// The files that the setup phase deferred, for the files phase to index
    /// again before the other files. Only a project scan defers them.
    deferred_paths: Vec<Utf8PathBuf>,

    /// The files for the files phase to index.
    handleable_paths: Vec<BiomePath>,

    /// The nested configuration files of the folder.
    configuration_files: Vec<BiomePath>,

    /// The dependencies of the files that the setup phase indexed.
    dependencies: Vec<Utf8PathBuf>,

    diagnostics: Vec<Diagnostic>,

    duration: Duration,
}

/// A project scan after its setup phase. [`Scanner::finish_project_scan()`]
/// runs its files phase.
pub(crate) struct ProjectScan {
    project_key: ProjectKey,
    project_path: Utf8PathBuf,
    scan_options: ScanOptions,
    setup: ScanSetupResult,
}

impl ProjectScan {
    /// Returns the nested configuration files of the project.
    pub(crate) fn configuration_files(&self) -> &[BiomePath] {
        &self.setup.configuration_files
    }
}

/// Resolves the physical folder that must be watched for a logical folder.
///
/// `is_symlink` tells whether `logical_path` itself is a symbolic link.
fn physical_watch_path(
    fs: &dyn FileSystem,
    logical_path: &Utf8Path,
    is_symlink: bool,
    aliases: &FxHashMap<Utf8PathBuf, FxHashSet<Utf8PathBuf>>,
) -> Utf8PathBuf {
    if is_symlink {
        let mut physical_path = logical_path.to_path_buf();
        for _ in 0..=3 {
            if !fs.path_is_symlink(&physical_path) {
                return physical_path;
            }

            let Ok(target) = fs.read_link(&physical_path) else {
                break;
            };
            physical_path = if target.is_absolute() {
                normalize_path(&target)
            } else {
                let parent = physical_path.parent().unwrap_or(Utf8Path::new(""));
                normalize_path(&parent.join(target))
            };
        }
    }

    aliases
        .iter()
        .flat_map(|(physical_root, logical_roots)| {
            logical_roots.iter().filter_map(move |logical_root| {
                logical_path
                    .strip_prefix(logical_root)
                    .ok()
                    .map(|relative| {
                        (
                            logical_root.components().count(),
                            physical_root.join(relative),
                        )
                    })
            })
        })
        .max_by_key(|(depth, _)| *depth)
        .map_or_else(|| logical_path.to_path_buf(), |(_, path)| path)
}

/// Removes the aliases at or below the retargeted symbolic link `logical_path` from physical
/// folders outside its new target `physical_path`.
///
/// Physical folders left without an alias are removed and collected into `unused_folders`.
fn remove_retargeted_aliases(
    aliases: &mut FxHashMap<Utf8PathBuf, FxHashSet<Utf8PathBuf>>,
    logical_path: &Utf8Path,
    physical_path: &Utf8Path,
    unused_folders: &mut FxHashSet<Utf8PathBuf>,
) {
    aliases.retain(|physical_root, logical_roots| {
        if physical_root.starts_with(physical_path) {
            return true;
        }
        logical_roots.retain(|logical_root| !logical_root.starts_with(logical_path));
        if logical_roots.is_empty() {
            unused_folders.insert(physical_root.clone());
            false
        } else {
            true
        }
    });
}

/// Returns every logical alias of a physical watcher folder.
fn logical_watch_paths(
    physical_path: &Utf8Path,
    aliases: &FxHashMap<Utf8PathBuf, FxHashSet<Utf8PathBuf>>,
) -> FxHashSet<Utf8PathBuf> {
    aliases
        .iter()
        .flat_map(|(physical_root, logical_roots)| {
            physical_path
                .strip_prefix(physical_root)
                .ok()
                .into_iter()
                .flat_map(move |relative| {
                    logical_roots
                        .iter()
                        .map(move |logical_root| logical_root.join(relative))
                })
        })
        .collect()
}

#[instrument(level = "debug", skip(ctx))]
fn scan_dependencies<W: WorkspaceScannerBridge>(
    project_path: &Utf8Path,
    dependencies: ModuleDependencies,
    ctx: &mut ScanContext<W>,
) -> (Duration, FxHashSet<Utf8PathBuf>) {
    let start = web_time::Instant::now();

    let dependencies: Vec<_> = dependencies
        .into_iter()
        .filter(|dependency_path| dependency_path.starts_with(project_path))
        .collect();

    // First collect all the folders of the given dependencies.
    let mut folders: FxHashSet<_> = {
        let mut folders = FxHashSet::default();
        for dependency_path in &dependencies {
            for ancestor in dependency_path.ancestors().skip(1) {
                if ancestor == project_path {
                    break;
                }

                if !folders.insert(ancestor) {
                    // If an ancestor was already in the set, its parents must
                    // be too.
                    break;
                }
            }
        }
        folders.into_iter().map(Utf8Path::to_path_buf).collect()
    };

    rayon::scope(|s| {
        fn index_dependency<'a, W: WorkspaceScannerBridge>(
            s: &Scope<'a>,
            ctx: &'a ScanContext<'a, W>,
            dependency_path: Utf8PathBuf,
        ) {
            let path = ctx.workspace.fs().biome_path(&dependency_path);
            let dependencies = open_file(ctx, path, ctx.trigger);
            ctx.dependencies
                .lock()
                .unwrap()
                .extend(dependencies.clone());

            for dependency_path in dependencies {
                let is_ignored = ctx
                    .workspace
                    .is_ignored(
                        ctx.project_key,
                        &ctx.scan_kind,
                        &dependency_path,
                        IndexRequestKind::Dependency(ctx.trigger),
                        None,
                    )
                    .unwrap_or(true);
                if !is_ignored {
                    s.spawn(move |s| index_dependency(s, ctx, dependency_path));
                }
            }
        }

        for dependency_path in dependencies {
            let is_ignored = ctx
                .workspace
                .is_ignored(
                    ctx.project_key,
                    &ctx.scan_kind,
                    &dependency_path,
                    IndexRequestKind::Dependency(ctx.trigger),
                    None,
                )
                .unwrap_or(true);
            if !is_ignored {
                s.spawn(|s| index_dependency(s, ctx, dependency_path));
            }
        }
    });

    // Extend the folders with those of the transitive dependencies.
    let dependencies_mutex = mem::take(&mut ctx.dependencies);
    let dependencies = dependencies_mutex.into_inner().unwrap();
    for dependency_path in dependencies {
        for ancestor in dependency_path.ancestors().skip(1) {
            if ancestor == project_path {
                break;
            }

            if !folders.insert(ancestor.to_path_buf()) {
                // If an ancestor was already in the set, its parents must
                // be too.
                break;
            }
        }
    }

    (start.elapsed(), folders)
}

struct DiagnosticsCollector {
    /// The minimum level of diagnostic we should collect.
    diagnostic_level: Severity,
}

impl DiagnosticsCollector {
    fn new(verbose: bool) -> Self {
        Self {
            diagnostic_level: if verbose {
                Severity::Hint
            } else {
                Severity::Error
            },
        }
    }

    /// Checks whether the given `diagnostic` should be collected or not.
    fn should_collect(&self, diagnostic: &Diagnostic) -> bool {
        diagnostic.severity() >= self.diagnostic_level || diagnostic.tags().is_internal()
    }

    fn run(&self, receiver: Receiver<Diagnostic>) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        while let Ok(diagnostic) = receiver.recv() {
            if self.should_collect(&diagnostic) {
                diagnostics.push(diagnostic);
            }
        }

        diagnostics
    }
}

/// Context object shared between directory traversal tasks.
pub(crate) struct ScanContext<'app, W: WorkspaceScannerBridge> {
    /// [Workspace] instance.
    pub(crate) workspace: &'app W,

    /// Key of the project within which this scanner is active.
    project_key: ProjectKey,

    /// Logical path interner used by the filesystem traversal.
    interner: PathInterner,

    /// Physical files selected after logical path policy is applied.
    physical_files: PhysicalFileOwners,

    /// Channel for reporting diagnostics during scanning.
    pub(crate) diagnostics_sender: Sender<Diagnostic>,

    /// List of paths that should be processed.
    pub(crate) evaluated_paths: HashSet<BiomePath>,

    /// What the scanner should target.
    scan_kind: ScanKind,

    /// The trigger that initiated this scan.
    trigger: IndexTrigger,

    /// The policy used to select paths during traversal.
    request_kind: IndexRequestKind,

    /// Whether the scanned folders need to be watched.
    watch: bool,

    /// Dependencies discovered during scanning.
    dependencies: Mutex<Vec<Utf8PathBuf>>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ScanKind {
    /// The scanner should not be triggered.
    #[default]
    NoScanner,
    /// Scans for limited, known files across the repository.
    KnownFiles,
    /// Scans for limited, knows files, but only within a set of predefined
    /// paths.
    #[serde(rename_all = "camelCase")]
    TargetedKnownFiles {
        /// The paths to target by the scanner.
        ///
        /// If a target path indicates a folder, all files within are scanned as well.
        ///
        /// Target paths must be absolute.
        target_paths: Vec<BiomePath>,

        /// Determines whether the file scanner should descend into
        /// subdirectories of the target paths.
        descend_from_targets: bool,
    },
    /// Scans the entire repository, indexing all files to enable project rules.
    Project,

    /// Scans the entire repository, indexing all files to enable type inference rules
    TypeAware,
}

impl ScanKind {
    /// Returns a clone of `self`, but drops any `target_paths` if present.
    ///
    /// This makes the scan kind cheaper to clone in use cases where the
    /// `target_paths` are no longer relevant, such as the watcher.
    pub const fn clone_without_targeting_info(&self) -> Self {
        match self {
            Self::NoScanner => Self::NoScanner,
            Self::KnownFiles => Self::KnownFiles,
            Self::Project => Self::Project,
            Self::TargetedKnownFiles { .. } => Self::KnownFiles,
            Self::TypeAware => Self::TypeAware,
        }
    }

    pub const fn is_project(&self) -> bool {
        matches!(self, Self::Project)
    }

    pub const fn is_type_aware(&self) -> bool {
        matches!(self, Self::TypeAware)
    }

    pub const fn is_known_files(&self) -> bool {
        matches!(self, Self::KnownFiles)
    }

    pub const fn is_none(&self) -> bool {
        matches!(self, Self::NoScanner)
    }
}

impl<W: WorkspaceScannerBridge> ScanContext<'_, W> {
    /// Send a message to the display thread
    pub(crate) fn send_diagnostic(&self, diagnostic: impl Into<Diagnostic>) {
        self.diagnostics_sender.send(diagnostic.into()).ok();
    }

    /// Indexes the files at `paths` in parallel, and adds their dependencies
    /// to [Self::dependencies].
    fn index_files_in_parallel(&self, paths: Vec<Utf8PathBuf>) {
        rayon::scope(|scope| {
            for path in paths {
                scope.spawn(move |_| {
                    let path = self.workspace.fs().biome_path(&path);
                    let dependencies = open_file(self, path, self.trigger);
                    self.dependencies.lock().unwrap().extend(dependencies);
                });
            }
        });
    }
}

impl<W: WorkspaceScannerBridge> TraversalContext for ScanContext<'_, W> {
    fn interner(&self) -> &PathInterner {
        &self.interner
    }

    fn store_physical_path(
        &self,
        physical_path: Utf8PathBuf,
        logical_path: BiomePath,
        is_symlink: bool,
    ) {
        self.physical_files.select(
            physical_path,
            logical_path,
            is_symlink,
            |previous, selected| {
                let evaluated_paths = self.evaluated_paths.pin();
                if let Some(previous) = previous {
                    evaluated_paths.remove(previous);
                }
                evaluated_paths.insert(selected.clone());
            },
        );
    }

    fn evaluated_paths(&self) -> HashSetRef<'_, BiomePath, RandomState, LocalGuard<'_>> {
        self.evaluated_paths.pin()
    }

    fn push_diagnostic(&self, error: Error) {
        self.send_diagnostic(Diagnostic::new(error));
    }

    // This is the main filtering logic applied by the scanner.
    //
    // Whether the scanner handles a file or not is based on:
    // - The file path and whether that path is being ignored, and/or whether
    //   the path belongs to a dependency.
    // - The kind of file system entry the path points to.
    // - The kind of scan we are performing.
    fn can_handle_with_kind(
        &self,
        path: &BiomePath,
        path_kind: PathKind,
        _ancestors_checked: bool,
    ) -> bool {
        match self.workspace.is_ignored(
            self.project_key,
            &self.scan_kind,
            path.as_path(),
            self.request_kind,
            Some(path_kind),
        ) {
            Ok(is_ignored) => !is_ignored,
            Err(_) => {
                // Pretend we can handle it so we can give a meaningful error
                // when it fails.
                true
            }
        }
    }

    fn handle_path(&self, path: BiomePath) {
        let dependencies = open_file(self, path, self.trigger);
        self.dependencies.lock().unwrap().extend(dependencies);
    }

    fn store_path(&self, path: BiomePath) {
        self.evaluated_paths.pin().insert(path);
    }

    fn should_store_dirs(&self) -> bool {
        self.watch
    }
}

/// Instructs the workspace to open a single file and submits diagnostics in
/// case of an error.
///
/// The call to the workspace method is also wrapped in a [catch_unwind] block
/// so panics are caught, and diagnostics are submitted in case of panic too.
fn open_file<W: WorkspaceScannerBridge>(
    ctx: &ScanContext<W>,
    path: BiomePath,
    trigger: IndexTrigger,
) -> ModuleDependencies {
    match catch_unwind(|| {
        ctx.workspace
            .index_file(ctx.project_key, path.clone(), trigger)
    }) {
        Ok(Ok((dependencies, diagnostics))) => {
            for diagnostic in diagnostics {
                ctx.push_diagnostic(diagnostic.with_file_path(path.as_str()));
            }
            dependencies
        }
        Ok(Err(err)) => {
            let mut error: Error = err.into();
            if !path.is_config() && error.severity() >= Severity::Error {
                error = error
                    .with_severity(Severity::Warning)
                    .with_file_path(path.as_str());
            }
            ctx.push_diagnostic(error);

            Default::default()
        }
        Err(err) => {
            let error = match err.downcast::<String>() {
                Ok(description) => Panic::with_file_and_message(path.as_path(), *description),
                Err(err) => match err.downcast::<&'static str>() {
                    Ok(description) => Panic::with_file_and_message(path.as_path(), *description),
                    Err(_) => Panic::with_file(path.as_path()),
                },
            };

            ctx.send_diagnostic(error);

            Default::default()
        }
    }
}

#[cfg(test)]
#[path = "scanner.tests.rs"]
mod tests;
