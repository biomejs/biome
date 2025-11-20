mod diagnostics;
mod migrate;
mod process_file;
mod std_in;
pub(crate) mod traverse;

use crate::cli_options::{CliOptions, CliReporter};
use crate::commands::MigrateSubCommand;
use crate::diagnostics::ReportDiagnostic;
use crate::execute::migrate::MigratePayload;
use crate::execute::traverse::{TraverseResult, traverse};
use crate::reporter::checkstyle::CheckstyleReporter;
use crate::reporter::github::{GithubReporter, GithubReporterVisitor};
use crate::reporter::gitlab::{GitLabReporter, GitLabReporterVisitor};
use crate::reporter::json::{JsonReporter, JsonReporterVisitor};
use crate::reporter::junit::{JunitReporter, JunitReporterVisitor};
use crate::reporter::rdjson::{RdJsonReporter, RdJsonReporterVisitor};
use crate::reporter::summary::{SummaryReporter, SummaryReporterVisitor};
use crate::reporter::terminal::{ConsoleReporter, ConsoleReporterVisitor};
use crate::{
    CliDiagnostic, CliSession, DiagnosticsPayload, Reporter, TEMPORARY_INTERNAL_REPORTER_FILE,
};
use biome_configuration::analyzer::AnalyzerSelector;
use biome_console::{ConsoleExt, markup};
use biome_diagnostics::{Category, category};
use biome_diagnostics::{Resource, SerdeJsonError};
use biome_fs::BiomePath;
use biome_grit_patterns::GritTargetLanguage;
use biome_service::projects::ProjectKey;
use biome_service::workspace::{
    CloseFileParams, FeatureName, FeaturesBuilder, FileContent, FixFileMode, FormatFileParams,
    OpenFileParams, PatternId, ScanKind,
};
use camino::{Utf8Path, Utf8PathBuf};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use tracing::{info, instrument};

/// Useful information during the traversal of files and virtual content
#[derive(Debug, Clone)]
pub struct Execution {
    /// How the information should be collected and reported
    report_mode: ReportMode,

    /// The modality of execution of the traversal
    traversal_mode: TraversalMode,

    /// The maximum number of diagnostics that can be printed in console
    max_diagnostics: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum ExecutionEnvironment {
    GitHub,
}

/// A type that holds the information to execute the CLI via `stdin
#[derive(Debug, Clone)]
pub struct Stdin(
    /// The virtual path to the file
    Utf8PathBuf,
    /// The content of the file
    String,
);

impl Stdin {
    pub fn as_path(&self) -> &Utf8Path {
        self.0.as_path()
    }

