use crate::cli_options::{cli_options, CliOptions, ColorsArg};
use crate::diagnostics::DeprecatedConfigurationFile;
use crate::logging::LoggingKind;
use crate::{CliDiagnostic, LoggingLevel, VERSION};
use biome_console::{markup, Console, ConsoleExt};
use biome_diagnostics::PrintDiagnostic;
use biome_service::configuration::json::JsonFormatter;
use biome_service::configuration::vcs::VcsConfiguration;
use biome_service::configuration::{
    configuration, files_configuration, formatter_configuration, javascript::javascript_formatter,
    json::json_formatter, linter_configuration, vcs::vcs_configuration, FilesConfiguration,
    FormatterConfiguration, JavascriptFormatter, LinterConfiguration, LoadedConfiguration,
};
use biome_service::{Configuration, ConfigurationDiagnostic, WorkspaceError};
use bpaf::Bpaf;
use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) mod check;
pub(crate) mod ci;
pub(crate) mod daemon;
pub(crate) mod format;
pub(crate) mod init;
pub(crate) mod lint;
pub(crate) mod migrate;
pub(crate) mod rage;
pub(crate) mod version;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version(VERSION))]
/// Biome official CLI. Use it to check the health of your project or run it to check single files.
pub enum BiomeCommand {
    /// Shows the Biome version information and quit
    #[bpaf(command)]
    Version(#[bpaf(external(cli_options), hide_usage)] CliOptions),

    #[bpaf(command)]
    /// Prints information for debugging
    Rage(
        #[bpaf(external(cli_options), hide_usage)] CliOptions,
        /// Prints the Biome daemon server logs
        #[bpaf(long("daemon-logs"), switch)]
        bool,
    ),
    /// Start the Biome daemon server process
    #[bpaf(command)]
    Start(
        /// Allows to set a custom path when discovering the configuration file `biome.json`
        #[bpaf(env("BIOME_CONFIG_PATH"), long("config-path"), argument("PATH"))]
        Option<PathBuf>,
    ),

    /// Stop the Biome daemon server process
    #[bpaf(command)]
    Stop,

    /// Runs formatter, linter and import sorting to the requested files.
    #[bpaf(command)]
    Check {
        /// Apply safe fixes, formatting and import sorting
        #[bpaf(long("apply"), switch)]
        apply: bool,
        /// Apply safe fixes and unsafe fixes, formatting and import sorting
        #[bpaf(long("apply-unsafe"), switch)]
        apply_unsafe: bool,
        /// Allow to enable or disable the formatter check.
        #[bpaf(
            long("formatter-enabled"),
            argument("true|false"),
            optional,
            hide_usage
        )]
        formatter_enabled: Option<bool>,
        /// Allow to enable or disable the linter check.
        #[bpaf(long("linter-enabled"), argument("true|false"), optional, hide_usage)]
        linter_enabled: Option<bool>,
        /// Allow to enable or disable the organize imports.
        #[bpaf(
            long("organize-imports-enabled"),
            argument("true|false"),
            optional,
            hide_usage
        )]
        organize_imports_enabled: Option<bool>,
        #[bpaf(external, hide_usage, optional)]
        configuration: Option<Configuration>,
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,
        /// Use this option when you want to format code piped from `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the extension of the file. Based on the extension, Biome knows how to check the code.
        ///
        /// Example: `echo 'let a;' | biome check --stdin-file-path=file.js`
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,
        /// Single file, single path or list of paths
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },
    /// Run various checks on a set of files.
    #[bpaf(command)]
    Lint {
        /// Apply safe fixes, formatting and import sorting
        #[bpaf(long("apply"), switch)]
        apply: bool,
        /// Apply safe fixes and unsafe fixes, formatting and import sorting
        #[bpaf(long("apply-unsafe"), switch)]
        apply_unsafe: bool,
        #[bpaf(external, hide_usage, optional)]
        linter_configuration: Option<LinterConfiguration>,

        #[bpaf(external, optional, hide_usage)]
        vcs_configuration: Option<VcsConfiguration>,

        #[bpaf(external, optional, hide_usage)]
        files_configuration: Option<FilesConfiguration>,

        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,
        /// Use this option when you want to format code piped from `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the extension of the file. Based on the extension, Biome knows how to lint the code.
        ///
        /// Example: `echo 'let a;' | biome lint --stdin-file-path=file.js`
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,
        /// Single file, single path or list of paths
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },
    /// Run the formatter on a set of files.
    #[bpaf(command)]
    Format {
        #[bpaf(external, optional, hide_usage)]
        formatter_configuration: Option<FormatterConfiguration>,

        #[bpaf(external, optional, hide_usage)]
        javascript_formatter: Option<JavascriptFormatter>,

        #[bpaf(external, optional, hide_usage)]
        json_formatter: Option<JsonFormatter>,

        #[bpaf(external, optional, hide_usage)]
        vcs_configuration: Option<VcsConfiguration>,

        #[bpaf(external, optional, hide_usage)]
        files_configuration: Option<FilesConfiguration>,
        /// Use this option when you want to format code piped from `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the extension of the file. Based on the extension, Biome knows how to format the code.
        ///
        /// Example: `echo 'let a;' | biome format --stdin-file-path=file.js`
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        /// Writes formatted files to file system.
        #[bpaf(switch)]
        write: bool,

        /// Single file, single path or list of paths.
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },
    /// Command to use in CI environments. Runs formatter, linter and import sorting to the requested files.
    ///
    /// Files won't be modified, the command is a read-only operation.
    #[bpaf(command)]
    Ci {
        /// Allow to enable or disable the formatter check.
        #[bpaf(long("formatter-enabled"), argument("true|false"), optional)]
        formatter_enabled: Option<bool>,
        /// Allow to enable or disable the linter check.
        #[bpaf(long("linter-enabled"), argument("true|false"), optional)]
        linter_enabled: Option<bool>,
        /// Allow to enable or disable the organize imports.
        #[bpaf(long("organize-imports-enabled"), argument("true|false"), optional)]
        organize_imports_enabled: Option<bool>,

        #[bpaf(external, hide_usage)]
        configuration: Configuration,
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        /// Single file, single path or list of paths
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },

    /// Bootstraps a new biome project. Creates a configuration file with some defaults.
    #[bpaf(command)]
    Init,
    /// Acts as a server for the Language Server Protocol over stdin/stdout
    #[bpaf(command("lsp-proxy"))]
    LspProxy(
        /// Allows to set a custom path when discovering the configuration file `biome.json`
        #[bpaf(env("BIOME_CONFIG_PATH"), long("config-path"), argument("PATH"))]
        Option<PathBuf>,
    ),
    /// It updates the configuration when there are breaking changes
    #[bpaf(command)]
    Migrate(
        #[bpaf(external(cli_options), hide_usage)] CliOptions,
        /// Writes the new configuration file to disk
        #[bpaf(long("write"), switch)]
        bool,
    ),

    #[bpaf(command("__run_server"), hide)]
    RunServer {
        #[bpaf(long("stop-on-disconnect"), hide_usage)]
        stop_on_disconnect: bool,
        /// Allows to set a custom path when discovering the configuration file `biome.json`
        #[bpaf(env("BIOME_CONFIG_PATH"), long("config-path"), argument("PATH"))]
        config_path: Option<PathBuf>,
    },
    #[bpaf(command("__print_socket"), hide)]
    PrintSocket,
}

