use crate::cli_options::CliOptions;
use crate::commands::CommandRunner;
use crate::{CliDiagnostic, Execution, TraversalMode};
use biome_configuration::{
    vcs::PartialVcsConfiguration, PartialConfiguration, PartialFilesConfiguration,
};
use biome_console::Console;
use biome_deserialize::Merge;
use biome_fs::FileSystem;
use biome_service::configuration::LoadedConfiguration;
use biome_service::workspace::ParsePatternParams;
use biome_service::{DynRef, Workspace, WorkspaceError};
use std::ffi::OsString;

pub(crate) struct SearchCommandPayload {
    pub(crate) files_configuration: Option<PartialFilesConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) pattern: String,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) vcs_configuration: Option<PartialVcsConfiguration>,
}

impl CommandRunner for SearchCommandPayload {
    const COMMAND_NAME: &'static str = "search";

    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        _fs: &DynRef<'_, dyn FileSystem>,
        _console: &mut dyn Console,
    ) -> Result<PartialConfiguration, WorkspaceError> {
        let LoadedConfiguration {
            mut configuration, ..
        } = loaded_configuration;
        configuration
            .files
            .merge_with(self.files_configuration.clone());
        configuration.vcs.merge_with(self.vcs_configuration.clone());

        Ok(configuration)
    }

    fn get_files_to_process(
        &self,
        _fs: &DynRef<'_, dyn FileSystem>,
        _configuration: &PartialConfiguration,
    ) -> Result<Vec<OsString>, CliDiagnostic> {
        Ok(self.paths.clone())
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        self.stdin_file_path.as_deref()
    }

    fn should_write(&self) -> bool {
        false
    }

    fn get_execution(
        &self,
        cli_options: &CliOptions,
        _console: &mut dyn Console,
        workspace: &dyn Workspace,
    ) -> Result<Execution, CliDiagnostic> {
        let pattern = workspace
            .parse_pattern(ParsePatternParams {
                pattern: self.pattern.clone(),
            })?
            .pattern_id;
        Ok(Execution::new(TraversalMode::Search {
            pattern,
            stdin: self.get_stdin(_console)?,
        })
        .set_report(cli_options))
    }
}
