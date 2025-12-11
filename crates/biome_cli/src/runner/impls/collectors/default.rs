use crate::TraversalSummary;
use crate::execute::traverse::TraverseResult;
use crate::runner::collector::Collector;
use crate::runner::crawler::CrawlerContext;
use crate::runner::diagnostics::{CIFormatDiffDiagnostic, ContentDiffAdvice, FormatDiffDiagnostic};
use crate::runner::execution::Execution;
use crate::runner::process_file::{DiffKind, Message};
use biome_diagnostics::{DiagnosticExt, DiagnosticTags, Error, Resource, Severity};
use biome_fs::FileSystem;
use camino::{Utf8Path, Utf8PathBuf};
use crossbeam::channel::Receiver;
use rustc_hash::FxHashSet;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

struct DefaultCollector<'ctx> {
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
    /// Mutable reference to a boolean flag tracking whether the console thread
    /// printed any info-level message
    infos: AtomicU32,
    /// Whether the console thread should print diagnostics in verbose mode
    verbose: bool,
    /// The diagnostic level the console thread should print
    diagnostic_level: Severity,

    /// Diagnostics that aren't collected
    not_printed_diagnostics: AtomicU32,

    /// Diagnostics that should be printed
    printed_diagnostics: AtomicU32,

    /// Suggested fixes that were skipped
    total_skipped_suggested_fixes: AtomicU32,

    /// The current working directory, borrowed from [FileSystem]
    working_directory: Option<&'ctx Utf8Path>,

    diagnostics_to_print: Mutex<Vec<Error>>,
}

impl<'ctx> DefaultCollector<'ctx> {
    pub(crate) fn new(working_directory: Option<&'ctx Utf8Path>) -> Self {
        Self {
            errors: AtomicU32::new(0),
            warnings: AtomicU32::new(0),
            infos: AtomicU32::new(0),
            remaining_diagnostics: AtomicU32::new(0),
            diagnostic_level: Severity::Hint,
            verbose: false,
            max_diagnostics: 20,
            not_printed_diagnostics: AtomicU32::new(0),
            printed_diagnostics: AtomicU32::new(0),
            total_skipped_suggested_fixes: AtomicU32::new(0),
            working_directory,
            diagnostics_to_print: Mutex::default(),
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

    fn infos(&self) -> u32 {
        self.infos.load(Ordering::Relaxed)
    }

    fn not_printed_diagnostics(&self) -> u32 {
        self.not_printed_diagnostics.load(Ordering::Relaxed)
    }

    fn skipped_fixes(&self) -> u32 {
        self.total_skipped_suggested_fixes.load(Ordering::Relaxed)
    }

    fn to_relative_file_path(&self, path: &str) -> String {
        let file_path = Utf8Path::new(&path);
        self.working_directory
            .as_ref()
            .and_then(|wd| file_path.strip_prefix(wd.as_str()).ok())
            .map(|path| path.to_string())
            .unwrap_or(file_path.to_string())
    }
}

impl<'ctx> Collector for DefaultCollector<'ctx> {
    type Result = TraverseResult;

    fn should_collect(&self) -> bool {
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

    fn diagnostic_level(&self) -> Severity {
        self.diagnostic_level
    }

    fn verbose(&self) -> bool {
        self.verbose
    }

    fn run(
        &self,
        receiver: Receiver<Message>,
        interner: Receiver<Utf8PathBuf>,
        execution: &dyn Execution,
    ) {
        let mut paths: FxHashSet<String> = FxHashSet::default();

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
                        self.warnings.fetch_add(1, Ordering::Relaxed);
                    }
                    // if err.severity() == Severity::Information {
                    //     self.infos.fetch_add(1, Ordering::Relaxed);
                    // }
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
                            let path = self.to_relative_file_path(path);
                            err = err.with_file_path(path.as_str());
                        }
                    }

                    let should_collect = self.should_collect();

                    if should_collect {
                        let mut diagnostics_to_print = self.diagnostics_to_print.lock().unwrap();

                        diagnostics_to_print.push(err);
                    }
                }

                Message::Diagnostics {
                    file_path,
                    content,
                    diagnostics,
                    skipped_diagnostics,
                } => {
                    // we transform the file string into a path object so we can correctly strip
                    // the working directory without having leading slash in the file name
                    let file_path = self.to_relative_file_path(&file_path);
                    self.not_printed_diagnostics
                        .fetch_add(skipped_diagnostics, Ordering::Relaxed);
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
                        if severity == Severity::Information {
                            self.infos.fetch_add(1, Ordering::Relaxed);
                        }

                        let should_collect = self.should_collect();

                        let diag = diag
                            .with_file_path(file_path.as_str())
                            .with_file_source_code(&content);
                        if should_collect || execution.is_ci() {
                            let mut diagnostics_to_print =
                                self.diagnostics_to_print.lock().unwrap();
                            diagnostics_to_print.push(diag)
                        }
                    }
                }
                Message::Diff {
                    file_name,
                    old,
                    new,
                    diff_kind,
                } => {
                    let file_path = self.to_relative_file_path(&file_name);
                    // A diff is an error in CI mode and in format check mode
                    let is_error = execution.is_ci() || !execution.is_check();
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

                    let should_collect = self.should_collect();

                    if should_collect {
                        match diff_kind {
                            DiffKind::Format => {
                                let diag = if execution.is_ci() {
                                    CIFormatDiffDiagnostic {
                                        diff: ContentDiffAdvice {
                                            old: old.clone(),
                                            new: new.clone(),
                                        },
                                    }
                                    .with_severity(severity)
                                    .with_file_source_code(old.clone())
                                    .with_file_path(file_path.clone())
                                } else {
                                    FormatDiffDiagnostic {
                                        diff: ContentDiffAdvice {
                                            old: old.clone(),
                                            new: new.clone(),
                                        },
                                    }
                                    .with_severity(severity)
                                    .with_file_source_code(old.clone())
                                    .with_file_path(file_path.clone())
                                };
                                if should_collect || execution.is_ci() {
                                    let mut diagnostics_to_print =
                                        self.diagnostics_to_print.lock().unwrap();

                                    diagnostics_to_print.push(diag);
                                }
                            }
                        }
                    }
                }
                Message::DiagnosticsWithActions { .. } => {}
            }
        }
    }

    fn result(self, duration: Duration, ctx: &dyn CrawlerContext) -> Self::Result {
        let errors = self.errors();
        let warnings = self.warnings();
        let infos = self.infos();
        let suggested_fixes_skipped = self.skipped_fixes();
        let diagnostics_not_printed = self.not_printed_diagnostics();
        let changed = ctx.changed();
        let skipped = ctx.skipped();
        let unchanged = ctx.unchanged();
        let matches = ctx.matches();
        let evaluated_paths = ctx.evaluated_paths();
        // last
        let diagnostics_to_print = self.diagnostics_to_print.into_inner().unwrap();

        TraverseResult {
            summary: TraversalSummary {
                changed,
                unchanged,
                duration,
                scanner_duration: None,
                errors,
                matches,
                warnings,
                infos,
                skipped,
                suggested_fixes_skipped,
                diagnostics_not_printed,
            },
            evaluated_paths,
            diagnostics: diagnostics_to_print,
        }
    }
}
