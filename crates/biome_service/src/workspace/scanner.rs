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
use super::{FeaturesBuilder, IsPathIgnoredParams};
use crate::diagnostics::Panic;
use crate::projects::ProjectKey;
use crate::workspace::DocumentFileSource;
use crate::{Workspace, WorkspaceError};
use biome_diagnostics::serde::Diagnostic;
use biome_diagnostics::{Diagnostic as _, Error, Severity};
use biome_fs::{BiomePath, PathInterner, PathKind, TraversalContext, TraversalScope};
use camino::Utf8Path;
use crossbeam::channel::{Receiver, Sender, unbounded};
use std::collections::BTreeSet;
use std::panic::catch_unwind;
use std::sync::RwLock;
use std::thread;
use std::time::{Duration, Instant};
use tracing::instrument;

pub(crate) struct ScanResult {
    /// Diagnostics reported while scanning the project.
    pub diagnostics: Vec<Diagnostic>,

    /// Duration of the full scan.
    pub duration: Duration,

    /// List of additional configuration files found inside the project (it doesn't contain the current one)
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
        scan_kind: ScanKind,
    ) -> Result<ScanResult, WorkspaceError> {
        let (interner, _path_receiver) = PathInterner::new();
        let (diagnostics_sender, diagnostics_receiver) = unbounded();

        let collector = DiagnosticsCollector::new();

        let (duration, diagnostics, configuration_files) = thread::scope(|scope| {
            let handler = thread::Builder::new()
                .name("biome::scanner".to_string())
                .spawn_scoped(scope, || collector.run(diagnostics_receiver))
                .expect("failed to spawn scanner thread");

            // The traversal context is scoped to ensure all the channels it
            // contains are properly closed once scanning finishes.
            let (duration, configuration_files) = scan_folder(
                folder,
                ScanContext {
                    workspace: self,
                    project_key,
                    interner,
                    diagnostics_sender,
                    evaluated_paths: Default::default(),
                    scan_kind,
                },
            );

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
}

/// Initiates the filesystem traversal tasks from the provided path and runs it to completion.
///
/// Returns the duration of the process and the evaluated paths.
#[instrument(level = "debug", skip(ctx))]
fn scan_folder(folder: &Utf8Path, ctx: ScanContext) -> (Duration, Vec<BiomePath>) {
    let start = Instant::now();
    let fs = ctx.workspace.fs();
    let ctx_ref = &ctx;
    fs.traversal(Box::new(move |scope: &dyn TraversalScope| {
        scope.evaluate(ctx_ref, folder.to_path_buf());
    }));

    let evaluated_paths = ctx.evaluated_paths();
    let mut configs = Vec::new();
    let mut manifests = Vec::new();
    let mut handleable_paths = Vec::new();
    let mut ignore_paths = Vec::new();
    // We want to process files that closest to the project root first. For example, we must process
    // first the `.gitignore` at the root of the project.
    let iter = evaluated_paths.into_iter();

    for path in iter {
        if path.is_config() {
            configs.push(path);
        } else if path.is_manifest() {
            manifests.push(path);
        } else if path.is_ignore() {
            ignore_paths.push(path);
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

    (start.elapsed(), configs)
}

struct DiagnosticsCollector {
    /// The minimum level of diagnostic we should collect.
    diagnostic_level: Severity,

    /// Whether we should collect verbose diagnostics.
    verbose: bool,
}

impl DiagnosticsCollector {
    fn new() -> Self {
        Self {
            diagnostic_level: Severity::Hint,
            verbose: false,
        }
    }

    /// Checks whether the given `diagnostic` should be collected or not.
    fn should_collect(&self, diagnostic: &Diagnostic) -> bool {
        diagnostic.severity() >= self.diagnostic_level
            && (self.verbose || !diagnostic.tags().is_verbose())
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ScanKind {
    /// The scanner should not be triggered
    None,
    /// It targets specific files
    KnownFiles,
    /// It targets the project, so it attempts to open all the files in the project.
    Project,
}

impl ScanKind {
    pub const fn is_project(self) -> bool {
        matches!(self, Self::Project)
    }

    pub const fn is_known_files(self) -> bool {
        matches!(self, Self::KnownFiles)
    }

    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
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

    // This is the first filtering we apply at the scanner. In this function `can_handle`
    // We roughly understand which files should be open by the scanner.
    // Here, we mostly do file operations by reading their metadata.
    fn can_handle(&self, path: &BiomePath) -> bool {
        if self
            .workspace
            .projects
            .is_ignored_by_scanner(self.project_key, path)
        {
            return false;
        }

        match self.workspace.fs().symlink_path_kind(path) {
            Ok(PathKind::Directory { .. }) => {
                if self.scan_kind.is_project() && path.is_dependency() {
                    // In project mode, the scanner always scans dependencies
                    // because they're a valuable source of type information.
                    true
                } else if !path.is_dependency() {
                    !self
                        .workspace
                        .is_path_ignored(IsPathIgnoredParams {
                            project_key: self.project_key,
                            path: path.clone(),
                            // The scanner only cares about the top-level
                            // `files.includes`
                            features: FeaturesBuilder::new().build(),
                        })
                        .unwrap_or_default()
                } else {
                    false
                }
            }
            Ok(PathKind::File { .. }) => match self.scan_kind {
                ScanKind::KnownFiles => path.is_required_during_scan() && !path.is_dependency(),
                ScanKind::Project => {
                    if path.is_dependency() {
                        path.is_package_json() || path.is_type_declaration()
                    } else {
                        path.is_required_during_scan()
                            || DocumentFileSource::try_from_path(path).is_ok()
                    }
                }
                ScanKind::None => false,
            },
            Err(_) => {
                // bail on fifo and socket files
                false
            }
        }
    }

    fn handle_path(&self, path: BiomePath) {
        open_file(self, &path);
    }

    fn store_path(&self, path: BiomePath) {
        self.evaluated_paths
            .write()
            .unwrap()
            .insert(BiomePath::new(path.as_path()));
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
            ctx.send_diagnostic(err);
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
