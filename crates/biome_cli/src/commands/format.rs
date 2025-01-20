use crate::cli_options::CliOptions;
use crate::commands::{get_files_to_process_with_cli_options, CommandRunner, LoadEditorConfig};
use crate::{CliDiagnostic, Execution, TraversalMode};
use biome_configuration::css::CssFormatterConfiguration;
use biome_configuration::graphql::GraphqlFormatterConfiguration;
use biome_configuration::javascript::JsFormatterConfiguration;
use biome_configuration::json::JsonFormatterConfiguration;
use biome_configuration::vcs::VcsConfiguration;
use biome_configuration::{Configuration, FilesConfiguration, FormatterConfiguration};
use biome_console::Console;
use biome_deserialize::Merge;
use biome_fs::FileSystem;
use biome_service::configuration::LoadedConfiguration;
use biome_service::projects::ProjectKey;
use biome_service::{Workspace, WorkspaceError};
use std::ffi::OsString;

pub(crate) struct FormatCommandPayload {
    pub(crate) javascript_formatter: Option<JsFormatterConfiguration>,
    pub(crate) json_formatter: Option<JsonFormatterConfiguration>,
    pub(crate) css_formatter: Option<CssFormatterConfiguration>,
    pub(crate) graphql_formatter: Option<GraphqlFormatterConfiguration>,
    pub(crate) formatter_configuration: Option<FormatterConfiguration>,
    pub(crate) vcs_configuration: Option<VcsConfiguration>,
    pub(crate) files_configuration: Option<FilesConfiguration>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) paths: Vec<OsString>,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

impl LoadEditorConfig for FormatCommandPayload {
    fn should_load_editor_config(&self, fs_configuration: &Configuration) -> bool {
        self.formatter_configuration
            .as_ref()
            .is_some_and(|c| c.use_editorconfig_resolved())
            || fs_configuration.use_editorconfig()
    }
}

impl CommandRunner for FormatCommandPayload {
    const COMMAND_NAME: &'static str = "format";

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
        let editorconfig_search_path = configuration_path.clone();
        let mut fs_configuration =
            self.load_editor_config(editorconfig_search_path, &biome_configuration, fs, console)?;
        // this makes biome configuration take precedence over editorconfig configuration
        fs_configuration.merge_with(biome_configuration);
        let mut configuration = fs_configuration;

        // merge formatter options
        if configuration
            .formatter
            .as_ref()
            .is_none_or(|f| f.is_enabled())
        {
            let formatter = configuration.formatter.get_or_insert_with(Default::default);
            if let Some(formatter_configuration) = self.formatter_configuration.clone() {
                formatter.merge_with(formatter_configuration);
            }

            formatter.enabled = Some(true.into());
        }
        if self.css_formatter.is_some() {
            let css = configuration.css.get_or_insert_with(Default::default);
            css.formatter.merge_with(self.css_formatter.clone());
        }
        if self.graphql_formatter.is_some() {
            let graphql = configuration.graphql.get_or_insert_with(Default::default);
            graphql.formatter.merge_with(self.graphql_formatter.clone());
        }

        if self.javascript_formatter.is_some() {
            let javascript = configuration
                .javascript
                .get_or_insert_with(Default::default);
            javascript
                .formatter
                .merge_with(self.javascript_formatter.clone());
        }
        if self.json_formatter.is_some() {
            let json = configuration.json.get_or_insert_with(Default::default);
            json.formatter.merge_with(self.json_formatter.clone());
        }

        configuration
            .files
            .merge_with(self.files_configuration.clone());
        configuration.vcs.merge_with(self.vcs_configuration.clone());

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
        project_key: ProjectKey,
    ) -> Result<Execution, CliDiagnostic> {
        Ok(Execution::new(TraversalMode::Format {
            project_key,
            ignore_errors: cli_options.skip_errors,
            write: self.should_write(),
            stdin: self.get_stdin(console)?,
            vcs_targeted: (self.staged, self.changed).into(),
        })
        .set_report(cli_options))
    }
}
