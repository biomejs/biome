use super::process_file::{process_file, DiffKind, FileStatus, Message};
use super::{Execution, TraversalMode};
use crate::cli_options::CliOptions;
use crate::execute::diagnostics::{
    AssistDiffDiagnostic, CIAssistDiffDiagnostic, CIFormatDiffDiagnostic, ContentDiffAdvice,
    FormatDiffDiagnostic, PanicDiagnostic,
};
use crate::reporter::TraversalSummary;
use crate::{CliDiagnostic, CliSession};
use biome_diagnostics::DiagnosticTags;
use biome_diagnostics::{category, DiagnosticExt, Error, Resource, Severity};
use biome_fs::{BiomePath, FileSystem, PathInterner};
use biome_fs::{TraversalContext, TraversalScope};
use biome_service::dome::Dome;
use biome_service::projects::ProjectKey;
use biome_service::workspace::{DropPatternParams, IsPathIgnoredParams};
use biome_service::{extension_error, workspace::SupportsFeatureParams, Workspace, WorkspaceError};
use camino::{Utf8Path, Utf8PathBuf};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rustc_hash::FxHashSet;
use std::collections::BTreeSet;
use std::sync::atomic::AtomicU32;
use std::sync::RwLock;
use std::{
    env::current_dir,
    ffi::OsString,
    panic::catch_unwind,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};
use tracing::{instrument, Span};

pub(crate) struct TraverseResult {
    pub(crate) summary: TraversalSummary,
    pub(crate) evaluated_paths: BTreeSet<BiomePath>,
    pub(crate) diagnostics: Vec<Error>,
}

pub(crate) fn traverse(
    execution: &Execution,
    session: &mut CliSession,
    project_key: ProjectKey,
    cli_options: &CliOptions,
    mut inputs: Vec<OsString>,
) -> Result<TraverseResult, CliDiagnostic> {
    if inputs.is_empty() {
        match &execution.traversal_mode {
            TraversalMode::Check { .. }
            | TraversalMode::Lint { .. }
            | TraversalMode::Format { .. }
            | TraversalMode::CI { .. }
            | TraversalMode::Search { .. } => {
                // If `--staged` or `--changed` is specified, it's acceptable for them to be empty, so ignore it.
                if !execution.is_vcs_targeted() {
                    match current_dir() {
                        Ok(current_dir) => inputs.push(current_dir.into_os_string()),
                        Err(err) => return Err(CliDiagnostic::io_error(err)),
                    }
                }
            }
            _ => {
                if execution.as_stdin_file().is_none() && !cli_options.no_errors_on_unmatched {
                    return Err(CliDiagnostic::missing_argument(
                        "<INPUT>",
                        format!("{}", execution.traversal_mode),
                    ));
                }
            }
        }
    }

    let (interner, recv_files) = PathInterner::new();
    let (sender, receiver) = unbounded();

    let changed = AtomicUsize::new(0);
    let unchanged = AtomicUsize::new(0);
    let matches = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);

    let workspace = &*session.app.workspace;
    let fs = workspace.fs();

    let max_diagnostics = execution.get_max_diagnostics();
    let remaining_diagnostics = AtomicU32::new(max_diagnostics);

    let working_directory = fs.working_directory();
    let printer = DiagnosticsPrinter::new(execution, working_directory.as_deref())
        .with_verbose(cli_options.verbose)
        .with_diagnostic_level(cli_options.diagnostic_level)
        .with_max_diagnostics(max_diagnostics);

    let (duration, evaluated_paths, diagnostics) = thread::scope(|s| {
        let handler = thread::Builder::new()
            .name(String::from("biome::console"))
            .spawn_scoped(s, || printer.run(receiver, recv_files))
            .expect("failed to spawn console thread");

        // The traversal context is scoped to ensure all the channels it
        // contains are properly closed once the traversal finishes
        let (elapsed, evaluated_paths) = traverse_inputs(
            fs,
            &inputs,
            &TraversalOptions {
                fs,
                workspace,
                project_key,
                execution,
                interner,
                matches: &matches,
                changed: &changed,
                unchanged: &unchanged,
                skipped: &skipped,
                messages: sender,
                remaining_diagnostics: &remaining_diagnostics,
                evaluated_paths: RwLock::default(),
            },
        );
        // wait for the main thread to finish
        let diagnostics = handler.join().unwrap();

        (elapsed, evaluated_paths, diagnostics)
    });

    // Make sure patterns are always cleaned up at the end of traversal.
    if let TraversalMode::Search { pattern, .. } = execution.traversal_mode() {
        let _ = session.app.workspace.drop_pattern(DropPatternParams {
            pattern: pattern.clone(),
        });
    }

    let errors = printer.errors();
    let warnings = printer.warnings();
    let changed = changed.load(Ordering::Relaxed);
    let unchanged = unchanged.load(Ordering::Relaxed);
    let matches = matches.load(Ordering::Relaxed);
    let skipped = skipped.load(Ordering::Relaxed);
    let suggested_fixes_skipped = printer.skipped_fixes();
    let diagnostics_not_printed = printer.not_printed_diagnostics();
    Ok(TraverseResult {
        summary: TraversalSummary {
            changed,
            unchanged,
            duration,
            errors,
            matches,
            warnings,
            skipped,
            suggested_fixes_skipped,
            diagnostics_not_printed,
        },
        evaluated_paths,
        diagnostics,
    })
}

