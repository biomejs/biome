use crate::changed::{get_changed_files, get_staged_files};
use crate::cli_options::{cli_options, CliOptions, CliReporter, ColorsArg};
use crate::diagnostics::{DeprecatedArgument, DeprecatedConfigurationFile};
use crate::execute::Stdin;
use crate::logging::LoggingKind;
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, LoggingLevel, VERSION,
};
use biome_configuration::analyzer::RuleSelector;
use biome_configuration::css::PartialCssLinter;
use biome_configuration::javascript::PartialJavascriptLinter;
use biome_configuration::json::PartialJsonLinter;
use biome_configuration::{
    css::partial_css_formatter, css::partial_css_linter, graphql::partial_graphql_formatter,
    graphql::partial_graphql_linter, javascript::partial_javascript_formatter,
    javascript::partial_javascript_linter, json::partial_json_formatter, json::partial_json_linter,
    partial_configuration, partial_files_configuration, partial_formatter_configuration,
    partial_linter_configuration, vcs::partial_vcs_configuration, vcs::PartialVcsConfiguration,
    PartialCssFormatter, PartialFilesConfiguration, PartialFormatterConfiguration,
    PartialGraphqlFormatter, PartialGraphqlLinter, PartialJavascriptFormatter,
    PartialJsonFormatter, PartialLinterConfiguration,
};
use biome_configuration::{BiomeDiagnostic, PartialConfiguration};
use biome_console::{markup, Console, ConsoleExt};
use biome_diagnostics::{Diagnostic, PrintDiagnostic};
use biome_fs::{BiomePath, FileSystem};
use biome_service::configuration::{
    load_configuration, load_editorconfig, LoadedConfiguration, PartialConfigurationExt,
};
use biome_service::documentation::Doc;
use biome_service::workspace::{FixFileMode, RegisterProjectFolderParams, UpdateSettingsParams};
use biome_service::{DynRef, Workspace, WorkspaceError};
use bpaf::Bpaf;
use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) mod check;
pub(crate) mod ci;
pub(crate) mod clean;
pub(crate) mod daemon;
pub(crate) mod explain;
pub(crate) mod format;
pub(crate) mod init;
pub(crate) mod lint;
pub(crate) mod migrate;
pub(crate) mod rage;
pub(crate) mod search;
pub(crate) mod version;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version(VERSION))]
/// Biome official CLI. Use it to check the health of your project or run it to check single files.
pub enum BiomeCommand {
    /// Shows the Biome version information and quit.
    #[bpaf(command)]
    Version(#[bpaf(external(cli_options), hide_usage)] CliOptions),

    #[bpaf(command)]
    /// Prints information for debugging.
    Rage(
        #[bpaf(external(cli_options), hide_usage)] CliOptions,
        /// Prints the Biome daemon server logs
        #[bpaf(long("daemon-logs"), switch)]
        bool,
        /// Prints the formatter options applied
        #[bpaf(long("formatter"), switch)]
        bool,
        /// Prints the linter options applied
        #[bpaf(long("linter"), switch)]
        bool,
    ),
    /// Starts the Biome daemon server process.
    #[bpaf(command)]
    Start {
        /// Allows to change the prefix applied to the file name of the logs.
        #[bpaf(
            env("BIOME_LOG_PREFIX_NAME"),
            long("log-prefix-name"),
            argument("STRING"),
            hide_usage,
            fallback(String::from("server.log")),
            display_fallback
        )]
        log_prefix_name: String,

