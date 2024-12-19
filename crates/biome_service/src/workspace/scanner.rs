use std::collections::BTreeSet;
use std::panic::catch_unwind;
use std::path::Path;
use std::sync::{Once, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPoolBuilder;

use biome_diagnostics::serde::Diagnostic;
use biome_diagnostics::{Diagnostic as _, Error, Severity};
use biome_fs::{BiomePath, PathInterner, TraversalContext, TraversalScope};

use crate::diagnostics::Panic;
use crate::workspace::{DocumentFileSource, FileContent, OpenFileParams};
use crate::{Workspace, WorkspaceError};

use super::server::WorkspaceServer;

pub(crate) struct ScanResult {
    /// Diagnostics reported while scanning the project.
    pub diagnostics: Vec<Diagnostic>,

    /// Duration of the full scan.
    pub duration: Duration,
}

pub(crate) fn scan(
    workspace: &WorkspaceServer,
    folder: &Path,
) -> Result<ScanResult, WorkspaceError> {
    init_thread_pool();

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
        let (duration, _evaluated_paths) = scan_folder(
            folder,
            &ScanContext {
                workspace,
                interner,
                diagnostics_sender,
                evaluated_paths: Default::default(),
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

/// Sets up the global Rayon thread pool the first time it's called.
///
/// This is used to assign friendly debug names to the threads of the pool.
fn init_thread_pool() {
    static INIT_ONCE: Once = Once::new();
    INIT_ONCE.call_once(|| {
        ThreadPoolBuilder::new()
            .thread_name(|index| format!("biome::workspace_worker_{index}"))
            .build_global()
            .expect("failed to initialize the global thread pool");
    });
}

/// Initiates the filesystem traversal tasks from the provided path and runs it to completion.
///
/// Returns the duration of the process and the evaluated paths.
fn scan_folder(folder: &Path, ctx: &ScanContext) -> (Duration, BTreeSet<BiomePath>) {
    let start = Instant::now();
    let fs = ctx.workspace.fs();
    fs.traversal(Box::new(move |scope: &dyn TraversalScope| {
        scope.evaluate(ctx, folder.to_path_buf());
    }));

    let paths = ctx.evaluated_paths();
    fs.traversal(Box::new(|scope: &dyn TraversalScope| {
        for path in paths {
            scope.handle(ctx, path.to_path_buf());
        }
    }));

    (start.elapsed(), ctx.evaluated_paths())
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

    /// File paths interner cache used by the filesystem traversal.
    interner: PathInterner,

    /// Channel for reporting diagnostics during scanning.
    pub(crate) diagnostics_sender: Sender<Diagnostic>,

    /// List of paths that should be processed.
    pub(crate) evaluated_paths: RwLock<BTreeSet<BiomePath>>,
}

impl<'app> ScanContext<'app> {
    /// Send a message to the display thread
    pub(crate) fn send_diagnostic(&self, diagnostic: impl Into<Diagnostic>) {
        self.diagnostics_sender.send(diagnostic.into()).ok();
    }
}

impl<'app> TraversalContext for ScanContext<'app> {
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
        DocumentFileSource::try_from_path(path).is_ok()
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
            path: path.clone(),
            content: FileContent::FromServer,
            document_file_source: None,
            version: 0,
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