impl BiomeCommand {
    pub const fn get_color(&self) -> Option<&ColorsArg> {
        match self {
            BiomeCommand::Version(cli_options)
            | BiomeCommand::Rage(cli_options, ..)
            | BiomeCommand::Check { cli_options, .. }
            | BiomeCommand::Lint { cli_options, .. }
            | BiomeCommand::Ci { cli_options, .. }
            | BiomeCommand::Format { cli_options, .. }
            | BiomeCommand::Migrate(cli_options, _) => cli_options.colors.as_ref(),
            BiomeCommand::LspProxy(_)
            | BiomeCommand::Start(_)
            | BiomeCommand::Stop
            | BiomeCommand::Init
            | BiomeCommand::RunServer { .. }
            | BiomeCommand::PrintSocket => None,
        }
    }

    pub const fn should_use_server(&self) -> bool {
        match self {
            BiomeCommand::Version(cli_options)
            | BiomeCommand::Rage(cli_options, ..)
            | BiomeCommand::Check { cli_options, .. }
            | BiomeCommand::Lint { cli_options, .. }
            | BiomeCommand::Ci { cli_options, .. }
            | BiomeCommand::Format { cli_options, .. }
            | BiomeCommand::Migrate(cli_options, _) => cli_options.use_server,
            BiomeCommand::Init
            | BiomeCommand::Start(_)
            | BiomeCommand::Stop
            | BiomeCommand::LspProxy(_)
            | BiomeCommand::RunServer { .. }
            | BiomeCommand::PrintSocket => false,
        }
    }