    fn as_content(&self) -> &str {
        self.1.as_str()
    }
}

impl From<(Utf8PathBuf, String)> for Stdin {
    fn from((path, content): (Utf8PathBuf, String)) -> Self {
        Self(path, content)
    }
}

#[derive(Default, Debug, Clone)]
pub struct VcsTargeted {
    pub staged: bool,
    pub changed: bool,
}

impl From<(bool, bool)> for VcsTargeted {
    fn from((staged, changed): (bool, bool)) -> Self {
        Self { staged, changed }
    }
}

#[derive(Debug, Clone)]
pub enum TraversalMode {
    /// This mode is enabled when running the command `biome check`
    Check {
        /// The type of fixes that should be applied when analyzing a file.
        ///
        /// It's [None] if the `check` command is called without `--apply` or `--apply-suggested`
        /// arguments.
        fix_file_mode: Option<FixFileMode>,
        /// An optional tuple.
        /// 1. The virtual path to the file
        /// 2. The content of the file
        stdin: Option<Stdin>,
        /// A flag to know vcs integrated options such as `--staged` or `--changed` are enabled
        vcs_targeted: VcsTargeted,

        /// Whether assist diagnostics should be promoted to error, and fail the CLI
        enforce_assist: bool,

        /// It skips parse errors
        skip_parse_errors: bool,
    },
    /// This mode is enabled when running the command `biome lint`
    Lint {
        /// The type of fixes that should be applied when analyzing a file.
        ///
        /// It's [None] if the `lint` command is called without `--apply` or `--apply-suggested`
        /// arguments.
        fix_file_mode: Option<FixFileMode>,
        /// An optional tuple.
        /// 1. The virtual path to the file
        /// 2. The content of the file
        stdin: Option<Stdin>,
        /// Run only the given rule or group of rules.
        /// If the severity level of a rule is `off`,
        /// then the severity level of the rule is set to `error` if it is a recommended rule or `warn` otherwise.
        only: Vec<AnalyzerSelector>,
        /// Skip the given rule or group of rules by setting the severity level of the rules to `off`.
        /// This option takes precedence over `--only`.
        skip: Vec<AnalyzerSelector>,
        /// A flag to know vcs integrated options such as `--staged` or `--changed` are enabled
        vcs_targeted: VcsTargeted,
        /// Suppress existing diagnostics with a `// biome-ignore` comment
        suppress: bool,
        /// Explanation for suppressing diagnostics with `--suppress` and `--reason`
        suppression_reason: Option<String>,

        /// It skips parse errors
        skip_parse_errors: bool,
    },
    /// This mode is enabled when running the command `biome ci`
    CI {
        /// Whether the CI is running in a specific environment, e.g. GitHub, GitLab, etc.
        environment: Option<ExecutionEnvironment>,
        /// A flag to know vcs integrated options such as `--staged` or `--changed` are enabled
        vcs_targeted: VcsTargeted,
        /// Whether assist diagnostics should be promoted to error, and fail the CLI
        enforce_assist: bool,
        /// It skips parse errors
        skip_parse_errors: bool,
    },
    /// This mode is enabled when running the command `biome format`
    Format {
        /// It skips parse errors
        skip_parse_errors: bool,
        /// It writes the new content on file
        write: bool,
        /// An optional tuple.
        /// 1. The virtual path to the file
        /// 2. The content of the file
        stdin: Option<Stdin>,
        /// A flag to know vcs integrated options such as `--staged` or `--changed` are enabled
        vcs_targeted: VcsTargeted,
    },
    /// This mode is enabled when running the command `biome migrate`
    Migrate {
        /// Write result to disk
        write: bool,
        /// The path to `biome.json`
        configuration_file_path: Utf8PathBuf,
        sub_command: Option<MigrateSubCommand>,
    },
    /// This mode is enabled when running the command `biome search`
    Search {
        /// The GritQL pattern to search for.
        ///
        /// Note that the search command does not support rewrites.
        pattern: PatternId,

        /// The language to query for.
        ///
        /// Grit queries are specific to the grammar of the language they
        /// target, so we currently do not support writing queries that apply
        /// to multiple languages at once.
        ///
        /// If none given, the default language is JavaScript.
        language: Option<GritTargetLanguage>,

        /// An optional tuple.
        /// 1. The virtual path to the file
        /// 2. The content of the file
        stdin: Option<Stdin>,
    },
}

impl Display for TraversalMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Check { .. } => write!(f, "check"),
            Self::CI { .. } => write!(f, "ci"),
            Self::Format { .. } => write!(f, "format"),
            Self::Migrate { .. } => write!(f, "migrate"),
            Self::Lint { .. } => write!(f, "lint"),
            Self::Search { .. } => write!(f, "search"),
        }
    }
}

impl TraversalMode {
    /// It returns the best [ScanKind] variant based on the [TraversalMode]
    pub fn to_scan_kind(&self) -> ScanKind {
        match self {
            Self::CI { .. } => ScanKind::Project,
            Self::Format { stdin, .. } => {
                if stdin.is_none() {
                    ScanKind::KnownFiles
                } else {
                    ScanKind::NoScanner
                }
            }
            Self::Check { stdin, .. } | Self::Lint { stdin, .. } | Self::Search { stdin, .. } => {
                if stdin.is_none() {
                    ScanKind::Project
                } else {
                    ScanKind::NoScanner
                }
            }
            Self::Migrate { .. } => ScanKind::NoScanner,
        }
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
}

impl Default for ReportMode {
    fn default() -> Self {
        Self::Terminal {
            with_summary: false,
        }
    }
}

impl From<CliReporter> for ReportMode {
    fn from(value: CliReporter) -> Self {
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
        }
    }
}

impl Execution {
    pub(crate) fn new(mode: TraversalMode) -> Self {
        Self {
            report_mode: ReportMode::default(),
            traversal_mode: mode,
            max_diagnostics: 20,
        }
    }

