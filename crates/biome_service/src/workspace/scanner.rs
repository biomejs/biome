//! Scanner for crawling the file system and loading documents into projects.
//!
//! Conceptually, the scanner is more than just the scanning logic in this
//! module. Rather, we also consider the [watcher](crate::WorkspaceWatcher)
//! (which is partly implemented outside the workspace because of its
//! threading requirements) to be a part of the scanner.
//!
//! In other words, the scanner is both the scanning logic in this module as
//! well as the watcher to allow continuous scanning.

use super::server::WorkspaceServer;
use super::{FeaturesBuilder, IgnoreKind, IsPathIgnoredParams};
use crate::diagnostics::Panic;
use crate::projects::ProjectKey;
use crate::workspace::DocumentFileSource;
use crate::{WatcherInstruction, Workspace, WorkspaceError};
use biome_diagnostics::serde::Diagnostic;
use biome_diagnostics::{Diagnostic as _, DiagnosticExt, Error, Severity};
use biome_fs::{BiomePath, PathInterner, PathKind, TraversalContext, TraversalScope};
use camino::{Utf8Path, Utf8PathBuf};
use crossbeam::channel::{Receiver, Sender, unbounded};
use std::collections::BTreeSet;
use std::panic::catch_unwind;
use std::sync::RwLock;
use std::thread;
use std::time::{Duration, Instant};
use tracing::instrument;

pub(crate) struct ScanOptions {
    /// The kind of scan to perform (targeted, project, etc.).
    pub scan_kind: ScanKind,

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

impl WorkspaceServer {
    /// Performs a file system scan and loads all supported documents into the
    /// project.
    #[instrument(level = "debug", skip(self))]
    pub(super) fn scan(
        &self,
        project_key: ProjectKey,
        folder: &Utf8Path,
        ScanOptions {
            scan_kind,
            verbose,
            watch,
        }: ScanOptions,
    ) -> Result<ScanResult, WorkspaceError> {
        let (interner, _path_receiver) = PathInterner::new();
        let (diagnostics_sender, diagnostics_receiver) = unbounded();

        let collector = DiagnosticsCollector::new(verbose);

        let (duration, diagnostics, configuration_files) = thread::scope(|scope| {
            let handler = thread::Builder::new()
                .name("biome::scanner".to_string())
                .spawn_scoped(scope, || collector.run(diagnostics_receiver))
                .expect("failed to spawn scanner thread");

            // The traversal context is scoped to ensure all the channels it
            // contains are properly closed once scanning finishes.
            let ScanFolderResult {
                duration,
                configuration_files,
                folders_to_watch,
            } = scan_folder(
                folder,
                ScanContext {
                    workspace: self,
                    project_key,
                    interner,
                    diagnostics_sender,
                    evaluated_paths: Default::default(),
                    scan_kind: scan_kind.clone(),
                    watch,
                },
            );

            for folder in folders_to_watch {
                let _ = self
                    .watcher_tx
                    .try_send(WatcherInstruction::WatchFolder(folder, scan_kind.clone()));
            }

            // Wait for the collector thread to finish.
            let diagnostics = handler.join().unwrap();

            (duration, diagnostics, configuration_files)
        });

        Ok(ScanResult {
            diagnostics,
            duration,
            configuration_files,
        })
    }

    pub(super) fn is_ignored_by_scanner(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &BiomePath,
        ignore_kind: IgnoreKind,
    ) -> Result<bool, WorkspaceError> {
        if self.projects.is_ignored_by_scanner(project_key, path) {
            return Ok(true);
        }

        let is_ignored = match self.fs().symlink_path_kind(path)? {
            PathKind::Directory { .. } => {
                if path.is_dependency() {
                    // Every mode ignores dependencies, except project mode.
                    return Ok(!scan_kind.is_project());
                }

                if self
                    .projects
                    .is_ignored_by_top_level_config(project_key, path, ignore_kind)
                {
                    return Ok(true); // Nobody cares about ignored paths.
                }

                if let ScanKind::TargetedKnownFiles {
                    target_paths,
                    descend_from_targets,
                } = &scan_kind
                    && !target_paths.iter().any(|target_path| {
                        target_path.starts_with(path.as_path())
                            || (*descend_from_targets && path.starts_with(target_path.as_path()))
                    })
                {
                    return Ok(true); // Path is not being targeted.
                }

                false
            }
            PathKind::File { .. } => match scan_kind {
                ScanKind::KnownFiles | ScanKind::TargetedKnownFiles { .. } => {
                    if path.is_config() {
                        self.projects
                            .is_ignored_by_top_level_config(project_key, path, ignore_kind)
                    } else {
                        !path.is_ignore() && !path.is_manifest()
                    }
                }
                ScanKind::Project => {
                    if path.is_dependency() {
                        // For dependencies, we ignore everything except
                        // manifests and type declarations.
                        !path.is_package_json() && !path.is_type_declaration()
                    } else {
                        !path.is_required_during_scan()
                            && DocumentFileSource::try_from_path(path).is_err()
                    }
                }
                ScanKind::NoScanner => true,
            },
        };

        Ok(is_ignored)
    }
}

struct ScanFolderResult {
    /// Duration of the scan.
    pub duration: Duration,

