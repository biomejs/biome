use crate::changed::{get_changed_files, get_staged_files};
use crate::cli_options::{CliOptions, CliReporterKind, ColorsArg, cli_options};
use crate::logging::log_options;
use crate::logging::{LogOptions, LoggingKind};
use crate::{CliDiagnostic, LoggingLevel, VERSION};
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
use biome_diagnostics::{Diagnostic, PrintDiagnostic, Severity};
use biome_fs::FileSystem;
use biome_service::configuration::LoadedConfiguration;
use biome_service::documentation::Doc;
use biome_service::workspace::{FixFileMode, SearchLanguage};
use biome_service::{WatcherOptions, watcher_options};
use bpaf::Bpaf;
use std::ffi::OsString;

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
pub(crate) mod upgrade;
pub(crate) mod version;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version(VERSION))]
/// Biome's command-line interface for checking, formatting, and linting files, managing configuration, and interacting with the daemon.
pub enum BiomeCommand {
    /// Prints the Biome CLI version and information about the connected daemon server, if any, then exits.
    #[bpaf(command)]
    Version(#[bpaf(external(cli_options), hide_usage)] CliOptions),

    /// Upgrades Biome to the latest stable version.
    ///
    /// This command upgrades the current Biome binary based on how it was installed:
    ///
    /// - **Standalone**: Upgrades a standalone binary in place to the latest stable version.
    ///
    /// - **Homebrew**: If Biome was installed with Homebrew, this command runs `brew upgrade biome`.
    ///
    /// Set `BIOME_DISTRIBUTION` to `npm`, `homebrew`, or `standalone` to override automatic
    /// detection.
    ///
    /// This command cannot update Biome binaries installed through npm. Update
    /// `@biomejs/biome` with your package manager instead.
    #[bpaf(command)]
    Upgrade,

    #[bpaf(command)]
    /// Prints a troubleshooting report with CLI, platform, environment, configuration, workspace, and daemon server information.
    Rage(
        #[bpaf(external(cli_options), hide_usage)] CliOptions,
        #[bpaf(external(log_options), hide_usage)] LogOptions,
        /// Includes the latest daemon server log. If the CLI is not connected to a daemon server, this flag also discovers running Biome daemon server instances.
        #[bpaf(long("daemon-logs"), switch)]
        bool,
        /// Includes the formatter settings loaded from the Biome configuration.
        #[bpaf(long("formatter"), switch)]
        bool,
        /// Includes the linter settings and enabled lint rules loaded from the Biome configuration.
        #[bpaf(long("linter"), switch)]
        bool,
    ),
    /// Starts the Biome daemon server used for integration with third-party clients, such as editor extensions.
    #[bpaf(command)]
    Start {
        #[bpaf(external(log_options))]
        log_options: LogOptions,

        #[bpaf(external(watcher_options))]
        watcher_options: WatcherOptions,
    },

    /// Stops the Biome daemon server if it is running.
    #[bpaf(command)]
    Stop,

    /// Checks the specified files for formatting, linting, and assist actions.
    #[bpaf(command)]
    Check {
        /// Applies formatting changes, safe lint fixes, and safe assist actions.
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Allows `--write` or `--fix` to apply unsafe fixes and assist actions.
        #[bpaf(long("unsafe"), switch)]
        unsafe_: bool,

        /// Alias for `--write`.
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        /// Enables or disables formatting checks for this command.
        #[bpaf(
            long("formatter-enabled"),
            argument("true|false"),
            optional,
            hide_usage
        )]
        formatter_enabled: Option<FormatterEnabled>,
        /// Enables or disables linting checks for this command.
        #[bpaf(long("linter-enabled"), argument("true|false"), optional, hide_usage)]
        linter_enabled: Option<LinterEnabled>,

        /// Enables or disables assist actions for this command.
        #[bpaf(long("assist-enabled"), argument("true|false"), optional)]
        assist_enabled: Option<AssistEnabled>,

        /// Enforces assist actions and causes the command to fail if required actions are not applied. Defaults to `true`.
        #[bpaf(long("enforce-assist"), argument("true|false"), fallback(true))]
        enforce_assist: bool,

        /// Allows formatting files that contain syntax errors when set to `true`. Defaults to `false`.
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

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

        /// Reports how long each rule takes to run. It excludes the time spent preparing the analysis,
        /// such as building the semantic model.
        #[bpaf(long("profile-rules"), switch)]
        profile_rules: bool,

