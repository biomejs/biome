use crate::WorkspaceError;
use crate::file_handlers::{
    Capabilities, DebugCapabilities, DocumentFileSource, EnabledForPath, ExtensionHandler,
    FormatterCapabilities, ParseResult, ParserCapabilities, SearchCapabilities,
};
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, OverrideSettings, ServiceLanguage,
    Settings, SettingsWithEditor,
};
use crate::workspace::GetSyntaxTreeResult;
use biome_analyze::AnalyzerOptions;
use biome_configuration::markdown::MarkdownFormatterEnabled;
use biome_configuration::yaml::YamlFormatterConfiguration;
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth, Printed, TrailingNewline};
use biome_fs::BiomePath;
use biome_parser::{AnyParse, NodeParse};
use biome_rowan::NodeCache;
use biome_yaml_formatter::{YamlFormatOptions, format_node};
use biome_yaml_parser::parse_yaml_with_cache;
use biome_yaml_syntax::{YamlLanguage, YamlRoot, YamlSyntaxNode};
use camino::Utf8Path;
use tracing::{debug, error};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct YamlFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub trailing_newline: Option<TrailingNewline>,
    pub enabled: Option<MarkdownFormatterEnabled>,
}

impl From<YamlFormatterConfiguration> for YamlFormatterSettings {
    fn from(configuration: YamlFormatterConfiguration) -> Self {
        Self {
            line_ending: configuration.line_ending,
            line_width: configuration.line_width,
            indent_width: configuration.indent_width,
            indent_style: configuration.indent_style,
            enabled: configuration.enabled,
            trailing_newline: configuration.trailing_newline,
        }
    }
}

impl ServiceLanguage for YamlLanguage {
    type FormatterSettings = YamlFormatterSettings;
    type LinterSettings = ();
    type AssistSettings = ();
    type FormatOptions = YamlFormatOptions;
    type ParserSettings = ();
    type ParserOptions = ();
    type EnvironmentSettings = ();

    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self> {
        &languages.yaml
    }

    fn resolve_environment(_settings: &Settings) -> Option<&Self::EnvironmentSettings> {
        todo!()
    }

    fn resolve_parse_options(
        _overrides: &OverrideSettings,
        _language: &Self::ParserSettings,
        _path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> Self::ParserOptions {
        todo!()
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> Self::FormatOptions {
        // TODO: apply markdown overrides once markdown override settings are introduced.
        let _ = (overrides, path);

        let indent_style = language
            .indent_style
            .or(global.indent_style)
            .unwrap_or_default();
        let line_width = language
            .line_width
            .or(global.line_width)
            .unwrap_or_default();
        let indent_width = language
            .indent_width
            .or(global.indent_width)
            .unwrap_or_default();
        let line_ending = language
            .line_ending
            .or(global.line_ending)
            .unwrap_or_default();
        let trailing_newline = language
            .trailing_newline
            .or(global.trailing_newline)
            .unwrap_or_default();
        YamlFormatOptions::new()
            .with_indent_style(indent_style)
            .with_indent_width(indent_width)
            .with_line_width(line_width)
            .with_line_ending(line_ending)
            .with_trailing_newline(trailing_newline)
    }

    fn resolve_analyzer_options(
        _global: &Settings,
        _language: &Self::LinterSettings,
        _environment: Option<&Self::EnvironmentSettings>,
        _path: &BiomePath,
        _file_source: &DocumentFileSource,
        _suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        todo!()
    }

    fn linter_enabled_for_file_path(_settings: &Settings, _path: &Utf8Path) -> bool {
        todo!()
    }

    fn formatter_enabled_for_file_path(_settings: &Settings, _path: &Utf8Path) -> bool {
        todo!()
    }

    fn assist_enabled_for_file_path(_settings: &Settings, _path: &Utf8Path) -> bool {
        todo!()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct YamlFileHandler;

impl ExtensionHandler for YamlFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            enabled_for_path: EnabledForPath {
                formatter: Some(formatter_enabled),
                linter: Some(linter_enabled),
                assist: Some(assist_enabled),
                search: None,
            },
            parser: ParserCapabilities {
                parse: Some(parse),
                parse_embedded_nodes: None,
            },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: Some(debug_formatter_ir),
                debug_type_info: None,
                debug_registered_types: None,
                debug_semantic_model: None,
            },
            analyzer: Default::default(),
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: None,
                format_on_type: None,
                format_embedded: None,
            },
            search: SearchCapabilities { search: None },
        }
    }
}

fn formatter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.formatter_enabled_for_file_path::<YamlLanguage>(path)
}

fn linter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.linter_enabled_for_file_path::<YamlLanguage>(path)
}

fn assist_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.assist_enabled_for_file_path::<YamlLanguage>(path)
}

fn parse(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: &SettingsWithEditor,
    cache: &mut NodeCache,
) -> ParseResult {
    let parse = parse_yaml_with_cache(text, cache);
    let any_parse =
        NodeParse::new(parse.syntax().as_send().unwrap(), parse.into_diagnostics()).into();

    ParseResult {
        any_parse,
        language: Some(file_source),
    }
}

fn debug_syntax_tree(_biome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: YamlSyntaxNode = parse.syntax();
    let tree: YamlRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<YamlLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

pub(crate) fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<YamlLanguage>(biome_path, document_file_source);
    debug!("{:?}", &options);
    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;
    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => {
            error!("The file {} couldn't be formatted", biome_path.as_str());
            Err(WorkspaceError::FormatError(error.into()))
        }
    }
}