/// Initiate the filesystem traversal tasks with the provided input paths and
/// run it to completion, returning the duration of the process and the evaluated paths
fn traverse_inputs(
    fs: &dyn FileSystem,
    inputs: &[OsString],
    ctx: &TraversalOptions,
) -> (Duration, BTreeSet<BiomePath>) {
    let start = Instant::now();
    fs.traversal(Box::new(move |scope: &dyn TraversalScope| {
        for input in inputs {
            scope.evaluate(
                ctx,
                Utf8PathBuf::from_path_buf(input.into()).expect("Valid UTF-8 path"),
            );
        }
    }));

    let paths = ctx.evaluated_paths();
    let dome = Dome::new(paths);
    let mut iter = dome.iter();
    fs.traversal(Box::new(|scope: &dyn TraversalScope| {
        while let Some(path) = iter.next_config() {
            scope.handle(ctx, path.to_path_buf());
        }

        while let Some(path) = iter.next_manifest() {
            scope.handle(ctx, path.to_path_buf());
        }

        for path in iter {
            scope.handle(ctx, path.to_path_buf());
        }
    }));

    (start.elapsed(), ctx.evaluated_paths())
}

// struct DiagnosticsReporter<'ctx> {}

struct DiagnosticsPrinter<'ctx> {
    ///  Execution of the traversal
    execution: &'ctx Execution,
    /// The maximum number of diagnostics the console thread is allowed to print
    max_diagnostics: u32,
    /// The approximate number of diagnostics the console will print before
    /// folding the rest into the "skipped diagnostics" counter
    remaining_diagnostics: AtomicU32,
    /// Mutable reference to a boolean flag tracking whether the console thread
    /// printed any error-level message
    errors: AtomicU32,
    /// Mutable reference to a boolean flag tracking whether the console thread
    /// printed any warnings-level message
    warnings: AtomicU32,
    /// Whether the console thread should print diagnostics in verbose mode
    verbose: bool,
    /// The diagnostic level the console thread should print
    diagnostic_level: Severity,

    not_printed_diagnostics: AtomicU32,
    printed_diagnostics: AtomicU32,
    total_skipped_suggested_fixes: AtomicU32,

    /// The current working directory, borrowed from [FileSystem]
    working_directory: Option<&'ctx Utf8Path>,
}

