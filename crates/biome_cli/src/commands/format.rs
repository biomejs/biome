use crate::CliDiagnostic;
use crate::cli_options::CliOptions;
use crate::commands::get_files_to_process_with_cli_options;
use crate::diagnostics::StdinDiagnostic;
use crate::runner::crawler::CrawlerContext;
use crate::runner::execution::Execution;
use crate::runner::process_file::{FileStatus, Message, ProcessFile, ProcessStdinFilePayload};
use crate::runner::runner_ext::{LoadEditorConfig, TraversalCommand};
use biome_configuration::css::{CssFormatterConfiguration, CssParserConfiguration};
use biome_configuration::graphql::GraphqlFormatterConfiguration;
use biome_configuration::html::HtmlFormatterConfiguration;
use biome_configuration::javascript::JsFormatterConfiguration;
use biome_configuration::json::{JsonFormatterConfiguration, JsonParserConfiguration};
use biome_configuration::vcs::VcsConfiguration;
use biome_configuration::{Configuration, FilesConfiguration, FormatterConfiguration};
use biome_console::{Console, ConsoleExt, markup};
use biome_deserialize::Merge;
use biome_diagnostics::{Category, Diagnostic, PrintDiagnostic, category};
use biome_fs::{BiomePath, FileSystem};
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use biome_service::workspace::{
    CloseFileParams, FeatureKind, FeatureName, FeaturesBuilder, FeaturesSupported, FileContent,
    FileFeaturesResult, FormatFileParams, OpenFileParams, ScanKind, SupportKind,
    SupportsFeatureParams,
};
use biome_service::{Workspace, WorkspaceError};
use camino::Utf8PathBuf;
use std::ffi::OsString;

pub(crate) struct FormatCommandPayload {
    pub(crate) javascript_formatter: Option<JsFormatterConfiguration>,
    pub(crate) json_formatter: Option<JsonFormatterConfiguration>,
    pub(crate) css_formatter: Option<CssFormatterConfiguration>,
    pub(crate) graphql_formatter: Option<GraphqlFormatterConfiguration>,
    pub(crate) html_formatter: Option<HtmlFormatterConfiguration>,
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
    pub(crate) json_parser: Option<JsonParserConfiguration>,
    pub(crate) css_parser: Option<CssParserConfiguration>,
}

struct FormatExecution {
    stdin_file_path: Option<String>,
}

impl Execution for FormatExecution {
    fn to_feature(&self) -> FeatureName {
        FeaturesBuilder::new().with_formatter().build()
    }

    fn can_handle(&self, features: FeaturesSupported) -> bool {
        features.supports_format()
    }

    fn supports_kind(&self, file_features: &FeaturesSupported) -> Option<SupportKind> {
        Some(file_features.support_kind_for(FeatureKind::Format))
    }

    fn as_diagnostic_category(&self) -> &'static Category {
        category!("format")
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        self.stdin_file_path.as_deref()
    }
}

impl LoadEditorConfig for FormatCommandPayload {
    fn should_load_editor_config(&self, fs_configuration: &Configuration) -> bool {
        self.formatter_configuration
            .as_ref()
            .is_some_and(|c| c.use_editorconfig_resolved())
            || fs_configuration.use_editorconfig()
    }
}

impl TraversalCommand for FormatCommandPayload {
    type ProcessFile = FormatProcessFile;