    pub(crate) fn new_ci(
        vcs_targeted: VcsTargeted,
        enforce_assist: bool,
        skip_parse_errors: bool,
    ) -> Self {
        // Ref: https://docs.github.com/actions/learn-github-actions/variables#default-environment-variables
        let is_github = std::env::var("GITHUB_ACTIONS")
            .ok()
            .is_some_and(|value| value == "true");

        Self {
            report_mode: ReportMode::default(),
            traversal_mode: TraversalMode::CI {
                environment: if is_github {
                    Some(ExecutionEnvironment::GitHub)
                } else {
                    None
                },
                vcs_targeted,
                enforce_assist,
                skip_parse_errors,
            },
            max_diagnostics: 20,
        }
    }

    /// It sets the reporting mode by reading the [CliOptions]
    pub(crate) fn set_report(mut self, cli_options: &CliOptions) -> Self {
        self.report_mode = cli_options.reporter.clone().into();
        self
    }

    pub(crate) fn traversal_mode(&self) -> &TraversalMode {
        &self.traversal_mode
    }

    pub(crate) fn get_max_diagnostics(&self) -> u32 {
        self.max_diagnostics
    }

    /// `true` only when running the traversal in [TraversalMode::Check] and `should_fix` is `true`
    pub(crate) fn as_fix_file_mode(&self) -> Option<&FixFileMode> {
        match &self.traversal_mode {
            TraversalMode::Check { fix_file_mode, .. }
            | TraversalMode::Lint { fix_file_mode, .. } => fix_file_mode.as_ref(),
            TraversalMode::Format { .. }
            | TraversalMode::CI { .. }
            | TraversalMode::Migrate { .. }
            | TraversalMode::Search { .. } => None,
        }
    }