    pub const fn has_metrics(&self) -> bool {
        false
    }

    pub fn is_verbose(&self) -> bool {
        match self {
            BiomeCommand::Check { cli_options, .. }
            | BiomeCommand::Lint { cli_options, .. }
            | BiomeCommand::Format { cli_options, .. }
            | BiomeCommand::Ci { cli_options, .. }
            | BiomeCommand::Migrate(cli_options, _) => cli_options.verbose,
            BiomeCommand::Version(_)
            | BiomeCommand::Rage(..)
            | BiomeCommand::Start(_)
            | BiomeCommand::Stop
            | BiomeCommand::Init
            | BiomeCommand::LspProxy(_)
            | BiomeCommand::RunServer { .. }
            | BiomeCommand::PrintSocket => false,
        }
    }

    pub fn log_level(&self) -> LoggingLevel {
        match self {
            BiomeCommand::Check { cli_options, .. }
            | BiomeCommand::Lint { cli_options, .. }
            | BiomeCommand::Format { cli_options, .. }
            | BiomeCommand::Ci { cli_options, .. }
            | BiomeCommand::Migrate(cli_options, _) => cli_options.log_level.clone(),
            BiomeCommand::Version(_)
            | BiomeCommand::LspProxy(_)
            | BiomeCommand::Rage(..)
            | BiomeCommand::Start(_)
            | BiomeCommand::Stop
            | BiomeCommand::Init
            | BiomeCommand::RunServer { .. }
            | BiomeCommand::PrintSocket => LoggingLevel::default(),
        }
    }
    pub fn log_kind(&self) -> LoggingKind {
        match self {
            BiomeCommand::Check { cli_options, .. }
            | BiomeCommand::Lint { cli_options, .. }
            | BiomeCommand::Format { cli_options, .. }
            | BiomeCommand::Ci { cli_options, .. }
            | BiomeCommand::Migrate(cli_options, _) => cli_options.log_kind.clone(),
            BiomeCommand::Version(_)
            | BiomeCommand::Rage(..)
            | BiomeCommand::LspProxy(_)
            | BiomeCommand::Start(_)
            | BiomeCommand::Stop
            | BiomeCommand::Init
            | BiomeCommand::RunServer { .. }
            | BiomeCommand::PrintSocket => LoggingKind::default(),
        }
    }
}

/// It accepts a [LoadedConfiguration] and it prints the diagnostics emitted during parsing and deserialization.
///
/// If it contains errors, it return an error.
pub(crate) fn validate_configuration_diagnostics(
    loaded_configuration: &LoadedConfiguration,
    console: &mut dyn Console,
    verbose: bool,
) -> Result<(), CliDiagnostic> {
    if let Some(file_path) = loaded_configuration
        .file_path
        .as_ref()
        .and_then(|f| f.file_name())
        .and_then(|f| f.to_str())
    {
        if file_path == "rome.json" {
            let diagnostic = DeprecatedConfigurationFile::new(file_path);
            console.error(markup!{
                {if verbose { PrintDiagnostic::verbose(&diagnostic) } else { PrintDiagnostic::simple(&diagnostic) }}
            });
        }
    }
    let diagnostics = loaded_configuration.as_diagnostics_iter();
    for diagnostic in diagnostics {
        console.error(markup!{
            {if verbose { PrintDiagnostic::verbose(diagnostic) } else { PrintDiagnostic::simple(diagnostic) }}
        });
    }
    if loaded_configuration.has_errors() {
        return Err(CliDiagnostic::workspace_error(
            WorkspaceError::Configuration(ConfigurationDiagnostic::invalid_configuration(
                "Biome exited because the configuration resulted in errors. Please fix them.",
            )),
        ));
    }

    Ok(())
}
