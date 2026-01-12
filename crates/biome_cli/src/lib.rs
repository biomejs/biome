#![doc = include_str!("../README.md")]
//!
//! # Module
//!
//! This is where the main CLI session starts. The module is responsible
//! to parse commands and arguments, redirect the execution of the commands and
//! execute the traversal of directory and files, based on the command that were passed.

#![deny(clippy::use_self)]

use biome_console::{ColorMode, Console};
use biome_service::{App, Workspace, WorkspaceRef};
use commands::search::SearchCommandPayload;
use std::env;

mod changed;
mod cli_options;
mod commands;
mod diagnostics;
mod execute;
mod logging;
mod panic;
pub(crate) mod reporter;
pub(crate) mod runner;
mod service;

use crate::cli_options::ColorsArg;
use crate::commands::check::CheckCommandPayload;
use crate::commands::ci::CiCommandPayload;
use crate::commands::format::FormatCommandPayload;
use crate::commands::lint::LintCommandPayload;
use crate::commands::migrate::MigrateCommandPayload;
pub use crate::commands::{BiomeCommand, biome_command};
pub use crate::logging::{LoggingLevel, setup_cli_subscriber};
use crate::runner::impls::commands::custom_execution::CustomExecutionCmdImpl;
use crate::runner::impls::commands::traversal::TraversalCommandImpl;
use crate::runner::run::run_command;
pub use diagnostics::CliDiagnostic;
pub use panic::setup_panic_handler;
pub use reporter::{DiagnosticsPayload, TraversalSummary};
pub use service::{SocketTransport, open_transport};

pub(crate) const VERSION: &str = match option_env!("BIOME_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

/// JSON file that is temporarily to handle internal files via [Workspace].
/// When using this file, make sure to close it via [Workspace::close_file].
pub const TEMPORARY_INTERNAL_REPORTER_FILE: &str = "__BIOME_INTERNAL_FILE__.json";

/// Global context for an execution of the CLI
pub struct CliSession<'app> {
    /// Instance of [App] used by this run of the CLI
    pub app: App<'app>,
}

impl<'app> CliSession<'app> {
    pub fn new(
        workspace: &'app dyn Workspace,
        console: &'app mut dyn Console,
    ) -> Result<Self, CliDiagnostic> {
        Ok(Self {
            app: App::new(console, WorkspaceRef::Borrowed(workspace)),
        })
    }

