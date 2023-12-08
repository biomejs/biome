use super::process_file::{process_file, DiffKind, FileStatus, Message};
use super::ExecutionEnvironment;
use crate::cli_options::CliOptions;
use crate::execute::diagnostics::{
    CIFormatDiffDiagnostic, CIOrganizeImportsDiffDiagnostic, ContentDiffAdvice,
    FormatDiffDiagnostic, OrganizeImportsDiffDiagnostic, PanicDiagnostic,
};
use crate::{
    CliDiagnostic, CliSession, Execution, FormatterReportFileDetail, FormatterReportSummary,
    Report, ReportDiagnostic, ReportDiff, ReportErrorKind, ReportKind, TraversalMode,
};
use biome_console::{fmt, markup, Console, ConsoleExt};
use biome_diagnostics::PrintGitHubDiagnostic;
use biome_diagnostics::{
    adapters::StdError, category, DiagnosticExt, Error, PrintDescription, PrintDiagnostic,
    Resource, Severity,
};
use biome_fs::{FileSystem, PathInterner, RomePath};
use biome_fs::{TraversalContext, TraversalScope};
use biome_service::workspace::{FeaturesBuilder, IsPathIgnoredParams};
use biome_service::{
    workspace::{FeatureName, SupportsFeatureParams},
    Workspace, WorkspaceError,
};
use crossbeam::{
    channel::{unbounded, Receiver, Sender},
    select,
};
use rustc_hash::FxHashSet;
use std::{
    ffi::OsString,
    io,
    panic::catch_unwind,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU16, AtomicUsize, Ordering},
        Once,
    },
    thread,
    time::{Duration, Instant},
};

struct CheckResult {
    count: usize,
    duration: Duration,
    errors: usize,
}
impl fmt::Display for CheckResult {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> io::Result<()> {
        markup!(<Info>"Checked "{self.count}" file(s) in "{self.duration}</Info>).fmt(fmt)?;

        if self.errors > 0 {
            markup!("\n"<Error>"Found "{self.errors}" error(s)"</Error>).fmt(fmt)?
        }
        Ok(())
    }
}

