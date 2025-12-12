use crate::CliDiagnostic;
use crate::cli_options::CliOptions;
use crate::commands::format::FormatProcessFile;
use crate::execute::traverse::TraverseResult;
use crate::runner::CommandRunner;
use crate::runner::execution::Execution;
use crate::runner::impls::collectors::default::DefaultCollector;
use crate::runner::impls::crawlers::default::DefaultCrawler;
use crate::runner::impls::finalizers::default::DefaultFinalizer;
use crate::runner::impls::handlers::default::DefaultHandler;
use crate::runner::process_file::ProcessFile;
use biome_configuration::Configuration;
use biome_console::Console;
use biome_deserialize::Merge;
use biome_fs::FileSystem;
use biome_service::configuration::load_editorconfig;
use biome_service::workspace::ScanKind;
use biome_service::{Workspace, WorkspaceError};
use camino::Utf8PathBuf;
use std::ffi::OsString;

pub trait LoadEditorConfig: TraversalCommand {
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

/// A trait that returns a [TraverseResult] from a traversal command.
pub(crate) trait TraversalCommand {
    type ProcessFile: ProcessFile;

    /// The name of the command that will appear in the diagnostics
    fn command_name() -> &'static str;

    /// The [ScanKind] to use for this command
    fn scan_kind() -> ScanKind;

    fn get_execution(
        &self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
        workspace: &dyn Workspace,
    ) -> Result<Box<dyn Execution>, CliDiagnostic>;

    fn merge_configuration(
        &mut self,
        loaded_configuration: Configuration,
        loaded_directory: Option<Utf8PathBuf>,
        loaded_file: Option<Utf8PathBuf>,
        fs: &dyn FileSystem,
        console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError>;

    fn should_write(&self) -> bool;

    fn get_files_to_process(
        &self,
        fs: &dyn FileSystem,
        configuration: &Configuration,
    ) -> Result<Vec<OsString>, CliDiagnostic>;
}

impl<TC, P> CommandRunner for TC
where
    P: ProcessFile,
    TC: TraversalCommand<ProcessFile = P>,
{
    type CrawlerOutput = TraverseResult;
    type Collector = DefaultCollector;
    type Crawler = DefaultCrawler<Self::ProcessFile>;
    type Finalizer = DefaultFinalizer;
    type Handler = DefaultHandler;
    type ProcessFile = P;

    const SHOULD_TARGET_VCS: bool = true;

    const REQUIRES_CRAWLING: bool = true;

    /// The name of the command that will appear in the diagnostics
    fn command_name() -> &'static str {
        Self::command_name()
    }
    /// The [ScanKind] to use for this command
    fn scan_kind() -> ScanKind {
        Self::scan_kind()
    }

    fn collector(
        &self,
        fs: &dyn FileSystem,
        execution: &dyn Execution,
        cli_options: &CliOptions,
    ) -> Self::Collector {
        DefaultCollector::new(fs.working_directory().as_deref())
            .with_verbose(cli_options.verbose)
            .with_diagnostic_level(cli_options.diagnostic_level)
            .with_max_diagnostics(execution.get_max_diagnostics(cli_options))
    }

    fn merge_configuration(
        &mut self,
        loaded_configuration: Configuration,
        loaded_directory: Option<Utf8PathBuf>,
        loaded_file: Option<Utf8PathBuf>,
        fs: &dyn FileSystem,
        console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
        self.merge_configuration(
            loaded_configuration,
            loaded_directory,
            loaded_file,
            fs,
            console,
        )
    }

    fn get_files_to_process(
        &self,
        fs: &dyn FileSystem,
        configuration: &Configuration,
    ) -> Result<Vec<OsString>, CliDiagnostic> {
        self.get_files_to_process(fs, configuration)
    }

    fn should_write(&self) -> bool {
        self.should_write()
    }

    fn get_execution(
        &self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
        workspace: &dyn Workspace,
    ) -> Result<Box<dyn Execution>, CliDiagnostic> {
        self.get_execution(cli_options, console, workspace)
    }
}