    fn command_name() -> &'static str {
        "format"
    }

    fn scan_kind() -> ScanKind {
        ScanKind::KnownFiles
    }

    fn get_execution(
        &self,
        _cli_options: &CliOptions,
        _console: &mut dyn Console,
        _workspace: &dyn Workspace,
    ) -> Result<Box<dyn Execution>, CliDiagnostic> {
        Ok(Box::new(FormatExecution {
            stdin_file_path: self.stdin_file_path.clone(),
        }))
    }

    fn merge_configuration(
        &mut self,
        loaded_configuration: Configuration,
        loaded_directory: Option<Utf8PathBuf>,
        _loaded_file: Option<Utf8PathBuf>,

        fs: &dyn FileSystem,
        _console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
        let mut configuration =
            self.combine_configuration(loaded_directory, loaded_configuration, fs)?;

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
        let css = configuration.css.get_or_insert_with(Default::default);
        if self.css_formatter.is_some() {
            css.formatter.merge_with(self.css_formatter.clone());
        }

        if self.css_parser.is_some() {
            css.parser.merge_with(self.css_parser.clone());
        }

        if self.graphql_formatter.is_some() {
            let graphql = configuration.graphql.get_or_insert_with(Default::default);
            graphql.formatter.merge_with(self.graphql_formatter.clone());
        }
        if self.html_formatter.is_some() {
            let html = configuration.html.get_or_insert_with(Default::default);
            html.formatter.merge_with(self.html_formatter.clone());
        }

        if self.javascript_formatter.is_some() {
            let javascript = configuration
                .javascript
                .get_or_insert_with(Default::default);
            javascript
                .formatter
                .merge_with(self.javascript_formatter.clone());
        }
        let json = configuration.json.get_or_insert_with(Default::default);

        if self.json_formatter.is_some() {
            json.formatter.merge_with(self.json_formatter.clone());
        }
        if self.json_parser.is_some() {
            json.parser.merge_with(self.json_parser.clone())
        }

        configuration
            .files
            .merge_with(self.files_configuration.clone());
        configuration.vcs.merge_with(self.vcs_configuration.clone());

        Ok(configuration)
    }

    fn should_write(&self) -> bool {
        self.write || self.fix
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
}

pub struct FormatProcessFile;

impl ProcessFile for FormatProcessFile {
    fn process_file<Ctx>(ctx: &Ctx, path: &BiomePath) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext,
    {
        todo!()
    }

    fn process_std_in(payload: ProcessStdinFilePayload) -> Result<(), CliDiagnostic> {
        let ProcessStdinFilePayload {
            workspace,
            fs,
            content,
            project_key,
            biome_path,
            console,
            cli_options,
        } = payload;
        let FileFeaturesResult {
            features_supported: file_features,
        } = workspace.file_features(SupportsFeatureParams {
            project_key,
            path: biome_path.clone(),
            features: FeaturesBuilder::new().with_formatter().build(),
            inline_config: None,
        })?;

        if file_features.is_ignored() {
            console.append(markup! {{content}});
            return Ok(());
        }

        if file_features.is_protected() {
            let protected_diagnostic = WorkspaceError::protected_file(biome_path.to_string());
            if protected_diagnostic.tags().is_verbose() {
                if cli_options.verbose {
                    console.error(markup! {{PrintDiagnostic::verbose(&protected_diagnostic)}})
                }
            } else {
                console.error(markup! {{PrintDiagnostic::simple(&protected_diagnostic)}})
            }
            console.append(markup! {{content}});
            return Ok(());
        };
        if file_features.supports_format() {
            workspace.open_file(OpenFileParams {
                project_key,
                path: biome_path.clone(),
                content: FileContent::from_client(content),
                document_file_source: None,
                persist_node_cache: false,
                inline_config: None,
            })?;
            let printed = workspace.format_file(FormatFileParams {
                project_key,
                path: biome_path.clone(),
                inline_config: None,
            })?;

            let code = printed.into_code();
            let output = if !file_features.supports_full_html_support() {
                match biome_path.extension() {
                    Some("astro") => AstroFileHandler::output(content, code.as_str()),
                    Some("vue") => VueFileHandler::output(content, code.as_str()),
                    Some("svelte") => SvelteFileHandler::output(content, code.as_str()),
                    _ => code,
                }
            } else {
                code
            };
            console.append(markup! {
                {output}
            });
            workspace
                .close_file(CloseFileParams {
                    project_key,
                    path: biome_path.clone(),
                })
                .map_err(|err| err.into())
        } else {
            console.append(markup! {
                {content}
            });
            console.error(markup! {
                <Warn>"The content was not formatted because the formatter is currently disabled."</Warn>
            });
            Err(StdinDiagnostic::new_not_formatted().into())
        }
    }
}
