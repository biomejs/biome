#![doc = include_str!("../README.md")]
//!
//! # Module
//!
//! This is where the main CLI session starts. The module is responsible
//! to parse commands and arguments, redirect the execution of the commands and
//! execute the traversal of directory and files, based on the command that were passed.

use biome_console::{ColorMode, Console};
use biome_fs::OsFileSystem;
use biome_service::{App, DynRef, Workspace, WorkspaceRef};
use commands::search::SearchCommandPayload;
use std::env;

mod changed;
mod cli_options;
mod commands;
mod diagnostics;
mod execute;
mod logging;
mod metrics;
mod panic;
mod reporter;
mod service;

use crate::cli_options::ColorsArg;
use crate::commands::check::CheckCommandPayload;
use crate::commands::ci::CiCommandPayload;
use crate::commands::format::FormatCommandPayload;
use crate::commands::lint::LintCommandPayload;
pub use crate::commands::{biome_command, BiomeCommand};
pub use crate::logging::{setup_cli_subscriber, LoggingLevel};
pub use diagnostics::CliDiagnostic;
pub use execute::{execute_mode, Execution, TraversalMode, VcsTargeted};
pub use panic::setup_panic_handler;
pub use reporter::{DiagnosticsPayload, Reporter, ReporterVisitor, TraversalSummary};
pub use service::{open_transport, SocketTransport};

#[cfg(debug_assertions)]
pub use crate::commands::daemon::biome_log_dir;

pub(crate) const VERSION: &str = match option_env!("BIOME_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

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
            app: App::new(
                DynRef::Owned(Box::<OsFileSystem>::default()),
                console,
                WorkspaceRef::Borrowed(workspace),
            ),
        })
    }

    /// Main function to run Biome CLI
    pub fn run(self, command: BiomeCommand) -> Result<(), CliDiagnostic> {
        let has_metrics = command.has_metrics();
        if has_metrics {
            crate::metrics::init_metrics();
        }

        let result = match command {
            BiomeCommand::Version(_) => commands::version::full_version(self),
            BiomeCommand::Rage(_, daemon_logs, formatter, linter) => {
                commands::rage::rage(self, daemon_logs, formatter, linter)
            }
            BiomeCommand::Clean => commands::clean::clean(self),
            BiomeCommand::Start(config_path) => commands::daemon::start(self, config_path),
            BiomeCommand::Stop => commands::daemon::stop(self),
            BiomeCommand::Check {
                apply,
                apply_unsafe,
                write,
                fix,
                unsafe_,
                cli_options,
                configuration,
                paths,
                stdin_file_path,
                linter_enabled,
                organize_imports_enabled,
                formatter_enabled,
                assists_enabled,
                staged,
                changed,
                since,
            } => commands::check::check(
                self,
                CheckCommandPayload {
                    apply_unsafe,
                    apply,
                    write,
                    fix,
                    unsafe_,
                    cli_options,
                    configuration,
                    paths,
                    stdin_file_path,
                    linter_enabled,
                    organize_imports_enabled,
                    formatter_enabled,
                    assists_enabled,
                    staged,
                    changed,
                    since,
                },
            ),
            BiomeCommand::Lint {
                apply,
                apply_unsafe,
                write,
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
            } => commands::lint::lint(
                self,
                LintCommandPayload {
                    apply_unsafe,
                    apply,
                    write,
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
                },
            ),
            BiomeCommand::Ci {
                linter_enabled,
                formatter_enabled,
                organize_imports_enabled,
                assists_enabled,
                configuration,
                paths,
                cli_options,
                changed,
                since,
            } => commands::ci::ci(
                self,
                CiCommandPayload {
                    linter_enabled,
                    formatter_enabled,
                    organize_imports_enabled,
                    assists_enabled,
                    configuration,
                    paths,
                    cli_options,
                    changed,
                    since,
                },
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
                staged,
                changed,
                since,
            } => commands::format::format(
                self,
                FormatCommandPayload {
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
                    staged,
                    changed,
                    since,
                },
            ),
            BiomeCommand::Explain { doc } => commands::explain::explain(self, doc),
            BiomeCommand::Init(emit_jsonc) => commands::init::init(self, emit_jsonc),
            BiomeCommand::LspProxy(config_path, _) => commands::daemon::lsp_proxy(config_path),
            BiomeCommand::Migrate {
                cli_options,
                write,
                fix,
                sub_command,
            } => commands::migrate::migrate(self, cli_options, write, fix, sub_command),
            BiomeCommand::Search {
                cli_options,
                files_configuration,
                paths,
                pattern,
                stdin_file_path,
                vcs_configuration,
            } => commands::search::search(
                self,
                SearchCommandPayload {
                    cli_options,
                    files_configuration,
                    paths,
                    pattern,
                    stdin_file_path,
                    vcs_configuration,
                },
            ),
            BiomeCommand::RunServer {
                stop_on_disconnect,
                config_path,
            } => commands::daemon::run_server(stop_on_disconnect, config_path),
            BiomeCommand::PrintSocket => commands::daemon::print_socket(),
        };

        if has_metrics {
            metrics::print_metrics();
        }

        result
    }
}

pub fn to_color_mode(color: Option<&ColorsArg>) -> ColorMode {
    match color {
        Some(ColorsArg::Off) => ColorMode::Disabled,
        Some(ColorsArg::Force) => ColorMode::Enabled,
        None => ColorMode::Auto,
    }
}
