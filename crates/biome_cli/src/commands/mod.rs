use crate::changed::{get_changed_files, get_staged_files};
use crate::cli_options::{CliOptions, CliReporter, ColorsArg, cli_options};
use crate::commands::scan_kind::derive_best_scan_kind;
use crate::execute::Stdin;
use crate::logging::LoggingKind;
use crate::{
    CliDiagnostic, CliSession, Execution, LoggingLevel, TraversalMode, VERSION, execute_mode,
    setup_cli_subscriber,
};
use biome_configuration::analyzer::assist::AssistEnabled;
use biome_configuration::analyzer::{AnalyzerSelector, LinterEnabled};
use biome_configuration::css::{
    CssFormatterConfiguration, CssLinterConfiguration, CssParserConfiguration,
};
use biome_configuration::formatter::{FormatWithErrorsEnabled, FormatterEnabled};
use biome_configuration::graphql::{GraphqlFormatterConfiguration, GraphqlLinterConfiguration};
use biome_configuration::html::{HtmlFormatterConfiguration, html_formatter_configuration};
use biome_configuration::javascript::{JsFormatterConfiguration, JsLinterConfiguration};
use biome_configuration::json::{
    JsonFormatterConfiguration, JsonLinterConfiguration, JsonParserConfiguration,
};
use biome_configuration::vcs::VcsConfiguration;
use biome_configuration::{BiomeDiagnostic, Configuration};
use biome_configuration::{
    FilesConfiguration, FormatterConfiguration, LinterConfiguration, configuration,
    css::{css_formatter_configuration, css_linter_configuration, css_parser_configuration},
    files_configuration, formatter_configuration,
    graphql::graphql_formatter_configuration,
    graphql::graphql_linter_configuration,
    javascript::js_formatter_configuration,
    javascript::js_linter_configuration,
    json::json_formatter_configuration,
    json::json_linter_configuration,
    json::json_parser_configuration,
    linter_configuration,
    vcs::vcs_configuration,
};
use biome_console::{Console, ConsoleExt, markup};
use biome_deserialize::Merge;
use biome_diagnostics::{Diagnostic, PrintDiagnostic, Severity};
use biome_fs::{BiomePath, FileSystem};
use biome_grit_patterns::GritTargetLanguage;
use biome_resolver::FsWithResolverProxy;
use biome_service::configuration::{
    LoadedConfiguration, ProjectScanComputer, load_configuration, load_editorconfig,
};
use biome_service::documentation::Doc;
use biome_service::projects::ProjectKey;
use biome_service::workspace::{
    FixFileMode, OpenProjectParams, ScanKind, ScanProjectParams, UpdateSettingsParams,
};
use biome_service::{Workspace, WorkspaceError};
use bpaf::Bpaf;
use camino::{Utf8Path, Utf8PathBuf};
use std::ffi::OsString;
use std::time::Duration;
use tracing::info;

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
mod scan_kind;
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
        log_path: Utf8PathBuf,
    },

    /// Stops the Biome daemon server process.
    #[bpaf(command)]
    Stop,

    /// Runs formatter, linter and import sorting to the requested files.
    #[bpaf(command)]
    Check {
        /// Apply safe fixes, formatting and import sorting
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Apply unsafe fixes. Should be used with `--write` or `--fix`
        #[bpaf(long("unsafe"), switch)]
        unsafe_: bool,

        /// Alias for `--write`, writes safe fixes, formatting and import sorting
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        /// Allow enabling or disabling the formatter check.
        #[bpaf(
            long("formatter-enabled"),
            argument("true|false"),
            optional,
            hide_usage
        )]
        formatter_enabled: Option<FormatterEnabled>,
        /// Allow enabling or disabling the linter check.
        #[bpaf(long("linter-enabled"), argument("true|false"), optional, hide_usage)]
        linter_enabled: Option<LinterEnabled>,

        /// Allow enabling or disabling the assist.
        #[bpaf(long("assist-enabled"), argument("true|false"), optional)]
        assist_enabled: Option<AssistEnabled>,

        /// Allows enforcing assist, and make the CLI fail if some actions aren't applied. Defaults to `true`.
        #[bpaf(long("enforce-assist"), argument("true|false"), fallback(true))]
        enforce_assist: bool,

        /// Whether formatting should be allowed to proceed if a given file
        /// has syntax errors
        #[bpaf(long("format-with-errors"), argument("true|false"))]
        format_with_errors: Option<FormatWithErrorsEnabled>,

        #[bpaf(external(json_parser_configuration), optional, hide_usage)]
        json_parser: Option<JsonParserConfiguration>,

        #[bpaf(external(css_parser_configuration), optional, hide_usage, hide)]
        css_parser: Option<CssParserConfiguration>,

        #[bpaf(external(configuration), hide_usage, optional)]
        configuration: Option<Configuration>,
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,
        /// Use this option when you want to format code piped from `stdin`, and
        /// print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the
        /// extension of the file. Based on the extension, Biome knows how to
        /// check the code.
        ///
        /// Also, if you have overrides configured and/or nested configurations,
        /// the path may determine the settings being applied.
        ///
        /// Example:
        /// ```shell
        /// echo 'let a;' | biome check --stdin-file-path=file.js --write
        /// ```
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

        /// Apply unsafe fixes. Should be used with `--write` or `--fix`
        #[bpaf(long("unsafe"), switch)]
        unsafe_: bool,

        /// Alias for `--write`, writes safe fixes
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        /// Fixes lint rule violations with comment suppressions instead of using a rule code action (fix)
        #[bpaf(long("suppress"))]
        suppress: bool,

        /// Explanation for suppressing diagnostics with `--suppress`
        #[bpaf(long("reason"), argument("STRING"))]
        suppression_reason: Option<String>,

        #[bpaf(external(json_parser_configuration), optional, hide_usage)]
        json_parser: Option<JsonParserConfiguration>,

        #[bpaf(external(css_parser_configuration), optional, hide_usage, hide)]
        css_parser: Option<CssParserConfiguration>,

        #[bpaf(external(linter_configuration), hide_usage, optional)]
        linter_configuration: Option<LinterConfiguration>,

        #[bpaf(external(vcs_configuration), optional, hide_usage)]
        vcs_configuration: Option<VcsConfiguration>,

        #[bpaf(external(files_configuration), optional, hide_usage)]
        files_configuration: Option<FilesConfiguration>,

        #[bpaf(external(js_linter_configuration), optional, hide_usage)]
        javascript_linter: Option<JsLinterConfiguration>,

        #[bpaf(external(json_linter_configuration), optional, hide_usage)]
        json_linter: Option<JsonLinterConfiguration>,

        #[bpaf(external(css_linter_configuration), optional, hide_usage, hide)]
        css_linter: Option<CssLinterConfiguration>,

        #[bpaf(external(graphql_linter_configuration), optional, hide_usage, hide)]
        graphql_linter: Option<GraphqlLinterConfiguration>,

        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        /// Run only the given rule, group of rules or domain.
        /// If the severity level of a rule is `off`,
        /// then the severity level of the rule is set to `error` if it is a recommended rule or `warn` otherwise.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome lint --only=correctness/noUnusedVariables --only=suspicious --only=test
        /// ```
        #[bpaf(long("only"), argument("GROUP|RULE|DOMAIN"))]
        only: Vec<AnalyzerSelector>,

        /// Skip the given rule, group of rules or domain by setting the severity level of the rules to `off`.
        /// This option takes precedence over `--only`.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome lint --skip=correctness/noUnusedVariables --skip=suspicious --skip=project
        /// ```
        #[bpaf(long("skip"), argument("GROUP|RULE|DOMAIN"))]
        skip: Vec<AnalyzerSelector>,

        /// Use this option when you want to format code piped from `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the extension of the file. Based on the extension, Biome knows how to lint the code.
        ///
        /// Example:
        /// ```shell
        /// echo 'let a;' | biome lint --stdin-file-path=file.js --write
        /// ```
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
        #[bpaf(external(formatter_configuration), optional, hide_usage)]
        formatter_configuration: Option<FormatterConfiguration>,

        #[bpaf(external(js_formatter_configuration), optional, hide_usage)]
        javascript_formatter: Option<JsFormatterConfiguration>,

        #[bpaf(external(json_formatter_configuration), optional, hide_usage)]
        json_formatter: Option<JsonFormatterConfiguration>,

        #[bpaf(external(json_parser_configuration), optional, hide_usage)]
        json_parser: Option<JsonParserConfiguration>,

        #[bpaf(external(css_parser_configuration), optional, hide_usage, hide)]
        css_parser: Option<CssParserConfiguration>,

        #[bpaf(external(graphql_formatter_configuration), optional, hide_usage, hide)]
        graphql_formatter: Option<GraphqlFormatterConfiguration>,

        #[bpaf(external(css_formatter_configuration), optional, hide_usage, hide)]
        css_formatter: Option<CssFormatterConfiguration>,

        #[bpaf(external(html_formatter_configuration), optional, hide_usage, hide)]
        html_formatter: Option<HtmlFormatterConfiguration>,

        #[bpaf(external(vcs_configuration), optional, hide_usage)]
        vcs_configuration: Option<VcsConfiguration>,

        #[bpaf(external(files_configuration), optional, hide_usage)]
        files_configuration: Option<FilesConfiguration>,
        /// Use this option when you want to format code piped from `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the extension of the file. Based on the extension, Biome knows how to format the code.
        ///
        /// Example:
        /// ```shell
        /// echo 'let a;' | biome format --stdin-file-path=file.js --write
        /// ```
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        /// Writes formatted files to a file system.
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Alias of `--write`, writes formatted files to a file system.
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
        /// flag, and the `defaultBranch` is not set in your biome.json
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
        /// Allow enabling or disabling the formatter check.
        #[bpaf(long("formatter-enabled"), argument("true|false"), optional)]
        formatter_enabled: Option<FormatterEnabled>,
        /// Allow enabling or disable the linter check.
        #[bpaf(long("linter-enabled"), argument("true|false"), optional)]
        linter_enabled: Option<LinterEnabled>,

        /// Allow enabling or disabling the assist.
        #[bpaf(long("assist-enabled"), argument("true|false"), optional)]
        assist_enabled: Option<AssistEnabled>,

        /// Whether formatting should be allowed to proceed if a given file
        /// has syntax errors
        #[bpaf(long("format-with-errors"), argument("true|false"))]
        format_with_errors: Option<FormatWithErrorsEnabled>,

        #[bpaf(external(json_parser_configuration), optional, hide_usage)]
        json_parser: Option<JsonParserConfiguration>,

        #[bpaf(external(css_parser_configuration), optional, hide_usage, hide)]
        css_parser: Option<CssParserConfiguration>,

        /// Allows enforcing assist, and make the CLI fail if some actions aren't applied. Defaults to `true`.
        #[bpaf(long("enforce-assist"), argument("true|false"), fallback(true))]
        enforce_assist: bool,

        #[bpaf(external(configuration), hide_usage, optional)]
        configuration: Option<Configuration>,
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

        /// The number of threads to use. This is useful when running the CLI in environments
        /// with limited resource, for example CI.
        #[bpaf(
            long("threads"),
            argument("NUMBER"),
            env("BIOME_THREADS"),
            optional,
            hide_usage
        )]
        threads: Option<usize>,

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
        log_path: Utf8PathBuf,
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
    /// ### Example
    ///
    /// ```shell
    /// biome search '`console.log($message)`' # find all `console.log` invocations
    /// ```
    #[bpaf(command)]
    Search {
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        #[bpaf(external(files_configuration), optional, hide_usage)]
        files_configuration: Option<FilesConfiguration>,

        #[bpaf(external(vcs_configuration), optional, hide_usage)]
        vcs_configuration: Option<VcsConfiguration>,

        /// Use this option when you want to search through code piped from
        /// `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the
        /// extension of the file. Based on the extension, Biome knows how to
        /// parse the code.
        ///
        /// Example:
        /// ```shell
        /// echo 'let a;' | biome search '`let $var`' --stdin-file-path=file.js
        /// ```
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        /// The language to which the pattern applies.
        ///
        /// Grit queries are specific to the grammar of the language they
        /// target, so we currently do not support writing queries that apply
        /// to multiple languages at once.
        ///
        /// When none, the default language is JavaScript.
        #[bpaf(long("language"), short('l'))]
        language: Option<GritTargetLanguage>,

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
    /// ### Examples
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
        /// Allows changing the prefix applied to the file name of the logs.
        #[bpaf(
            env("BIOME_LOG_PREFIX_NAME"),
            long("log-prefix-name"),
            argument("STRING"),
            hide_usage,
            fallback(String::from("server.log")),
            display_fallback
        )]
        log_prefix_name: String,
        /// Allows changing the folder where logs are stored.
        #[bpaf(
            env("BIOME_LOG_PATH"),
            long("log-path"),
            argument("PATH"),
            hide_usage,
            fallback(
                biome_fs::ensure_cache_dir().join("biome-logs")
            ),
        )]
        log_path: Utf8PathBuf,

        #[bpaf(long("stop-on-disconnect"), hide_usage)]
        stop_on_disconnect: bool,
    },
    #[bpaf(command("__print_socket"), hide)]
    PrintSocket,

    #[bpaf(command("__where_am_i"), hide)]
    WhereAmI,
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
        matches!(self, Self::Prettier)
    }
}

