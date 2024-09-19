use biome_analyze::{AnalyzerConfiguration, AnalyzerOptions};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_html_formatter::{format_node, HtmlFormatOptions};
use biome_html_parser::parse_html_with_cache;
use biome_html_syntax::HtmlLanguage;
use biome_parser::AnyParse;
use biome_rowan::NodeCache;

use crate::{
    settings::{ServiceLanguage, Settings, WorkspaceSettingsHandle},
    WorkspaceError,
};

use super::{
    AnalyzerCapabilities, Capabilities, DebugCapabilities, DocumentFileSource, ExtensionHandler,
    FormatterCapabilities, ParseResult, ParserCapabilities, SearchCapabilities,
};

impl ServiceLanguage for HtmlLanguage {
    type FormatterSettings = ();
    type LinterSettings = ();
    type OrganizeImportsSettings = ();
    type FormatOptions = HtmlFormatOptions;
    type ParserSettings = ();
    type EnvironmentSettings = ();

    fn lookup_settings(
        languages: &crate::settings::LanguageListSettings,
    ) -> &crate::settings::LanguageSettings<Self> {
        &languages.html
    }

    fn resolve_format_options(
        _global: Option<&crate::settings::FormatSettings>,
        _overrides: Option<&crate::settings::OverrideSettings>,
        _language: Option<&Self::FormatterSettings>,
        _path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
    ) -> Self::FormatOptions {
        // TODO: actually resolve options
        HtmlFormatOptions::default()
    }

    fn resolve_analyzer_options(
        _global: Option<&crate::settings::Settings>,
        _linter: Option<&crate::settings::LinterSettings>,
        _overrides: Option<&crate::settings::OverrideSettings>,
        _language: Option<&Self::LinterSettings>,
        path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
    ) -> AnalyzerOptions {
        AnalyzerOptions {
            configuration: AnalyzerConfiguration::default(),
            file_path: path.to_path_buf(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct HtmlFileHandler;

impl ExtensionHandler for HtmlFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: None,
                debug_control_flow: None,
                debug_formatter_ir: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: None,
                code_actions: None,
                rename: None,
                fix_all: None,
                organize_imports: None,
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: None,
                format_on_type: None,
            },
            search: SearchCapabilities { search: None },
        }
    }
}

fn parse(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: Option<&Settings>,
    cache: &mut NodeCache,
) -> ParseResult {
    let parse = parse_html_with_cache(text, cache);

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<HtmlLanguage>(biome_path, document_file_source);

    tracing::debug!("Format with the following options: \n{}", options);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}
