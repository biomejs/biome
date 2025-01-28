use super::{
    check_fix_incompatible_arguments, CommandRunner, FixFileModeOptions, MigrateSubCommand,
};
use crate::cli_options::CliOptions;
use crate::diagnostics::MigrationDiagnostic;
use crate::execute::{Execution, TraversalMode};
use crate::CliDiagnostic;
use biome_configuration::Configuration;
use biome_console::{markup, Console, ConsoleExt};
use biome_fs::FileSystem;
use biome_service::configuration::LoadedConfiguration;
use biome_service::projects::ProjectKey;
use biome_service::{Workspace, WorkspaceError};
use camino::Utf8PathBuf;
use std::ffi::OsString;

pub(crate) struct MigrateCommandPayload {
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) sub_command: Option<MigrateSubCommand>,
    pub(crate) configuration_file_path: Option<Utf8PathBuf>,
    pub(crate) configuration_directory_path: Option<Utf8PathBuf>,
}

impl CommandRunner for MigrateCommandPayload {
    const COMMAND_NAME: &'static str = "migrate";

    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        _fs: &dyn FileSystem,
        _console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
        self.configuration_file_path = loaded_configuration.file_path;
        self.configuration_directory_path = loaded_configuration.directory_path;
        Ok(loaded_configuration.configuration)
    }

    fn get_files_to_process(
        &self,
        _fs: &dyn FileSystem,
        _configuration: &Configuration,
    ) -> Result<Vec<OsString>, CliDiagnostic> {
        Ok(vec![])
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        None
    }

    fn should_write(&self) -> bool {
        self.write || self.fix
    }

    fn get_execution(
        &self,
        _cli_options: &CliOptions,
        console: &mut dyn Console,
        _workspace: &dyn Workspace,
        project_key: ProjectKey,
    ) -> Result<Execution, CliDiagnostic> {
        if let Some(path) = self.configuration_file_path.clone() {
            Ok(Execution::new(TraversalMode::Migrate {
                project_key,
                write: self.should_write(),
                configuration_file_path: path,
                sub_command: self.sub_command.clone(),
            }))
        } else {
            console.log(markup! {
            <Info>"If this project has not yet been set up with Biome yet, please follow the "<Hyperlink href="https://biomejs.dev/guides/getting-started/">"Getting Started guide"</Hyperlink>" first."</Info>
        });
            Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                reason: "Biome couldn't find the Biome configuration file.".to_string(),
            }))
        }
    }

    fn check_incompatible_arguments(&self) -> Result<(), CliDiagnostic> {
        check_fix_incompatible_arguments(FixFileModeOptions {
            write: self.write,
            fix: self.fix,
            unsafe_: false,
            suppress: false,
            suppression_reason: None,
        })
    }

    fn should_validate_configuration_diagnostics(&self) -> bool {
        false
    }
}