impl BiomeCommand {
    const fn cli_options(&self) -> Option<&CliOptions> {
        match self {
            Self::Version(cli_options)
            | Self::Rage(cli_options, ..)
            | Self::Check { cli_options, .. }
            | Self::Lint { cli_options, .. }
            | Self::Ci { cli_options, .. }
            | Self::Format { cli_options, .. }
            | Self::Migrate { cli_options, .. }
            | Self::Search { cli_options, .. } => Some(cli_options),
            Self::LspProxy { .. }
            | Self::Start { .. }
            | Self::Stop
            | Self::Init(_)
            | Self::Explain { .. }
            | Self::RunServer { .. }
            | Self::Clean { .. }
            | Self::PrintSocket => None,
            Self::WhereAmI => None,
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
                if matches!(self, Self::Ci { .. }) && cli_options.colors.is_none() {
                    return Some(&ColorsArg::Force);
                }
                // Normal behaviors
                cli_options.colors.as_ref()
            }
            None => None,
        }
    }

    pub const fn get_threads(&self) -> Option<usize> {
        match self {
            Self::Ci { threads, .. } => *threads,
            _ => None,
        }
    }

    pub const fn should_use_server(&self) -> bool {
        match self.cli_options() {
            Some(cli_options) => cli_options.use_server,
            None => false,
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.cli_options()
            .is_some_and(|cli_options| cli_options.verbose)
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

/// It accepts a [LoadedConfiguration] and it prints the diagnostics emitted during parsing and deserialization.
///
/// If it contains [errors](Severity::Error) or higher, it returns an error.
pub(crate) fn validate_configuration_diagnostics(
    loaded_configuration: &LoadedConfiguration,
    console: &mut dyn Console,
    verbose: bool,
) -> Result<(), CliDiagnostic> {
    let diagnostics = loaded_configuration.as_diagnostics_iter();

    // We want to print the diagnostics only if there are errors. Other diagnostics such as
    // information/warnings will be printed during the traversal
    if loaded_configuration.has_errors() {
        for diagnostic in diagnostics {
            if diagnostic.tags().is_verbose() && verbose {
                console.error(markup! {{PrintDiagnostic::verbose(diagnostic)}})
            } else {
                console.error(markup! {{PrintDiagnostic::simple(diagnostic)}})
            }
        }

        return Err(CliDiagnostic::workspace_error(
            BiomeDiagnostic::invalid_configuration(
                "Biome exited because the configuration resulted in errors. Please fix them.",
            )
            .into(),
        ));
    }

    Ok(())
}

pub(crate) fn print_diagnostics_from_workspace_result(
    diagnostics: &[biome_diagnostics::serde::Diagnostic],
    console: &mut dyn Console,
    verbose: bool,
) -> Result<(), CliDiagnostic> {
    let mut has_errors = false;
    let mut has_internal = false;
    for diagnostic in diagnostics {
        has_errors = has_errors || diagnostic.severity() >= Severity::Error;
        has_internal = has_internal || diagnostic.tags().is_internal();
        if has_internal || has_errors {
            if diagnostic.tags().is_verbose() && verbose {
                console.error(markup! {{PrintDiagnostic::verbose(diagnostic)}})
            } else {
                console.error(markup! {{PrintDiagnostic::simple(diagnostic)}})
            }
        }
    }

    if has_errors {
        return Err(CliDiagnostic::workspace_error(
            BiomeDiagnostic::invalid_configuration(
                "Biome exited because the configuration resulted in errors. Please fix them.",
            )
            .into(),
        ));
    }

    Ok(())
}

fn get_files_to_process_with_cli_options(
    since: Option<&str>,
    changed: bool,
    staged: bool,
    fs: &dyn FileSystem,
    configuration: &Configuration,
) -> Result<Option<Vec<OsString>>, CliDiagnostic> {
    if since.is_some() {
        if !changed {
            return Err(CliDiagnostic::incompatible_arguments(
                "--since",
                "--changed",
                "In order to use --since, you must also use --changed.",
            ));
        }
        if staged {
            return Err(CliDiagnostic::incompatible_arguments(
                "--since",
                "--staged",
                "--staged selects files that you have staged in version control. --since can't be used in this context.",
            ));
        }
    }

    if changed {
        if staged {
            return Err(CliDiagnostic::incompatible_arguments(
                "--changed",
                "--staged",
                "--staged selects files that you have staged in version control. --changed selects files that have been committed since your default branch. You must either use --changed or --staged, but not both.",
            ));
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
    write: bool,
    suppress: bool,
    suppression_reason: Option<String>,
    fix: bool,
    unsafe_: bool,
}

/// - [Result]: if the given options are incompatible
/// - [Option]: if no fixes are requested
/// - [FixFileMode]: if safe or unsafe fixes are requested
pub(crate) fn determine_fix_file_mode(
    options: FixFileModeOptions,
) -> Result<Option<FixFileMode>, CliDiagnostic> {
    let FixFileModeOptions {
        write,
        fix,
        suppress,
        suppression_reason: _,
        unsafe_,
    } = options;

    check_fix_incompatible_arguments(options)?;

    let safe_fixes = write || fix;
    let unsafe_fixes = (write || safe_fixes) && unsafe_;

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
        write,
        suppress,
        suppression_reason,
        fix,
        ..
    } = options;
    if write && fix {
        return Err(CliDiagnostic::incompatible_arguments(
            "--write",
            "--fix",
            "These arguments do the same thing, but --fix is deprecated. Prefer to use --write.",
        ));
    } else if suppress && write {
        return Err(CliDiagnostic::incompatible_arguments(
            "--suppress",
            "--write",
            "--write is used to write fixes, and --suppress is used to suppress diagnostics. Remove one of these arguments depending on what you want to do.",
        ));
    } else if suppress && fix {
        return Err(CliDiagnostic::incompatible_arguments(
            "--suppress",
            "--fix",
            "--fix is used to write fixes, and --suppress is used to suppress diagnostics. Remove one of these arguments depending on what you want to do. Also, --fix is deprecated. Prefer to use --write.",
        ));
    } else if !suppress && suppression_reason.is_some() {
        return Err(CliDiagnostic::unexpected_argument(
            "--reason",
            "`--reason` is only valid when `--suppress` is used.",
        ));
    };
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
        setup_cli_subscriber(
            cli_options.log_file.as_deref(),
            cli_options.log_level,
            cli_options.log_kind,
            cli_options.colors.as_ref(),
        );
        let console = &mut *session.app.console;
        let workspace = &*session.app.workspace;
        let fs = workspace.fs();
        self.check_incompatible_arguments()?;
        let ConfiguredWorkspace {
            execution,
            paths,
            duration,
            configuration_files,
            project_key,
        } = self.configure_workspace(fs, console, workspace, cli_options)?;
        execute_mode(
            execution,
            session,
            cli_options,
            paths,
            duration,
            configuration_files,
            project_key,
        )
    }

    /// This function prepares the workspace with the following:
    /// - Loading the configuration file.
    /// - Configure the VCS integration
    /// - Computes the paths to traverse/handle. This changes based on the VCS arguments that were passed.
    /// - Register a project folder using the working directory.
    /// - Updates the settings that belong to the project registered
    fn configure_workspace(
        &mut self,
        fs: &dyn FsWithResolverProxy,
        console: &mut dyn Console,
        workspace: &dyn Workspace,
        cli_options: &CliOptions,
    ) -> Result<ConfiguredWorkspace, CliDiagnostic> {
        // Load configuration
        let configuration_path_hint = cli_options.as_configuration_path_hint();
        let loaded_configuration = load_configuration(fs, configuration_path_hint)?;
        if self.should_validate_configuration_diagnostics() {
            validate_configuration_diagnostics(
                &loaded_configuration,
                console,
                cli_options.verbose,
            )?;
        }
        info!(
            "Configuration file loaded: {:?}, diagnostics detected {}",
            loaded_configuration.file_path,
            loaded_configuration.diagnostics.len(),
        );
        let configuration_dir_path = loaded_configuration.directory_path.clone();

        // Merge the FS configuration with the CLI arguments
        let configuration = self.merge_configuration(loaded_configuration, fs, console)?;

        let execution = self.get_execution(cli_options, console, workspace)?;

        let working_dir = fs.working_directory().unwrap_or_default();
        let root_configuration_dir = configuration_dir_path
            .clone()
            .unwrap_or_else(|| working_dir.clone());
        // Using `--config-path`, users can point to a (root) config file that
        // is not actually at the root of the project. So between the working
        // directory and configuration directory, we use whichever one is higher
        // up in the file system.
        let project_dir = if root_configuration_dir.starts_with(&working_dir) {
            &working_dir
        } else {
            &root_configuration_dir
        };

        let paths = self.get_files_to_process(fs, &configuration)?;
        let paths = validated_paths_for_execution(paths, &execution, &working_dir)?;

        // Open the project
        let open_project_result = workspace.open_project(OpenProjectParams {
            path: BiomePath::new(project_dir),
            open_uninitialized: true,
        })?;

        let scan_kind_computer =
            if let TraversalMode::Lint { only, skip, .. } = execution.traversal_mode() {
                ProjectScanComputer::new(&configuration).with_rule_selectors(skip, only)
            } else {
                ProjectScanComputer::new(&configuration)
            };
        let scan_kind = derive_best_scan_kind(
            scan_kind_computer.compute(),
            &execution,
            &root_configuration_dir,
            &working_dir,
            &configuration,
        );

        // Update the settings of the project
        let result = workspace.update_settings(UpdateSettingsParams {
            project_key: open_project_result.project_key,
            workspace_directory: Some(BiomePath::new(project_dir)),
            configuration,
        })?;
        if self.should_validate_configuration_diagnostics() {
            print_diagnostics_from_workspace_result(
                result.diagnostics.as_slice(),
                console,
                cli_options.verbose,
            )?;
        }

        // Scan the project
        let scan_kind = match (scan_kind, execution.traversal_mode()) {
            (scan_kind, TraversalMode::Migrate { .. }) => scan_kind,
            (ScanKind::KnownFiles, _) => {
                let target_paths = paths
                    .iter()
                    .map(|path| BiomePath::new(working_dir.join(path)))
                    .collect();
                ScanKind::TargetedKnownFiles {
                    target_paths,
                    descend_from_targets: true,
                }
            }
            (scan_kind, _) => scan_kind,
        };
        let result = workspace.scan_project(ScanProjectParams {
            project_key: open_project_result.project_key,
            watch: cli_options.use_server,
            force: false, // TODO: Maybe we'll want a CLI flag for this.
            scan_kind,
            verbose: cli_options.verbose,
        })?;

        if self.should_validate_configuration_diagnostics() {
            print_diagnostics_from_workspace_result(
                result.diagnostics.as_slice(),
                console,
                cli_options.verbose,
            )?;
        }

        Ok(ConfiguredWorkspace {
            execution,
            paths,
            duration: Some(result.duration),
            configuration_files: result.configuration_files,
            project_key: open_project_result.project_key,
        })
    }

    /// Computes [Stdin] if the CLI has the necessary information.
    ///
    /// ## Errors
    /// - If the user didn't provide anything via `stdin` but the option `--stdin-file-path` is passed.
    fn get_stdin(&self, console: &mut dyn Console) -> Result<Option<Stdin>, CliDiagnostic> {
        let stdin = if let Some(stdin_file_path) = self.get_stdin_file_path() {
            let input_code = console.read();
            if let Some(input_code) = input_code {
                let path = Utf8PathBuf::from(stdin_file_path);
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

    // #region Methods that consumers must implement

    /// Implements this method if you need to merge CLI arguments to the loaded configuration.
    ///
    /// The CLI arguments take precedence over the option configured in the configuration file.
    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        fs: &dyn FileSystem,
        console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError>;

    /// It returns the paths that need to be handled/traversed.
    fn get_files_to_process(
        &self,
        fs: &dyn FileSystem,
        configuration: &Configuration,
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

    // #endregion
}

/// Validates `paths` so they can be safely passed to the given `execution`.
///
/// - Converts paths from `OsString` to `String`.
/// - If the `execution` expects paths to be given, we may initialise them with
///   the current directory if they were empty otherwise.
fn validated_paths_for_execution(
    paths: Vec<OsString>,
    execution: &Execution,
    working_dir: &Utf8Path,
) -> Result<Vec<String>, CliDiagnostic> {
    let mut paths = paths
        .into_iter()
        .map(|path| path.into_string().map_err(WorkspaceError::non_utf8_path))
        .collect::<Result<Vec<_>, _>>()?;

    if paths.is_empty() {
        match execution.traversal_mode() {
            TraversalMode::Check { .. }
            | TraversalMode::Lint { .. }
            | TraversalMode::Format { .. }
            | TraversalMode::CI { .. }
            | TraversalMode::Search { .. } => {
                if execution.is_vcs_targeted() {
                    // If `--staged` or `--changed` is specified, it's
                    // acceptable for them to be empty, so ignore it.
                } else {
                    paths.push(working_dir.to_string());
                }
            }
            TraversalMode::Migrate { .. } => {
                // Migrate doesn't do any traversal, so it doesn't care.
            }
        }
    }

    Ok(paths)
}

pub(crate) struct ConfiguredWorkspace {
    /// Execution context
    pub execution: Execution,
    /// Paths to crawl
    pub paths: Vec<String>,
    /// The duration of the scanning
    pub duration: Option<Duration>,
    /// Configuration files found inside the project
    pub configuration_files: Vec<BiomePath>,
    /// The unique identifier of the project
    pub project_key: ProjectKey,
}

pub trait LoadEditorConfig: CommandRunner {
    /// Whether this command should load the `.editorconfig` file.
    fn should_load_editor_config(&self, fs_configuration: &Configuration) -> bool;

    /// It loads the `.editorconfig` from the file system, parses it and deserialize it into a [Configuration]
    fn load_editor_config(
        &self,
        configuration_path: Option<Utf8PathBuf>,
        fs_configuration: &Configuration,
        fs: &dyn FileSystem,
    ) -> Result<Option<Configuration>, WorkspaceError> {
        Ok(if self.should_load_editor_config(fs_configuration) {
            let (editorconfig, _editorconfig_diagnostics) = {
                let search_path = fs.working_directory().unwrap_or_default();

                load_editorconfig(fs, search_path, configuration_path)?
            };
            editorconfig
        } else {
            Default::default()
        })
    }

    fn combine_configuration(
        &self,
        configuration_path: Option<Utf8PathBuf>,
        biome_configuration: Configuration,
        fs: &dyn FileSystem,
    ) -> Result<Configuration, WorkspaceError> {
        Ok(
            if let Some(mut fs_configuration) =
                self.load_editor_config(configuration_path, &biome_configuration, fs)?
            {
                // If both `biome.json` and `.editorconfig` exist, formatter settings from the biome.json take precedence.
                fs_configuration.merge_with(biome_configuration);
                fs_configuration
            } else {
                biome_configuration
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incompatible_arguments() {
        assert!(
            check_fix_incompatible_arguments(FixFileModeOptions {
                write: true,
                fix: true,
                unsafe_: false,
                suppress: false,
                suppression_reason: None
            })
            .is_err()
        );
    }

    #[test]
    fn safe_fixes() {
        for (write, suppress, suppression_reason, fix, unsafe_) in [
            (true, false, None, false, false), // --write
            (false, false, None, true, false), // --fix
        ] {
            assert_eq!(
                determine_fix_file_mode(FixFileModeOptions {
                    write,
                    suppress,
                    suppression_reason,
                    fix,
                    unsafe_
                },)
                .unwrap(),
                Some(FixFileMode::SafeFixes)
            );
        }
    }

    #[test]
    fn safe_and_unsafe_fixes() {
        for (write, fix, unsafe_, suppress, suppression_reason) in [
            (true, false, true, false, None), // --write --unsafe
            (false, true, true, false, None), // --fix --unsafe
        ] {
            assert_eq!(
                determine_fix_file_mode(FixFileModeOptions {
                    write,
                    suppress,
                    suppression_reason,
                    fix,
                    unsafe_
                },)
                .unwrap(),
                Some(FixFileMode::SafeAndUnsafeFixes)
            );
        }
    }

    #[test]
    fn no_fix() {
        let (write, suppress, suppression_reason, fix, unsafe_) =
            (false, false, None, false, false);
        assert_eq!(
            determine_fix_file_mode(FixFileModeOptions {
                write,
                suppress,
                suppression_reason,
                fix,
                unsafe_
            },)
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