    pub(crate) fn as_diagnostic_category(&self) -> &'static Category {
        match self.traversal_mode {
            TraversalMode::Check { .. } => category!("check"),
            TraversalMode::Lint { .. } => category!("lint"),
            TraversalMode::CI { .. } => category!("ci"),
            TraversalMode::Format { .. } => category!("format"),
            TraversalMode::Migrate { .. } => category!("migrate"),
            TraversalMode::Search { .. } => category!("search"),
        }
    }

    pub(crate) const fn is_ci(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::CI { .. })
    }

    pub(crate) const fn is_search(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::Search { .. })
    }

    pub(crate) const fn is_check(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::Check { .. })
    }

    pub(crate) const fn is_lint(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::Lint { .. })
    }

    #[instrument(level = "debug", skip(self), fields(result))]
    pub(crate) fn is_safe_fixes_enabled(&self) -> bool {
        let result = match self.traversal_mode {
            TraversalMode::Check { fix_file_mode, .. } => {
                fix_file_mode == Some(FixFileMode::SafeFixes)
            }
            _ => false,
        };
        tracing::Span::current().record("result", result);
        result
    }

    #[instrument(level = "debug", skip(self), fields(result))]
    pub(crate) fn is_safe_and_unsafe_fixes_enabled(&self) -> bool {
        let result = match self.traversal_mode {
            TraversalMode::Check { fix_file_mode, .. } => {
                fix_file_mode == Some(FixFileMode::SafeAndUnsafeFixes)
            }
            _ => false,
        };

        tracing::Span::current().record("result", result);
        result
    }

    pub(crate) const fn is_format(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::Format { .. })
    }

    pub(crate) const fn is_format_write(&self) -> bool {
        if let TraversalMode::Format { write, .. } = self.traversal_mode {
            write
        } else {
            false
        }
    }

    /// Whether the traversal mode requires write access to files
    pub(crate) const fn requires_write_access(&self) -> bool {
        match self.traversal_mode {
            TraversalMode::Check { fix_file_mode, .. }
            | TraversalMode::Lint { fix_file_mode, .. } => fix_file_mode.is_some(),
            TraversalMode::CI { .. } | TraversalMode::Search { .. } => false,
            TraversalMode::Format { write, .. } | TraversalMode::Migrate { write, .. } => write,
        }
    }

    pub(crate) fn as_stdin_file(&self) -> Option<&Stdin> {
        match &self.traversal_mode {
            TraversalMode::Format { stdin, .. }
            | TraversalMode::Lint { stdin, .. }
            | TraversalMode::Check { stdin, .. }
            | TraversalMode::Search { stdin, .. } => stdin.as_ref(),
            TraversalMode::CI { .. } | TraversalMode::Migrate { .. } => None,
        }
    }

    pub(crate) fn is_vcs_targeted(&self) -> bool {
        match &self.traversal_mode {
            TraversalMode::Check { vcs_targeted, .. }
            | TraversalMode::Lint { vcs_targeted, .. }
            | TraversalMode::Format { vcs_targeted, .. }
            | TraversalMode::CI { vcs_targeted, .. } => vcs_targeted.staged || vcs_targeted.changed,
            TraversalMode::Migrate { .. } | TraversalMode::Search { .. } => false,
        }
    }

    /// Returns [true] if the user used the `--write`/`--fix` option
    pub(crate) fn is_write(&self) -> bool {
        match self.traversal_mode {
            TraversalMode::Check { fix_file_mode, .. } => fix_file_mode.is_some(),
            TraversalMode::Lint { fix_file_mode, .. } => fix_file_mode.is_some(),
            TraversalMode::CI { .. } => false,
            TraversalMode::Format { write, .. } => write,
            TraversalMode::Migrate { write, .. } => write,
            TraversalMode::Search { .. } => false,
        }
    }

    pub fn new_format(vcs_targeted: VcsTargeted) -> Self {
        Self {
            traversal_mode: TraversalMode::Format {
                skip_parse_errors: false,
                write: false,
                stdin: None,
                vcs_targeted,
            },
            report_mode: ReportMode::default(),
            max_diagnostics: 0,
        }
    }

    pub fn report_mode(&self) -> &ReportMode {
        &self.report_mode
    }
    pub(crate) fn to_feature(&self) -> FeatureName {
        match self.traversal_mode {
            TraversalMode::Format { .. } => FeaturesBuilder::new().with_formatter().build(),
            TraversalMode::Lint { .. } => FeaturesBuilder::new().with_linter().build(),
            TraversalMode::Check { .. } | TraversalMode::CI { .. } => FeaturesBuilder::new()
                .with_formatter()
                .with_linter()
                .with_assist()
                .build(),
            TraversalMode::Migrate { .. } => FeatureName::empty(),
            TraversalMode::Search { .. } => FeaturesBuilder::new().with_search().build(),
        }
    }

    #[instrument(level = "debug", skip(self), fields(result))]
    pub(crate) fn should_write(&self) -> bool {
        let result = match self.traversal_mode {
            TraversalMode::Format { write, .. } => write,

            _ => self.is_safe_fixes_enabled() || self.is_safe_and_unsafe_fixes_enabled(),
        };
        tracing::Span::current().record("result", result);
        result
    }

    #[instrument(level = "debug", skip(self), fields(result))]
    pub(crate) fn should_skip_parse_errors(&self) -> bool {
        let result = match self.traversal_mode {
            TraversalMode::Format {
                skip_parse_errors, ..
            }
            | TraversalMode::Check {
                skip_parse_errors, ..
            }
            | TraversalMode::Lint {
                skip_parse_errors, ..
            }
            | TraversalMode::CI {
                skip_parse_errors, ..
            } => skip_parse_errors,

            _ => false,
        };
        tracing::Span::current().record("result", result);

        result
    }

    pub(crate) fn should_enforce_assist(&self) -> bool {
        match self.traversal_mode {
            TraversalMode::CI { enforce_assist, .. } => enforce_assist,
            TraversalMode::Check { enforce_assist, .. } => enforce_assist,
            _ => false,
        }
    }
}