        /// Allows to change the folder where logs are stored.
        #[bpaf(
            env("BIOME_LOG_PATH"),
            long("log-path"),
            argument("PATH"),
            hide_usage,
            fallback(biome_fs::ensure_cache_dir().join("biome-logs")),
        )]
        log_path: PathBuf,
        /// Allows to set a custom file path to the configuration file,
        /// or a custom directory path to find `biome.json` or `biome.jsonc`
        #[bpaf(env("BIOME_CONFIG_PATH"), long("config-path"), argument("PATH"))]
        config_path: Option<PathBuf>,
    },

    /// Stops the Biome daemon server process.
    #[bpaf(command)]
    Stop,

    /// Runs formatter, linter and import sorting to the requested files.
    #[bpaf(command)]
    Check {
        /// Writes safe fixes, formatting and import sorting
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Allow to do unsafe fixes, should be used with `--write` or `--fix`
        #[bpaf(long("unsafe"), switch)]
        unsafe_: bool,

        /// Alias for `--write`, writes safe fixes, formatting and import sorting
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        /// Alias for `--write`, writes safe fixes, formatting and import sorting (deprecated, use `--write`)
        #[bpaf(long("apply"), switch, hide_usage)]
        apply: bool,

        /// Alias for `--write --unsafe`, writes safe and unsafe fixes, formatting and import sorting (deprecated, use `--write --unsafe`)
        #[bpaf(long("apply-unsafe"), switch, hide_usage)]
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

        /// Allow to enable or disable the assists.
        #[bpaf(long("assists-enabled"), argument("true|false"), optional)]
        assists_enabled: Option<bool>,

        #[bpaf(external(partial_configuration), hide_usage, optional)]
        configuration: Option<PartialConfiguration>,
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,
        /// Use this option when you want to format code piped from `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the extension of the file. Based on the extension, Biome knows how to check the code.
        ///
        /// Example: `echo 'let a;' | biome check --stdin-file-path=file.js`
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        /// When set to true, only the files that have been staged (the ones prepared to be committed)
        /// will be linted. This option should be used when working locally.
        #[bpaf(long("staged"), switch)]
        staged: bool,

        /// When set to true, only the files that have been changed compared to your `defaultBranch`
        /// configuration will be linted. This option should be used in CI environments.
        #[bpaf(long("changed"), switch)]
        changed: bool,

        /// Use this to specify the base branch to compare against when you're using the --changed
        /// flag and the `defaultBranch` is not set in your `biome.json`
        #[bpaf(long("since"), argument("REF"))]
        since: Option<String>,

        /// Single file, single path or list of paths
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },
    /// Run various checks on a set of files.
    #[bpaf(command)]
    Lint {
        /// Writes safe fixes
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Fix diagnostics with suppression comments if the language supports it.
        #[bpaf(long("suppress"))]
        suppress: bool,

        /// Allow to do unsafe fixes, should be used with `--write` or `--fix`
        #[bpaf(long("unsafe"), switch)]
        unsafe_: bool,

        /// Alias for `--write`, writes safe fixes
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        /// Alias for `--write`, writes safe fixes (deprecated, use `--write`)
        #[bpaf(long("apply"), switch, hide_usage)]
        apply: bool,

        /// Alias for `--write --unsafe`, writes safe and unsafe fixes (deprecated, use `--write --unsafe`)
        #[bpaf(long("apply-unsafe"), switch, hide_usage)]
        apply_unsafe: bool,

        #[bpaf(external(partial_linter_configuration), hide_usage, optional)]
        linter_configuration: Option<PartialLinterConfiguration>,

        #[bpaf(external(partial_vcs_configuration), optional, hide_usage)]
        vcs_configuration: Option<PartialVcsConfiguration>,

        #[bpaf(external(partial_files_configuration), optional, hide_usage)]
        files_configuration: Option<PartialFilesConfiguration>,

        #[bpaf(external(partial_javascript_linter), optional, hide_usage)]
        javascript_linter: Option<PartialJavascriptLinter>,

        #[bpaf(external(partial_json_linter), optional, hide_usage)]
        json_linter: Option<PartialJsonLinter>,

        #[bpaf(external(partial_css_linter), optional, hide_usage, hide)]
        css_linter: Option<PartialCssLinter>,

        #[bpaf(external(partial_graphql_linter), optional, hide_usage, hide)]
        graphql_linter: Option<PartialGraphqlLinter>,

        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        /// Run only the given rule or group of rules.
        /// If the severity level of a rule is `off`,
        /// then the severity level of the rule is set to `error` if it is a recommended rule or `warn` otherwise.
        ///
        /// Example: `biome lint --only=correctness/noUnusedVariables --only=suspicious`
        #[bpaf(long("only"), argument("GROUP|RULE"))]
        only: Vec<RuleSelector>,

        /// Skip the given rule or group of rules by setting the severity level of the rules to `off`.
        /// This option takes precedence over `--only`.
        ///
        /// Example: `biome lint --skip=correctness/noUnusedVariables --skip=suspicious`
        #[bpaf(long("skip"), argument("GROUP|RULE"))]
        skip: Vec<RuleSelector>,

        /// Use this option when you want to format code piped from `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the extension of the file. Based on the extension, Biome knows how to lint the code.
        ///
        /// Example: `echo 'let a;' | biome lint --stdin-file-path=file.js`
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,
        /// When set to true, only the files that have been staged (the ones prepared to be committed)
        /// will be linted.
        #[bpaf(long("staged"), switch)]
        staged: bool,
        /// When set to true, only the files that have been changed compared to your `defaultBranch`
        /// configuration will be linted.
        #[bpaf(long("changed"), switch)]
        changed: bool,
        /// Use this to specify the base branch to compare against when you're using the --changed
        /// flag and the `defaultBranch` is not set in your biome.json
        #[bpaf(long("since"), argument("REF"))]
        since: Option<String>,
        /// Single file, single path or list of paths
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },
    /// Run the formatter on a set of files.
    #[bpaf(command)]
    Format {
        #[bpaf(external(partial_formatter_configuration), optional, hide_usage)]
        formatter_configuration: Option<PartialFormatterConfiguration>,

        #[bpaf(external(partial_javascript_formatter), optional, hide_usage)]
        javascript_formatter: Option<PartialJavascriptFormatter>,

        #[bpaf(external(partial_json_formatter), optional, hide_usage)]
        json_formatter: Option<PartialJsonFormatter>,

        #[bpaf(external(partial_css_formatter), optional, hide_usage, hide)]
        css_formatter: Option<PartialCssFormatter>,

        #[bpaf(external(partial_graphql_formatter), optional, hide_usage, hide)]
        graphql_formatter: Option<PartialGraphqlFormatter>,

        #[bpaf(external(partial_vcs_configuration), optional, hide_usage)]
        vcs_configuration: Option<PartialVcsConfiguration>,

        #[bpaf(external(partial_files_configuration), optional, hide_usage)]
        files_configuration: Option<PartialFilesConfiguration>,
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
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Alias of `--write`, writes formatted files to file system.
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        /// When set to true, only the files that have been staged (the ones prepared to be committed)
        /// will be linted.
        #[bpaf(long("staged"), switch)]
        staged: bool,

        /// When set to true, only the files that have been changed compared to your `defaultBranch`
        /// configuration will be linted.
        #[bpaf(long("changed"), switch)]
        changed: bool,

        /// Use this to specify the base branch to compare against when you're using the --changed
        /// flag and the `defaultBranch` is not set in your biome.json
        #[bpaf(long("since"), argument("REF"))]
        since: Option<String>,

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

        /// Allow to enable or disable the assists.
        #[bpaf(long("assists-enabled"), argument("true|false"), optional)]
        assists_enabled: Option<bool>,

        #[bpaf(external(partial_configuration), hide_usage, optional)]
        configuration: Option<PartialConfiguration>,
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        /// When set to true, only the files that have been changed compared to your `defaultBranch`
        /// configuration will be linted.
        #[bpaf(long("changed"), switch)]
        changed: bool,

        /// Use this to specify the base branch to compare against when you're using the --changed
        /// flag and the `defaultBranch` is not set in your biome.json
        #[bpaf(long("since"), argument("REF"))]
        since: Option<String>,

        /// Single file, single path or list of paths
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },

    /// Bootstraps a new biome project. Creates a configuration file with some defaults.
    #[bpaf(command)]
    Init(
        /// Tells Biome to emit a `biome.jsonc` file.
        #[bpaf(long("jsonc"), switch)]
        bool,
    ),
    /// Acts as a server for the Language Server Protocol over stdin/stdout.
    #[bpaf(command("lsp-proxy"))]
    LspProxy {
        /// Allows to change the prefix applied to the file name of the logs.
        #[bpaf(
            env("BIOME_LOG_PREFIX_NAME"),
            long("log-prefix-name"),
            argument("STRING"),
            hide_usage,
            fallback(String::from("server.log")),
            display_fallback
        )]
        log_prefix_name: String,
        /// Allows to change the folder where logs are stored.
        #[bpaf(
            env("BIOME_LOG_PATH"),
            long("log-path"),
            argument("PATH"),
            hide_usage,
            fallback(biome_fs::ensure_cache_dir().join("biome-logs")),
        )]
        log_path: PathBuf,
        /// Allows to set a custom file path to the configuration file,
        /// or a custom directory path to find `biome.json` or `biome.jsonc`
        #[bpaf(env("BIOME_CONFIG_PATH"), long("config-path"), argument("PATH"))]
        config_path: Option<PathBuf>,
        /// Bogus argument to make the command work with vscode-languageclient
        #[bpaf(long("stdio"), hide, hide_usage, switch)]
        stdio: bool,
    },
    /// Updates the configuration when there are breaking changes.
    #[bpaf(command)]
    Migrate {
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        /// Writes the new configuration file to disk
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Alias of `--write`, writes the new configuration file to disk
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        #[bpaf(external(migrate_sub_command), optional)]
        sub_command: Option<MigrateSubCommand>,
    },

    /// EXPERIMENTAL: Searches for Grit patterns across a project.
    ///
    /// Note: GritQL escapes code snippets using backticks, but most shells
    /// interpret backticks as command invocations. To avoid this, it's best to
    /// put single quotes around your Grit queries.
    ///
    /// ## Example
    ///
    /// ```shell
    /// biome search '`console.log($message)`' # find all `console.log` invocations
    /// ```
    #[bpaf(command)]
    Search {
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        #[bpaf(external(partial_files_configuration), optional, hide_usage)]
        files_configuration: Option<PartialFilesConfiguration>,

        #[bpaf(external(partial_vcs_configuration), optional, hide_usage)]
        vcs_configuration: Option<PartialVcsConfiguration>,

        /// Use this option when you want to search through code piped from
        /// `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the
        /// extension of the file. Based on the extension, Biome knows how to
        /// parse the code.
        ///
        /// Example: `echo 'let a;' | biome search '`let $var`' --stdin-file-path=file.js`
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        /// The GritQL pattern to search for.
        ///
        /// Note that the search command (currently) does not support rewrites.
        #[bpaf(positional("PATTERN"))]
        pattern: String,

        /// Single file, single path or list of paths.
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },

    /// Shows documentation of various aspects of the CLI.
    ///
    /// ## Examples
    ///
    /// ```shell
    /// biome explain noDebugger
    /// ```
    ///
    /// ```shell
    /// biome explain daemon-logs
    /// ```
    #[bpaf(command)]
    Explain {
        /// Single name to display documentation for.
        #[bpaf(positional("NAME"))]
        doc: Doc,
    },

    #[bpaf(command)]
    /// Cleans the logs emitted by the daemon.
    Clean,

    #[bpaf(command("__run_server"), hide)]
    RunServer {
        /// Allows to change the prefix applied to the file name of the logs.
        #[bpaf(
            env("BIOME_LOG_PREFIX_NAME"),
            long("log-prefix-name"),
            argument("STRING"),
            hide_usage,
            fallback(String::from("server.log")),
            display_fallback
        )]
        log_prefix_name: String,
        /// Allows to change the folder where logs are stored.
        #[bpaf(
            env("BIOME_LOG_PATH"),
            long("log-path"),
            argument("PATH"),
            hide_usage,
            fallback(biome_fs::ensure_cache_dir().join("biome-logs")),
        )]
        log_path: PathBuf,

        #[bpaf(long("stop-on-disconnect"), hide_usage)]
        stop_on_disconnect: bool,
        /// Allows to set a custom file path to the configuration file,
        /// or a custom directory path to find `biome.json` or `biome.jsonc`
        #[bpaf(env("BIOME_CONFIG_PATH"), long("config-path"), argument("PATH"))]
        config_path: Option<PathBuf>,
    },
    #[bpaf(command("__print_socket"), hide)]
    PrintSocket,
}

