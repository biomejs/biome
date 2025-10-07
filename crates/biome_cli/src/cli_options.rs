use crate::LoggingLevel;
use crate::logging::LoggingKind;
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
    #[bpaf(
        long("config-path"),
        env("BIOME_CONFIG_PATH"),
        argument("PATH"),
        optional
    )]
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
    #[bpaf(long("skip-parse-errors"), switch)]
    pub skip_parse_errors: bool,

    /// Silence errors that would be emitted in case no files were processed during the execution of the command.
    #[bpaf(long("no-errors-on-unmatched"), switch)]
    pub no_errors_on_unmatched: bool,

    /// Tell Biome to exit with an error code if some diagnostics emit warnings.
    #[bpaf(long("error-on-warnings"), switch)]
    pub error_on_warnings: bool,

    /// Allows to change how diagnostics and summary are reported.
    #[bpaf(
        long("reporter"),
        argument("json|json-pretty|github|junit|summary|gitlab|checkstyle|rdjson"),
        fallback(CliReporter::default())
    )]
    pub reporter: CliReporter,

    /// Optional path to redirect log messages to.
    ///
    /// If omitted, logs are printed to stdout.
    #[bpaf(long("log-file"))]
    pub log_file: Option<String>,

    /// The level of logging. In order, from the most verbose to the least
    /// verbose: debug, info, warn, error.
    ///
    /// The value `none` won't show any logging.
    #[bpaf(
        long("log-level"),
        argument("none|debug|info|warn|error"),
        fallback(LoggingLevel::default()),
        display_fallback
    )]
    pub log_level: LoggingLevel,

    /// How the log should look like.
    #[bpaf(
        long("log-kind"),
        argument("pretty|compact|json"),
        fallback(LoggingKind::default()),
        display_fallback
    )]
    pub log_kind: LoggingKind,

    /// The level of diagnostics to show. In order, from the lowest to the most important: info, warn, error. Passing `--diagnostic-level=error` will cause Biome to print only diagnostics that contain only errors.
    #[bpaf(
        long("diagnostic-level"),
        argument("info|warn|error"),
        fallback(Severity::default()),
        display_fallback
    )]
    pub diagnostic_level: Severity,
}

impl CliOptions {
    /// Computes the [ConfigurationPathHint] based on the options passed by the user
    pub(crate) fn as_configuration_path_hint(&self) -> ConfigurationPathHint {
        match self.config_path.as_ref() {
            None => ConfigurationPathHint::default(),
            Some(path) => {
                let path = Utf8PathBuf::from(path);
                let path = path.strip_prefix("./").unwrap_or(&path);
                ConfigurationPathHint::FromUser(path.to_path_buf())
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

#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
    /// Reports diagnostics grouped by category and number of hits. Reports formatter diagnostics grouped by file.
    Summary,
    /// Reports diagnostics using the [GitLab Code Quality report](https://docs.gitlab.com/ee/ci/testing/code_quality.html#implement-a-custom-tool).
    GitLab,
    /// Reports diagnostics in Checkstyle XML format
    Checkstyle,
    /// Reports diagnostics using the [Reviewdog JSON format](https://deepwiki.com/reviewdog/reviewdog/3.2-reviewdog-diagnostic-format)
    RdJson,
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
            "checkstyle" => Ok(Self::Checkstyle),
            "rdjson" => Ok(Self::RdJson),
            _ => Err(format!(
                "value {s:?} is not valid for the --reporter argument"
            )),
        }
    }
}

impl Display for CliReporter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => f.write_str("default"),
            Self::Json => f.write_str("json"),
            Self::JsonPretty => f.write_str("json-pretty"),
            Self::Summary => f.write_str("summary"),
            Self::GitHub => f.write_str("github"),
            Self::Junit => f.write_str("junit"),
            Self::GitLab => f.write_str("gitlab"),
            Self::Checkstyle => f.write_str("checkstyle"),
            Self::RdJson => f.write_str("rdjson"),
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
