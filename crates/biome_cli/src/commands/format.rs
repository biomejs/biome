use crate::cli_options::CliOptions;
use crate::commands::{get_files_to_process_with_cli_options, CommandRunner, LoadEditorConfig};
use crate::diagnostics::DeprecatedArgument;
use crate::{CliDiagnostic, Execution, TraversalMode};
use biome_configuration::vcs::PartialVcsConfiguration;
use biome_configuration::{
    PartialConfiguration, PartialCssFormatter, PartialFilesConfiguration,
    PartialFormatterConfiguration, PartialGraphqlFormatter, PartialJavascriptFormatter,
    PartialJsonFormatter,
};
use biome_console::{markup, Console, ConsoleExt};
use biome_deserialize::Merge;
use biome_diagnostics::PrintDiagnostic;
use biome_fs::FileSystem;
use biome_service::configuration::LoadedConfiguration;
use biome_service::{DynRef, Workspace, WorkspaceError};
use std::ffi::OsString;

pub(crate) struct FormatCommandPayload {
    pub(crate) javascript_formatter: Option<PartialJavascriptFormatter>,
    pub(crate) json_formatter: Option<PartialJsonFormatter>,
    pub(crate) css_formatter: Option<PartialCssFormatter>,
    pub(crate) graphql_formatter: Option<PartialGraphqlFormatter>,
    pub(crate) formatter_configuration: Option<PartialFormatterConfiguration>,
    pub(crate) vcs_configuration: Option<PartialVcsConfiguration>,
    pub(crate) files_configuration: Option<PartialFilesConfiguration>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) paths: Vec<OsString>,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

impl LoadEditorConfig for FormatCommandPayload {
    fn should_load_editor_config(&self, fs_configuration: &PartialConfiguration) -> bool {
        self.formatter_configuration
            .as_ref()
            .and_then(|c| c.use_editorconfig)
            .unwrap_or(fs_configuration.use_editorconfig().unwrap_or_default())
    }
}

impl CommandRunner for FormatCommandPayload {
    const COMMAND_NAME: &'static str = "format";

    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        fs: &DynRef<'_, dyn FileSystem>,
        console: &mut dyn Console,
    ) -> Result<PartialConfiguration, WorkspaceError> {
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

        // TODO: remove in biome 2.0
        if let Some(config) = self.formatter_configuration.as_mut() {
            if let Some(indent_size) = config.indent_size {
                let diagnostic = DeprecatedArgument::new(markup! {
                    "The argument "<Emphasis>"--indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--indent-width"</Emphasis>" instead."
                });
                console.error(markup! {
                    {PrintDiagnostic::simple(&diagnostic)}
                });

                if config.indent_width.is_none() {
                    config.indent_width = Some(indent_size);
                }
            }
        }
        // TODO: remove in biome 2.0
        if let Some(js_formatter) = self.javascript_formatter.as_mut() {
            if let Some(indent_size) = js_formatter.indent_size {
                let diagnostic = DeprecatedArgument::new(markup! {
                    "The argument "<Emphasis>"--javascript-formatter-indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--javascript-formatter-indent-width"</Emphasis>" instead."
                });
                console.error(markup! {
                    {PrintDiagnostic::simple(&diagnostic)}
                });

                if js_formatter.indent_width.is_none() {
                    js_formatter.indent_width = Some(indent_size);
                }
            }

            if let Some(trailing_comma) = js_formatter.trailing_comma {
                let diagnostic = DeprecatedArgument::new(markup! {
                    "The argument "<Emphasis>"--trailing-comma"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--trailing-commas"</Emphasis>" instead."
                });
                console.error(markup! {
                    {PrintDiagnostic::simple(&diagnostic)}
                });

                if js_formatter.trailing_commas.is_none() {
                    js_formatter.trailing_commas = Some(trailing_comma);
                }
            }
        }
        // TODO: remove in biome 2.0
        if let Some(json_formatter) = self.json_formatter.as_mut() {
            if let Some(indent_size) = json_formatter.indent_size {
                let diagnostic = DeprecatedArgument::new(markup! {
                    "The argument "<Emphasis>"--json-formatter-indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--json-formatter-indent-width"</Emphasis>" instead."
                });
                console.error(markup! {
                    {PrintDiagnostic::simple(&diagnostic)}
                });

                if json_formatter.indent_width.is_none() {
                    json_formatter.indent_width = Some(indent_size);
                }
            }
        }

        // merge formatter options
        if !configuration
            .formatter
            .as_ref()
            .is_some_and(PartialFormatterConfiguration::is_disabled)
        {
            let formatter = configuration.formatter.get_or_insert_with(Default::default);
            if let Some(formatter_configuration) = self.formatter_configuration.clone() {
                formatter.merge_with(formatter_configuration);
            }

            formatter.enabled = Some(true);
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
        Ok(Execution::new(TraversalMode::Format {
            ignore_errors: cli_options.skip_errors,
            write: self.should_write(),
            stdin: self.get_stdin(console)?,
            vcs_targeted: (self.staged, self.changed).into(),
        })
        .set_report(cli_options))
    }
}
