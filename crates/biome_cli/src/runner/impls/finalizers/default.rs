use crate::cli_options::{CliOptions, CliReporter, CliReporterKind};
use crate::reporter::checkstyle::CheckstyleReporter;
use crate::reporter::github::{GithubReporter, GithubReporterVisitor};
use crate::reporter::gitlab::{GitLabReporter, GitLabReporterVisitor};
use crate::reporter::json::{JsonReporter, JsonReporterVisitor};
use crate::reporter::junit::{JunitReporter, JunitReporterVisitor};
use crate::reporter::rdjson::{RdJsonReporter, RdJsonReporterVisitor};
use crate::reporter::sarif::{SarifReporter, SarifReporterVisitor};
use crate::reporter::summary::{SummaryReporter, SummaryReporterVisitor};
use crate::reporter::terminal::{ConsoleReporter, ConsoleReporterVisitor};
use crate::reporter::{ConsoleReporterWriter, FileReporterWriter, Reporter, ReporterWriter};
use crate::runner::execution::Execution;
use crate::runner::finalizer::{FinalizePayload, Finalizer};
use crate::runner::impls::commands::traversal::TraverseResult;
use crate::{CliDiagnostic, DiagnosticsPayload, TraversalSummary};
use biome_console::{Console, markup};
use biome_diagnostics::{PrintDiagnostic, Resource};
use biome_fs::{BiomePath, FileSystem, OpenOptions};
use biome_json_formatter::context::JsonFormatOptions;
use biome_rowan::AstNode;
use std::cmp::Ordering;
use std::collections::BTreeSet;

pub(crate) struct DefaultFinalizer;

impl Finalizer for DefaultFinalizer {
    type Input = TraverseResult;

    fn finalize(payload: FinalizePayload<'_, Self::Input>) -> Result<(), CliDiagnostic> {
        let FinalizePayload {
            fs,
            scan_duration,
            console,
            cli_options,
            crawler_output: result,
            execution,
            paths,
        } = payload;

        let TraverseResult {
            mut summary,
            evaluated_paths,
            mut diagnostics,
        } = result;

        diagnostics.sort_unstable_by(|a, b| match a.severity().cmp(&b.severity()) {
            Ordering::Equal => {
                let a = a.location();
                let b = b.location();
                match (a.resource, b.resource) {
                    (Some(Resource::File(a)), Some(Resource::File(b))) => a.cmp(b),
                    (Some(Resource::File(_)), None) => Ordering::Greater,
                    (None, Some(Resource::File(_))) => Ordering::Less,
                    _ => Ordering::Equal,
                }
            }
            result => result,
        });

        // We join the duration of the scanning with the duration of the traverse.
        summary.scanner_duration = scan_duration;
        let errors = summary.errors;
        let skipped = summary.skipped;
        let processed = summary.changed + summary.unchanged;
        let should_exit_on_warnings = summary.warnings > 0 && cli_options.error_on_warnings;
        let diagnostics_payload = DiagnosticsPayload {
            diagnostic_level: cli_options.diagnostic_level,
            diagnostics,
            max_diagnostics: cli_options.max_diagnostics,
        };

        let mut file_reporter_writer = FileReporterWriter::default();

        if !cli_options.cli_reporter.is_empty() {
            for cli_reporter in &cli_options.cli_reporter {
                print_to_reporter(PrintToReporter {
                    cli_reporter,
                    cli_options,
                    diagnostics_payload: &diagnostics_payload,
                    summary,
                    evaluated_paths: evaluated_paths.clone(),
                    file_reporter_writer: &mut file_reporter_writer,
                    console,
                    fs,
                    execution,
                })?;
            }
        } else {
            if let Some(reporter) = execution.environment_to_reporter() {
                let contains_current_reporter = cli_options
                    .cli_reporter
                    .iter()
                    .any(|r| r.kind == reporter.kind);

                if !contains_current_reporter {
                    print_to_reporter(PrintToReporter {
                        cli_reporter: &reporter,
                        cli_options,
                        diagnostics_payload: &diagnostics_payload,
                        summary,
                        evaluated_paths: evaluated_paths.clone(),
                        file_reporter_writer: &mut file_reporter_writer,
                        console,
                        fs,
                        execution,
                    })?;
                }
            }
            let mut console_reporter_writer = ConsoleReporterWriter(console);
            let reporter = ConsoleReporter {
                summary,
                diagnostics_payload: &diagnostics_payload,
                execution,
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
                evaluated_paths: evaluated_paths.clone(),
            };
            reporter.write(&mut console_reporter_writer, &mut ConsoleReporterVisitor)?;
        }

        // Processing emitted error diagnostics, exit with a non-zero code
        if processed.saturating_sub(skipped) == 0 && !cli_options.no_errors_on_unmatched {
            Err(CliDiagnostic::no_files_processed(
                execution.as_diagnostic_category(),
                paths,
            ))
        } else if errors > 0 || should_exit_on_warnings {
            let category = execution.as_diagnostic_category();
            if should_exit_on_warnings {
                if execution.is_safe_fixes_enabled() {
                    Err(CliDiagnostic::apply_warnings(category))
                } else {
                    Err(CliDiagnostic::check_warnings(category))
                }
            } else if execution.is_safe_fixes_enabled() {
                Err(CliDiagnostic::apply_error(category))
            } else {
                Err(CliDiagnostic::check_error(category))
            }
        } else {
            Ok(())
        }
    }
}