#[derive(Debug, Bpaf, Clone)]
pub enum MigrateSubCommand {
    /// It attempts to find the files `.prettierrc`/`prettier.json` and `.prettierignore`, and map the Prettier's configuration into Biome's configuration file.
    #[bpaf(command)]
    Prettier,
    /// It attempts to find the ESLint configuration file in the working directory, and update the Biome's configuration file as a result.
    #[bpaf(command)]
    Eslint {
        /// Includes rules inspired from an eslint rule in the migration
        #[bpaf(long("include-inspired"))]
        include_inspired: bool,
        /// Includes nursery rules in the migration
        #[bpaf(long("include-nursery"))]
        include_nursery: bool,
    },
}

impl MigrateSubCommand {
    pub const fn is_prettier(&self) -> bool {
        matches!(self, MigrateSubCommand::Prettier)
    }
}

impl BiomeCommand {
    const fn cli_options(&self) -> Option<&CliOptions> {
        match self {
            BiomeCommand::Version(cli_options)
            | BiomeCommand::Rage(cli_options, ..)
            | BiomeCommand::Check { cli_options, .. }
            | BiomeCommand::Lint { cli_options, .. }
            | BiomeCommand::Ci { cli_options, .. }
            | BiomeCommand::Format { cli_options, .. }
            | BiomeCommand::Migrate { cli_options, .. }
            | BiomeCommand::Search { cli_options, .. } => Some(cli_options),
            BiomeCommand::LspProxy { .. }
            | BiomeCommand::Start { .. }
            | BiomeCommand::Stop
            | BiomeCommand::Init(_)
            | BiomeCommand::Explain { .. }
            | BiomeCommand::RunServer { .. }
            | BiomeCommand::Clean { .. }
            | BiomeCommand::PrintSocket => None,
        }
    }

