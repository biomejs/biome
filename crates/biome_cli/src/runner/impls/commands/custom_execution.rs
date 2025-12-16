use crate::cli_options::CliOptions;
use crate::runner::execution::Execution;
use crate::runner::{CommandRunner, ConfiguredWorkspace};
use crate::{CliDiagnostic, CliSession};
use biome_configuration::Configuration;
use biome_console::Console;
use biome_fs::FileSystem;
use biome_service::workspace::ScanKind;
use biome_service::{Workspace, WorkspaceError};
use camino::Utf8PathBuf;
use std::ffi::OsString;
use std::ops::{Deref, DerefMut};

/// A command that doesn't require crawling but requires a custom execution
pub(crate) struct CustomExecutionCmdImpl<C>(pub C)
where
    C: CustomExecutionCmd;

impl<C> Deref for CustomExecutionCmdImpl<C>
where
    C: CustomExecutionCmd,
{
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C> DerefMut for CustomExecutionCmdImpl<C>
where
    C: CustomExecutionCmd,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub(crate) trait CustomExecutionCmd {
    /// Alias of [CommandRunner::command_name]
    fn command_name(&self) -> &'static str;

    /// Alias of [CommandRunner::minimal_scan_kind]
    fn minimal_scan_kind(&self) -> Option<ScanKind>;

    /// Alias of [crate::runner::CommandRunner::should_write]
    fn should_write(&self) -> bool;

    /// Alias of [CommandRunner::get_execution]
    fn get_execution(
        &self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
        workspace: &dyn Workspace,
    ) -> Result<Box<dyn Execution>, CliDiagnostic>;

    /// Alias for [CommandRunner::check_incompatible_arguments]
    fn check_incompatible_arguments(&self) -> Result<(), CliDiagnostic>;

    /// Alias for [CommandRunner::should_validate_configuration_diagnostics]
    fn should_validate_configuration_diagnostics(&self) -> bool;

    /// Alias for [CommandRunner::execute_without_crawling]
    fn execute_without_crawling(
        &mut self,
        session: CliSession,
        configured_workspace: ConfiguredWorkspace,
    ) -> Result<(), CliDiagnostic>;

    /// Alias for [CommandRunner::merge_configuration]
    fn merge_configuration(
        &mut self,
        _: Configuration,
        _: Option<Utf8PathBuf>,
        _: Option<Utf8PathBuf>,
        _: &dyn FileSystem,
        _: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError>;
}

impl<SC> CommandRunner for CustomExecutionCmdImpl<SC>
where
    SC: CustomExecutionCmd,
{
    type CrawlerOutput = ();
    type Collector = ();
    type Crawler = ();
    type Finalizer = ();
    type Handler = ();
    type ProcessFile = ();

    fn command_name(&self) -> &'static str {
        self.deref().command_name()
    }

    fn requires_crawling(&self) -> bool {
        false
    }

    fn minimal_scan_kind(&self) -> Option<ScanKind> {
        self.deref().minimal_scan_kind()
    }

    fn collector(&self, _: &dyn FileSystem, _: &dyn Execution, _: &CliOptions) -> Self::Collector {}

    fn merge_configuration(
        &mut self,
        loaded_configuration: Configuration,
        loaded_directory: Option<Utf8PathBuf>,
        loaded_file: Option<Utf8PathBuf>,
        fs: &dyn FileSystem,
        console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
        self.deref_mut().merge_configuration(
            loaded_configuration,
            loaded_directory,
            loaded_file,
            fs,
            console,
        )
    }

    fn get_files_to_process(
        &self,
        _: &dyn FileSystem,
        _: &Configuration,
    ) -> Result<Vec<OsString>, CliDiagnostic> {
        Ok(vec![])
    }

    fn get_execution(
        &self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
        workspace: &dyn Workspace,
    ) -> Result<Box<dyn Execution>, CliDiagnostic> {
        self.deref().get_execution(cli_options, console, workspace)
    }

    fn check_incompatible_arguments(&self) -> Result<(), CliDiagnostic> {
        self.deref().check_incompatible_arguments()
    }

    fn should_validate_configuration_diagnostics(&self) -> bool {
        self.deref().should_validate_configuration_diagnostics()
    }

    fn execute_without_crawling(
        &mut self,
        _session: CliSession,
        _configured_workspace: ConfiguredWorkspace,
    ) -> Result<(), CliDiagnostic> {
        self.deref_mut()
            .execute_without_crawling(_session, _configured_workspace)
    }
}
