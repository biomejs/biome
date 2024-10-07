use super::{determine_fix_file_mode, FixFileModeOptions, LoadEditorConfig};
use crate::cli_options::CliOptions;
use crate::commands::{get_files_to_process_with_cli_options, CommandRunner};
use crate::{CliDiagnostic, Execution, TraversalMode};
use biome_configuration::analyzer::assists::PartialAssistsConfiguration;
use biome_configuration::{
    organize_imports::PartialOrganizeImports, PartialConfiguration, PartialFormatterConfiguration,
    PartialLinterConfiguration,
};
use biome_console::Console;
use biome_deserialize::Merge;
use biome_fs::FileSystem;
use biome_service::{configuration::LoadedConfiguration, DynRef, Workspace, WorkspaceError};
use std::ffi::OsString;

pub(crate) struct CheckCommandPayload {
    pub(crate) apply: bool,
    pub(crate) apply_unsafe: bool,
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) unsafe_: bool,
    pub(crate) configuration: Option<PartialConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) formatter_enabled: Option<bool>,
    pub(crate) linter_enabled: Option<bool>,
    pub(crate) organize_imports_enabled: Option<bool>,
    pub(crate) assists_enabled: Option<bool>,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

impl LoadEditorConfig for CheckCommandPayload {
    fn should_load_editor_config(&self, fs_configuration: &PartialConfiguration) -> bool {
        self.configuration
            .as_ref()
            .and_then(|c| c.use_editorconfig())
            .unwrap_or(fs_configuration.use_editorconfig().unwrap_or_default())
    }
}

impl CommandRunner for CheckCommandPayload {
    const COMMAND_NAME: &'static str = "check";

    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        fs: &DynRef<'_, dyn FileSystem>,
        console: &mut dyn Console,
    ) -> Result<PartialConfiguration, WorkspaceError> {
        let editorconfig_search_path = loaded_configuration.directory_path.clone();
        let LoadedConfiguration {
            configuration: biome_configuration,
            ..
        } = loaded_configuration;
        let mut fs_configuration =
            self.load_editor_config(editorconfig_search_path, &biome_configuration, fs, console)?;
        // this makes biome configuration take precedence over editorconfig configuration
        fs_configuration.merge_with(biome_configuration);

        let formatter = fs_configuration
            .formatter
            .get_or_insert_with(PartialFormatterConfiguration::default);

        if self.formatter_enabled.is_some() {
            formatter.enabled = self.formatter_enabled;
        }

        let linter = fs_configuration
            .linter
            .get_or_insert_with(PartialLinterConfiguration::default);

        if self.linter_enabled.is_some() {
            linter.enabled = self.linter_enabled;
        }

        let organize_imports = fs_configuration
            .organize_imports
            .get_or_insert_with(PartialOrganizeImports::default);

        if self.organize_imports_enabled.is_some() {
            organize_imports.enabled = self.organize_imports_enabled;
        }

        let assists = fs_configuration
            .assists
            .get_or_insert_with(PartialAssistsConfiguration::default);

        if self.assists_enabled.is_some() {
            assists.enabled = self.assists_enabled;
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
        fs: &DynRef<'_, dyn FileSystem>,
        configuration: &PartialConfiguration,
    ) -> Result<Vec<OsString>, CliDiagnostic> {
        let paths = get_files_to_process_with_cli_options(
            self.since.as_deref(),
            self.changed,
            self.staged,
            fs,
            configuration,
        )?
        .unwrap_or(self.paths.clone());

        Ok(paths)
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        self.stdin_file_path.as_deref()
    }

    fn should_write(&self) -> bool {
        self.write || self.fix
    }

    fn get_execution(
        &self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
        _workspace: &dyn Workspace,
    ) -> Result<Execution, CliDiagnostic> {
        let fix_file_mode = determine_fix_file_mode(
            FixFileModeOptions {
                apply: self.apply,
                apply_unsafe: self.apply_unsafe,
                write: self.write,
                fix: self.fix,
                unsafe_: self.unsafe_,
            },
            console,
        )?;

        Ok(Execution::new(TraversalMode::Check {
            fix_file_mode,
            stdin: self.get_stdin(console)?,
            vcs_targeted: (self.staged, self.changed).into(),
        })
        .set_report(cli_options))
    }
}
