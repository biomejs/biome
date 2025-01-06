use crate::changed::get_changed_files;
use crate::cli_options::CliOptions;
use crate::commands::{CommandRunner, LoadEditorConfig};
use crate::{CliDiagnostic, Execution};
use biome_configuration::analyzer::assist::{AssistConfiguration, AssistEnabled};
use biome_configuration::analyzer::LinterEnabled;
use biome_configuration::formatter::FormatterEnabled;
use biome_configuration::{Configuration, FormatterConfiguration, LinterConfiguration};
use biome_console::Console;
use biome_deserialize::Merge;
use biome_fs::FileSystem;
use biome_service::configuration::LoadedConfiguration;
use biome_service::projects::ProjectKey;
use biome_service::{Workspace, WorkspaceError};
use std::ffi::OsString;

pub(crate) struct CiCommandPayload {
    pub(crate) formatter_enabled: Option<FormatterEnabled>,
    pub(crate) linter_enabled: Option<LinterEnabled>,
    pub(crate) assist_enabled: Option<AssistEnabled>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) configuration: Option<Configuration>,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

impl LoadEditorConfig for CiCommandPayload {
    fn should_load_editor_config(&self, fs_configuration: &Configuration) -> bool {
        self.configuration
            .as_ref()
            .is_some_and(|c| c.use_editorconfig())
            || fs_configuration.use_editorconfig()
    }
}

impl CommandRunner for CiCommandPayload {
    const COMMAND_NAME: &'static str = "ci";

    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        fs: &dyn FileSystem,
        console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
        let LoadedConfiguration {
            configuration: biome_configuration,
            directory_path: configuration_path,
            ..
        } = loaded_configuration;

        let mut fs_configuration =
            self.load_editor_config(configuration_path, &biome_configuration, fs, console)?;
        // this makes biome configuration take precedence over editorconfig configuration
        fs_configuration.merge_with(biome_configuration);

        let formatter = fs_configuration
            .formatter
            .get_or_insert_with(FormatterConfiguration::default);

        if self.formatter_enabled.is_some() {
            formatter.enabled = self.formatter_enabled;
        }

        let linter = fs_configuration
            .linter
            .get_or_insert_with(LinterConfiguration::default);

        if self.linter_enabled.is_some() {
            linter.enabled = self.linter_enabled;
        }

        let assist = fs_configuration
            .assist
            .get_or_insert_with(AssistConfiguration::default);

        if self.assist_enabled.is_some() {
            assist.enabled = self.assist_enabled;
        }

        if let Some(mut configuration) = self.configuration.clone() {
            if let Some(linter) = configuration.linter.as_mut() {
                // Don't overwrite rules from the CLI configuration.
                // Otherwise, rules that are disabled in the config file might
                // become re-enabled due to the defaults included in the CLI
                // configuration.
                linter.rules = None;
            }
            fs_configuration.merge_with(configuration);
        }

        Ok(fs_configuration)
    }

    fn get_files_to_process(
        &self,
        fs: &dyn FileSystem,
        configuration: &Configuration,
    ) -> Result<Vec<OsString>, CliDiagnostic> {
        if self.changed {
            get_changed_files(fs, configuration, self.since.as_deref())
        } else {
            Ok(self.paths.clone())
        }
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        None
    }

    fn should_write(&self) -> bool {
        false
    }

    fn get_execution(
        &self,
        cli_options: &CliOptions,
        _console: &mut dyn Console,
        _workspace: &dyn Workspace,
        project_key: ProjectKey,
    ) -> Result<Execution, CliDiagnostic> {
        Ok(Execution::new_ci(project_key, (false, self.changed).into()).set_report(cli_options))
    }

    fn check_incompatible_arguments(&self) -> Result<(), CliDiagnostic> {
        if self.formatter_enabled.is_some_and(|v| !v.value())
            && self.linter_enabled.is_some_and(|v| !v.value())
            && self.assist_enabled.is_some_and(|v| !v.value())
        {
            return Err(CliDiagnostic::incompatible_end_configuration("Formatter, linter and assist are disabled, can't perform the command. At least one feature needs to be enabled. This is probably and error."));
        }
        if self.since.is_some() && !self.changed {
            return Err(CliDiagnostic::incompatible_arguments("since", "changed"));
        }
        Ok(())
    }
}