impl Finalizer for () {
    type Input = ();

    fn finalize(_: FinalizePayload<'_, Self::Input>) -> Result<(), CliDiagnostic> {
        Ok(())
    }
}

struct PrintToReporter<'a> {
    cli_reporter: &'a CliReporter,
    cli_options: &'a CliOptions,
    diagnostics_payload: &'a DiagnosticsPayload,
    summary: TraversalSummary,
    evaluated_paths: BTreeSet<BiomePath>,
    file_reporter_writer: &'a mut FileReporterWriter,
    console: &'a mut dyn Console,
    fs: &'a dyn FileSystem,
    execution: &'a dyn Execution,
}

fn print_to_reporter(params: PrintToReporter) -> Result<(), CliDiagnostic> {
    let PrintToReporter {
        cli_reporter,
        cli_options,
        diagnostics_payload,
        summary,
        evaluated_paths,
        file_reporter_writer,
        console,
        fs,
        execution,
    } = params;

    let mut console_reporter_writer = ConsoleReporterWriter(console);
    match cli_reporter.kind {
        CliReporterKind::Default => {
            let reporter = ConsoleReporter {
                summary,
                diagnostics_payload,
                execution,
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
                evaluated_paths: evaluated_paths.clone(),
            };
            if cli_reporter.is_file_report() {
                reporter.write(file_reporter_writer, &mut ConsoleReporterVisitor)?;
            } else {
                reporter.write(&mut console_reporter_writer, &mut ConsoleReporterVisitor)?;
            }
        }
        CliReporterKind::Summary => {
            let reporter = SummaryReporter {
                summary,
                diagnostics_payload,
                execution,
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
                evaluated_paths: evaluated_paths.clone(),
            };
            if cli_reporter.is_file_report() {
                reporter.write(file_reporter_writer, &mut SummaryReporterVisitor)?;
            } else {
                reporter.write(&mut console_reporter_writer, &mut SummaryReporterVisitor)?;
            }
        }
        CliReporterKind::Json | CliReporterKind::JsonPretty => {
            console_reporter_writer.error(markup! {
                            <Warn>"The "<Emphasis>"--json"</Emphasis>" option is "<Underline>"unstable/experimental"</Underline>" and its output might change between patches/minor releases."</Warn>
                        });
            let reporter = JsonReporter {
                summary,
                diagnostics_payload,
                execution,
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };

            let writer: &mut dyn ReporterWriter = if cli_reporter.is_file_report() {
                file_reporter_writer
            } else {
                &mut console_reporter_writer
            };

            let mut buffer = JsonReporterVisitor::new(summary);
            reporter.write(writer, &mut buffer)?;
            let root = buffer.to_json();
            if cli_reporter.kind == CliReporterKind::JsonPretty {
                let formatted =
                    biome_json_formatter::format_node(JsonFormatOptions::default(), root.syntax())
                        .expect("To format the JSON report")
                        .print()
                        .expect("To print the JSON report");

                writer.log(markup! {
                    {formatted.as_code()}
                });
            } else {
                let code = root.to_string();
                writer.log(markup! {
                    {code}
                });
            }
        }
        CliReporterKind::GitHub => {
            let reporter = GithubReporter {
                diagnostics_payload,
                execution,
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };
            if cli_reporter.is_file_report() {
                reporter.write(file_reporter_writer, &mut GithubReporterVisitor)?;
            } else {
                reporter.write(&mut console_reporter_writer, &mut GithubReporterVisitor)?;
            }
        }
        CliReporterKind::GitLab => {
            let reporter = GitLabReporter {
                diagnostics_payload,
                execution,
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };
            if cli_reporter.is_file_report() {
                reporter.write(
                    file_reporter_writer,
                    &mut GitLabReporterVisitor::new(fs.working_directory()),
                )?;
            } else {
                reporter.write(
                    &mut console_reporter_writer,
                    &mut GitLabReporterVisitor::new(fs.working_directory()),
                )?;
            }
        }
        CliReporterKind::Junit => {
            let reporter = JunitReporter {
                summary,
                diagnostics_payload,
                execution,
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };

            if cli_reporter.is_file_report() {
                reporter.write(file_reporter_writer, &mut JunitReporterVisitor::new())?;
            } else {
                reporter.write(
                    &mut console_reporter_writer,
                    &mut JunitReporterVisitor::new(),
                )?;
            }
        }
        CliReporterKind::Checkstyle => {
            let reporter = CheckstyleReporter {
                summary,
                diagnostics_payload,
                execution,
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };
            if cli_reporter.is_file_report() {
                reporter.write(
                    file_reporter_writer,
                    &mut crate::reporter::checkstyle::CheckstyleReporterVisitor,
                )?;
            } else {
                reporter.write(
                    &mut console_reporter_writer,
                    &mut crate::reporter::checkstyle::CheckstyleReporterVisitor,
                )?;
            }
        }
        CliReporterKind::RdJson => {
            let reporter = RdJsonReporter {
                diagnostics_payload,
                execution,
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };
            if cli_reporter.is_file_report() {
                reporter.write(file_reporter_writer, &mut RdJsonReporterVisitor)?;
            } else {
                reporter.write(&mut console_reporter_writer, &mut RdJsonReporterVisitor)?;
            }
        }
        CliReporterKind::Sarif => {
            let reporter = SarifReporter {
                diagnostics_payload,
                execution,
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };

            if cli_reporter.is_file_report() {
                reporter.write(file_reporter_writer, &mut SarifReporterVisitor::new())?;
            } else {
                reporter.write(
                    &mut console_reporter_writer,
                    &mut SarifReporterVisitor::new(),
                )?;
            }
        }
    }

    if let Some(destination) = cli_reporter.destination.as_deref()
        && let Some(output) = file_reporter_writer.dump()
    {
        let open_options = OpenOptions::default().write(true).create(true);
        let mut file = match fs.open_with_options(destination, open_options) {
            Ok(file) => file,
            Err(err) => {
                let diagnostics = CliDiagnostic::from(err);
                console_reporter_writer.error(markup! {
                    {PrintDiagnostic::simple(&diagnostics)}
                });
                return Ok(());
            }
        };

        let result = file.set_content(output.as_bytes());
        if let Err(err) = result {
            let diagnostics = CliDiagnostic::from(err);
            console_reporter_writer.error(markup! {
                {PrintDiagnostic::simple(&diagnostics)}
            })
        }

        file_reporter_writer.clear();
    }
    Ok(())
}
