//! Scanner for crawling the file system and loading documents into projects.
//!
//! Conceptually, the scanner is more than just the scanning logic in this
//! module. Rather, we also consider the [watcher](crate::WorkspaceWatcher)
//! (which is partly implemented outside the workspace because of its
//! threading requirements) to be a part of the scanner.
//!
//! In other words, the scanner is both the scanning logic in this module as
//! well as the watcher to allow continuous scanning.

use super::ServiceDataNotification;
use super::server::WorkspaceServer;
use crate::diagnostics::Panic;
use crate::projects::ProjectKey;
use crate::workspace::{DocumentFileSource, FileContent, OpenFileParams};
use crate::workspace_watcher::WatcherSignalKind;
use crate::{Workspace, WorkspaceError};
use biome_diagnostics::serde::Diagnostic;
use biome_diagnostics::{Diagnostic as _, Error, Severity};
use biome_fs::{BiomePath, PathInterner, TraversalContext, TraversalScope};
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

        let (duration, diagnostics) = thread::scope(|scope| {
            let handler = thread::Builder::new()
                .name("biome::scanner".to_string())
                .spawn_scoped(scope, || collector.run(diagnostics_receiver))
                .expect("failed to spawn scanner thread");

            // The traversal context is scoped to ensure all the channels it
            // contains are properly closed once scanning finishes.
            let duration = scan_folder(
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

            (duration, diagnostics)
        });

        Ok(ScanResult {
            diagnostics,
            duration,
        })
    }
}

/// Initiates the filesystem traversal tasks from the provided path and runs it to completion.
///
/// Returns the duration of the process and the evaluated paths.
#[instrument(level = "debug", skip(ctx))]
fn scan_folder(folder: &Utf8Path, ctx: ScanContext) -> Duration {
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
    for path in evaluated_paths {
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

    let mut paths = configs;
    paths.append(&mut manifests);
    ctx.workspace
        .update_project_layout_for_paths(WatcherSignalKind::AddedOrChanged, &paths);
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

    ctx.workspace
        .update_module_graph(WatcherSignalKind::AddedOrChanged, &handleable_paths);

    let _ = ctx
        .workspace
        .notification_tx
        .send(ServiceDataNotification::Updated);

    start.elapsed()
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

/// Path entries that we want to ignore during the scanning phase.
pub const SCANNER_IGNORE_ENTRIES: &[&[u8]] = &[
    b".cache",
    b".git",
    b".hg",
    b".svn",
    b".yarn",
    b".timestamp",
    b".DS_Store",
];

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

    fn can_handle(&self, path: &BiomePath) -> bool {
        if path
            .file_name()
            .is_some_and(|file_name| SCANNER_IGNORE_ENTRIES.contains(&file_name.as_bytes()))
        {
            return false;
        }

        if path.is_dir() {
            return true;
        }
        if self.scan_kind.is_known_files() {
            // we don't need to check files inside node_modules if we are in known files mode
            if path.is_dependency() {
                false
            } else {
                path.is_ignore() || path.is_config() || path.is_manifest()
            }
        } else if path.is_dependency() {
            path.ends_with(".d.ts")
        } else {
            DocumentFileSource::try_from_path(path).is_ok() || path.is_ignore()
        }
    }

    fn handle_path(&self, path: BiomePath) {
        open_file(self, &path)
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
        ctx.workspace.open_file_by_scanner(OpenFileParams {
            project_key: ctx.project_key,
            path: path.clone(),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
        })
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
