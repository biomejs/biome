//! # Command Runner Architecture
//!
//! This module provides a trait-based infrastructure for implementing CLI commands
//! that operate on files in a Biome project. It's designed to be flexible enough to
//! support both standard file-traversing commands (like `format`, `lint`, `check`)
//! and special-case commands that don't traverse files (like `migrate`).
//!
//! ## Architecture Overview
//!
//! ```text
//! CommandRunner::run()
//!   │
//!   ├─→ configure_workspace() ────→ ConfiguredWorkspace
//!   │                                  ├─ project_key
//!   │                                  ├─ configuration_files
//!   │                                  ├─ execution context
//!   │                                  └─ paths to process
//!   │
//!   ├─→ [if requires_crawling() = false]
//!   │    └─→ execute_without_crawling() ──→ Custom logic (e.g., migrate)
//!   │
//!   └─→ [if requires_crawling() = true]
//!        │
//!        ├─→ Crawler::crawl()
//!        │    │
//!        │    ├─→ spawns Collector thread ──→ Collects diagnostics/messages
//!        │    │
//!        │    └─→ traverses files via Handler
//!        │         │
//!        │         ├─→ Handler::can_handle() ──→ Filter files
//!        │         │
//!        │         └─→ Handler::handle_path()
//!        │              │
//!        │              └─→ ProcessFile::process_file()
//!        │                   └─→ Calls workspace APIs
//!        │                   └─→ Returns FileStatus
//!        │
//!        └─→ Finalizer::finalize() ──→ Print results, exit codes
//! ```
//!
//! ## Core Traits
//!
//! ### [`CommandRunner`]
//! The main trait that orchestrates command execution. Commands typically don't implement
//! this directly, but instead use one of the helper traits below that provide automatic
//! `CommandRunner` implementations.
//!
//! ### [`impls::commands::traversal::TraversalCommand`]
//! High-level trait for commands that traverse files. Commands implement this trait
//! and wrap their implementation in [`impls::commands::traversal::TraversalCommandImpl`]
//! to automatically get a `CommandRunner` implementation with sensible defaults.
//! Used for commands like `format`, `lint`, and `check`.
//!
//! ### [`impls::commands::custom_execution::CustomExecutionCmd`]
//! High-level trait for commands that don't traverse files but need custom execution.
//! Commands implement this trait and wrap their implementation in
//! [`impls::commands::custom_execution::CustomExecutionCmdImpl`] to automatically get a
//! `CommandRunner` implementation that bypasses file crawling. Used for commands like `migrate`.
//!
//! ### [`Crawler`]
//! Orchestrates the file traversal process. Spawns a collector thread, walks the
//! file system, and produces structured output.
//!
//! ### [`Handler`]
//! Defines per-file processing logic. Filters which files to process and dispatches
//! to the appropriate file processor. Provides sensible defaults for both filtering
//! and processing.
//!
//! ### [`ProcessFile`]
//! Defines the actual file processing logic. This is where command-specific workspace
//! API calls happen (formatting, linting, searching, etc.).
//!
//! ### [`Collector`]
//! Runs in a separate thread to collect diagnostics and messages during traversal.
//! Filters and prints diagnostics based on severity and verbosity.
//!
//! ### [`Finalizer`]
//! Prints final results and determines exit code. Can format output in different
//! styles (terminal, JSON, GitHub Actions, etc.).
//!
//! ### [`Execution`]
//! Defines what features a command needs from the workspace and checks if files
//! support those features.
//!
//! ## Design Principles
//!
//! - **Separation of Concerns**: Each trait handles a distinct aspect of command execution
//! - **Composability**: Share implementations across commands where it makes sense
//! - **Type Safety**: Associated types ensure components fit together correctly
//! - **Sensible Defaults**: Most traits provide default implementations that work for common cases
//! - **Flexibility**: Override only what you need for command-specific behavior
//! - **Wrapper Pattern**: High-level traits use wrapper types to provide automatic implementations
//!
//! ## Implementing Commands
//!
//! ### For file-traversing commands (format, lint, check):
//! 1. Implement [`impls::commands::traversal::TraversalCommand`] for your command type
//! 2. Wrap it in [`impls::commands::traversal::TraversalCommandImpl`]
//! 3. Call `run()` on the wrapper to execute the command
//!
//! ### For custom execution commands (migrate):
//! 1. Implement [`impls::commands::custom_execution::CustomExecutionCmd`] for your command type
//! 2. Wrap it in [`impls::commands::custom_execution::CustomExecutionCmdImpl`]
//! 3. Call `run()` on the wrapper to execute the command
//!
//! ## Module Organization
//!
//! - [`collector`]: Diagnostic collection during traversal
//! - [`crawler`]: File system traversal orchestration
//! - [`execution`]: Feature requirements and capability checks
//! - [`finalizer`]: Results presentation and reporting
//! - [`handler`]: Per-file filtering and processing dispatch
//! - [`process_file`]: File processing trait and status types
//! - [`scan_kind`]: Utilities for determining scan strategy
//! - [`impls`]: Concrete implementations and helper traits
//!   - [`impls::commands`]: High-level command traits with automatic `CommandRunner` implementations
//!   - [`impls::collectors`]: Collector implementations
//!   - [`impls::crawlers`]: Crawler implementations
//!   - [`impls::finalizers`]: Finalizer implementations
//!   - [`impls::handlers`]: Handler implementations
//!   - [`impls::process_file`]: ProcessFile implementations