        /// Enables type inference profiling output.
        #[bpaf(long("profile-type-inference"), switch, hide, hide_usage)]
        profile_type_inference: bool,

        /// Reads code from standard input and writes the processed code to standard output.
        ///
        /// Biome uses `PATH` to select settings and determine the input type from its extension.
        /// Virtual paths don't need to exist or belong to the project. They bypass `files.includes`
        /// and VCS ignore checks.
        ///
        /// Example:
        ///
        /// ```shell
        /// echo 'let a;' | biome check --stdin-file-path=file.js --write
        /// ```
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        /// Checks only staged files. This option is intended for local use.
        #[bpaf(long("staged"), switch)]
        staged: bool,

        /// Checks only files with committed changes compared to the configured `vcs.defaultBranch`
        /// or the base reference specified by `--since`. Staged and unstaged changes are not included.
        /// This option is intended for CI.
        #[bpaf(long("changed"), switch)]
        changed: bool,

        /// Sets the base reference used by `--changed`, overriding `vcs.defaultBranch`. Requires `--changed`.
        #[bpaf(long("since"), argument("REF"))]
        since: Option<String>,

        /// Runs only the given lint rule, assist action, group of rules and actions, or domain. If a
        /// selected rule's severity is `off`, Biome sets it to `error` for a recommended rule or
        /// `warn` otherwise. The `plugin` group runs only analyzer plugins.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome check --only=correctness/noUnusedVariables --only=suspicious --only=test --only=plugin
        /// ```
        #[bpaf(long("only"), argument("GROUP|RULE|DOMAIN|ACTION|PLUGIN"))]
        only: Vec<AnalyzerSelector>,

        /// Skips the given lint rule, assist action, group of rules and actions, or domain. Matching
        /// lint rules are set to `off`. This option takes precedence over `--only`. The `plugin`
        /// group skips analyzer plugins.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome check --skip=correctness/noUnusedVariables --skip=suspicious --skip=project --skip=plugin
        /// ```
        #[bpaf(long("skip"), argument("GROUP|RULE|DOMAIN|ACTION|PLUGIN"))]
        skip: Vec<AnalyzerSelector>,

        /// After the initial run, watches the selected paths and reprocesses files modified afterward.
        #[bpaf(long("watch"), switch)]
        watch: bool,