impl<'ctx> DiagnosticsPrinter<'ctx> {
    fn new(execution: &'ctx Execution, working_directory: Option<&'ctx Utf8Path>) -> Self {
        Self {
            errors: AtomicU32::new(0),
            warnings: AtomicU32::new(0),
            remaining_diagnostics: AtomicU32::new(0),
            execution,
            diagnostic_level: Severity::Hint,
            verbose: false,
            max_diagnostics: 20,
            not_printed_diagnostics: AtomicU32::new(0),
            printed_diagnostics: AtomicU32::new(0),
            total_skipped_suggested_fixes: AtomicU32::new(0),
            working_directory,
        }
    }

    fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    fn with_max_diagnostics(mut self, value: u32) -> Self {
        self.max_diagnostics = value;
        self
    }

    fn with_diagnostic_level(mut self, value: Severity) -> Self {
        self.diagnostic_level = value;
        self
    }

    fn errors(&self) -> u32 {
        self.errors.load(Ordering::Relaxed)
    }

    fn warnings(&self) -> u32 {
        self.warnings.load(Ordering::Relaxed)
    }

    fn not_printed_diagnostics(&self) -> u32 {
        self.not_printed_diagnostics.load(Ordering::Relaxed)
    }

    fn skipped_fixes(&self) -> u32 {
        self.total_skipped_suggested_fixes.load(Ordering::Relaxed)
    }

    /// Checks if the diagnostic we received from the thread should be considered or not. Logic:
    /// - it should not be considered if its severity level is lower than the one provided via CLI;
    /// - it should not be considered if it's a verbose diagnostic and the CLI **didn't** request a `--verbose` option.
    fn should_skip_diagnostic(&self, severity: Severity, diagnostic_tags: DiagnosticTags) -> bool {
        if severity < self.diagnostic_level {
            return true;
        }

        if diagnostic_tags.is_verbose() && !self.verbose {
            return true;
        }

        false
    }

    /// Count the diagnostic, and then returns a boolean that tells if it should be printed
    fn should_print(&self) -> bool {
        let printed_diagnostics = self.printed_diagnostics.load(Ordering::Relaxed);
        let should_print = printed_diagnostics < self.max_diagnostics;
        if should_print {
            self.printed_diagnostics.fetch_add(1, Ordering::Relaxed);
            self.remaining_diagnostics.store(
                self.max_diagnostics.saturating_sub(printed_diagnostics),
                Ordering::Relaxed,
            );
        } else {
            self.not_printed_diagnostics.fetch_add(1, Ordering::Relaxed);
        }

        should_print
    }

