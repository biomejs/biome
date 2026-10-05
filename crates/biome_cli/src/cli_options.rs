use biome_configuration::ConfigurationPathHint;
use biome_diagnostics::Severity;
use bpaf::Bpaf;
use camino::{Utf8Path, Utf8PathBuf};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// These options are available to many commands and control general CLI behavior, diagnostics, and output.
#[derive(Debug, Default, Clone, Bpaf)]
pub struct CliOptions {
    /// Controls ANSI styling: `off` disables it, while `force` enables it even when the output does not appear to support ANSI escapes.
    #[bpaf(long("colors"), argument("off|force"))]
    pub colors: Option<ColorsArg>,

    /// Connects to a running instance of the Biome daemon server.
    #[bpaf(long("use-server"), switch, fallback(false))]
    pub use_server: bool,

    /// Shows additional diagnostic details and lists the files that were processed or modified.
    #[bpaf(long("verbose"), switch, fallback(false))]
    pub verbose: bool,

    /// Sets the path to the configuration file or the directory where Biome should search for `biome.json` or `biome.jsonc`.
    /// This option disables the default configuration file resolution.
    #[bpaf(
        long("config-path"),
        env("BIOME_CONFIG_PATH"),
        argument("PATH"),
        optional
    )]
    pub config_path: Option<String>,

    /// Limits the number of diagnostics displayed. Use `none` to remove the limit.
    #[bpaf(
        long("max-diagnostics"),
        argument("none|NUMBER"),
        fallback(MaxDiagnostics::default()),
        display_fallback
    )]
    pub max_diagnostics: MaxDiagnostics,

    /// Skips files containing syntax errors instead of emitting an error diagnostic.
    #[bpaf(long("skip-parse-errors"), switch)]
    pub skip_parse_errors: bool,

    /// Does not emit an error when no files are processed.
    #[bpaf(long("no-errors-on-unmatched"), switch)]
    pub no_errors_on_unmatched: bool,

    /// Exits with an error status if any warning diagnostics are emitted.
    #[bpaf(long("error-on-warnings"), switch)]
    pub error_on_warnings: bool,

    /// Changes how diagnostics and the run summary are written.
    #[bpaf(external, many)]
    pub cli_reporter: Vec<CliReporter>,

    /// Sets the minimum diagnostic severity to display: `info`, `warn`, or `error`.
    /// For example, `--diagnostic-level=error` displays only error-level diagnostics.
    #[bpaf(
        long("diagnostic-level"),
        argument("info|warn|error"),
        fallback(Severity::default()),
        display_fallback
    )]
    pub diagnostic_level: Severity,
}

impl CliOptions {
    /// Computes the [ConfigurationPathHint] based on the options passed by the user.
    pub(crate) fn as_configuration_path_hint(
        &self,
        working_directory: &Utf8Path,
    ) -> ConfigurationPathHint {
        match self.config_path.as_ref() {
            None => ConfigurationPathHint::default(),
            Some(path) => {
                let path = Utf8PathBuf::from(path);
                let path = path.strip_prefix("./").unwrap_or(&path);
                if path.is_absolute() {
                    ConfigurationPathHint::FromUser(path.to_path_buf())
                } else {
                    ConfigurationPathHint::FromUser(working_directory.join(path))
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ColorsArg {
    Off,
    Force,
}

impl ColorsArg {
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Force)
    }
    pub fn is_disabled(&self) -> bool {
        matches!(self, Self::Off)
    }
}

impl FromStr for ColorsArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "force" => Ok(Self::Force),
            _ => Err(format!(
                "value {s:?} is not valid for the --colors argument"
            )),
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Bpaf)]
#[bpaf(adjacent)]
pub struct CliReporter {
    /// Changes how diagnostics and the run summary are written.
    #[bpaf(
        long("reporter"),
        argument(
            "default|concise|summary|json|json-pretty|github|gitlab|junit|checkstyle|rdjson|sarif"
        ),
        fallback(CliReporterKind::default()),
        display_fallback
    )]
    pub(crate) kind: CliReporterKind,

    /// Writes the associated reporter's output to `PATH`. Without `--reporter`, it writes the default reporter's output.
    #[bpaf(long("reporter-file"), argument("PATH"))]
    pub(crate) destination: Option<Utf8PathBuf>,
}