/// Based on the [mode](TraversalMode), the function might launch a traversal of the file system
/// or handles the stdin file.
pub fn execute_mode(
    mut execution: Execution,
    mut session: CliSession,
    cli_options: &CliOptions,
    paths: Vec<String>,
    scanner_duration: Option<Duration>,
    nested_configuration_files: Vec<BiomePath>,
    project_key: ProjectKey,
) -> Result<(), CliDiagnostic> {
    // If a custom reporter was provided, let's lift the limit so users can see all of them
    execution.max_diagnostics = if cli_options.reporter.is_default() {
        cli_options.max_diagnostics.into()
    } else {
        info!(
            "Removing the limit of --max-diagnostics, because of a reporter different from the default one: {}",
            cli_options.reporter
        );
        u32::MAX
    };

    // migrate command doesn't do any traversal.
    if let TraversalMode::Migrate {
        write,
        configuration_file_path,
        sub_command,
    } = execution.traversal_mode
    {
        let payload = MigratePayload {
            session,
            project_key,
            write,
            configuration_file_path,
            sub_command,
            nested_configuration_files,
        };
        return migrate::run(payload);
    }

    // don't do any traversal if there's some content coming from stdin
    if let Some(stdin) = execution.as_stdin_file() {
        let biome_path = BiomePath::new(stdin.as_path());
        return std_in::run(
            session,
            project_key,
            &execution,
            biome_path,
            stdin.as_content(),
            cli_options,
        );
    }

    let TraverseResult {
        mut summary,
        evaluated_paths,
        mut diagnostics,
    } = traverse(
        &execution,
        &mut session,
        project_key,
        cli_options,
        paths.clone(),
    )?;
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
    summary.scanner_duration = scanner_duration;
    let console = session.app.console;
    let workspace = &*session.app.workspace;
    let fs = workspace.fs();
    let errors = summary.errors;
    let skipped = summary.skipped;
    let processed = summary.changed + summary.unchanged;
    let should_exit_on_warnings = summary.warnings > 0 && cli_options.error_on_warnings;
    let diagnostics_payload = DiagnosticsPayload {
        diagnostic_level: cli_options.diagnostic_level,
        diagnostics,
        max_diagnostics: cli_options.max_diagnostics,
    };

    match execution.report_mode {
        ReportMode::Terminal { with_summary } => {
            if with_summary {
                let reporter = SummaryReporter {
                    summary,
                    diagnostics_payload,
                    execution: execution.clone(),
                    verbose: cli_options.verbose,
                    working_directory: fs.working_directory().clone(),
                    evaluated_paths,
                };
                reporter.write(&mut SummaryReporterVisitor(console))?;
            } else {
                let reporter = ConsoleReporter {
                    summary,
                    diagnostics_payload,
                    execution: execution.clone(),
                    evaluated_paths,
                    verbose: cli_options.verbose,
                    working_directory: fs.working_directory().clone(),
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
                diagnostics: diagnostics_payload,
                execution: execution.clone(),
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };
            let mut buffer = JsonReporterVisitor::new(summary);
            reporter.write(&mut buffer)?;
            if pretty {
                let content = serde_json::to_string(&buffer).map_err(|error| {
                    CliDiagnostic::Report(ReportDiagnostic::Serialization(SerdeJsonError::from(
                        error,
                    )))
                })?;
                let report_file = BiomePath::new(TEMPORARY_INTERNAL_REPORTER_FILE);
                session.app.workspace.open_file(OpenFileParams {
                    project_key,
                    content: FileContent::from_client(content),
                    path: report_file.clone(),
                    document_file_source: None,
                    persist_node_cache: false,
                })?;
                let code = session.app.workspace.format_file(FormatFileParams {
                    project_key,
                    path: report_file.clone(),
                })?;
                console.log(markup! {
                    {code.as_code()}
                });
                session.app.workspace.close_file(CloseFileParams {
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
                diagnostics_payload,
                execution: execution.clone(),
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };
            reporter.write(&mut GithubReporterVisitor(console))?;
        }
        ReportMode::GitLab => {
            let reporter = GitLabReporter {
                diagnostics: diagnostics_payload,
                execution: execution.clone(),
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };
            reporter.write(&mut GitLabReporterVisitor::new(
                console,
                session.app.workspace.fs().working_directory(),
            ))?;
        }
        ReportMode::Junit => {
            let reporter = JunitReporter {
                summary,
                diagnostics_payload,
                execution: execution.clone(),
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };
            reporter.write(&mut JunitReporterVisitor::new(console))?;
        }
        ReportMode::Checkstyle => {
            let reporter = CheckstyleReporter {
                summary,
                diagnostics_payload,
                execution: execution.clone(),
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };
            reporter
                .write(&mut crate::reporter::checkstyle::CheckstyleReporterVisitor::new(console))?;
        }
        ReportMode::RdJson => {
            let reporter = RdJsonReporter {
                diagnostics_payload,
                execution: execution.clone(),
                verbose: cli_options.verbose,
                working_directory: fs.working_directory().clone(),
            };
            reporter.write(&mut RdJsonReporterVisitor(console))?;
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
