mod diagnostics;
mod migrate;
mod process_file;
mod std_in;
mod traverse;

use crate::cli_options::CliOptions;
use crate::execute::migrate::MigratePayload;
use crate::execute::traverse::traverse;
use crate::{CliDiagnostic, CliSession};
use biome_diagnostics::{category, Category};
use biome_fs::BiomePath;
use biome_service::workspace::{FeatureName, FixFileMode};
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

/// Useful information during the traversal of files and virtual content
pub(crate) struct Execution {
    /// How the information should be collected and reported
    report_mode: ReportMode,

    /// The modality of execution of the traversal
    traversal_mode: TraversalMode,

    /// The maximum number of diagnostics that can be printed in console
    max_diagnostics: u16,
}

impl Execution {
    pub(crate) fn as_feature_name(&self) -> FeatureName {
        match self.traversal_mode {
            TraversalMode::Format { .. } => FeatureName::Format,
            _ => FeatureName::Lint,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ExecutionEnvironment {
    GitHub,
}

/// A type that holds the information to execute the CLI via `stdin
#[derive(Debug)]
pub(crate) struct Stdin(
    /// The virtual path to the file
    PathBuf,
    /// The content of the file
    String,
);

impl Stdin {
    fn as_path(&self) -> &Path {
        self.0.as_path()
    }

    fn as_content(&self) -> &str {
        self.1.as_str()
    }
}

impl From<(PathBuf, String)> for Stdin {
    fn from((path, content): (PathBuf, String)) -> Self {
        Self(path, content)
    }
}

#[derive(Debug)]
pub(crate) enum TraversalMode {
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
    },
    /// This mode is enabled when running the command `biome lint`
    Lint {
        /// The type of fixes that should be applied when analyzing a file.
        ///
        /// It's [None] if the `check` command is called without `--apply` or `--apply-suggested`
        /// arguments.
        fix_file_mode: Option<FixFileMode>,
        /// An optional tuple.
        /// 1. The virtual path to the file
        /// 2. The content of the file
        stdin: Option<Stdin>,
    },
    /// This mode is enabled when running the command `biome ci`
    CI {
        /// Whether the CI is running in a specific environment, e.g. GitHub, GitLab, etc.
        environment: Option<ExecutionEnvironment>,
    },
    /// This mode is enabled when running the command `biome format`
    Format {
        /// It ignores parse errors
        ignore_errors: bool,
        /// It writes the new content on file
        write: bool,
        /// An optional tuple.
        /// 1. The virtual path to the file
        /// 2. The content of the file
        stdin: Option<Stdin>,
    },
    /// This mode is enabled when running the command `biome migrate`
    Migrate {
        /// Write result to disk
        write: bool,
        /// The path directory where `biome.json` is placed
        configuration_file_path: PathBuf,
        /// The path to `biome.json`
        configuration_directory_path: PathBuf,
        /// Migrate from prettier
        prettier: bool,
    },
}

impl Display for TraversalMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TraversalMode::Check { .. } => write!(f, "check"),
            TraversalMode::CI { .. } => write!(f, "ci"),
            TraversalMode::Format { .. } => write!(f, "format"),
            TraversalMode::Migrate { .. } => write!(f, "migrate"),
            TraversalMode::Lint { .. } => write!(f, "lint"),
        }
    }
}

/// Tells to the execution of the traversal how the information should be reported
#[derive(Copy, Clone, Default)]
pub(crate) enum ReportMode {
    /// Reports information straight to the console, it's the default mode
    #[default]
    Terminal,
    /// Reports information in JSON format
    Json,
}

impl Execution {
    pub(crate) fn new(mode: TraversalMode) -> Self {
        Self {
            report_mode: ReportMode::default(),
            traversal_mode: mode,
            max_diagnostics: 20,
        }
    }

    pub(crate) fn new_ci() -> Self {
        // Ref: https://docs.github.com/actions/learn-github-actions/variables#default-environment-variables
        let is_github = std::env::var("GITHUB_ACTIONS")
            .ok()
            .map(|value| value == "true")
            .unwrap_or(false);

        Self {
            report_mode: ReportMode::default(),
            traversal_mode: TraversalMode::CI {
                environment: if is_github {
                    Some(ExecutionEnvironment::GitHub)
                } else {
                    None
                },
            },
            max_diagnostics: 20,
        }
    }