pub(crate) mod collector;
pub(crate) mod crawler;
pub(crate) mod diagnostics;
pub(crate) mod execution;
pub(crate) mod finalizer;
pub(crate) mod handler;
pub(crate) mod impls;
pub(crate) mod process_file;
pub(crate) mod run;
pub(crate) mod scan_kind;

use crate::cli_options::CliOptions;
use crate::commands::{
    print_diagnostics_from_workspace_result, validate_configuration_diagnostics,
};
use crate::diagnostics::StdinDiagnostic;
use crate::logging::LogOptions;
use crate::runner::collector::Collector;
use crate::runner::crawler::Crawler;
use crate::runner::execution::{Execution, Stdin};
use crate::runner::finalizer::{FinalizePayload, Finalizer};
use crate::runner::handler::Handler;
use crate::runner::process_file::{ProcessFile, ProcessStdinFilePayload};
use crate::runner::scan_kind::derive_best_scan_kind;
use crate::{CliDiagnostic, CliSession, setup_cli_subscriber};
use biome_configuration::Configuration;
use biome_console::{Console, ConsoleExt, markup};
use biome_diagnostics::PrintDiagnostic;
use biome_fs::{BiomePath, FileSystem};
use biome_resolver::FsWithResolverProxy;
use biome_service::configuration::{
    LoadedConfiguration, LoadedLocation, ProjectScanComputer, load_configuration,
};
use biome_service::diagnostics::ConfigurationOutsideProject;
use biome_service::projects::ProjectKey;
use biome_service::settings::ModuleGraphResolutionKind;
use biome_service::workspace::{
    OpenProjectParams, ScanKind, ScanProjectParams, UpdateSettingsParams,
};
use biome_service::{Workspace, WorkspaceError};
use camino::{Utf8Path, Utf8PathBuf};
use std::ffi::OsString;
use std::time::Duration;
use tracing::info;

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
/// - [CommandRunner::execute_without_crawling] (only if requires_crawling() = false)
pub(crate) trait CommandRunner {
    type CrawlerOutput;
    type Collector: Collector;
    type Crawler: Crawler<
            Self::CrawlerOutput,
            Handler = Self::Handler,
            ProcessFile = Self::ProcessFile,
            Collector = Self::Collector,
        >;
    type Finalizer: Finalizer<Input = Self::CrawlerOutput>;
    type Handler: Handler;
    type ProcessFile: ProcessFile;

    /// The name of the command that will appear in the diagnostics
    fn command_name(&self) -> &'static str;

    /// Whether this command requires file crawling.
    ///
    /// If `false`, the command must implement [CommandRunner::execute_without_crawling]
    /// and will bypass the standard crawler/collector/finalizer flow.
    ///
    /// This is useful for commands like `migrate` that operate on configuration files
    /// directly rather than traversing source files.
    fn requires_crawling(&self) -> bool;

    /// The [ScanKind] that could be used for this command. Some commands shouldn't implement this
    /// because it should be derived from the configuration.
    fn minimal_scan_kind(&self) -> Option<ScanKind>;

    /// Generates the collector to use for this command.
    fn collector(
        &self,
        fs: &dyn FileSystem,
        execution: &dyn Execution,
        cli_options: &CliOptions,
    ) -> Self::Collector;

