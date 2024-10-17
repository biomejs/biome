use super::{
    check_fix_incompatible_arguments, CommandRunner, FixFileModeOptions, MigrateSubCommand,
};
use crate::cli_options::CliOptions;
use crate::diagnostics::MigrationDiagnostic;
use crate::execute::{Execution, TraversalMode};
use crate::CliDiagnostic;
use biome_configuration::PartialConfiguration;
use biome_console::{markup, Console, ConsoleExt};
use biome_fs::FileSystem;
use biome_service::configuration::LoadedConfiguration;
use biome_service::{DynRef, Workspace, WorkspaceError};
use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) struct MigrateCommandPayload {
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) sub_command: Option<MigrateSubCommand>,
    pub(crate) configuration_file_path: Option<PathBuf>,
    pub(crate) configuration_directory_path: Option<PathBuf>,
}

impl CommandRunner for MigrateCommandPayload {
    const COMMAND_NAME: &'static str = "migrate";

    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        _fs: &DynRef<'_, dyn FileSystem>,
        _console: &mut dyn Console,
    ) -> Result<PartialConfiguration, WorkspaceError> {
        self.configuration_file_path = loaded_configuration.file_path;
        self.configuration_directory_path = loaded_configuration.directory_path;
        Ok(loaded_configuration.configuration)
    }

    fn get_files_to_process(
        &self,
        _fs: &DynRef<'_, dyn FileSystem>,
        _configuration: &PartialConfiguration,
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
    ) -> Result<Execution, CliDiagnostic> {
        if let (Some(path), Some(directory_path)) = (
            self.configuration_file_path.clone(),
            self.configuration_directory_path.clone(),
        ) {
            Ok(Execution::new(TraversalMode::Migrate {
                write: self.should_write(),
                configuration_file_path: path,
                configuration_directory_path: directory_path,
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
            apply: false,
            apply_unsafe: false,
            write: self.write,
            fix: self.fix,
            unsafe_: false,
            suppress: false,
        })
    }

    fn should_validate_configuration_diagnostics(&self) -> bool {
        false
    }
}
