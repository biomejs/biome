use crate::cli_options::CliReporter;
use crate::diagnostics::ReportDiagnostic;
use crate::reporter::Reporter;
use crate::reporter::checkstyle::CheckstyleReporter;
use crate::reporter::github::{GithubReporter, GithubReporterVisitor};
use crate::reporter::gitlab::{GitLabReporter, GitLabReporterVisitor};
use crate::reporter::json::{JsonReporter, JsonReporterVisitor};
use crate::reporter::junit::{JunitReporter, JunitReporterVisitor};
use crate::reporter::rdjson::{RdJsonReporter, RdJsonReporterVisitor};
use crate::reporter::sarif::{SarifReporter, SarifReporterVisitor};
use crate::reporter::summary::{SummaryReporter, SummaryReporterVisitor};
use crate::reporter::terminal::{ConsoleReporter, ConsoleReporterVisitor};
use crate::runner::finalizer::{FinalizePayload, Finalizer};
use crate::runner::impls::commands::traversal::TraverseResult;
use crate::{CliDiagnostic, DiagnosticsPayload, TEMPORARY_INTERNAL_REPORTER_FILE};
use biome_console::{ConsoleExt, markup};
use biome_diagnostics::{Resource, SerdeJsonError};
use biome_fs::BiomePath;
use biome_service::workspace::{CloseFileParams, FileContent, FormatFileParams, OpenFileParams};
use std::cmp::Ordering;

pub(crate) struct DefaultFinalizer;

impl Finalizer for DefaultFinalizer {
    type Input = TraverseResult;