    /// Creates an instance of [Execution] by passing [traversal mode](TraversalMode) and [report mode](ReportMode)
    pub(crate) fn with_report(traversal_mode: TraversalMode, report_mode: ReportMode) -> Self {
        Self {
            traversal_mode,
            report_mode,
            max_diagnostics: 20,
        }
    }

    /// Tells if the reporting is happening straight to terminal
    pub(crate) fn should_report_to_terminal(&self) -> bool {
        matches!(self.report_mode, ReportMode::Terminal)
    }

    pub(crate) fn traversal_mode(&self) -> &TraversalMode {
        &self.traversal_mode
    }

    pub(crate) fn get_max_diagnostics(&self) -> u16 {
        self.max_diagnostics
    }

    /// `true` only when running the traversal in [TraversalMode::Check] and `should_fix` is `true`
    pub(crate) fn as_fix_file_mode(&self) -> Option<&FixFileMode> {
        match &self.traversal_mode {
            TraversalMode::Check { fix_file_mode, .. }
            | TraversalMode::Lint { fix_file_mode, .. } => fix_file_mode.as_ref(),
            TraversalMode::Format { .. }
            | TraversalMode::CI { .. }
            | TraversalMode::Migrate { .. } => None,
        }
    }

    pub(crate) fn as_diagnostic_category(&self) -> &'static Category {
        match self.traversal_mode {
            TraversalMode::Check { .. } => category!("check"),
            TraversalMode::Lint { .. } => category!("lint"),
            TraversalMode::CI { .. } => category!("ci"),
            TraversalMode::Format { .. } => category!("format"),
            TraversalMode::Migrate { .. } => category!("migrate"),
        }
    }

    pub(crate) const fn is_ci(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::CI { .. })
    }

    pub(crate) const fn is_check(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::Check { .. })
    }

    pub(crate) const fn is_lint(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::Lint { .. })
    }

    pub(crate) const fn is_check_apply(&self) -> bool {
        matches!(
            self.traversal_mode,
            TraversalMode::Check {
                fix_file_mode: Some(FixFileMode::SafeFixes),
                ..
            }
        )
    }

    pub(crate) const fn is_check_apply_unsafe(&self) -> bool {
        matches!(
            self.traversal_mode,
            TraversalMode::Check {
                fix_file_mode: Some(FixFileMode::SafeAndUnsafeFixes),
                ..
            }
        )
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
            TraversalMode::CI { .. } => false,
            TraversalMode::Format { write, .. } => write,
            TraversalMode::Migrate { write: dry_run, .. } => dry_run,
        }
    }

    pub(crate) fn as_stdin_file(&self) -> Option<&Stdin> {
        match &self.traversal_mode {
            TraversalMode::Format { stdin, .. }
            | TraversalMode::Lint { stdin, .. }
            | TraversalMode::Check { stdin, .. } => stdin.as_ref(),
            TraversalMode::CI { .. } | TraversalMode::Migrate { .. } => None,
        }
    }
}

/// Based on the [mode](ExecutionMode), the function might launch a traversal of the file system
/// or handles the stdin file.
pub(crate) fn execute_mode(
    mut mode: Execution,
    session: CliSession,
    cli_options: &CliOptions,
    paths: Vec<OsString>,
) -> Result<(), CliDiagnostic> {
    mode.max_diagnostics = cli_options.max_diagnostics;

    // don't do any traversal if there's some content coming from stdin
    if let Some(stdin) = mode.as_stdin_file() {
        let biome_path = BiomePath::new(stdin.as_path());
        std_in::run(
            session,
            &mode,
            biome_path,
            stdin.as_content(),
            cli_options.verbose,
        )
    } else if let TraversalMode::Migrate {
        write,
        configuration_file_path,
        configuration_directory_path,
        prettier,
    } = mode.traversal_mode
    {
        let payload = MigratePayload {
            session,
            write,
            configuration_file_path,
            configuration_directory_path,
            verbose: cli_options.verbose,
            prettier,
        };
        migrate::run(payload)
    } else {
        traverse(mode, session, cli_options, paths)
    }
}
