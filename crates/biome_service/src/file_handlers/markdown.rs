use super::{
    Capabilities, DebugCapabilities, DocumentFileSource, EnabledForPath, ExtensionHandler,
    FormatterCapabilities, ParseResult, ParserCapabilities, SearchCapabilities,
};
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, OverrideSettings, ServiceLanguage,
    Settings, SettingsWithEditor, check_feature_activity,
};
use crate::workspace::GetSyntaxTreeResult;
use biome_analyze::AnalyzerOptions;
use biome_configuration::analyzer::assist::AssistEnabled;
use biome_configuration::analyzer::linter::LinterEnabled;
use biome_configuration::formatter::FormatterEnabled;
use biome_formatter::{
    IndentStyle, IndentWidth, LineEnding, LineWidth, SimpleFormatOptions, TrailingNewline,
};
use biome_fs::BiomePath;
use biome_markdown_parser::{MarkdownParseOptions, parse_markdown_with_cache};
use biome_markdown_syntax::{MarkdownLanguage, MarkdownSyntaxNode, MdDocument};
use biome_parser::{AnyParse, NodeParse};
use biome_rowan::NodeCache;
use camino::Utf8Path;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MarkdownFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub trailing_newline: Option<TrailingNewline>,
    pub enabled: Option<FormatterEnabled>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MarkdownLinterSettings {
    pub enabled: Option<LinterEnabled>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MarkdownAssistSettings {
    pub enabled: Option<AssistEnabled>,
}

impl ServiceLanguage for MarkdownLanguage {
    type FormatterSettings = MarkdownFormatterSettings;
    type LinterSettings = MarkdownLinterSettings;
    type AssistSettings = MarkdownAssistSettings;
    type FormatOptions = SimpleFormatOptions;
    type ParserSettings = ();
    type ParserOptions = MarkdownParseOptions;
    type EnvironmentSettings = ();

    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.markdown
    }

    fn resolve_environment(_settings: &Settings) -> Option<&Self::EnvironmentSettings> {
        None
    }

    fn resolve_parse_options(
        _overrides: &OverrideSettings,
        _language: &Self::ParserSettings,
        _path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> Self::ParserOptions {
        MarkdownParseOptions::default()
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &BiomePath,
        _document_file_source: &DocumentFileSource,
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
        SimpleFormatOptions {
            indent_style,
            indent_width,
            line_width,
            line_ending,
            trailing_newline,
        }
    }

    fn resolve_analyzer_options(
        _global: &Settings,
        _language: &Self::LinterSettings,
        _environment: Option<&Self::EnvironmentSettings>,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        AnalyzerOptions::default()
            .with_file_path(path.as_path())
            .with_suppression_reason(suppression_reason)
    }

    fn linter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        // TODO: evaluate markdown override patterns once markdown override settings are introduced.
        let _ = path;

        check_feature_activity(
            settings.languages.markdown.linter.enabled,
            settings.linter.enabled,
        )
        .unwrap_or_default()
        .into()
    }

    fn formatter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        // TODO: evaluate markdown override patterns once markdown override settings are introduced.
        let _ = path;

        check_feature_activity(
            settings.languages.markdown.formatter.enabled,
            settings.formatter.enabled,
        )
        .unwrap_or_default()
        .into()
    }

    fn assist_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        // TODO: evaluate markdown override patterns once markdown override settings are introduced.
        let _ = path;

        check_feature_activity(
            settings.languages.markdown.assist.enabled,
            settings.assist.enabled,
        )
        .unwrap_or_default()
        .into()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct MarkdownFileHandler;

impl ExtensionHandler for MarkdownFileHandler {
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
                debug_formatter_ir: None,
                debug_type_info: None,
                debug_registered_types: None,
                debug_semantic_model: None,
            },
            analyzer: Default::default(),
            formatter: FormatterCapabilities {
                format: None,
                format_range: None,
                format_on_type: None,
                format_embedded: None,
            },
            search: SearchCapabilities { search: None },
        }
    }
}

fn formatter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.formatter_enabled_for_file_path::<MarkdownLanguage>(path)
}

fn linter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.linter_enabled_for_file_path::<MarkdownLanguage>(path)
}

fn assist_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.assist_enabled_for_file_path::<MarkdownLanguage>(path)
}

fn parse(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    settings: &SettingsWithEditor,
    cache: &mut NodeCache,
) -> ParseResult {
    let options = settings.parse_options::<MarkdownLanguage>(_biome_path, &file_source);
    let parse = parse_markdown_with_cache(text, cache, options);
    let any_parse =
        NodeParse::new(parse.syntax().as_send().unwrap(), parse.into_diagnostics()).into();

    ParseResult {
        any_parse,
        language: Some(file_source),
    }
}

fn debug_syntax_tree(_biome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: MarkdownSyntaxNode = parse.syntax();
    let tree: MdDocument = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}