    pub const fn get_color(&self) -> Option<&ColorsArg> {
        match self.cli_options() {
            Some(cli_options) => {
                // To properly display GitHub annotations we need to disable colors
                if matches!(cli_options.reporter, CliReporter::GitHub) {
                    return Some(&ColorsArg::Off);
                }
                // We want force colors in CI, to give e better UX experience
                // Unless users explicitly set the colors flag
                if matches!(self, BiomeCommand::Ci { .. }) && cli_options.colors.is_none() {
                    return Some(&ColorsArg::Force);
                }
                // Normal behaviors
                cli_options.colors.as_ref()
            }
            None => None,
        }
    }

    pub const fn should_use_server(&self) -> bool {
        match self.cli_options() {
            Some(cli_options) => cli_options.use_server,
            None => false,
        }
    }

    pub const fn has_metrics(&self) -> bool {
        false
    }

    pub fn is_verbose(&self) -> bool {
        self.cli_options()
            .map_or(false, |cli_options| cli_options.verbose)
    }

    pub fn log_level(&self) -> LoggingLevel {
        self.cli_options()
            .map_or(LoggingLevel::default(), |cli_options| cli_options.log_level)
    }

    pub fn log_kind(&self) -> LoggingKind {
        self.cli_options()
            .map_or(LoggingKind::default(), |cli_options| cli_options.log_kind)
    }
}