    fn run(&self, receiver: Receiver<Message>, interner: Receiver<Utf8PathBuf>) -> Vec<Error> {
        let mut paths: FxHashSet<String> = FxHashSet::default();

        let mut diagnostics_to_print = vec![];

        while let Ok(msg) = receiver.recv() {
            match msg {
                Message::SkippedFixes {
                    skipped_suggested_fixes,
                } => {
                    self.total_skipped_suggested_fixes
                        .fetch_add(skipped_suggested_fixes, Ordering::Relaxed);
                }

                Message::Failure => {
                    self.errors.fetch_add(1, Ordering::Relaxed);
                }

                Message::Error(mut err) => {
                    let location = err.location();
                    if self.should_skip_diagnostic(err.severity(), err.tags()) {
                        continue;
                    }
                    if err.severity() == Severity::Warning {
                        // *warnings += 1;
                        self.warnings.fetch_add(1, Ordering::Relaxed);
                        // self.warnings.set(self.warnings.get() + 1)
                    }
                    if let Some(Resource::File(file_path)) = location.resource.as_ref() {
                        // Retrieves the file name from the file ID cache, if it's a miss
                        // flush entries from the interner channel until it's found
                        let file_name = match paths.get(*file_path) {
                            Some(path) => Some(path),
                            None => loop {
                                match interner.recv() {
                                    Ok(path) => {
                                        paths.insert(path.to_string());
                                        if path.as_str() == *file_path {
                                            break paths.get(&path.to_string());
                                        }
                                    }
                                    // In case the channel disconnected without sending
                                    // the path we need, print the error without a file
                                    // name (normally this should never happen)
                                    Err(_) => break None,
                                }
                            },
                        };

                        if let Some(path) = file_name {
                            err = err.with_file_path(path.as_str());
                        }
                    }

                    let should_print = self.should_print();

                    if should_print {
                        diagnostics_to_print.push(err);
                    }
                }

                Message::Diagnostics {
                    file_path,
                    content,
                    diagnostics,
                    skipped_diagnostics,
                } => {
                    self.not_printed_diagnostics
                        .fetch_add(skipped_diagnostics, Ordering::Relaxed);

                    // is CI mode we want to print all the diagnostics
                    if self.execution.is_ci() {
                        for diag in diagnostics {
                            let severity = diag.severity();
                            if self.should_skip_diagnostic(severity, diag.tags()) {
                                continue;
                            }

                            if severity == Severity::Error {
                                self.errors.fetch_add(1, Ordering::Relaxed);
                            }
                            if severity == Severity::Warning {
                                self.warnings.fetch_add(1, Ordering::Relaxed);
                            }

                            let file_path = self
                                .working_directory
                                .and_then(|wd| file_path.strip_prefix(wd.as_str()))
                                .unwrap_or(file_path.as_str());
                            let diag = diag
                                .with_file_path(file_path)
                                .with_file_source_code(&content);
                            diagnostics_to_print.push(diag);
                        }
                    } else {
                        for diag in diagnostics {
                            let severity = diag.severity();
                            if self.should_skip_diagnostic(severity, diag.tags()) {
                                continue;
                            }
                            if severity == Severity::Error {
                                self.errors.fetch_add(1, Ordering::Relaxed);
                            }
                            if severity == Severity::Warning {
                                self.warnings.fetch_add(1, Ordering::Relaxed);
                            }

                            let should_print = self.should_print();

                            if should_print {
                                let file_path = self
                                    .working_directory
                                    .and_then(|wd| file_path.strip_prefix(wd.as_str()))
                                    .unwrap_or(file_path.as_str());
                                let diag = diag
                                    .with_file_path(file_path)
                                    .with_file_source_code(&content);
                                diagnostics_to_print.push(diag)
                            }
                        }
                    }
                }
                Message::Diff {
                    file_name,
                    old,
                    new,
                    diff_kind,
                } => {
                    let file_name = self
                        .working_directory
                        .and_then(|wd| file_name.strip_prefix(wd.as_str()))
                        .unwrap_or(file_name.as_str());
                    // A diff is an error in CI mode and in format check mode
                    let is_error = self.execution.is_ci() || !self.execution.is_format_write();
                    if is_error {
                        self.errors.fetch_add(1, Ordering::Relaxed);
                    }

                    let severity: Severity = if is_error {
                        Severity::Error
                    } else {
                        // we set lowest
                        Severity::Hint
                    };

                    if self.should_skip_diagnostic(severity, DiagnosticTags::empty()) {
                        continue;
                    }

                    let should_print = self.should_print();

                    if should_print {
                        if self.execution.is_ci() {
                            match diff_kind {
                                DiffKind::Format => {
                                    let diag = CIFormatDiffDiagnostic {
                                        file_name: file_name.to_string(),
                                        diff: ContentDiffAdvice {
                                            old: old.clone(),
                                            new: new.clone(),
                                        },
                                    };
                                    diagnostics_to_print.push(
                                        diag.with_severity(severity)
                                            .with_file_source_code(old.clone()),
                                    );
                                }
                                DiffKind::Assist => {
                                    let diag = CIAssistDiffDiagnostic {
                                        file_name: file_name.to_string(),
                                        diff: ContentDiffAdvice {
                                            old: old.clone(),
                                            new: new.clone(),
                                        },
                                    };
                                    diagnostics_to_print.push(
                                        diag.with_severity(severity)
                                            .with_file_source_code(old.clone()),
                                    )
                                }
                            };
                        } else {
                            match diff_kind {
                                DiffKind::Format => {
                                    let diag = FormatDiffDiagnostic {
                                        file_name: file_name.to_string(),
                                        diff: ContentDiffAdvice {
                                            old: old.clone(),
                                            new: new.clone(),
                                        },
                                    };
                                    diagnostics_to_print.push(
                                        diag.with_severity(severity)
                                            .with_file_source_code(old.clone()),
                                    )
                                }
                                DiffKind::Assist => {
                                    let diag = AssistDiffDiagnostic {
                                        file_name: file_name.to_string(),
                                        diff: ContentDiffAdvice {
                                            old: old.clone(),
                                            new: new.clone(),
                                        },
                                    };
                                    diagnostics_to_print.push(
                                        diag.with_severity(severity)
                                            .with_file_source_code(old.clone()),
                                    )
                                }
                            };
                        }
                    }
                }
            }
        }
        diagnostics_to_print
    }
}

