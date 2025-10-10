use super::{FixFileModeOptions, LoadEditorConfig, determine_fix_file_mode};
use crate::cli_options::CliOptions;
use crate::commands::{CommandRunner, get_files_to_process_with_cli_options};
use crate::{CliDiagnostic, Execution, TraversalMode};
use biome_configuration::analyzer::LinterEnabled;
use biome_configuration::analyzer::assist::{AssistConfiguration, AssistEnabled};
use biome_configuration::css::CssParserConfiguration;
use biome_configuration::formatter::{FormatWithErrorsEnabled, FormatterEnabled};
use biome_configuration::json::JsonParserConfiguration;
use biome_configuration::{Configuration, FormatterConfiguration, LinterConfiguration};
use biome_console::Console;
use biome_deserialize::Merge;
use biome_fs::FileSystem;
use biome_service::{Workspace, WorkspaceError, configuration::LoadedConfiguration};
use std::ffi::OsString;

pub(crate) struct CheckCommandPayload {
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) unsafe_: bool,
    pub(crate) configuration: Option<Configuration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) formatter_enabled: Option<FormatterEnabled>,
    pub(crate) linter_enabled: Option<LinterEnabled>,
    pub(crate) assist_enabled: Option<AssistEnabled>,
    pub(crate) enforce_assist: bool,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
    pub(crate) format_with_errors: Option<FormatWithErrorsEnabled>,
    pub(crate) json_parser: Option<JsonParserConfiguration>,
    pub(crate) css_parser: Option<CssParserConfiguration>,
}

impl LoadEditorConfig for CheckCommandPayload {
    fn should_load_editor_config(&self, fs_configuration: &Configuration) -> bool {
        self.configuration
            .as_ref()
            .is_some_and(|c| c.use_editorconfig())
            || fs_configuration.use_editorconfig()
    }
}

impl CommandRunner for CheckCommandPayload {
    const COMMAND_NAME: &'static str = "check";

    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        fs: &dyn FileSystem,
        _console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
        let LoadedConfiguration {
            configuration: biome_configuration,
            directory_path,
            ..
        } = loaded_configuration;
        let mut configuration =
            self.combine_configuration(directory_path, biome_configuration, fs)?;

        let formatter = configuration
            .formatter
            .get_or_insert_with(FormatterConfiguration::default);

        if self.formatter_enabled.is_some() {
            formatter.enabled = self.formatter_enabled;
        }
        if self.format_with_errors.is_some() {
            formatter.format_with_errors = self.format_with_errors;
        }

        let linter = configuration
            .linter
            .get_or_insert_with(LinterConfiguration::default);

        if self.linter_enabled.is_some() {
            linter.enabled = self.linter_enabled;
        }

        let assist = configuration
            .assist
            .get_or_insert_with(AssistConfiguration::default);

        if self.assist_enabled.is_some() {
            assist.enabled = self.assist_enabled;
        }

        let css = configuration.css.get_or_insert_with(Default::default);
        if self.css_parser.is_some() {
            css.parser.merge_with(self.css_parser.clone());
        }

        let json = configuration.json.get_or_insert_with(Default::default);
        if self.json_parser.is_some() {
            json.parser.merge_with(self.json_parser.clone())
        }

        if let Some(mut conf) = self.configuration.clone() {
            if let Some(linter) = conf.linter.as_mut() {
                // Don't overwrite rules from the CLI configuration.
                // Otherwise, rules that are disabled in the config file might
                // become re-enabled due to the defaults included in the CLI
                // configuration.
                linter.rules = None;
            }
            if let Some(assist) = conf.assist.as_mut() {
                // Don't overwrite actions from the CLI configuration.
                // Otherwise, actions that are disabled in the config file might
                // become re-enabled due to the defaults included in the CLI
                // configuration.
                assist.actions = None
            }
            configuration.merge_with(conf);
        }

        Ok(configuration)
    }

    fn get_files_to_process(
        &self,
        fs: &dyn FileSystem,
        configuration: &Configuration,
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
        let fix_file_mode = determine_fix_file_mode(FixFileModeOptions {
            write: self.write,
            suppress: false,
            suppression_reason: None,
            fix: self.fix,
            unsafe_: self.unsafe_,
        })?;

        Ok(Execution::new(TraversalMode::Check {
            fix_file_mode,
            stdin: self.get_stdin(console)?,
            vcs_targeted: (self.staged, self.changed).into(),
            enforce_assist: self.enforce_assist,
            skip_parse_errors: cli_options.skip_parse_errors,
        })
        .set_report(cli_options))
    }
}