/// It accepts a [LoadedPartialConfiguration] and it prints the diagnostics emitted during parsing and deserialization.
///
/// If it contains [errors](Severity::Error) or higher, it returns an error.
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
            if diagnostic.tags().is_verbose() && verbose {
                console.error(markup! {{PrintDiagnostic::verbose(&diagnostic)}})
            } else {
                console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}})
            }
        }
    }
    let diagnostics = loaded_configuration.as_diagnostics_iter();
    for diagnostic in diagnostics {
        if diagnostic.tags().is_verbose() && verbose {
            console.error(markup! {{PrintDiagnostic::verbose(diagnostic)}})
        } else {
            console.error(markup! {{PrintDiagnostic::simple(diagnostic)}})
        }
    }

    if loaded_configuration.has_errors() {
        return Err(CliDiagnostic::workspace_error(
            BiomeDiagnostic::invalid_configuration(
                "Biome exited because the configuration resulted in errors. Please fix them.",
            )
            .into(),
        ));
    }

    Ok(())
}

fn resolve_manifest(
    fs: &DynRef<'_, dyn FileSystem>,
) -> Result<Option<(BiomePath, String)>, WorkspaceError> {
    let result = fs.auto_search(
        &fs.working_directory().unwrap_or_default(),
        &["package.json"],
        false,
    )?;

    if let Some(result) = result {
        return Ok(Some((BiomePath::new(result.file_path), result.content)));
    }

    Ok(None)
}