/// Context object shared between directory traversal tasks
pub(crate) struct TraversalOptions<'ctx, 'app> {
    /// Shared instance of [FileSystem]
    pub(crate) fs: &'app dyn FileSystem,
    /// Instance of [Workspace] used by this instance of the CLI
    pub(crate) workspace: &'ctx dyn Workspace,
    /// Key of the project in which we're traversing.
    pub(crate) project_key: ProjectKey,
    /// Determines how the files should be processed
    pub(crate) execution: &'ctx Execution,
    /// File paths interner cache used by the filesystem traversal
    interner: PathInterner,
    /// Shared atomic counter storing the number of changed files
    changed: &'ctx AtomicUsize,
    /// Shared atomic counter storing the number of unchanged files
    unchanged: &'ctx AtomicUsize,
    /// Shared atomic counter storing the number of unchanged files
    matches: &'ctx AtomicUsize,
    /// Shared atomic counter storing the number of skipped files
    skipped: &'ctx AtomicUsize,
    /// Channel sending messages to the display thread
    pub(crate) messages: Sender<Message>,
    /// The approximate number of diagnostics the console will print before
    /// folding the rest into the "skipped diagnostics" counter
    pub(crate) remaining_diagnostics: &'ctx AtomicU32,

    /// List of paths that should be processed
    pub(crate) evaluated_paths: RwLock<BTreeSet<BiomePath>>,
}

impl TraversalOptions<'_, '_> {
    pub(crate) fn increment_changed(&self, path: &BiomePath) {
        self.changed.fetch_add(1, Ordering::Relaxed);
        self.evaluated_paths
            .write()
            .unwrap()
            .replace(path.to_written());
    }
    pub(crate) fn increment_unchanged(&self) {
        self.unchanged.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn increment_matches(&self, num_matches: usize) {
        self.matches.fetch_add(num_matches, Ordering::Relaxed);
    }

    /// Send a message to the display thread
    pub(crate) fn push_message(&self, msg: impl Into<Message>) {
        self.messages.send(msg.into()).ok();
    }

    pub(crate) fn miss_handler_err(&self, err: WorkspaceError, biome_path: &BiomePath) {
        self.push_diagnostic(
            err.with_category(category!("files/missingHandler"))
                .with_file_path(biome_path.to_string())
                .with_tags(DiagnosticTags::VERBOSE),
        );
    }

    pub(crate) fn protected_file(&self, biome_path: &BiomePath) {
        self.push_diagnostic(WorkspaceError::protected_file(biome_path.to_string()).into())
    }
}

impl TraversalContext for TraversalOptions<'_, '_> {
    fn interner(&self) -> &PathInterner {
        &self.interner
    }

    fn evaluated_paths(&self) -> BTreeSet<BiomePath> {
        self.evaluated_paths.read().unwrap().clone()
    }

    fn push_diagnostic(&self, error: Error) {
        self.push_message(error);
    }

