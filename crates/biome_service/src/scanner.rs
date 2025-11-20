//! Scanner for crawling the file system and indexing documents within projects.
//!
//! In addition to scanning and indexing, the scanner also has a
//! [`watcher`](watcher::Watcher) that can watch scanned folders to keep the
//! index in sync with the filesystem.

mod watcher;
mod workspace_bridges;

#[cfg(test)]
mod test_utils;

use biome_diagnostics::serde::Diagnostic;
use biome_diagnostics::{Diagnostic as _, DiagnosticExt, Error, Severity};
use biome_fs::{BiomePath, PathInterner, PathKind, TraversalContext, TraversalScope};
use biome_module_graph::ModuleDependencies;
use camino::{Utf8Path, Utf8PathBuf};
use crossbeam::channel::{Receiver, Sender, unbounded};
use papaya::{HashMap, HashSet};
use rayon::Scope;
use rustc_hash::FxHashSet;
use std::collections::BTreeSet;
use std::panic::catch_unwind;
use std::sync::{Mutex, RwLock};
use std::time::Duration;
use std::{mem, thread};
use tracing::instrument;

use crate::diagnostics::Panic;
use crate::projects::ProjectKey;
use crate::workspace::{ScanProjectResult, ServiceNotification, WorkspaceError};

pub use watcher::{Watcher, WatcherInstruction};
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

/// The kind of request for why a path is being indexed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexRequestKind {
    /// There is an explicit request for the path to be indexed, due to being
    /// included in the configuration.
    Explicit(IndexTrigger),

    /// The file is being indexed because it is a dependency of one or more
    /// files that were explicitly indexed.
    Dependency(IndexTrigger),
}

impl IndexRequestKind {
    pub fn trigger(self) -> IndexTrigger {
        match self {
            Self::Explicit(trigger) | Self::Dependency(trigger) => trigger,
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

    /// All the folders that are being watched by the watcher.
    watched_folders: HashSet<Utf8PathBuf>,

    /// Channel sender for instructions to the [`Watcher`].
    watcher_tx: Sender<WatcherInstruction>,
}

struct ScannedProject {
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

    /// Indexes the project with the given `project_key`, using the given
    /// `scan_kind`.
    ///
    /// If [`ScanOptions::watch`] is `true`, this will install a watcher on the
    /// project folder.
    pub fn index_project<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        scan_options: ScanOptions,
    ) -> Result<ScanProjectResult, WorkspaceError> {
        let project_path = workspace
            .get_project_path(project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        if !scan_options.force && self.is_watched(&project_path) {
            return Ok(ScanProjectResult {
                diagnostics: Default::default(),
                duration: Default::default(),
                configuration_files: Default::default(),
            });
        }

        let scanned_project = ScannedProject {
            scan_kind: scan_options.scan_kind.clone_without_targeting_info(),
            watched: scan_options.watch,
        };

        let result = self.scan(
            workspace,
            project_key,
            &project_path,
            IndexTrigger::InitialScan,
            scan_options,
        )?;

        self.projects.pin().insert(project_key, scanned_project);

        workspace.notify(ServiceNotification::IndexUpdated);

        Ok(ScanProjectResult {
            diagnostics: result.diagnostics,
            duration: result.duration,
            configuration_files: result.configuration_files,
        })
    }

    /// Indexes the folder with given `path` within the project with the given
    /// `project_key`.
    ///
    /// The project must already have been scanned using
    /// [`Self::index_project()`]. The folder will be scanned with the same
    /// scan kind as the project.
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
            scan_options,
        )?;

        // No need to notify after this scan, because the individual file
        // updates will already trigger notifications.
        Ok(result.diagnostics)
    }

    pub fn index_dependencies<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        project_path: &Utf8Path,
        dependencies: ModuleDependencies,
        trigger: IndexTrigger,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let scan_kind = self
            .get_scan_kind_for_project(project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        let (interner, _path_receiver) = PathInterner::new();

        let (diagnostics_sender, diagnostics_receiver) = unbounded();
        let collector = DiagnosticsCollector::new(false);

        let diagnostics = thread::scope(|scope| {
            let handler = thread::Builder::new()
                .name("biome::scanner".to_string())
                .spawn_scoped(scope, || collector.run(diagnostics_receiver))
                .expect("failed to spawn scanner thread");

            // The traversal context is scoped to ensure all the channels it
            // contains are properly closed once scanning finishes.
            let mut ctx = ScanContext {
                workspace,
                project_key,
                interner,
                diagnostics_sender,
                evaluated_paths: Default::default(),
                scan_kind,
                trigger,
                watch: true,
                dependencies: Default::default(),
            };

            let (_duration, folders_to_watch) =
                scan_dependencies(project_path, dependencies, &mut ctx);

            let _ = self
                .watcher_tx
                .try_send(WatcherInstruction::WatchFolders(folders_to_watch));

            // Make sure the diagnostics sender is dropped.
            drop(ctx);

            // Wait for the collector thread to finish.
            handler.join().unwrap()
        });

        Ok(diagnostics)
    }