fn get_files_to_process_with_cli_options(
    since: Option<&str>,
    changed: bool,
    staged: bool,
    fs: &DynRef<'_, dyn FileSystem>,
    configuration: &PartialConfiguration,
) -> Result<Option<Vec<OsString>>, CliDiagnostic> {
    if since.is_some() {
        if !changed {
            return Err(CliDiagnostic::incompatible_arguments("since", "changed"));
        }
        if staged {
            return Err(CliDiagnostic::incompatible_arguments("since", "staged"));
        }
    }

    if changed {
        if staged {
            return Err(CliDiagnostic::incompatible_arguments("changed", "staged"));
        }
        Ok(Some(get_changed_files(fs, configuration, since)?))
    } else if staged {
        Ok(Some(get_staged_files(fs)?))
    } else {
        Ok(None)
    }
}

/// Holds the options to determine the fix file mode.
pub(crate) struct FixFileModeOptions {
    apply: bool,
    apply_unsafe: bool,
    write: bool,
    suppress: bool,
    fix: bool,
    unsafe_: bool,
}

/// - [Result]: if the given options are incompatible
/// - [Option]: if no fixes are requested
/// - [FixFileMode]: if safe or unsafe fixes are requested
pub(crate) fn determine_fix_file_mode(
    options: FixFileModeOptions,
    console: &mut dyn Console,
) -> Result<Option<FixFileMode>, CliDiagnostic> {
    let FixFileModeOptions {
        apply,
        apply_unsafe,
        write,
        fix,
        suppress,
        unsafe_,
    } = options;

    if apply || apply_unsafe {
        let (deprecated, alternative) = if apply {
            ("--apply", "--write")
        } else {
            ("--apply-unsafe", "--write --unsafe")
        };
        let diagnostic = DeprecatedArgument::new(markup! {
            "The argument "<Emphasis>{deprecated}</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>{alternative}</Emphasis>" instead."
        });
        console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
    }

    check_fix_incompatible_arguments(options)?;

    let safe_fixes = apply || write || fix;
    let unsafe_fixes = apply_unsafe || ((write || safe_fixes) && unsafe_);

    if unsafe_fixes {
        Ok(Some(FixFileMode::SafeAndUnsafeFixes))
    } else if safe_fixes {
        Ok(Some(FixFileMode::SafeFixes))
    } else if suppress {
        Ok(Some(FixFileMode::ApplySuppressions))
    } else {
        Ok(None)
    }
}

/// Checks if the fix file options are incompatible.
fn check_fix_incompatible_arguments(options: FixFileModeOptions) -> Result<(), CliDiagnostic> {
    let FixFileModeOptions {
        apply,
        apply_unsafe,
        write,
        suppress,
        fix,
        unsafe_,
    } = options;
    if apply && apply_unsafe {
        return Err(CliDiagnostic::incompatible_arguments(
            "--apply",
            "--apply-unsafe",
        ));
    } else if apply_unsafe && unsafe_ {
        return Err(CliDiagnostic::incompatible_arguments(
            "--apply-unsafe",
            "--unsafe",
        ));
    } else if apply && (fix || write) {
        return Err(CliDiagnostic::incompatible_arguments(
            "--apply",
            if fix { "--fix" } else { "--write" },
        ));
    } else if apply_unsafe && (fix || write) {
        return Err(CliDiagnostic::incompatible_arguments(
            "--apply-unsafe",
            if fix { "--fix" } else { "--write" },
        ));
    } else if write && fix {
        return Err(CliDiagnostic::incompatible_arguments("--write", "--fix"));
    } else if suppress && write {
        return Err(CliDiagnostic::incompatible_arguments(
            "--suppress",
            "--write",
        ));
    } else if suppress && fix {
        return Err(CliDiagnostic::incompatible_arguments("--suppress", "--fix"));
    }
    Ok(())
}

/// Generic interface for executing commands.
///
/// Consumers must implement the following methods:
///
/// - [CommandRunner::merge_configuration]
/// - [CommandRunner::get_files_to_process]
/// - [CommandRunner::get_stdin_file_path]
/// - [CommandRunner::should_write]
/// - [CommandRunner::get_execution]
///
/// Optional methods:
/// - [CommandRunner::check_incompatible_arguments]
pub(crate) trait CommandRunner: Sized {
    const COMMAND_NAME: &'static str;