        /// The optional `PATH` arguments accept one or more paths to files or directories. If omitted, Biome checks files in the current working directory.
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },
    /// Runs the linter on the specified files.
    #[bpaf(command)]
    Lint {
        /// Applies safe lint fixes.
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Allows `--write` or `--fix` to apply unsafe lint fixes.
        #[bpaf(long("unsafe"), switch)]
        unsafe_: bool,

        /// Alias for `--write`.
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        /// Writes comment suppressions for lint rule violations instead of applying rule fixes.
        #[bpaf(long("suppress"))]
        suppress: bool,

        /// Adds an explanation to suppressions created by `--suppress`. This flag requires `--suppress`.
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

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

        /// Runs only the given lint rule, rule group, or domain. If a rule's severity is `off`, Biome
        /// sets it to `error` for a recommended rule or `warn` otherwise. The `plugin` group runs
        /// only analyzer plugins.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome lint --only=correctness/noUnusedVariables --only=suspicious --only=test --only=plugin
        /// ```
        #[bpaf(long("only"), argument("GROUP|RULE|DOMAIN|PLUGIN"))]
        only: Vec<AnalyzerSelector>,

        /// Skips the given lint rule, rule group, or domain by setting the affected rules to `off`.
        /// This option takes precedence over `--only`. The `plugin` group skips analyzer plugins.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome lint --skip=correctness/noUnusedVariables --skip=suspicious --skip=project --skip=plugin
        /// ```
        #[bpaf(long("skip"), argument("GROUP|RULE|DOMAIN|PLUGIN"))]
        skip: Vec<AnalyzerSelector>,

        /// Reads code from standard input and writes the processed code to standard output.
        ///
        /// Biome uses `PATH` to select settings and determine the input type from its extension.
        /// Virtual paths don't need to exist or belong to the project. They bypass `files.includes`
        /// and VCS ignore checks.
        ///
        /// Example:
        ///
        /// ```shell
        /// echo 'let a;' | biome lint --stdin-file-path=file.js --write
        /// ```
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        /// Lints only staged files. This option is intended for local use.
        #[bpaf(long("staged"), switch)]
        staged: bool,

        /// Lints only files with committed changes compared to the configured `vcs.defaultBranch`
        /// or the base reference specified by `--since`. Staged and unstaged changes are not included.
        /// This option is intended for CI.
        #[bpaf(long("changed"), switch)]
        changed: bool,

        /// Sets the base reference used by `--changed`, overriding `vcs.defaultBranch`. Requires `--changed`.
        #[bpaf(long("since"), argument("REF"))]
        since: Option<String>,
        /// Reports how long each rule takes to run. It excludes the time spent preparing the analysis,
        /// such as building the semantic model.
        #[bpaf(long("profile-rules"), switch)]
        profile_rules: bool,

        /// Capture type-inference request and query timings.
        #[bpaf(long("profile-type-inference"), switch, hide, hide_usage)]
        profile_type_inference: bool,

        /// After the initial run, watches the selected paths and reprocesses files modified afterward.
        #[bpaf(long("watch"), switch)]
        watch: bool,

        /// The optional `PATH` arguments accept one or more paths to files or directories. If omitted, Biome lints files in the current working directory.
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },
    /// Formats the specified files.
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

        #[bpaf(external(css_parser_configuration), optional, hide_usage)]
        css_parser: Option<CssParserConfiguration>,

        #[bpaf(external(graphql_formatter_configuration), optional, hide_usage)]
        graphql_formatter: Option<GraphqlFormatterConfiguration>,

        #[bpaf(external(css_formatter_configuration), optional, hide_usage)]
        css_formatter: Option<CssFormatterConfiguration>,

        #[bpaf(external(html_formatter_configuration), optional, hide_usage)]
        html_formatter: Option<HtmlFormatterConfiguration>,

        #[bpaf(external(vcs_configuration), optional, hide_usage)]
        vcs_configuration: Option<VcsConfiguration>,

        #[bpaf(external(files_configuration), optional, hide_usage)]
        files_configuration: Option<FilesConfiguration>,
        /// Reads code from standard input and writes the processed code to standard output.
        ///
        /// Biome uses `PATH` to select settings and determine the input type from its extension.
        /// Virtual paths don't need to exist or belong to the project. They bypass `files.includes`
        /// and VCS ignore checks.
        ///
        /// Example:
        ///
        /// ```shell
        /// echo 'let a;' | biome format --stdin-file-path=file.js
        /// ```
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

        /// Applies formatting changes.
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Alias for `--write`.
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        /// Formats only staged files. This option is intended for local use.
        #[bpaf(long("staged"), switch)]
        staged: bool,

        /// Formats only files with committed changes compared to the configured `vcs.defaultBranch`
        /// or the base reference specified by `--since`. Staged and unstaged changes are not included.
        /// This option is intended for CI.
        #[bpaf(long("changed"), switch)]
        changed: bool,

        /// Sets the base reference used by `--changed`, overriding `vcs.defaultBranch`. Requires `--changed`.
        #[bpaf(long("since"), argument("REF"))]
        since: Option<String>,

        /// After the initial run, watches the selected paths and reprocesses files modified afterward.
        #[bpaf(long("watch"), switch)]
        watch: bool,

        /// The optional `PATH` arguments accept one or more paths to files or directories. If omitted, Biome formats files in the current working directory.
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },
    /// Runs formatting checks, linting checks, and assist actions in CI without modifying files.
    #[bpaf(command)]
    Ci {
        /// Enables or disables formatting checks for this command.
        #[bpaf(long("formatter-enabled"), argument("true|false"), optional)]
        formatter_enabled: Option<FormatterEnabled>,
        /// Enables or disables linting checks for this command.
        #[bpaf(long("linter-enabled"), argument("true|false"), optional)]
        linter_enabled: Option<LinterEnabled>,

        /// Enables or disables assist actions for this command.
        #[bpaf(long("assist-enabled"), argument("true|false"), optional)]
        assist_enabled: Option<AssistEnabled>,

        /// Allows formatting files that contain syntax errors when set to `true`. Defaults to `false`.
        #[bpaf(long("format-with-errors"), argument("true|false"))]
        format_with_errors: Option<FormatWithErrorsEnabled>,

        #[bpaf(external(json_parser_configuration), optional, hide_usage)]
        json_parser: Option<JsonParserConfiguration>,

        #[bpaf(external(css_parser_configuration), optional, hide_usage, hide)]
        css_parser: Option<CssParserConfiguration>,

        /// Enforces assist actions and causes the command to fail if required actions are not applied. Defaults to `true`.
        #[bpaf(long("enforce-assist"), argument("true|false"), fallback(true))]
        enforce_assist: bool,

        #[bpaf(external(configuration), hide_usage, optional)]
        configuration: Option<Configuration>,
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

        /// Checks only files with committed changes compared to the configured `vcs.defaultBranch`
        /// or the base reference specified by `--since`. Staged and unstaged changes are not included.
        /// This option is intended for CI.
        #[bpaf(long("changed"), switch)]
        changed: bool,

        /// Sets the base reference used by `--changed`, overriding `vcs.defaultBranch`. Requires `--changed`.
        #[bpaf(long("since"), argument("REF"))]
        since: Option<String>,

        /// Sets the number of threads to use. This is useful in environments with limited resources, such as CI.
        #[bpaf(
            long("threads"),
            argument("NUMBER"),
            env("BIOME_THREADS"),
            optional,
            hide_usage
        )]
        threads: Option<usize>,

        /// Runs only the given lint rule, assist action, group of rules and actions, or domain. If a
        /// selected rule's severity is `off`, Biome sets it to `error` for a recommended rule or
        /// `warn` otherwise. The `plugin` group runs only analyzer plugins.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome ci --only=correctness/noUnusedVariables --only=suspicious --only=test --only=plugin
        /// ```
        #[bpaf(long("only"), argument("GROUP|RULE|DOMAIN|ACTION|PLUGIN"))]
        only: Vec<AnalyzerSelector>,

        /// Skips the given lint rule, assist action, group of rules and actions, or domain. Matching
        /// lint rules are set to `off`. This option takes precedence over `--only`. The `plugin`
        /// group skips analyzer plugins.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome ci --skip=correctness/noUnusedVariables --skip=suspicious --skip=project --skip=plugin
        /// ```
        #[bpaf(long("skip"), argument("GROUP|RULE|DOMAIN|ACTION|PLUGIN"))]
        skip: Vec<AnalyzerSelector>,

        /// The optional `PATH` arguments accept one or more paths to files or directories. If omitted, Biome checks files in the current working directory.
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },

    /// Creates a default Biome configuration file in the current working directory.
    #[bpaf(command)]
    Init(
        /// Creates `biome.jsonc` instead of `biome.json`.
        #[bpaf(long("jsonc"), switch)]
        bool,
    ),
    /// Ensures that the Biome daemon server is running, then forwards Language Server Protocol messages between the daemon server and standard input and output.
    #[bpaf(command("lsp-proxy"))]
    LspProxy {
        /// Provides the argument expected by `vscode-languageclient`.
        #[bpaf(long("stdio"), hide, hide_usage, switch)]
        stdio: bool,

        #[bpaf(external(log_options))]
        log_options: LogOptions,

        #[bpaf(external(watcher_options))]
        watcher_options: WatcherOptions,
    },
    /// Previews configuration updates required by breaking changes. The `prettier` and `eslint` subcommands instead import settings from those tools.
    ///
    /// By default, this command displays the proposed changes without modifying files. Use `--write` to apply them.
    #[bpaf(command)]
    Migrate {
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

        /// Applies the proposed migration or imported settings.
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Alias for `--write`.
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        #[bpaf(external(migrate_sub_command), optional)]
        sub_command: Option<MigrateSubCommand>,
    },

    /// EXPERIMENTAL: Finds code that matches a GritQL pattern, optionally limited to specific files or directories.
    ///
    /// Put the query inside single quotes when the shell supports them. This prevents the shell
    /// from interpreting GritQL's backticks as commands.
    ///
    /// ### Example
    ///
    /// ```shell
    /// biome search '`console.log($message)`' ./src
    /// ```
    #[bpaf(command)]
    Search {
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

        #[bpaf(external(files_configuration), optional, hide_usage)]
        files_configuration: Option<FilesConfiguration>,

        #[bpaf(external(vcs_configuration), optional, hide_usage)]
        vcs_configuration: Option<VcsConfiguration>,

        /// Retained for CLI compatibility; standard-input search is not supported.
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide)]
        stdin_file_path: Option<String>,

        /// Selects the language grammar used for the pattern and searched code: CSS, JavaScript, or JSON.
        ///
        /// GritQL patterns are specific to their target grammar, so a search cannot target multiple
        /// languages at once.
        ///
        /// Defaults to `javascript`.
        #[bpaf(long("language"), short('l'), argument("css|javascript|json"))]
        language: Option<SearchLanguage>,

        /// The GritQL query to find. Rewrite queries aren't supported because `biome search` is read-only.
        #[bpaf(positional("PATTERN"))]
        pattern: String,

        /// One or more files or directories to search. Defaults to the current working directory.
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },

    /// Prints documentation for a lint rule or the path to the daemon server log directory.
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
        /// `NAME` is a lint rule name or `daemon-logs`.
        #[bpaf(positional("NAME"))]
        doc: Doc,
    },

    #[bpaf(command)]
    /// Removes the Biome daemon server log files.
    Clean,

    #[bpaf(command("__run_server"), hide)]
    RunServer {
        #[bpaf(external(log_options))]
        log_options: LogOptions,

        #[bpaf(long("stop-on-disconnect"), hide_usage)]
        stop_on_disconnect: bool,

        #[bpaf(external(watcher_options))]
        watcher_options: WatcherOptions,
    },
    #[bpaf(command("__print_socket"), hide)]
    PrintSocket,

    #[bpaf(command("__where_am_i"), hide)]
    WhereAmI,
}

