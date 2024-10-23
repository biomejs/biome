use crate::{
    settings::{ServiceLanguage, Settings, WorkspaceSettingsHandle},
    WorkspaceError,
};
use biome_analyze::{AnalyzerConfiguration, AnalyzerOptions};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth, Printed};
use biome_fs::BiomePath;
use biome_grit_formatter::{context::GritFormatOptions, format_node};
use biome_grit_parser::parse_grit_with_cache;
use biome_grit_syntax::GritLanguage;
use biome_parser::AnyParse;
use biome_rowan::NodeCache;

use super::{
    AnalyzerCapabilities, Capabilities, DebugCapabilities, DocumentFileSource, ExtensionHandler,
    FormatterCapabilities, ParseResult, ParserCapabilities, SearchCapabilities,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GritFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub enabled: Option<bool>,
}

impl Default for GritFormatterSettings {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            indent_style: Default::default(),
            indent_width: Default::default(),
            line_ending: Default::default(),
            line_width: Default::default(),
        }
    }
}

impl ServiceLanguage for GritLanguage {
    type FormatterSettings = GritFormatterSettings;
    type LinterSettings = ();
    type OrganizeImportsSettings = ();
    type FormatOptions = GritFormatOptions;
    type ParserSettings = ();
    type EnvironmentSettings = ();
    fn lookup_settings(
        languages: &crate::settings::LanguageListSettings,
    ) -> &crate::settings::LanguageSettings<Self> {
        &languages.grit
    }

    fn resolve_format_options(
        global: Option<&crate::settings::FormatSettings>,
        overrides: Option<&crate::settings::OverrideSettings>,
        language: Option<&Self::FormatterSettings>,
        path: &biome_fs::BiomePath,
        file_source: &super::DocumentFileSource,
    ) -> Self::FormatOptions {
        let indent_style = language
            .and_then(|l| l.indent_style)
            .or(global.and_then(|g| g.indent_style))
            .unwrap_or_default();
        let line_width = language
            .and_then(|l| l.line_width)
            .or(global.and_then(|g| g.line_width))
            .unwrap_or_default();
        let indent_width = language
            .and_then(|l| l.indent_width)
            .or(global.and_then(|g| g.indent_width))
            .unwrap_or_default();

        let line_ending = language
            .and_then(|l| l.line_ending)
            .or(global.and_then(|g| g.line_ending))
            .unwrap_or_default();

        let options = GritFormatOptions::new(file_source.to_grit_file_source().unwrap_or_default())
            .with_indent_style(indent_style)
            .with_indent_width(indent_width)
            .with_line_width(line_width)
            .with_line_ending(line_ending);
        if let Some(overrides) = overrides {
            overrides.to_override_grit_format_options(path, options)
        } else {
            options
        }
    }

    fn resolve_analyzer_options(
        _global: Option<&crate::settings::Settings>,
        _linter: Option<&crate::settings::LinterSettings>,
        _overrides: Option<&crate::settings::OverrideSettings>,
        _language: Option<&Self::LinterSettings>,
        path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
    ) -> biome_analyze::AnalyzerOptions {
        AnalyzerOptions {
            configuration: AnalyzerConfiguration::default(),
            file_path: path.to_path_buf(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct GritFileHandler;

impl ExtensionHandler for GritFileHandler {
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
    let parse = parse_grit_with_cache(text, cache);

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
    let options = settings.format_options::<GritLanguage>(biome_path, document_file_source);

    tracing::debug!("Format with the following options: \n{}", options);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}