    fn validated_paths_for_execution(
        &self,
        paths: Vec<OsString>,
        working_dir: &Utf8Path,
        execution: &dyn Execution,
    ) -> Result<Vec<String>, CliDiagnostic> {
        let mut paths = paths
            .into_iter()
            .map(|path| path.into_string().map_err(WorkspaceError::non_utf8_path))
            .collect::<Result<Vec<_>, _>>()?;

        if paths.is_empty() {
            if execution.is_vcs_targeted() {
                // If `--staged` or `--changed` is specified, it's
                // acceptable for them to be empty, so ignore it.
            } else {
                paths.push(working_dir.to_string());
            }
        }

        Ok(paths)
    }

    fn setup_logging(&self, log_options: &LogOptions, cli_options: &CliOptions) {
        setup_cli_subscriber(
            log_options.log_file.as_deref(),
            log_options.log_level,
            log_options.log_kind,
            cli_options.colors.as_ref(),
        );
    }

    /// The main command to use.
    fn run(
        &mut self,
        session: CliSession,
        log_options: &LogOptions,
        cli_options: &CliOptions,
    ) -> Result<(), CliDiagnostic> {
        self.setup_logging(log_options, cli_options);
        self.check_incompatible_arguments()?;

        let console = &mut *session.app.console;
        let workspace = &*session.app.workspace;
        let fs = workspace.fs();

        let configured_workspace = self.configure_workspace(fs, console, workspace, cli_options)?;

        // Commands that don't require crawling can implement custom execution logic
        if !self.requires_crawling() {
            return self.execute_without_crawling(session, configured_workspace);
        }

        let ConfiguredWorkspace {
            execution,
            paths,
            duration,
            configuration_files: _,
            project_key,
        } = configured_workspace;

        if let Some(stdin) = self.get_stdin(console, execution.as_ref())? {
            let biome_path = BiomePath::new(stdin.as_path());
            if biome_path.extension().is_none() {
                console.error(markup! {
                    {PrintDiagnostic::simple(&CliDiagnostic::from(StdinDiagnostic::new_no_extension()))}
                });
                console.append(markup! {{stdin.as_content()}});
                return Ok(());
            }

            return Self::ProcessFile::process_std_in(ProcessStdinFilePayload {
                biome_path: &biome_path,
                project_key,
                workspace,
                execution: execution.as_ref(),
                content: stdin.as_content(),
                cli_options,
                console,
                skip_ignore_check: Self::ProcessFile::should_skip_ignore_check(
                    &biome_path,
                    workspace,
                ),
            });
        }

        let collector = self.collector(fs, execution.as_ref(), cli_options);
        let mut output: Self::CrawlerOutput = Self::Crawler::crawl(
            execution.as_ref(),
            workspace,
            fs,
            project_key,
            paths.clone(),
            collector,
        )?;

        Self::Finalizer::before_finalize(project_key, fs, workspace, &mut output)?;

        Self::Finalizer::finalize(FinalizePayload {
            cli_options,
            execution: execution.as_ref(),
            fs,
            console,
            workspace,
            scan_duration: duration,
            crawler_output: output,
            paths,
        })
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
        let working_dir = fs.working_directory().unwrap_or_default();
        // Load configuration
        let configuration_path_hint = cli_options.as_configuration_path_hint(working_dir.as_path());
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
        let LoadedConfiguration {
            extended_configurations,
            configuration,
            diagnostics: _,
            directory_path,
            file_path,
            mut loaded_location,
        } = loaded_configuration;

        // Merge the FS configuration with the CLI arguments
        let configuration = self.merge_configuration(
            configuration,
            directory_path.clone(),
            file_path,
            fs,
            console,
        )?;

        let execution = self.get_execution(cli_options, console, workspace)?;

        let root_configuration_dir = directory_path
            .clone()
            .unwrap_or_else(|| working_dir.clone());
        // Using `--config-path`, users can point to a (root) config file that
        // is not actually at the root of the project. So between the working
        // directory and configuration directory, we use whichever one is higher
        // up in the file system.
        let project_dir = if root_configuration_dir.starts_with(&working_dir) {
            &working_dir
        } else {
            loaded_location = LoadedLocation::InProject;
            &root_configuration_dir
        };
        if !loaded_location.is_in_project() {
            console.log(markup! {
                {PrintDiagnostic::simple(&ConfigurationOutsideProject)}
            })
        }

        let paths = self.get_files_to_process(fs, &configuration)?;
        let paths = self.validated_paths_for_execution(paths, &working_dir, execution.as_ref())?;

        // Open the project
        let open_project_result = workspace.open_project(OpenProjectParams {
            path: BiomePath::new(project_dir),
            open_uninitialized: true,
        })?;

        let stdin = execution.get_stdin_file_path().map(Utf8PathBuf::from);
        let computed_scan_kind =
            execution.scan_kind_computer(ProjectScanComputer::new(&configuration));

        let scan_kind = derive_best_scan_kind(
            computed_scan_kind,
            stdin.as_deref(),
            &root_configuration_dir,
            &working_dir,
            &configuration,
            self.minimal_scan_kind(),
        );

        // Scan the project
        let scan_kind =
            execution.compute_scan_kind(paths.as_slice(), working_dir.as_path(), scan_kind);

        // Update the settings of the project
        let result = workspace.update_settings(UpdateSettingsParams {
            project_key: open_project_result.project_key,
            workspace_directory: Some(BiomePath::new(project_dir)),
            configuration,
            extended_configurations: extended_configurations
                .into_iter()
                .map(|(path, config)| (BiomePath::from(path), config))
                .collect(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::from(&scan_kind),
        })?;
        if self.should_validate_configuration_diagnostics() {
            print_diagnostics_from_workspace_result(
                result.diagnostics.as_slice(),
                console,
                cli_options.verbose,
            )?;
        }

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
    fn get_stdin(
        &self,
        console: &mut dyn Console,
        execution: &dyn Execution,
    ) -> Result<Option<Stdin>, CliDiagnostic> {
        let stdin = if let Some(stdin_file_path) = execution.get_stdin_file_path() {
            let input_code = console.read();
            if let Some(input_code) = input_code {
                let path = Utf8PathBuf::from(stdin_file_path);
                Some((path, input_code).into())
            } else {
                // we provided the argument without a piped stdin, we bail
                return Err(CliDiagnostic::missing_argument(
                    "stdin",
                    self.command_name(),
                ));
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
        loaded_configuration: Configuration,
        loaded_directory: Option<Utf8PathBuf>,
        loaded_file: Option<Utf8PathBuf>,
        fs: &dyn FileSystem,
        console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError>;

    /// It returns the paths that need to be handled/traversed.
    fn get_files_to_process(
        &self,
        fs: &dyn FileSystem,
        configuration: &Configuration,
    ) -> Result<Vec<OsString>, CliDiagnostic>;

    /// Returns the [Execution] mode.
    fn get_execution(
        &self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
        workspace: &dyn Workspace,
    ) -> Result<Box<dyn Execution>, CliDiagnostic>;

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

    /// Custom execution for commands that don't require file crawling.
    ///
    /// This method is only called when [CommandRunner::requires_crawling] returns `false`.
    /// Commands like `migrate` can implement this to bypass the standard
    /// crawler/collector/finalizer flow and execute their custom logic directly.
    ///
    /// The [ConfiguredWorkspace] provides access to:
    /// - `project_key`: The project identifier
    /// - `configuration_files`: Nested configuration files discovered during project scan
    /// - `execution`: The execution context
    /// - `paths`: The validated paths (may be empty for non-crawling commands)
    /// - `duration`: The duration of the project scan
    ///
    /// # Default implementation
    ///
    /// Panics if called, as commands with `requires_crawling() = true` should never reach this code path.
    fn execute_without_crawling(
        &mut self,
        _session: CliSession,
        _configured_workspace: ConfiguredWorkspace,
    ) -> Result<(), CliDiagnostic> {
        panic!(
            "{} command has requires_crawling() = false but did not implement execute_without_crawling()",
            self.command_name()
        )
    }

    // #endregion
}

pub(crate) struct ConfiguredWorkspace {
    /// Execution context
    pub execution: Box<dyn Execution>,
    /// Paths to crawl
    pub paths: Vec<String>,
    /// The duration of the scanning
    pub duration: Option<Duration>,
    /// Configuration files found inside the project
    pub configuration_files: Vec<BiomePath>,
    /// The unique identifier of the project
    pub project_key: ProjectKey,
}