#[derive(Debug, Bpaf, Clone)]
pub enum MigrateSubCommand {
    /// Imports Prettier configuration and ignore settings into the Biome configuration.
    #[bpaf(command)]
    Prettier,
    /// Imports an ESLint configuration and ignore settings from the current working directory into the Biome configuration.
    #[bpaf(command)]
    Eslint {
        /// Includes Biome rules inspired by ESLint rules in the migration.
        #[bpaf(long("include-inspired"))]
        include_inspired: bool,
        /// Includes nursery rules in the migration.
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
            | Self::Upgrade
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

    const fn log_options(&self) -> Option<&LogOptions> {
        match self {
            Self::Check { log_options, .. }
            | Self::Lint { log_options, .. }
            | Self::Ci { log_options, .. }
            | Self::Format { log_options, .. }
            | Self::Migrate { log_options, .. }
            | Self::Rage(_, log_options, ..)
            | Self::Search { log_options, .. } => Some(log_options),
            Self::Version(_)
            | Self::Upgrade
            | Self::LspProxy { .. }
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

    pub fn get_color(&self) -> Option<&ColorsArg> {
        match self.cli_options() {
            Some(cli_options) => {
                // To properly display GitHub annotations we need to disable colors
                if cli_options
                    .cli_reporter
                    .iter()
                    .any(|r| r.kind == CliReporterKind::GitHub)
                {
                    return Some(&ColorsArg::Off);
                }
                // In CI, we force colors for a better UX unless users explicitly set `--colors`.
                // In GitHub Actions, we disable colors so the auto-enabled GitHub reporter
                // output is not corrupted by ANSI escape codes.
                if matches!(self, Self::Ci { .. }) && cli_options.colors.is_none() {
                    if is_github_actions() {
                        return Some(&ColorsArg::Off);
                    }
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
        if matches!(
            self,
            Self::Check {
                profile_type_inference: true,
                ..
            } | Self::Lint {
                profile_type_inference: true,
                ..
            }
        ) {
            return false;
        }
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
        self.log_options()
            .map_or(LoggingLevel::default(), |log_options| log_options.log_level)
    }

    pub fn log_kind(&self) -> LoggingKind {
        self.log_options()
            .map_or(LoggingKind::default(), |log_options| log_options.log_kind)
    }
}

/// Returns `true` when running in GitHub Actions in release builds.
///
/// In debug builds, this always returns `false` to avoid false positives in
/// tests that run `biome ci` under CI (CI-ception).
fn is_github_actions() -> bool {
    if cfg!(debug_assertions) {
        return false;
    }
    // Ref: https://docs.github.com/actions/learn-github-actions/variables#default-environment-variables
    std::env::var("GITHUB_ACTIONS")
        .ok()
        .is_some_and(|v| v == "true")
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
    fn type_inference_profile_forces_in_process_validation() {
        use bpaf::Args;

        for command_name in ["lint", "check"] {
            let command = biome_command()
                .run_inner(Args::from(
                    [command_name, "--profile-type-inference", "--use-server"].as_slice(),
                ))
                .expect("profile options must parse");
            assert!(!command.should_use_server());
            assert!(
                command
                    .cli_options()
                    .is_some_and(|options| options.use_server)
            );
        }
    }

    #[test]
    fn check_options() {
        biome_command().check_invariants(false);
    }
}
