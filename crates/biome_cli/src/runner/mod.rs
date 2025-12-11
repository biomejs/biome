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
//!   ├─→ [if REQUIRES_CRAWLING = false]
//!   │    └─→ execute_without_crawling() ──→ Custom logic (e.g., migrate)
//!   │
//!   └─→ [if REQUIRES_CRAWLING = true]
//!        │
//!        ├─→ Crawler::crawl()
//!        │    │
//!        │    ├─→ spawns Collector thread ──→ Collects diagnostics/messages
//!        │    │
//!        │    └─→ traverses files via Inspector
//!        │         │
//!        │         ├─→ Inspector::can_handle() ──→ Filter files
//!        │         │
//!        │         └─→ Inspector::handle_path()
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
//! The main trait that orchestrates command execution. Commands implement this to
//! define their behavior, configure the workspace, and specify which files to process.
//!
//! ### [`Crawler`]
//! Orchestrates the file traversal process. Spawns a collector thread, walks the
//! file system, and produces structured output.
//!
//! ### [`Inspector`]
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
//!
//! ## Non-Crawling Commands
//!
//! Commands that don't traverse files (like `migrate`) can set `REQUIRES_CRAWLING = false`
//! and implement `execute_without_crawling()` to bypass the standard traversal flow.
//!
//! ## Module Organization
//!
//! - [`collector`]: Diagnostic collection during traversal
//! - [`crawler`]: File system traversal orchestration
//! - [`execution`]: Feature requirements and capability checks
//! - [`finalizer`]: Results presentation and reporting
//! - [`inspector`]: Per-file filtering and processing dispatch
//! - [`process_file`]: File processing trait and status types
//! - [`scan_kind`]: Utilities for determining scan strategy

pub(crate) mod collector;
pub(crate) mod crawler;
pub(crate) mod diagnostics;
pub(crate) mod execution;
pub(crate) mod finalizer;
pub(crate) mod impls;
pub(crate) mod inspector;
pub(crate) mod process_file;
mod run;
pub(crate) mod scan_kind;

use crate::cli_options::CliOptions;
use crate::commands::{
    print_diagnostics_from_workspace_result, validate_configuration_diagnostics,
};
use crate::execute::Stdin;
use crate::logging::LogOptions;
use crate::runner::collector::Collector;
use crate::runner::crawler::Crawler;
use crate::runner::execution::Execution;
use crate::runner::finalizer::Finalizer;
use crate::runner::process_file::ProcessFile;
use crate::runner::scan_kind::derive_best_scan_kind;
use crate::{CliDiagnostic, CliSession, TraversalMode, execute_mode, setup_cli_subscriber};
use biome_configuration::Configuration;
use biome_console::Console;
use biome_deserialize::Merge;
use biome_fs::{BiomePath, FileSystem};
use biome_resolver::FsWithResolverProxy;
use biome_service::configuration::{
    LoadedConfiguration, ProjectScanComputer, load_configuration, load_editorconfig,
};
use biome_service::projects::ProjectKey;
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
/// - [CommandRunner::execute_without_crawling] (only if REQUIRES_CRAWLING = false)
pub(crate) trait CommandRunner: Sized {
    type CrawlerOutput;

    type Crawler: Crawler<Output = Self::CrawlerOutput>;
    type Finalizer: Finalizer<Input = Self::CrawlerOutput>;

    /// The name of the command that will appear in the diagnostics
    const COMMAND_NAME: &'static str;

    /// The [ScanKind] to use for this command
    const SCAN_KIND: ScanKind;

    /// Whether this command should be aware of the VCS integration
    const SHOULD_TARGET_VCS: bool;

    /// Whether this command requires file crawling.
    ///
    /// If `false`, the command must implement [CommandRunner::execute_without_crawling]
    /// and will bypass the standard crawler/collector/finalizer flow.
    ///
    /// This is useful for commands like `migrate` that operate on configuration files
    /// directly rather than traversing source files.
    const REQUIRES_CRAWLING: bool;

    fn collector(cli_options: &CliOptions) -> impl Collector;

