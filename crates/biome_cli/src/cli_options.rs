use crate::logging::LoggingKind;
use crate::LoggingLevel;
use biome_configuration::ConfigurationPathHint;
use biome_diagnostics::Severity;
use bpaf::Bpaf;
use camino::Utf8PathBuf;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Global options applied to all commands
#[derive(Debug, Clone, Bpaf)]
pub struct CliOptions {
    /// Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
    #[bpaf(long("colors"), argument("off|force"))]
    pub colors: Option<ColorsArg>,

    /// Connect to a running instance of the Biome daemon server.
    #[bpaf(long("use-server"), switch, fallback(false))]
    pub use_server: bool,

    /// Print additional diagnostics, and some diagnostics show more information. Also, print out what files were processed and which ones were modified.
    #[bpaf(long("verbose"), switch, fallback(false))]
    pub verbose: bool,

    /// Set the file path to the configuration file, or the directory path to find `biome.json` or `biome.jsonc`.
    /// If used, it disables the default configuration file resolution.
    #[bpaf(long("config-path"), argument("PATH"), optional)]
    pub config_path: Option<String>,

    /// Cap the amount of diagnostics displayed. When `none` is provided, the limit is lifted.
    #[bpaf(
        long("max-diagnostics"),
        argument("none|<NUMBER>"),
        fallback(MaxDiagnostics::default()),
        display_fallback
    )]
    pub max_diagnostics: MaxDiagnostics,

    /// Skip over files containing syntax errors instead of emitting an error diagnostic.
    #[bpaf(long("skip-errors"), switch)]
    pub skip_errors: bool,

    /// Silence errors that would be emitted in case no files were processed during the execution of the command.
    #[bpaf(long("no-errors-on-unmatched"), switch)]
    pub no_errors_on_unmatched: bool,

    /// Tell Biome to exit with an error code if some diagnostics emit warnings.
    #[bpaf(long("error-on-warnings"), switch)]
    pub error_on_warnings: bool,

    /// Allows to change how diagnostics and summary are reported.
    #[bpaf(
        long("reporter"),
        argument("json|json-pretty|github|junit|summary|gitlab"),
        fallback(CliReporter::default())
    )]
    pub reporter: CliReporter,

    #[bpaf(
        long("log-level"),
        argument("none|debug|info|warn|error"),
        fallback(LoggingLevel::default()),
        display_fallback
    )]
    /// The level of logging. In order, from the most verbose to the least verbose: debug, info, warn, error.
    ///
    /// The value `none` won't show any logging.
    pub log_level: LoggingLevel,

    /// How the log should look like.
    #[bpaf(
        long("log-kind"),
        argument("pretty|compact|json"),
        fallback(LoggingKind::default()),
        display_fallback
    )]
    pub log_kind: LoggingKind,

    #[bpaf(
        long("diagnostic-level"),
        argument("info|warn|error"),
        fallback(Severity::default()),
        display_fallback
    )]
    /// The level of diagnostics to show. In order, from the lowest to the most important: info, warn, error. Passing `--diagnostic-level=error` will cause Biome to print only diagnostics that contain only errors.
    pub diagnostic_level: Severity,
}

impl CliOptions {
    /// Computes the [ConfigurationPathHint] based on the options passed by the user
    pub(crate) fn as_configuration_path_hint(&self) -> ConfigurationPathHint {
        match self.config_path.as_ref() {
            None => ConfigurationPathHint::default(),
            Some(path) => ConfigurationPathHint::FromUser(Utf8PathBuf::from(path)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ColorsArg {
    Off,
    Force,
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

#[derive(Debug, Default, Clone)]
pub enum CliReporter {
    /// The default reporter
    #[default]
    Default,
    /// Reports information using the JSON format
    Json,
    /// Reports information using the JSON format, formatted.
    JsonPretty,
    /// Diagnostics are printed for GitHub workflow commands
    GitHub,
    /// Diagnostics and summary are printed in JUnit format
    Junit,
    /// Reports linter diagnostics grouped by category and number of hits. Reports formatter diagnostics grouped by file.
    Summary,
    /// Reports linter diagnostics using the [GitLab Code Quality report](https://docs.gitlab.com/ee/ci/testing/code_quality.html#implement-a-custom-tool).
    GitLab,
}

impl CliReporter {
    pub(crate) const fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }
}

impl FromStr for CliReporter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "json-pretty" => Ok(Self::JsonPretty),
            "summary" => Ok(Self::Summary),
            "github" => Ok(Self::GitHub),
            "junit" => Ok(Self::Junit),
            "gitlab" => Ok(Self::GitLab),
            _ => Err(format!(
                "value {s:?} is not valid for the --reporter argument"
            )),
        }
    }
}

impl Display for CliReporter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliReporter::Default => f.write_str("default"),
            CliReporter::Json => f.write_str("json"),
            CliReporter::JsonPretty => f.write_str("json-pretty"),
            CliReporter::Summary => f.write_str("summary"),
            CliReporter::GitHub => f.write_str("github"),
            CliReporter::Junit => f.write_str("junit"),
            CliReporter::GitLab => f.write_str("gitlab"),
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
            MaxDiagnostics::None => None,
            MaxDiagnostics::Limit(value) => Some(*value),
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
            MaxDiagnostics::None => {
                write!(f, "none")
            }
            MaxDiagnostics::Limit(value) => {
                write!(f, "{value}")
            }
        }
    }
}

impl FromStr for MaxDiagnostics {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(MaxDiagnostics::None),
            _ => {
                if let Ok(value) = s.parse::<u32>() {
                    Ok(MaxDiagnostics::Limit(value))
                } else {
                    Err(format!("Invalid value provided. Provide 'none' to lift the limit, or a number between 0 and {}.", u32::MAX))
                }
            }
        }
    }
}

impl From<MaxDiagnostics> for u64 {
    fn from(value: MaxDiagnostics) -> Self {
        match value {
            MaxDiagnostics::None => u64::MAX,
            MaxDiagnostics::Limit(value) => value as u64,
        }
    }
}

impl From<MaxDiagnostics> for u32 {
    fn from(value: MaxDiagnostics) -> Self {
        match value {
            MaxDiagnostics::None => u32::MAX,
            MaxDiagnostics::Limit(value) => value,
        }
    }
}
