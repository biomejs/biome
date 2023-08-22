use bpaf::Bpaf;
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

    /// Print additional verbose advices on diagnostics
    #[bpaf(long("verbose"), switch, fallback(false))]
    pub verbose: bool,

    /// Set the filesystem path to the directory of the biome.json configuration file
    #[bpaf(long("config-path"), argument("PATH"), optional)]
    pub config_path: Option<String>,

    /// Cap the amount of diagnostics displayed.
    #[bpaf(
        long("max-diagnostics"),
        argument("NUMBER"),
        fallback(20),
        display_fallback
    )]
    pub max_diagnostics: u16,

    /// Skip over files containing syntax errors instead of emitting an error diagnostic.
    #[bpaf(long("skip-errors"), switch)]
    pub skip_errors: bool,

    /// Silence errors that would be emitted in case no files were processed during the execution of the command.
    #[bpaf(long("no-errors-on-unmatched"), switch)]
    pub no_errors_on_unmatched: bool,

    /// Tell Biome to exit with an error code if some diagnostics emit warnings.
    #[bpaf(long("error-on-warnings"), switch)]
    pub error_on_warnings: bool,

    /// Reports information using the JSON format
    #[bpaf(long("json"), switch, hide_usage, hide)]
    pub json: bool,
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