    fn validated_paths_for_execution(
        &self,
        paths: Vec<OsString>,
        working_dir: &Utf8Path,
    ) -> Result<Vec<String>, CliDiagnostic> {
        let mut paths = paths
            .into_iter()
            .map(|path| path.into_string().map_err(WorkspaceError::non_utf8_path))
            .collect::<Result<Vec<_>, _>>()?;

        if paths.is_empty() {
            if Self::SHOULD_TARGET_VCS {
                // If `--staged` or `--changed` is specified, it's
                // acceptable for them to be empty, so ignore it.
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
        mut session: CliSession,
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
        if !Self::REQUIRES_CRAWLING {
            return self.execute_without_crawling(session, configured_workspace);
        }

        let ConfiguredWorkspace {
            execution,
            paths,
            duration,
            configuration_files,
            project_key,
        } = configured_workspace;

        if let Some(stdin) = execution.as_stdin_file() {
            let biome_path = BiomePath::new(stdin.as_path());
            return crate::execute::std_in::run(
                session,
                project_key,
                &execution,
                biome_path,
                stdin.as_content(),
                cli_options,
            );
        }

        let collector = Self::collector(cli_options);
        let mut output: <Self::Crawler as Crawler>::Output = Self::Crawler::crawl(
            execution,
            workspace,
            fs,
            project_key,
            paths.clone(),
            configuration_files,
            collector,
        )?;

        Self::Finalizer::before_finalize(project_key, fs, workspace, &mut output)?;

        Self::Finalizer::finalize(duration, console, output, cli_options)
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
            &root_configuration_dir
        };

        let paths = self.get_files_to_process(fs, &configuration)?;
        let paths = self.validated_paths_for_execution(paths, &working_dir)?;

        // Open the project
        let open_project_result = workspace.open_project(OpenProjectParams {
            path: BiomePath::new(project_dir),
            open_uninitialized: true,
        })?;

        let scan_kind_computer = execution.scan_kind_computer(&configuration);

        let stdin = self.get_stdin(console, execution.as_ref())?;
        let scan_kind = derive_best_scan_kind(
            scan_kind_computer.compute(),
            stdin.as_ref(),
            &root_configuration_dir,
            &working_dir,
            &configuration,
            Self::SCAN_KIND,
        );

        // Update the settings of the project
        let result = workspace.update_settings(UpdateSettingsParams {
            project_key: open_project_result.project_key,
            workspace_directory: Some(BiomePath::new(project_dir)),
            configuration,
            extended_configurations: extended_configurations
                .into_iter()
                .map(|(path, config)| (BiomePath::from(path), config))
                .collect(),
        })?;
        if self.should_validate_configuration_diagnostics() {
            print_diagnostics_from_workspace_result(
                result.diagnostics.as_slice(),
                console,
                cli_options.verbose,
            )?;
        }

        // Scan the project
        let scan_kind =
            execution.compute_scan_kind(paths.as_slice(), working_dir.as_path(), scan_kind);
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

    /// Whether the command should write the files.
    fn should_write(&self) -> bool;

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
    /// This method is only called when [CommandRunner::REQUIRES_CRAWLING] is `false`.
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
    /// Panics if called, as commands with `REQUIRES_CRAWLING = true` should never reach this code path.
    fn execute_without_crawling(
        &mut self,
        _session: CliSession,
        _configured_workspace: ConfiguredWorkspace,
    ) -> Result<(), CliDiagnostic> {
        panic!(
            "{} command has REQUIRES_CRAWLING = false but did not implement execute_without_crawling()",
            Self::COMMAND_NAME
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

pub(crate) trait LoadEditorConfig: CommandRunner {
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
    use crate::cli_options::{CliOptions, cli_options};
    use crate::logging::{LogOptions, log_options};
    use crate::runner::CommandRunner;
    use crate::runner::collector::Collector;
    use crate::runner::crawler::{Crawler, CrawlerContext, CrawlerOptions};
    use crate::runner::execution::Execution;
    use crate::runner::finalizer::Finalizer;
    use crate::runner::inspector::Inspector;
    use crate::runner::process_file::{FileStatus, Message, ProcessFile};
    use crate::runner::run::run_command;
    use crate::{CliDiagnostic, CliSession, TraversalMode};
    use biome_configuration::Configuration;
    use biome_console::{BufferConsole, Console};
    use biome_diagnostics::{DiagnosticTags, Error, Severity};
    use biome_fs::{BiomePath, FileSystem, MemoryFileSystem, OsFileSystem, TraversalContext};
    use biome_service::projects::ProjectKey;
    use biome_service::workspace::{FeatureName, FeaturesBuilder, FeaturesSupported, ScanKind};
    use biome_service::{Workspace, WorkspaceError, workspace};
    use camino::{Utf8Path, Utf8PathBuf};
    use crossbeam::channel::Receiver;
    use std::collections::BTreeSet;
    use std::ffi::OsString;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_command_runner_with_output() {
        struct TestCommandRunner;
        struct TestCollector;
        struct TestCrawler;
        struct TestFinalizer;
        struct TestProcessFile;
        struct TestOutput {}
        struct TestExecution;

        impl Execution for TestExecution {
            fn to_feature(&self) -> FeatureName {
                FeaturesBuilder::new().with_all().build()
            }

            fn can_handle(&self, features: FeaturesSupported) -> bool {
                features.supports_format()
            }
        }

        #[derive(Default)]
        struct TestInspector;

        impl Inspector for TestInspector {
            type ProcessFile = TestProcessFile;

            // Uses default handle_path() implementation which calls ProcessFile::process_file()
        }

        impl Finalizer for TestFinalizer {
            type Input = TestOutput;

            fn finalize(
                scan_duration: Option<Duration>,
                console: &mut dyn Console,
                input: Self::Input,
            ) -> Result<(), CliDiagnostic> {
                Ok(())
            }
        }

        impl Crawler for TestCrawler {
            type Output = TestOutput;
            type CollectorOutput = TestOutput;
            type Inspector = TestInspector;

            fn output<C>(
                _result: C::Result,
                _evaluated_paths: BTreeSet<BiomePath>,
                _duration: Duration,
            ) -> Self::Output
            where
                C: Collector,
            {
                TestOutput {}
            }
        }

        impl Collector for TestCollector {
            type Result = ();

            fn should_collect(&self) -> bool {
                true
            }

            fn diagnostic_level(&self) -> Severity {
                Severity::Error
            }

            fn verbose(&self) -> bool {
                false
            }

            fn run(
                &self,
                receiver: Receiver<Message>,
                interner: Receiver<Utf8PathBuf>,
                execution: &dyn Execution,
            ) {
            }

            fn result(self, _duration: Duration, _ctx: &dyn CrawlerContext) -> Self::Result {
                ()
            }
        }

        impl ProcessFile for TestProcessFile {
            fn process_file<Ctx>(ctx: &Ctx, path: BiomePath) -> Result<FileStatus, Message>
            where
                Ctx: CrawlerContext,
            {
                Ok(FileStatus::Unchanged)
            }
        }

        impl CommandRunner for TestCommandRunner {
            type CrawlerOutput = TestOutput;
            type Crawler = TestCrawler;
            type Finalizer = TestFinalizer;
            const COMMAND_NAME: &'static str = "crawl";
            const SCAN_KIND: ScanKind = ScanKind::KnownFiles;
            const SHOULD_TARGET_VCS: bool = false;
            const REQUIRES_CRAWLING: bool = true;

            fn collector(cli_options: &CliOptions) -> impl Collector {
                TestCollector
            }

            fn validated_paths_for_execution(
                &self,
                paths: Vec<OsString>,
                working_dir: &Utf8Path,
            ) -> Result<Vec<String>, CliDiagnostic> {
                Ok(paths
                    .into_iter()
                    .map(|path| path.to_string_lossy().to_string())
                    .collect())
            }

            fn merge_configuration(
                &mut self,
                _loaded_configuration: Configuration,
                _loaded_directory: Option<Utf8PathBuf>,
                _loaded_file: Option<Utf8PathBuf>,
                _fs: &dyn FileSystem,
                _console: &mut dyn Console,
            ) -> Result<Configuration, WorkspaceError> {
                Ok(Configuration::default())
            }

            fn get_files_to_process(
                &self,
                _fs: &dyn FileSystem,
                _configuration: &Configuration,
            ) -> Result<Vec<OsString>, CliDiagnostic> {
                Ok(vec![OsString::from("file.js")])
            }

            fn should_write(&self) -> bool {
                false
            }

            fn get_execution(
                &self,
                _cli_options: &CliOptions,
                _console: &mut dyn Console,
                _workspace: &dyn Workspace,
            ) -> Result<Box<dyn Execution>, CliDiagnostic> {
                Ok(Box::new(TestExecution))
            }
        }

        let command = TestCommandRunner;
        let mut fs = MemoryFileSystem::default();
        fs.insert("file.js".into(), "let f;");
        let workspace = workspace::server(Arc::new(fs), None);
        let mut console = BufferConsole::default();
        let session = CliSession::new(&*workspace, &mut console).unwrap();
        let result = run_command(
            session,
            &LogOptions::default(),
            &CliOptions::default(),
            command,
        );

        assert!(result.is_ok());
    }
}