    /// Updates the index of the file with the given `path`.
    ///
    /// This is done through an instruction instead of calling
    /// [`WorkspaceScannerBridge::index_file()`] directly to ensure it is only
    /// done if the watcher is active.
    pub fn reindex_file(&self, path: Utf8PathBuf) {
        let _ = self.watcher_tx.send(WatcherInstruction::ReindexFile(path));
    }

    /// Returns whether the given path is watched.
    ///
    /// Note that only folders are watched. Passing the path to a file will
    /// always return `false`.
    fn is_watched(&self, path: &Utf8Path) -> bool {
        self.watched_folders.pin().contains(path)
    }

    /// Informs the workspace the given folder is watched.
    ///
    /// Returns `true` if the folder was not yet watched, `false` otherwise.
    fn insert_watched_folder(&self, path: Utf8PathBuf) -> bool {
        self.watched_folders.pin().insert(path)
    }

    /// Removes watched folders from the workspace.
    ///
    /// `callback` is invoked for every currently watched folder with the path
    /// of said folder. Folders for which the `callback` returns `true` are
    /// removed, others are retained.
    fn remove_watched_folders(&self, mut callback: impl FnMut(&Utf8Path) -> bool) {
        self.watched_folders.pin().retain(|path| !callback(path));
    }

    /// Performs a file system scan and indexes all supported documents that
    /// qualify for the given `scan_kind`.
    #[instrument(level = "debug", skip(self, workspace))]
    fn scan<W: WorkspaceScannerBridge>(
        &self,
        workspace: &W,
        project_key: ProjectKey,
        folder: &Utf8Path,
        trigger: IndexTrigger,
        ScanOptions {
            scan_kind,
            force,
            verbose,
            watch,
        }: ScanOptions,
    ) -> Result<ScanResult, WorkspaceError> {
        let (interner, _path_receiver) = PathInterner::new();
        let (diagnostics_sender, diagnostics_receiver) = unbounded();

        let collector = DiagnosticsCollector::new(verbose);

        // The traversal context is scoped to ensure all the channels it
        // contains are properly closed once scanning finishes.
        #[cfg_attr(target_family = "wasm", allow(unused_mut))]
        let mut ctx = ScanContext {
            workspace,
            project_key,
            interner,
            diagnostics_sender,
            evaluated_paths: Default::default(),
            scan_kind,
            trigger,
            watch,
            dependencies: Default::default(),
        };

        #[cfg(not(target_family = "wasm"))]
        let (duration, diagnostics, configuration_files) = thread::scope(|scope| {
            let handler = thread::Builder::new()
                .name("biome::scanner".to_string())
                .spawn_scoped(scope, || collector.run(diagnostics_receiver))
                .expect("failed to spawn scanner thread");

            let ScanFolderResult {
                mut duration,
                configuration_files,
            } = self.scan_folder(folder, &ctx);

            let dependencies_mutex = mem::take(&mut ctx.dependencies);
            let dependencies = dependencies_mutex.into_inner().unwrap();
            if !dependencies.is_empty() {
                let (dependencies_duration, folders_to_watch) =
                    scan_dependencies(folder, dependencies.into_iter().collect(), &mut ctx);

                duration += dependencies_duration;

                let _ = self
                    .watcher_tx
                    .try_send(WatcherInstruction::WatchFolders(folders_to_watch));
            }

            // Make sure the diagnostics sender is dropped.
            drop(ctx);

            // Wait for the collector thread to finish.
            let diagnostics = handler.join().unwrap();

            (duration, diagnostics, configuration_files)
        });

        // On WASM platforms, run the scanner without spawning a thread.
        // The watcher is also not available, and we don't scan dependencies
        // either.
        #[cfg(target_family = "wasm")]
        let (duration, diagnostics, configuration_files) = {
            let ScanFolderResult {
                duration,
                configuration_files,
                ..
            } = self.scan_folder(folder, &ctx);

            // Close the diagnostics channel before collecting to avoid a deadlock on WASM.
            drop(ctx);
            let diagnostics = collector.run(diagnostics_receiver);

            (duration, diagnostics, configuration_files)
        };

        Ok(ScanResult {
            diagnostics,
            duration,
            configuration_files,
        })
    }