    /// The main command to use.
    fn run(&mut self, session: CliSession, cli_options: &CliOptions) -> Result<(), CliDiagnostic> {
        setup_cli_subscriber(cli_options.log_level, cli_options.log_kind);
        let fs = &session.app.fs;
        let console = &mut *session.app.console;
        let workspace = &*session.app.workspace;
        self.check_incompatible_arguments()?;
        let (execution, paths) = self.configure_workspace(fs, console, workspace, cli_options)?;
        execute_mode(execution, session, cli_options, paths)
    }

    /// This function prepares the workspace with the following:
    /// - Loading the configuration file.
    /// - Configure the VCS integration
    /// - Computes the paths to traverse/handle. This changes based on the VCS arguments that were passed.
    /// - Register a project folder using the working directory.
    /// - Resolves the closets manifest AKA `package.json` and registers it.
    /// - Updates the settings that belong to the project registered
    fn configure_workspace(
        &mut self,
        fs: &DynRef<'_, dyn FileSystem>,
        console: &mut dyn Console,
        workspace: &dyn Workspace,
        cli_options: &CliOptions,
    ) -> Result<(Execution, Vec<OsString>), CliDiagnostic> {
        let loaded_configuration =
            load_configuration(fs, cli_options.as_configuration_path_hint())?;
        if self.should_validate_configuration_diagnostics() {
            validate_configuration_diagnostics(
                &loaded_configuration,
                console,
                cli_options.verbose,
            )?;
        }
        let configuration_path = loaded_configuration.directory_path.clone();
        let configuration = self.merge_configuration(loaded_configuration, fs, console)?;
        let vcs_base_path = configuration_path.or(fs.working_directory());
        let (vcs_base_path, gitignore_matches) =
            configuration.retrieve_gitignore_matches(fs, vcs_base_path.as_deref())?;
        let paths = self.get_files_to_process(fs, &configuration)?;
        workspace.register_project_folder(RegisterProjectFolderParams {
            path: fs.working_directory(),
            set_as_current_workspace: true,
        })?;

        let manifest_data = resolve_manifest(fs)?;

        if let Some(manifest_data) = manifest_data {
            workspace.set_manifest_for_project(manifest_data.into())?;
        }
        workspace.update_settings(UpdateSettingsParams {
            workspace_directory: fs.working_directory(),
            configuration,
            vcs_base_path,
            gitignore_matches,
        })?;

        let execution = self.get_execution(cli_options, console, workspace)?;
        Ok((execution, paths))
    }

    /// Computes [Stdin] if the CLI has the necessary information.
    ///
    /// ## Errors
    /// - If the user didn't provide anything via `stdin` but the option `--stdin-file-path` is passed.
    fn get_stdin(&self, console: &mut dyn Console) -> Result<Option<Stdin>, CliDiagnostic> {
        let stdin = if let Some(stdin_file_path) = self.get_stdin_file_path() {
            let input_code = console.read();
            if let Some(input_code) = input_code {
                let path = PathBuf::from(stdin_file_path);
                Some((path, input_code).into())
            } else {
                // we provided the argument without a piped stdin, we bail
                return Err(CliDiagnostic::missing_argument("stdin", Self::COMMAND_NAME));
            }
        } else {
            None
        };

        Ok(stdin)
    }

    // Below, the methods that consumers must implement.

    /// Implements this method if you need to merge CLI arguments to the loaded configuration.
    ///
    /// The CLI arguments take precedence over the option configured in the configuration file.
    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        fs: &DynRef<'_, dyn FileSystem>,
        console: &mut dyn Console,
    ) -> Result<PartialConfiguration, WorkspaceError>;

    /// It returns the paths that need to be handled/traversed.
    fn get_files_to_process(
        &self,
        fs: &DynRef<'_, dyn FileSystem>,
        configuration: &PartialConfiguration,
    ) -> Result<Vec<OsString>, CliDiagnostic>;

    /// It returns the file path to use in `stdin` mode.
    fn get_stdin_file_path(&self) -> Option<&str>;

    /// Whether the command should write the files.
    fn should_write(&self) -> bool;

    /// Returns the [Execution] mode.
    fn get_execution(
        &self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
        workspace: &dyn Workspace,
    ) -> Result<Execution, CliDiagnostic>;

    // Below, methods that consumers can implement

    /// Optional method that can be implemented to check if some CLI arguments aren't compatible.
    ///
    /// The method is called before loading the configuration from disk.
    fn check_incompatible_arguments(&self) -> Result<(), CliDiagnostic> {
        Ok(())
    }

    /// Checks whether the configuration has errors.
    fn should_validate_configuration_diagnostics(&self) -> bool {
        true
    }
}