    /// Main function to run Biome CLI
    pub fn run(self, command: BiomeCommand) -> Result<(), CliDiagnostic> {
        match command {
            BiomeCommand::Version(_) => commands::version::full_version(self),
            BiomeCommand::Rage(_, _, daemon_logs, formatter, linter) => {
                commands::rage::rage(self, daemon_logs, formatter, linter)
            }
            BiomeCommand::Clean => commands::clean::clean(self),
            BiomeCommand::Start {
                log_options,
                watcher_options,
            } => commands::daemon::start(self, watcher_options, log_options),
            BiomeCommand::Stop => commands::daemon::stop(self),
            BiomeCommand::Check {
                write,
                fix,
                unsafe_,
                cli_options,
                configuration,
                paths,
                stdin_file_path,
                linter_enabled,
                formatter_enabled,
                assist_enabled,
                enforce_assist,
                staged,
                changed,
                since,
                format_with_errors,
                json_parser,
                css_parser,
                log_options,
                only,
                skip,
            } => run_command(
                self,
                &log_options,
                &cli_options,
                TraversalCommandImpl(CheckCommandPayload {
                    write,
                    fix,
                    unsafe_,
                    configuration,
                    paths,
                    stdin_file_path,
                    linter_enabled,
                    formatter_enabled,
                    assist_enabled,
                    enforce_assist,
                    staged,
                    changed,
                    since,
                    format_with_errors,
                    json_parser,
                    css_parser,
                    only,
                    skip,
                }),
            ),
            BiomeCommand::Lint {
                write,
                suppress,
                suppression_reason,
                fix,
                unsafe_,
                cli_options,
                linter_configuration,
                paths,
                only,
                skip,
                stdin_file_path,
                vcs_configuration,
                files_configuration,
                staged,
                changed,
                since,
                css_linter,
                javascript_linter,
                json_linter,
                graphql_linter,
                css_parser,
                json_parser,
                log_options,
            } => run_command(
                self,
                &log_options,
                &cli_options,
                TraversalCommandImpl(LintCommandPayload {
                    write,
                    suppress,
                    suppression_reason,
                    fix,
                    unsafe_,
                    linter_configuration,
                    paths,
                    only,
                    skip,
                    stdin_file_path,
                    vcs_configuration,
                    files_configuration,
                    staged,
                    changed,
                    since,
                    css_linter,
                    javascript_linter,
                    json_linter,
                    graphql_linter,
                    css_parser,
                    json_parser,
                }),
            ),
            BiomeCommand::Ci {
                linter_enabled,
                formatter_enabled,
                assist_enabled,
                enforce_assist,
                configuration,
                paths,
                cli_options,
                changed,
                since,
                format_with_errors,
                css_parser,
                json_parser,
                log_options,
                only,
                skip,
                ..
            } => run_command(
                self,
                &log_options,
                &cli_options,
                TraversalCommandImpl(CiCommandPayload {
                    linter_enabled,
                    formatter_enabled,
                    assist_enabled,
                    enforce_assist,
                    configuration,
                    paths,
                    changed,
                    since,
                    format_with_errors,
                    css_parser,
                    json_parser,
                    only,
                    skip,
                }),
            ),
            BiomeCommand::Format {
                javascript_formatter,
                formatter_configuration,
                stdin_file_path,
                write,
                fix,
                cli_options,
                paths,
                vcs_configuration,
                files_configuration,
                json_formatter,
                css_formatter,
                graphql_formatter,
                html_formatter,
                staged,
                changed,
                since,
                css_parser,
                json_parser,
                log_options,
            } => run_command(
                self,
                &log_options,
                &cli_options,
                TraversalCommandImpl(FormatCommandPayload {
                    javascript_formatter,
                    formatter_configuration,
                    stdin_file_path,
                    write,
                    fix,
                    paths,
                    vcs_configuration,
                    files_configuration,
                    json_formatter,
                    css_formatter,
                    graphql_formatter,
                    html_formatter,
                    staged,
                    changed,
                    since,
                    css_parser,
                    json_parser,
                }),
            ),
            BiomeCommand::Explain { doc } => commands::explain::explain(self, doc),
            BiomeCommand::Init(emit_jsonc) => commands::init::init(self, emit_jsonc),
            BiomeCommand::LspProxy {
                watcher_options,
                stdio: _,
                log_options,
            } => commands::daemon::lsp_proxy(watcher_options, log_options),
            BiomeCommand::Migrate {
                cli_options,
                log_options,
                write,
                fix,
                sub_command,
            } => run_command(
                self,
                &log_options,
                &cli_options,
                CustomExecutionCmdImpl(MigrateCommandPayload {
                    write,
                    fix,
                    sub_command,
                    configuration_directory_path: None,
                    configuration_file_path: None,
                }),
            ),
            BiomeCommand::Search {
                cli_options,
                log_options,
                files_configuration,
                paths,
                pattern,
                language,
                stdin_file_path,
                vcs_configuration,
            } => run_command(
                self,
                &log_options,
                &cli_options,
                TraversalCommandImpl(SearchCommandPayload {
                    files_configuration,
                    paths,
                    pattern,
                    language,
                    stdin_file_path,
                    vcs_configuration,
                }),
            ),
            BiomeCommand::RunServer {
                stop_on_disconnect,
                watcher_options,
                log_options,
            } => commands::daemon::run_server(stop_on_disconnect, watcher_options, log_options),
            BiomeCommand::PrintSocket => commands::daemon::print_socket(),
            BiomeCommand::WhereAmI => {
                if let Ok(path) = env::current_exe() {
                    print!("{}", path.display());
                }
                Ok(())
            }
        }
    }
}

pub fn to_color_mode(color: Option<&ColorsArg>) -> ColorMode {
    match color {
        Some(ColorsArg::Off) => ColorMode::Disabled,
        Some(ColorsArg::Force) => ColorMode::Enabled,
        None => ColorMode::Auto,
    }
}