    /// List of nested configuration files found inside the folder.
    ///
    /// The root configuration is not included in this.
    pub configuration_files: Vec<BiomePath>,

    /// List of directories that need to be watched.
    pub folders_to_watch: Vec<Utf8PathBuf>,
}

/// Initiates the filesystem traversal tasks from the provided path and runs it to completion.
///
/// Returns the duration of the process and the evaluated paths.
#[instrument(level = "debug", skip(ctx))]
fn scan_folder(folder: &Utf8Path, ctx: ScanContext) -> ScanFolderResult {
    let start = Instant::now();
    let fs = ctx.workspace.fs();
    let ctx_ref = &ctx;
    fs.traversal(Box::new(move |scope: &dyn TraversalScope| {
        scope.evaluate(ctx_ref, folder.to_path_buf());
    }));

    let evaluated_paths = ctx.evaluated_paths();
    let mut configs = Vec::new();
    let mut manifests = Vec::new();
    let mut handleable_paths = Vec::with_capacity(evaluated_paths.len());
    let mut ignore_paths = Vec::new();
    let mut folders_to_watch = Vec::new();

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
            folders_to_watch.push(path.into());
        } else {
            handleable_paths.push(path);
        }
    }
    fs.traversal(Box::new(|scope: &dyn TraversalScope| {
        for path in &configs {
            scope.handle(ctx_ref, path.to_path_buf());
        }
    }));
    fs.traversal(Box::new(|scope: &dyn TraversalScope| {
        for path in &manifests {
            scope.handle(ctx_ref, path.to_path_buf());
        }
    }));

    let result = ctx
        .workspace
        .update_project_config_files(ctx.project_key, &configs);

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

    let result = ctx
        .workspace
        .update_project_ignore_files(ctx.project_key, &ignore_paths);

    if let Err(error) = result {
        ctx.send_diagnostic(error);
    }

    fs.traversal(Box::new(|scope: &dyn TraversalScope| {
        for path in &handleable_paths {
            scope.handle(ctx_ref, path.to_path_buf());
        }
    }));

    ScanFolderResult {
        duration: start.elapsed(),
        configuration_files: configs,
        folders_to_watch,
    }
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
        diagnostic.severity() >= self.diagnostic_level
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
pub(crate) struct ScanContext<'app> {
    /// [Workspace] instance.
    pub(crate) workspace: &'app WorkspaceServer,

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

    /// Whether the scanned folders need to be watched.
    watch: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ScanKind {
    /// The scanner should not be triggered.
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

impl ScanContext<'_> {
    /// Send a message to the display thread
    pub(crate) fn send_diagnostic(&self, diagnostic: impl Into<Diagnostic>) {
        self.diagnostics_sender.send(diagnostic.into()).ok();
    }
}

impl TraversalContext for ScanContext<'_> {
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
        match self.workspace.is_ignored_by_scanner(
            self.project_key,
            &self.scan_kind,
            path,
            IgnoreKind::Path,
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
        open_file(self, &path);
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
fn open_file(ctx: &ScanContext, path: &BiomePath) {
    match catch_unwind(move || {
        let is_ignored = if ctx.scan_kind.is_project() && path.is_dependency() {
            !(path.is_package_json() || path.is_type_declaration())
        } else if path.is_required_during_scan() {
            // Required files are only ignored if they are in an ignored
            // directory.
            path.parent()
                .and_then(|dir_path| {
                    ctx.workspace
                        .is_path_ignored(IsPathIgnoredParams {
                            project_key: ctx.project_key,
                            path: dir_path.into(),
                            features: FeaturesBuilder::new().build(),
                            ignore_kind: IgnoreKind::Path,
                        })
                        .ok()
                })
                .unwrap_or_default()
        } else {
            ctx.workspace
                .is_path_ignored(IsPathIgnoredParams {
                    project_key: ctx.project_key,
                    path: path.clone(),
                    features: FeaturesBuilder::new().build(),
                    ignore_kind: IgnoreKind::Path,
                })
                .unwrap_or_default()
        };
        if is_ignored {
            return Ok(());
        }

        ctx.workspace
            .open_file_during_initial_scan(ctx.project_key, path.clone())
    }) {
        Ok(Ok(())) => {}
        Ok(Err(err)) => {
            let mut error: Error = err.into();
            if !path.is_config() && error.severity() == Severity::Error {
                error = error.with_severity(Severity::Warning);
            }
            ctx.send_diagnostic(Diagnostic::new(error));
        }
        Err(err) => {
            let error = match err.downcast::<String>() {
                Ok(description) => Panic::with_file_and_message(path, *description),
                Err(err) => match err.downcast::<&'static str>() {
                    Ok(description) => Panic::with_file_and_message(path, *description),
                    Err(_) => Panic::with_file(path),
                },
            };

            ctx.send_diagnostic(error);
        }
    }
}

#[cfg(test)]
#[path = "scanner.tests.rs"]
mod tests;
