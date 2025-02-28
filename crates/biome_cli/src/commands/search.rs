use crate::cli_options::CliOptions;
use crate::commands::CommandRunner;
use crate::{CliDiagnostic, Execution, TraversalMode};
use biome_configuration::vcs::VcsConfiguration;
use biome_configuration::{Configuration, FilesConfiguration};
use biome_console::Console;
use biome_deserialize::Merge;
use biome_fs::FileSystem;
use biome_grit_patterns::GritTargetLanguage;
use biome_service::configuration::LoadedConfiguration;
use biome_service::projects::ProjectKey;
use biome_service::workspace::ParsePatternParams;
use biome_service::{Workspace, WorkspaceError};
use std::ffi::OsString;

pub(crate) struct SearchCommandPayload {
    pub(crate) files_configuration: Option<FilesConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) pattern: String,
    pub(crate) language: Option<GritTargetLanguage>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) vcs_configuration: Option<VcsConfiguration>,
}

impl CommandRunner for SearchCommandPayload {
    const COMMAND_NAME: &'static str = "search";

    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        _fs: &dyn FileSystem,
        _console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
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
        _fs: &dyn FileSystem,
        _configuration: &Configuration,
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
        console: &mut dyn Console,
        workspace: &dyn Workspace,
        project_key: ProjectKey,
    ) -> Result<Execution, CliDiagnostic> {
        let pattern = workspace
            .parse_pattern(ParsePatternParams {
                pattern: self.pattern.clone(),
                default_language: self.language.clone().unwrap_or_default(),
            })?
            .pattern_id;
        Ok(Execution::new(TraversalMode::Search {
            project_key,
            pattern,
            language: self.language.clone(),
            stdin: self.get_stdin(console)?,
        })
        .set_report(cli_options))
    }
}