impl CliReporter {
    pub(crate) fn is_file_report(&self) -> bool {
        self.destination.is_some()
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum CliReporterKind {
    /// Full diagnostics and a summary for use in a terminal.
    #[default]
    Default,
    /// Machine-readable JSON.
    Json,
    /// Pretty-printed JSON using Biome's default JSON formatting settings.
    JsonPretty,
    /// [GitHub workflow commands](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-commands) that create annotations.
    GitHub,
    /// JUnit XML for test and CI integrations.
    Junit,
    /// Diagnostics grouped by category and rule, and formatter diagnostics grouped by file.
    Summary,
    /// A [GitLab Code Quality report](https://docs.gitlab.com/ci/testing/code_quality/#implement-a-custom-tool).
    GitLab,
    /// [Checkstyle XML](https://checkstyle.org/).
    Checkstyle,
    /// [Reviewdog diagnostic JSON](https://deepwiki.com/reviewdog/reviewdog/3.2-reviewdog-diagnostic-format).
    RdJson,
    /// [SARIF](https://sarifweb.azurewebsites.net/) for static analysis integrations.
    Sarif,
    /// One line per diagnostic, with formatter diagnostics grouped together.
    Concise,
}

impl CliReporter {
    pub(crate) const fn is_default(&self) -> bool {
        matches!(self.kind, CliReporterKind::Default)
    }
}

impl FromStr for CliReporterKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "json" => Ok(Self::Json),
            "json-pretty" => Ok(Self::JsonPretty),
            "summary" => Ok(Self::Summary),
            "github" => Ok(Self::GitHub),
            "junit" => Ok(Self::Junit),
            "gitlab" => Ok(Self::GitLab),
            "checkstyle" => Ok(Self::Checkstyle),
            "rdjson" => Ok(Self::RdJson),
            "sarif" => Ok(Self::Sarif),
            "concise" => Ok(Self::Concise),
            _ => Err(format!(
                "value {s:?} is not valid for the --reporter argument"
            )),
        }
    }
}

impl Display for CliReporterKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default { .. } => f.write_str("default"),
            Self::Json { .. } => f.write_str("json"),
            Self::JsonPretty { .. } => f.write_str("json-pretty"),
            Self::Summary { .. } => f.write_str("summary"),
            Self::GitHub { .. } => f.write_str("github"),
            Self::Junit { .. } => f.write_str("junit"),
            Self::GitLab { .. } => f.write_str("gitlab"),
            Self::Checkstyle { .. } => f.write_str("checkstyle"),
            Self::RdJson { .. } => f.write_str("rdjson"),
            Self::Sarif { .. } => f.write_str("sarif"),
            Self::Concise { .. } => f.write_str("concise"),
        }
    }
}

#[derive(Debug, Clone, Copy, Bpaf)]
pub enum MaxDiagnostics {
    None,
    Limit(u32),
}

impl MaxDiagnostics {
    pub fn ok(&self) -> Option<u32> {
        match self {
            Self::None => None,
            Self::Limit(value) => Some(*value),
        }
    }

    pub fn exceeded(&self, count: usize) -> bool {
        match self {
            Self::None => false,
            Self::Limit(limit) => count as u32 > *limit,
        }
    }
}

impl Default for MaxDiagnostics {
    fn default() -> Self {
        Self::Limit(20)
    }
}

impl Display for MaxDiagnostics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => {
                write!(f, "none")
            }
            Self::Limit(value) => {
                write!(f, "{value}")
            }
        }
    }
}

impl FromStr for MaxDiagnostics {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            _ => {
                if let Ok(value) = s.parse::<u32>() {
                    Ok(Self::Limit(value))
                } else {
                    Err(format!(
                        "Invalid value provided. Provide 'none' to lift the limit, or a number between 0 and {}.",
                        u32::MAX
                    ))
                }
            }
        }
    }
}

impl From<MaxDiagnostics> for u64 {
    fn from(value: MaxDiagnostics) -> Self {
        match value {
            MaxDiagnostics::None => Self::MAX,
            MaxDiagnostics::Limit(value) => value as Self,
        }
    }
}

impl From<MaxDiagnostics> for u32 {
    fn from(value: MaxDiagnostics) -> Self {
        match value {
            MaxDiagnostics::None => Self::MAX,
            MaxDiagnostics::Limit(value) => value,
        }
    }
}