    #[instrument(level = "debug", skip(self, biome_path))]
    fn can_handle(&self, biome_path: &BiomePath) -> bool {
        let path = biome_path.as_path();
        if self.fs.path_is_dir(path) || self.fs.path_is_symlink(path) {
            // handle:
            // - directories
            // - symlinks
            // - unresolved symlinks
            //   e.g `symlink/subdir` where symlink points to a directory that includes `subdir`.
            //   Note that `symlink/subdir` is not an existing file.
            let can_handle = !self
                .workspace
                .is_path_ignored(IsPathIgnoredParams {
                    project_key: self.project_key,
                    path: biome_path.clone(),
                    features: self.execution.to_feature(),
                })
                .unwrap_or_else(|err| {
                    self.push_diagnostic(err.into());
                    false
                });
            Span::current().record("can_handle", can_handle);

            return can_handle;
        }

        // bail on fifo and socket files
        if !self.fs.path_is_file(path) {
            Span::current().record("can_handle", false);
            return false;
        }

        let file_features = self.workspace.file_features(SupportsFeatureParams {
            project_key: self.project_key,
            path: biome_path.clone(),
            features: self.execution.to_feature(),
        });

        let file_features = match file_features {
            Ok(file_features) => {
                if file_features.is_protected() {
                    self.protected_file(biome_path);
                    Span::current().record("can_handle", false);
                    return false;
                }

                if file_features.is_not_supported() && !file_features.is_ignored() {
                    // we should throw a diagnostic if we can't handle a file that isn't ignored
                    self.miss_handler_err(extension_error(biome_path), biome_path);
                    Span::current().record("can_handle", false);
                    return false;
                }
                file_features
            }
            Err(err) => {
                self.miss_handler_err(err, biome_path);
                Span::current().record("can_handle", false);
                return false;
            }
        };
        let result = match self.execution.traversal_mode() {
            TraversalMode::Check { .. } | TraversalMode::CI { .. } => {
                file_features.supports_lint()
                    || file_features.supports_format()
                    || file_features.supports_assist()
            }
            TraversalMode::Format { .. } => file_features.supports_format(),
            TraversalMode::Lint { .. } => file_features.supports_lint(),
            // Imagine if Biome can't handle its own configuration file...
            TraversalMode::Migrate { .. } => true,
            TraversalMode::Search { .. } => file_features.supports_search(),
        };
        Span::current().record("can_handle", result);
        result
    }

    fn handle_path(&self, path: BiomePath) {
        handle_file(self, &path)
    }

    fn store_path(&self, path: BiomePath) {
        self.evaluated_paths
            .write()
            .unwrap()
            .insert(BiomePath::new(path.as_path()));
    }
}

/// This function wraps the [process_file] function implementing the traversal
/// in a [catch_unwind] block and emit diagnostics in case of error (either the
/// traversal function returns Err or panics)
fn handle_file(ctx: &TraversalOptions, path: &BiomePath) {
    match catch_unwind(move || process_file(ctx, path)) {
        Ok(Ok(FileStatus::Changed)) => {
            ctx.increment_changed(path);
        }
        Ok(Ok(FileStatus::Unchanged)) => {
            ctx.increment_unchanged();
        }
        Ok(Ok(FileStatus::SearchResult(num_matches, msg))) => {
            ctx.increment_unchanged();
            ctx.increment_matches(num_matches);
            ctx.push_message(msg);
        }
        Ok(Ok(FileStatus::Message(msg))) => {
            ctx.increment_unchanged();
            ctx.push_message(msg);
        }
        Ok(Ok(FileStatus::Protected(file_path))) => {
            ctx.increment_unchanged();
            ctx.push_diagnostic(WorkspaceError::protected_file(file_path).into());
        }
        Ok(Ok(FileStatus::Ignored)) => {}
        Ok(Err(err)) => {
            ctx.increment_unchanged();
            ctx.skipped.fetch_add(1, Ordering::Relaxed);
            ctx.push_message(err);
        }
        Err(err) => {
            let message = match err.downcast::<String>() {
                Ok(msg) => format!("processing panicked: {msg}"),
                Err(err) => match err.downcast::<&'static str>() {
                    Ok(msg) => format!("processing panicked: {msg}"),
                    Err(_) => String::from("processing panicked"),
                },
            };

            ctx.push_message(PanicDiagnostic { message }.with_file_path(path.to_string()));
        }
    }
}