///
pub(crate) fn traverse(
    execution: Execution,
    session: CliSession,
    cli_options: &CliOptions,
    inputs: Vec<OsString>,
) -> Result<(), CliDiagnostic> {
    init_thread_pool();
    if inputs.is_empty() && execution.as_stdin_file().is_none() {
        return Err(CliDiagnostic::missing_argument(
            "<INPUT>",
            format!("{}", execution.traversal_mode),
        ));
    }

    let (interner, recv_files) = PathInterner::new();
    let (send_msgs, recv_msgs) = unbounded();
    let (sender_reports, recv_reports) = unbounded();

    let processed = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);

    let fs = &*session.app.fs;
    let workspace = &*session.app.workspace;
    let console = &mut *session.app.console;

    let max_diagnostics = execution.get_max_diagnostics();
    let remaining_diagnostics = AtomicU16::new(max_diagnostics);

    let mut errors: usize = 0;
    let mut warnings: usize = 0;
    let mut report = Report::default();

    let duration = thread::scope(|s| {
        let handler = thread::Builder::new()
            .name(String::from("biome::console"))
            .spawn_scoped(s, || {
                process_messages(ProcessMessagesOptions {
                    execution: &execution,
                    console,
                    recv_reports,
                    recv_files,
                    recv_msgs,
                    max_diagnostics,
                    remaining_diagnostics: &remaining_diagnostics,
                    errors: &mut errors,
                    report: &mut report,
                    verbose: cli_options.verbose,
                    warnings: &mut warnings,
                    diagnostic_level: &cli_options.diagnostic_level,
                });
            })
            .expect("failed to spawn console thread");

        // The traversal context is scoped to ensure all the channels it
        // contains are properly closed once the traversal finishes
        let elapsed = traverse_inputs(
            fs,
            inputs,
            &TraversalOptions {
                fs,
                workspace,
                execution: &execution,
                interner,
                processed: &processed,
                skipped: &skipped,
                messages: send_msgs,
                sender_reports,
                remaining_diagnostics: &remaining_diagnostics,
            },
        );

        // wait for the main thread to finish
        handler.join().unwrap();

        elapsed
    });

    let count = processed.load(Ordering::Relaxed);
    let skipped = skipped.load(Ordering::Relaxed);

    if execution.should_report_to_terminal() {
        match execution.traversal_mode() {
            TraversalMode::Check { .. } | TraversalMode::Lint { .. } => {
                if execution.as_fix_file_mode().is_some() {
                    console.log(markup! {
                        <Info>"Fixed "{count}" file(s) in "{duration}</Info>
                    });
                } else {
                    console.log(markup!({
                        CheckResult {
                            count,
                            duration,
                            errors,
                        }
                    }));
                }
            }
            TraversalMode::CI { .. } => {
                console.log(markup!({
                    CheckResult {
                        count,
                        duration,
                        errors,
                    }
                }));
            }
            TraversalMode::Format { write: false, .. } => {
                console.log(markup! {
                    <Info>"Compared "{count}" file(s) in "{duration}</Info>
                });
            }
            TraversalMode::Format { write: true, .. } => {
                console.log(markup! {
                    <Info>"Formatted "{count}" file(s) in "{duration}</Info>
                });
            }

            TraversalMode::Migrate { write: false, .. } => {
                console.log(markup! {
                    <Info>"Checked your configuration file in "{duration}</Info>
                });
            }

            TraversalMode::Migrate { write: true, .. } => {
                console.log(markup! {
                    <Info>"Migrated your configuration file in "{duration}</Info>
                });
            }
        }
    } else {
        if let TraversalMode::Format { write, .. } = execution.traversal_mode() {
            let mut summary = FormatterReportSummary::default();
            if *write {
                summary.set_files_written(count);
            } else {
                summary.set_files_compared(count);
            }
            report.set_formatter_summary(summary);
        }

        let to_print = report.as_serialized_reports()?;
        console.log(markup! {
            {to_print}
        });
        return Ok(());
    }

    if skipped > 0 {
        console.log(markup! {
            <Warn>"Skipped "{skipped}" file(s)"</Warn>
        });
    }

    let should_exit_on_warnings = warnings > 0 && cli_options.error_on_warnings;
    // Processing emitted error diagnostics, exit with a non-zero code
    if count.saturating_sub(skipped) == 0 && !cli_options.no_errors_on_unmatched {
        Err(CliDiagnostic::no_files_processed())
    } else if errors > 0 || should_exit_on_warnings {
        let category = execution.as_diagnostic_category();
        if should_exit_on_warnings {
            if execution.is_check_apply() {
                Err(CliDiagnostic::apply_warnings(category))
            } else {
                Err(CliDiagnostic::check_warnings(category))
            }
        } else if execution.is_check_apply() {
            Err(CliDiagnostic::apply_error(category))
        } else {
            Err(CliDiagnostic::check_error(category))
        }
    } else {
        Ok(())
    }
}

/// This function will setup the global Rayon thread pool the first time it's called
///
/// This is currently only used to assign friendly debug names to the threads of the pool
fn init_thread_pool() {
    static INIT_ONCE: Once = Once::new();
    INIT_ONCE.call_once(|| {
        rayon::ThreadPoolBuilder::new()
            .thread_name(|index| format!("biome::worker_{index}"))
            .build_global()
            .expect("failed to initialize the global thread pool");
    });
}

/// Initiate the filesystem traversal tasks with the provided input paths and
/// run it to completion, returning the duration of the process
fn traverse_inputs(fs: &dyn FileSystem, inputs: Vec<OsString>, ctx: &TraversalOptions) -> Duration {
    let start = Instant::now();
    fs.traversal(Box::new(move |scope: &dyn TraversalScope| {
        scope.traverse_paths(ctx, inputs.iter().map(PathBuf::from).collect());
    }));

    start.elapsed()
}

struct ProcessMessagesOptions<'ctx> {
    ///  Execution of the traversal
    execution: &'ctx Execution,
    /// Mutable reference to the [console](Console)
    console: &'ctx mut dyn Console,
    /// Receiver channel for reporting statistics
    recv_reports: Receiver<ReportKind>,
    /// Receiver channel that expects info when a file is processed
    recv_files: Receiver<PathBuf>,
    /// Receiver channel that expects info when a message is sent
    recv_msgs: Receiver<Message>,
    /// The maximum number of diagnostics the console thread is allowed to print
    max_diagnostics: u16,
    /// The approximate number of diagnostics the console will print before
    /// folding the rest into the "skipped diagnostics" counter
    remaining_diagnostics: &'ctx AtomicU16,
    /// Mutable reference to a boolean flag tracking whether the console thread
    /// printed any error-level message
    errors: &'ctx mut usize,
    /// Mutable reference to a boolean flag tracking whether the console thread
    /// printed any warnings-level message
    warnings: &'ctx mut usize,
    /// Mutable handle to a [Report] instance the console thread should write
    /// stats into
    report: &'ctx mut Report,
    /// Whether the console thread should print diagnostics in verbose mode
    verbose: bool,
    /// The diagnostic level the console thread should print
    diagnostic_level: &'ctx Severity,
}