    /// Initiates the filesystem traversal tasks from the provided path and runs it to completion.
    ///
    /// Returns the duration of the process and the evaluated paths.
    #[instrument(level = "debug", skip(self, ctx))]
    fn scan_folder<W: WorkspaceScannerBridge>(
        &self,
        folder: &Utf8Path,
        ctx: &ScanContext<W>,
    ) -> ScanFolderResult {
        let start = web_time::Instant::now();

        let workspace = ctx.workspace;
        let fs = workspace.fs();
        fs.traversal(Box::new(move |scope: &dyn TraversalScope| {
            scope.evaluate(ctx, folder.to_path_buf());
        }));

        let evaluated_paths = ctx.evaluated_paths();
        let mut configs = Vec::new();
        let mut manifests = Vec::new();
        let mut handleable_paths = Vec::with_capacity(evaluated_paths.len());
        let mut ignore_paths = Vec::new();
        let mut folders_to_watch = FxHashSet::<Utf8PathBuf>::default();

        // We want to process files that closest to the project root first. For
        // example, we must process first the `.gitignore` at the root of the
        // project.
        for path in evaluated_paths {
            if path.is_config() {
                configs.push(path);
            } else if path.is_manifest() {
                manifests.push(path);
            } else if path.is_ignore() {
                ignore_paths.push(path);
            } else if ctx.watch && fs.symlink_path_kind(&path).is_ok_and(PathKind::is_dir) {
                folders_to_watch.insert(path.into());
            } else {
                handleable_paths.push(path);
            }
        }
        let _ = self
            .watcher_tx
            .try_send(WatcherInstruction::WatchFolders(folders_to_watch));
        fs.traversal(Box::new(|scope: &dyn TraversalScope| {
            for path in &configs {
                scope.handle(ctx, path.to_path_buf());
            }
        }));
        fs.traversal(Box::new(|scope: &dyn TraversalScope| {
            for path in &manifests {
                scope.handle(ctx, path.to_path_buf());
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

        let result = workspace.update_project_ignore_files(ctx.project_key, &ignore_paths);
        if let Err(error) = result {
            ctx.send_diagnostic(error);
        }

        fs.traversal(Box::new(|scope: &dyn TraversalScope| {
            for path in &handleable_paths {
                scope.handle(ctx, path.to_path_buf());
            }
        }));

        ScanFolderResult {
            duration: start.elapsed(),
            configuration_files: configs,
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
        self.unload_folder(project_path);

        self.projects.pin().remove(&project_key);
    }
}

struct ScanFolderResult {
    /// Duration of the scan.
    pub duration: Duration,

    /// List of nested configuration files found inside the folder.
    ///
    /// The root configuration is not included in this.
    pub configuration_files: Vec<BiomePath>,
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
            let dependencies = open_file(ctx, BiomePath::new(dependency_path), ctx.trigger);
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

    /// File paths interner cache used by the filesystem traversal.
    interner: PathInterner,

    /// Channel for reporting diagnostics during scanning.
    pub(crate) diagnostics_sender: Sender<Diagnostic>,

    /// List of paths that should be processed.
    pub(crate) evaluated_paths: RwLock<BTreeSet<BiomePath>>,

    /// What the scanner should target.
    scan_kind: ScanKind,

    /// The trigger that initiated this scan.
    trigger: IndexTrigger,

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
        }
    }

    pub const fn is_project(&self) -> bool {
        matches!(self, Self::Project)
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
}

impl<W: WorkspaceScannerBridge> TraversalContext for ScanContext<'_, W> {
    fn interner(&self) -> &PathInterner {
        &self.interner
    }

    fn evaluated_paths(&self) -> BTreeSet<BiomePath> {
        self.evaluated_paths.read().unwrap().clone()
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
    fn can_handle(&self, path: &BiomePath) -> bool {
        match self.workspace.is_ignored(
            self.project_key,
            &self.scan_kind,
            path,
            IndexRequestKind::Explicit(self.trigger),
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
        self.evaluated_paths.write().unwrap().insert(path);
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
                Ok(description) => Panic::with_file_and_message(&path, *description),
                Err(err) => match err.downcast::<&'static str>() {
                    Ok(description) => Panic::with_file_and_message(&path, *description),
                    Err(_) => Panic::with_file(&path),
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
