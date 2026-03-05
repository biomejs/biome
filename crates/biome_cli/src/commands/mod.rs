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
use biome_grit_patterns::GritTargetLanguage;
use biome_service::configuration::LoadedConfiguration;
use biome_service::documentation::Doc;
use biome_service::workspace::FixFileMode;
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
        #[bpaf(external(log_options), hide_usage)] LogOptions,
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
        #[bpaf(external(log_options))]
        log_options: LogOptions,

        #[bpaf(external(watcher_options))]
        watcher_options: WatcherOptions,
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

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

        /// Enable rule profiling output.
        /// Captures timing only for rule execution, not preprocessing such as querying or building the semantic model.
        #[bpaf(long("profile-rules"), switch)]
        profile_rules: bool,

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
        /// The provided path may also affect whether the input is treated as
        /// ignored. If the path doesn't exist on disk (virtual path), Biome
        /// won't require it to be part of the project file set, and ignore
        /// checks (`files.includes` and VCS ignore rules) are skipped.
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

        /// Run only the given lint rule, assist action, group of rules and actions, or domain.
        /// If the severity level of a rule is `off`,
        /// then the severity level of the rule is set to `error` if it is a recommended rule or `warn` otherwise.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome check --only=correctness/noUnusedVariables --only=suspicious --only=test
        /// ```
        #[bpaf(long("only"), argument("GROUP|RULE|DOMAIN|ACTION"))]
        only: Vec<AnalyzerSelector>,

        /// Skip the given lint rule, assist action, group of rules and actions, or domain by setting the severity level of the rules to `off`.
        /// This option takes precedence over `--only`.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome check --skip=correctness/noUnusedVariables --skip=suspicious --skip=project
        /// ```
        #[bpaf(long("skip"), argument("GROUP|RULE|DOMAIN|ACTION"))]
        skip: Vec<AnalyzerSelector>,

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

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

        /// Run only the given lint rule, assist action, group of rules and actions, or domain.
        /// If the severity level of a rule is `off`,
        /// then the severity level of the rule is set to `error` if it is a recommended rule or `warn` otherwise.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome lint --only=correctness/noUnusedVariables --only=suspicious --only=test
        /// ```
        #[bpaf(long("only"), argument("GROUP|RULE|DOMAIN|ACTION"))]
        only: Vec<AnalyzerSelector>,

        /// Skip the given lint rule, assist action, group of rules and actions, or domain by setting the severity level of the rules to `off`.
        /// This option takes precedence over `--only`.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome lint --skip=correctness/noUnusedVariables --skip=suspicious --skip=project
        /// ```
        #[bpaf(long("skip"), argument("GROUP|RULE|DOMAIN|ACTION"))]
        skip: Vec<AnalyzerSelector>,

        /// Use this option when you want to format code piped from `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the extension of the file. Based on the extension, Biome knows how to lint the code.
        ///
        /// The provided path may also affect whether the input is treated as
        /// ignored. If the path doesn't exist on disk (virtual path), Biome
        /// won't require it to be part of the project file set, and ignore
        /// checks (`files.includes` and VCS ignore rules) are skipped.
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
        /// Enable rule profiling output.
        /// Captures timing only for rule execution, not preprocessing such as querying or building the semantic model.
        #[bpaf(long("profile-rules"), switch)]
        profile_rules: bool,

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
        /// Use this option when you want to format code piped from `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the extension of the file. Based on the extension, Biome knows how to format the code.
        ///
        /// The provided path may also affect whether the input is treated as
        /// ignored. If the path doesn't exist on disk (virtual path), Biome
        /// won't require it to be part of the project file set, and ignore
        /// checks (`files.includes` and VCS ignore rules) are skipped.
        ///
        /// Example:
        /// ```shell
        /// echo 'let a;' | biome format --stdin-file-path=file.js --write
        /// ```
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

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

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

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

        /// Run only the given lint rule, assist action, group of rules and actions, or domain.
        /// If the severity level of a rule is `off`,
        /// then the severity level of the rule is set to `error` if it is a recommended rule or `warn` otherwise.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome ci --only=correctness/noUnusedVariables --only=suspicious --only=test
        /// ```
        #[bpaf(long("only"), argument("GROUP|RULE|DOMAIN|ACTION"))]
        only: Vec<AnalyzerSelector>,

        /// Skip the given lint rule, assist action, group of rules and actions, or domain by setting the severity level of the rules to `off`.
        /// This option takes precedence over `--only`.
        ///
        /// Example:
        ///
        /// ```shell
        /// biome ci --skip=correctness/noUnusedVariables --skip=suspicious --skip=project
        /// ```
        #[bpaf(long("skip"), argument("GROUP|RULE|DOMAIN|ACTION"))]
        skip: Vec<AnalyzerSelector>,

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
        /// Bogus argument to make the command work with vscode-languageclient
        #[bpaf(long("stdio"), hide, hide_usage, switch)]
        stdio: bool,

        #[bpaf(external(log_options))]
        log_options: LogOptions,

        #[bpaf(external(watcher_options))]
        watcher_options: WatcherOptions,
    },
    /// Updates the configuration when there are breaking changes.
    #[bpaf(command)]
    Migrate {
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

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

        #[bpaf(external, hide_usage)]
        log_options: LogOptions,

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
    fn check_options() {
        biome_command().check_invariants(false);
    }
}