    fn finalize(payload: FinalizePayload<'_, Self::Input>) -> Result<(), CliDiagnostic> {
        let FinalizePayload {
            project_key,
            fs,
            workspace,
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

        let report_modes: Vec<ReportMode> = cli_options
            .reporter
            .iter()
            .map(|reporter| ReportMode::from(reporter))
            .collect();

        for report_mode in report_modes {
            match report_mode {
                ReportMode::Terminal { with_summary } => {
                    if with_summary {
                        let reporter = SummaryReporter {
                            summary,
                            diagnostics_payload: diagnostics_payload.clone(),
                            execution,
                            verbose: cli_options.verbose,
                            working_directory: fs.working_directory().clone(),
                            evaluated_paths: evaluated_paths.clone(),
                        };
                        reporter.write(&mut SummaryReporterVisitor(console))?;
                    } else {
                        let reporter = ConsoleReporter {
                            summary,
                            diagnostics_payload: diagnostics_payload.clone(),
                            execution,
                            verbose: cli_options.verbose,
                            working_directory: fs.working_directory().clone(),
                            evaluated_paths: evaluated_paths.clone(),
                        };
                        reporter.write(&mut ConsoleReporterVisitor(console))?;
                    }
                }
                ReportMode::Json { pretty } => {
                    console.error(markup! {
                        <Warn>"The "<Emphasis>"--json"</Emphasis>" option is "<Underline>"unstable/experimental"</Underline>" and its output might change between patches/minor releases."</Warn>
                    });
                    let reporter = JsonReporter {
                        summary,
                        diagnostics: diagnostics_payload.clone(),
                        execution,
                        verbose: cli_options.verbose,
                        working_directory: fs.working_directory().clone(),
                    };
                    let mut buffer = JsonReporterVisitor::new(summary);
                    reporter.write(&mut buffer)?;
                    if pretty {
                        let content = serde_json::to_string(&buffer).map_err(|error| {
                            CliDiagnostic::Report(ReportDiagnostic::Serialization(
                                SerdeJsonError::from(error),
                            ))
                        })?;
                        let report_file = BiomePath::new(TEMPORARY_INTERNAL_REPORTER_FILE);
                        workspace.open_file(OpenFileParams {
                            project_key,
                            content: FileContent::from_client(content),
                            path: report_file.clone(),
                            document_file_source: None,
                            persist_node_cache: false,
                            inline_config: None,
                        })?;
                        let code = workspace.format_file(FormatFileParams {
                            project_key,
                            path: report_file.clone(),
                            inline_config: None,
                        })?;
                        console.log(markup! {
                            {code.as_code()}
                        });
                        workspace.close_file(CloseFileParams {
                            project_key,
                            path: report_file,
                        })?;
                    } else {
                        console.log(markup! {
                            {buffer}
                        });
                    }
                }
                ReportMode::GitHub => {
                    let reporter = GithubReporter {
                        diagnostics_payload: diagnostics_payload.clone(),
                        execution,
                        verbose: cli_options.verbose,
                        working_directory: fs.working_directory().clone(),
                    };
                    reporter.write(&mut GithubReporterVisitor(console))?;
                }
                ReportMode::GitLab => {
                    let reporter = GitLabReporter {
                        diagnostics_payload: diagnostics_payload.clone(),
                        execution,
                        verbose: cli_options.verbose,
                        working_directory: fs.working_directory().clone(),
                    };
                    reporter.write(&mut GitLabReporterVisitor::new(
                        console,
                        workspace.fs().working_directory(),
                    ))?;
                }
                ReportMode::Junit => {
                    let reporter = JunitReporter {
                        summary,
                        diagnostics_payload: diagnostics_payload.clone(),
                        execution,
                        verbose: cli_options.verbose,
                        working_directory: fs.working_directory().clone(),
                    };
                    reporter.write(&mut JunitReporterVisitor::new(console))?;
                }
                ReportMode::Checkstyle => {
                    let reporter = CheckstyleReporter {
                        summary,
                        diagnostics_payload: diagnostics_payload.clone(),
                        execution,
                        verbose: cli_options.verbose,
                        working_directory: fs.working_directory().clone(),
                    };
                    reporter.write(
                        &mut crate::reporter::checkstyle::CheckstyleReporterVisitor::new(console),
                    )?;
                }
                ReportMode::RdJson => {
                    let reporter = RdJsonReporter {
                        diagnostics_payload: diagnostics_payload.clone(),
                        execution,
                        verbose: cli_options.verbose,
                        working_directory: fs.working_directory().clone(),
                    };
                    reporter.write(&mut RdJsonReporterVisitor(console))?;
                }
                ReportMode::Sarif => {
                    let reporter = SarifReporter {
                        diagnostics_payload: diagnostics_payload.clone(),
                        execution,
                        verbose: cli_options.verbose,
                        working_directory: fs.working_directory().clone(),
                    };
                    reporter.write(&mut SarifReporterVisitor::new(console))?;
                }
            }
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

/// Tells to the execution of the traversal how the information should be reported
#[derive(Copy, Clone, Debug)]
pub enum ReportMode {
    /// Reports information straight to the console, it's the default mode
    Terminal { with_summary: bool },
    /// Reports information in JSON format
    Json { pretty: bool },
    /// Reports information for GitHub
    GitHub,
    /// JUnit output
    /// Ref: https://github.com/testmoapp/junitxml?tab=readme-ov-file#basic-junit-xml-structure
    Junit,
    /// Reports information in the [GitLab Code Quality](https://docs.gitlab.com/ee/ci/testing/code_quality.html#implement-a-custom-tool) format.
    GitLab,
    /// Reports diagnostics in [Checkstyle XML format](https://checkstyle.org/).
    Checkstyle,
    /// Reports information in [reviewdog JSON format](https://deepwiki.com/reviewdog/reviewdog/3.2-reviewdog-diagnostic-format)
    RdJson,
    /// Reports diagnostics using the SARIF format
    Sarif,
}

impl Default for ReportMode {
    fn default() -> Self {
        Self::Terminal {
            with_summary: false,
        }
    }
}

impl From<&CliReporter> for ReportMode {
    fn from(value: &CliReporter) -> Self {
        match value {
            CliReporter::Default => Self::Terminal {
                with_summary: false,
            },
            CliReporter::Summary => Self::Terminal { with_summary: true },
            CliReporter::Json => Self::Json { pretty: false },
            CliReporter::JsonPretty => Self::Json { pretty: true },
            CliReporter::GitHub => Self::GitHub,
            CliReporter::Junit => Self::Junit,
            CliReporter::GitLab => Self::GitLab {},
            CliReporter::Checkstyle => Self::Checkstyle,
            CliReporter::RdJson => Self::RdJson,
            CliReporter::Sarif => Self::Sarif,
        }
    }
}