pub trait LoadEditorConfig: CommandRunner {
    /// Whether this command should load the `.editorconfig` file.
    fn should_load_editor_config(&self, fs_configuration: &PartialConfiguration) -> bool;

    /// It loads the `.editorconfig` from the file system, parses it and deserialize it into a [PartialConfiguration]
    fn load_editor_config(
        &self,
        configuration_path: Option<PathBuf>,
        fs_configuration: &PartialConfiguration,
        fs: &DynRef<'_, dyn FileSystem>,
        console: &mut dyn Console,
    ) -> Result<PartialConfiguration, WorkspaceError> {
        Ok(if self.should_load_editor_config(fs_configuration) {
            let (editorconfig, editorconfig_diagnostics) = {
                let search_path = configuration_path
                    .clone()
                    .unwrap_or_else(|| fs.working_directory().unwrap_or_default());
                load_editorconfig(fs, search_path)?
            };
            for diagnostic in editorconfig_diagnostics {
                console.error(markup! {
                    {PrintDiagnostic::simple(&diagnostic)}
                })
            }
            editorconfig.unwrap_or_default()
        } else {
            Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use biome_console::BufferConsole;

    use super::*;

    #[test]
    fn incompatible_arguments() {
        for (apply, apply_unsafe, write, suppress, fix, unsafe_) in [
            (true, true, false, false, false, false), // --apply --apply-unsafe
            (true, false, true, false, false, false), // --apply --write
            (true, false, false, false, true, false), // --apply --fix
            (false, true, false, false, false, true), // --apply-unsafe --unsafe
            (false, true, true, false, false, false), // --apply-unsafe --write
            (false, true, false, false, true, false), // --apply-unsafe --fix
            (false, false, true, false, true, false), // --write --fix
        ] {
            assert!(check_fix_incompatible_arguments(FixFileModeOptions {
                apply,
                apply_unsafe,
                write,
                suppress,
                fix,
                unsafe_
            })
            .is_err());
        }
    }

    #[test]
    fn safe_fixes() {
        let mut console = BufferConsole::default();

        for (apply, apply_unsafe, write, suppress, fix, unsafe_) in [
            (true, false, false, false, false, false), // --apply
            (false, false, true, false, false, false), // --write
            (false, false, false, false, true, false), // --fix
        ] {
            assert_eq!(
                determine_fix_file_mode(
                    FixFileModeOptions {
                        apply,
                        apply_unsafe,
                        write,
                        suppress,
                        fix,
                        unsafe_
                    },
                    &mut console
                )
                .unwrap(),
                Some(FixFileMode::SafeFixes)
            );
        }
    }

    #[test]
    fn safe_and_unsafe_fixes() {
        let mut console = BufferConsole::default();

        for (apply, apply_unsafe, write, suppress, fix, unsafe_) in [
            (false, true, false, false, false, false), // --apply-unsafe
            (false, false, true, false, false, true),  // --write --unsafe
            (false, false, false, false, true, true),  // --fix --unsafe
        ] {
            assert_eq!(
                determine_fix_file_mode(
                    FixFileModeOptions {
                        apply,
                        apply_unsafe,
                        write,
                        suppress,
                        fix,
                        unsafe_
                    },
                    &mut console
                )
                .unwrap(),
                Some(FixFileMode::SafeAndUnsafeFixes)
            );
        }
    }

    #[test]
    fn no_fix() {
        let mut console = BufferConsole::default();

        let (apply, apply_unsafe, write, suppress, fix, unsafe_) =
            (false, false, false, false, false, false);
        assert_eq!(
            determine_fix_file_mode(
                FixFileModeOptions {
                    apply,
                    apply_unsafe,
                    write,
                    suppress,
                    fix,
                    unsafe_
                },
                &mut console
            )
            .unwrap(),
            None
        );
    }

    /// Tests that all CLI options adhere to the invariants expected by `bpaf`.
    #[test]
    fn check_options() {
        biome_command().check_invariants(false);
    }
}