/// This thread receives [Message]s from the workers through the `recv_msgs`
/// and `recv_files` channels and handles them based on [Execution]
fn process_messages(options: ProcessMessagesOptions) {
    let ProcessMessagesOptions {
        execution: mode,
        console,
        recv_reports,
        recv_files,
        recv_msgs,
        max_diagnostics,
        remaining_diagnostics,
        errors,
        report,
        verbose,
        warnings,
        diagnostic_level,
    } = options;

    let mut paths: FxHashSet<String> = FxHashSet::default();
    let mut printed_diagnostics: u16 = 0;
    let mut not_printed_diagnostics = 0;
    let mut total_skipped_suggested_fixes = 0;

    let mut is_msg_open = true;
    let mut is_report_open = true;
    let mut diagnostics_to_print = vec![];
    while is_msg_open || is_report_open {
        let msg = select! {
            recv(recv_msgs) -> msg => match msg {
                Ok(msg) => msg,
                Err(_) => {
                    is_msg_open = false;
                    continue;
                },
            },
            recv(recv_reports) -> stat => {
                match stat {
                    Ok(stat) => {
                        report.push_detail_report(stat);
                    }
                    Err(_) => {
                        is_report_open = false;
                    },
                }
                continue;
            }
        };

        match msg {
            Message::SkippedFixes {
                skipped_suggested_fixes,
            } => {
                total_skipped_suggested_fixes += skipped_suggested_fixes;
            }

            Message::ApplyError(error) => {
                *errors += 1;
                let should_print = printed_diagnostics < max_diagnostics;
                if should_print {
                    printed_diagnostics += 1;
                    remaining_diagnostics.store(
                        max_diagnostics.saturating_sub(printed_diagnostics),
                        Ordering::Relaxed,
                    );
                } else {
                    not_printed_diagnostics += 1;
                }
                if mode.should_report_to_terminal() && should_print {
                    diagnostics_to_print.push(Error::from(error));
                }
            }

            Message::Error(mut err) => {
                let location = err.location();
                if err.severity() == Severity::Warning {
                    *warnings += 1;
                }
                if let Some(Resource::File(file_path)) = location.resource.as_ref() {
                    // Retrieves the file name from the file ID cache, if it's a miss
                    // flush entries from the interner channel until it's found
                    let file_name = match paths.get(*file_path) {
                        Some(path) => Some(path),
                        None => loop {
                            match recv_files.recv() {
                                Ok(path) => {
                                    paths.insert(path.display().to_string());
                                    if path.display().to_string() == *file_path {
                                        break paths.get(&path.display().to_string());
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

                let should_print = printed_diagnostics < max_diagnostics;
                if should_print {
                    printed_diagnostics += 1;
                    remaining_diagnostics.store(
                        max_diagnostics.saturating_sub(printed_diagnostics),
                        Ordering::Relaxed,
                    );
                } else {
                    not_printed_diagnostics += 1;
                }

                if mode.should_report_to_terminal() {
                    if should_print {
                        diagnostics_to_print.push(err);
                    }
                } else {
                    let location = err.location();
                    let path = match &location.resource {
                        Some(Resource::File(file)) => Some(*file),
                        _ => None,
                    };

                    let file_name = path.unwrap_or("<unknown>");
                    let title = PrintDescription(&err).to_string();
                    let code = err.category().and_then(|code| code.name().parse().ok());

                    report.push_detail_report(ReportKind::Error(
                        file_name.to_string(),
                        ReportErrorKind::Diagnostic(ReportDiagnostic {
                            code,
                            title,
                            severity: err.severity(),
                        }),
                    ));
                }
            }

            Message::Diagnostics {
                name,
                content,
                diagnostics,
                skipped_diagnostics,
            } => {
                not_printed_diagnostics += skipped_diagnostics;

                // is CI mode we want to print all the diagnostics
                if mode.is_ci() {
                    for diag in diagnostics {
                        let severity = diag.severity();
                        if severity == Severity::Error {
                            *errors += 1;
                        }
                        if severity == Severity::Warning {
                            *warnings += 1;
                        }

                        let diag = diag.with_file_path(&name).with_file_source_code(&content);
                        diagnostics_to_print.push(diag);
                    }
                } else {
                    for diag in diagnostics {
                        let severity = diag.severity();
                        if severity == Severity::Error {
                            *errors += 1;
                        }
                        if severity == Severity::Warning {
                            *warnings += 1;
                        }

                        let should_print = printed_diagnostics < max_diagnostics;
                        if should_print {
                            printed_diagnostics += 1;
                            remaining_diagnostics.store(
                                max_diagnostics.saturating_sub(printed_diagnostics),
                                Ordering::Relaxed,
                            );
                        } else {
                            not_printed_diagnostics += 1;
                        }

                        if mode.should_report_to_terminal() {
                            if should_print {
                                let diag =
                                    diag.with_file_path(&name).with_file_source_code(&content);
                                diagnostics_to_print.push(diag)
                            }
                        } else {
                            report.push_detail_report(ReportKind::Error(
                                name.to_string(),
                                ReportErrorKind::Diagnostic(ReportDiagnostic {
                                    code: diag.category().and_then(|code| code.name().parse().ok()),
                                    title: String::from("test here"),
                                    severity,
                                }),
                            ));
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
                // A diff is an error in CI mode and in format check mode
                if mode.is_ci() || !mode.is_format_write() {
                    *errors += 1;
                }

                let should_print = printed_diagnostics < max_diagnostics;
                if should_print {
                    printed_diagnostics += 1;
                    remaining_diagnostics.store(
                        max_diagnostics.saturating_sub(printed_diagnostics),
                        Ordering::Relaxed,
                    );
                } else {
                    not_printed_diagnostics += 1;
                }

                if mode.should_report_to_terminal() {
                    if should_print {
                        if mode.is_ci() {
                            match diff_kind {
                                DiffKind::Format => {
                                    let diag = CIFormatDiffDiagnostic {
                                        file_name: file_name.clone(),
                                        diff: ContentDiffAdvice {
                                            old: old.clone(),
                                            new: new.clone(),
                                        },
                                    };
                                    diagnostics_to_print.push(Error::from(diag))
                                }
                                DiffKind::OrganizeImports => {
                                    let diag = CIOrganizeImportsDiffDiagnostic {
                                        file_name: file_name.clone(),
                                        diff: ContentDiffAdvice {
                                            old: old.clone(),
                                            new: new.clone(),
                                        },
                                    };
                                    diagnostics_to_print.push(Error::from(diag))
                                }
                            };
                        } else {
                            match diff_kind {
                                DiffKind::Format => {
                                    let diag = FormatDiffDiagnostic {
                                        file_name: file_name.clone(),
                                        diff: ContentDiffAdvice {
                                            old: old.clone(),
                                            new: new.clone(),
                                        },
                                    };
                                    diagnostics_to_print.push(Error::from(diag))
                                }
                                DiffKind::OrganizeImports => {
                                    let diag = OrganizeImportsDiffDiagnostic {
                                        file_name: file_name.clone(),
                                        diff: ContentDiffAdvice {
                                            old: old.clone(),
                                            new: new.clone(),
                                        },
                                    };
                                    diagnostics_to_print.push(Error::from(diag))
                                }
                            };
                        }
                    }
                } else {
                    report.push_detail_report(ReportKind::Error(
                        file_name,
                        ReportErrorKind::Diff(ReportDiff {
                            before: old,
                            after: new,
                            severity: Severity::Error,
                        }),
                    ));
                }
            }
        }
    }
    let running_on_github = matches!(
        mode.traversal_mode(),
        TraversalMode::CI {
            environment: Some(ExecutionEnvironment::GitHub),
        }
    );

    for diagnostic in diagnostics_to_print {
        if diagnostic.severity() >= *diagnostic_level {
            console.error(markup! {
                {if verbose { PrintDiagnostic::verbose(&diagnostic) } else { PrintDiagnostic::simple(&diagnostic) }}
            });
        }

        if running_on_github {
            console.log(markup! {{PrintGitHubDiagnostic::simple(&diagnostic)}});
        }
    }

    if mode.is_check() && total_skipped_suggested_fixes > 0 {
        console.log(markup! {
            <Warn>"Skipped "{total_skipped_suggested_fixes}" suggested fixes.\n"</Warn>
            <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"biome check --apply-unsafe\n"</Emphasis></Info>
        })
    }

    if !mode.is_ci() && not_printed_diagnostics > 0 {
        console.log(markup! {
            <Warn>"The number of diagnostics exceeds the number allowed by Biome.\n"</Warn>
            <Info>"Diagnostics not shown: "</Info><Emphasis>{not_printed_diagnostics}</Emphasis><Info>"."</Info>
        })
    }
}

/// Context object shared between directory traversal tasks
pub(crate) struct TraversalOptions<'ctx, 'app> {
    /// Shared instance of [FileSystem]
    pub(crate) fs: &'app dyn FileSystem,
    /// Instance of [Workspace] used by this instance of the CLI
    pub(crate) workspace: &'ctx dyn Workspace,
    /// Determines how the files should be processed
    pub(crate) execution: &'ctx Execution,
    /// File paths interner cache used by the filesystem traversal
    interner: PathInterner,
    /// Shared atomic counter storing the number of processed files
    processed: &'ctx AtomicUsize,
    /// Shared atomic counter storing the number of skipped files
    skipped: &'ctx AtomicUsize,
    /// Channel sending messages to the display thread
    pub(crate) messages: Sender<Message>,
    /// Channel sending reports to the reports thread
    sender_reports: Sender<ReportKind>,
    /// The approximate number of diagnostics the console will print before
    /// folding the rest into the "skipped diagnostics" counter
    pub(crate) remaining_diagnostics: &'ctx AtomicU16,
}

impl<'ctx, 'app> TraversalOptions<'ctx, 'app> {
    pub(crate) fn increment_processed(&self) {
        self.processed.fetch_add(1, Ordering::Relaxed);
    }

    /// Send a message to the display thread
    pub(crate) fn push_message(&self, msg: impl Into<Message>) {
        self.messages.send(msg.into()).ok();
    }

    pub(crate) fn push_format_stat(&self, path: String, stat: FormatterReportFileDetail) {
        self.sender_reports
            .send(ReportKind::Formatter(path, stat))
            .ok();
    }

    pub(crate) fn miss_handler_err(&self, err: WorkspaceError, rome_path: &RomePath) {
        self.push_diagnostic(
            StdError::from(err)
                .with_category(category!("files/missingHandler"))
                .with_file_path(rome_path.display().to_string()),
        );
    }
}

impl<'ctx, 'app> TraversalContext for TraversalOptions<'ctx, 'app> {
    fn interner(&self) -> &PathInterner {
        &self.interner
    }

    fn push_diagnostic(&self, error: Error) {
        self.push_message(error);
    }

    fn can_handle(&self, rome_path: &RomePath) -> bool {
        if rome_path.is_dir() {
            let can_handle = !self
                .workspace
                .is_path_ignored(IsPathIgnoredParams {
                    rome_path: rome_path.clone(),
                    feature: self.execution.as_feature_name(),
                })
                .unwrap_or_else(|err| {
                    self.push_diagnostic(err.into());
                    false
                });
            return can_handle;
        }

        let file_features = self.workspace.file_features(SupportsFeatureParams {
            path: rome_path.clone(),
            feature: FeaturesBuilder::new()
                .with_linter()
                .with_formatter()
                .with_organize_imports()
                .build(),
        });

        let file_features = match file_features {
            Ok(file_features) => file_features,
            Err(err) => {
                self.miss_handler_err(err, rome_path);

                return false;
            }
        };

        match self.execution.traversal_mode() {
            TraversalMode::Check { .. } => {
                file_features.supports_for(&FeatureName::Lint)
                    || file_features.supports_for(&FeatureName::Format)
                    || file_features.supports_for(&FeatureName::OrganizeImports)
            }
            TraversalMode::CI { .. } => {
                file_features.supports_for(&FeatureName::Lint)
                    || file_features.supports_for(&FeatureName::Format)
                    || file_features.supports_for(&FeatureName::OrganizeImports)
            }
            TraversalMode::Format { .. } => file_features.supports_for(&FeatureName::Format),
            TraversalMode::Lint { .. } => file_features.supports_for(&FeatureName::Lint),
            // Imagine if Biome can't handle its own configuration file...
            TraversalMode::Migrate { .. } => true,
        }
    }

    fn handle_file(&self, path: &Path) {
        handle_file(self, path)
    }
}

/// This function wraps the [process_file] function implementing the traversal
/// in a [catch_unwind] block and emit diagnostics in case of error (either the
/// traversal function returns Err or panics)
fn handle_file(ctx: &TraversalOptions, path: &Path) {
    match catch_unwind(move || process_file(ctx, path)) {
        Ok(Ok(FileStatus::Success)) => {}
        Ok(Ok(FileStatus::Message(msg))) => {
            ctx.push_message(msg);
        }
        Ok(Ok(FileStatus::Ignored)) => {}
        Ok(Err(err)) => {
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

            ctx.push_message(
                PanicDiagnostic { message }.with_file_path(path.display().to_string()),
            );
        }
    }
}
