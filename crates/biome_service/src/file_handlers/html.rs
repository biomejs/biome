use super::{
    AnalyzerCapabilities, Capabilities, DebugCapabilities, DocumentFileSource, EnabledForPath,
    ExtensionHandler, FixAllParams, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, SearchCapabilities,
};
use crate::settings::{check_feature_activity, check_override_feature_activity};
use crate::workspace::FixFileResult;
use crate::workspace::{EmbeddedCssContent, EmbeddedJsContent};
use crate::{
    WorkspaceError,
    settings::{ServiceLanguage, Settings},
    workspace::GetSyntaxTreeResult,
};
use biome_analyze::AnalyzerOptions;
use biome_configuration::html::{HtmlFormatterConfiguration, HtmlFormatterEnabled};
use biome_css_parser::{CssParserOptions, parse_css_with_offset_and_cache};
use biome_diagnostics::{Diagnostic, Severity};
use biome_formatter::{
    AttributePosition, BracketSameLine, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed,
};
use biome_fs::BiomePath;
use biome_html_formatter::context::SelfCloseVoidElements;
use biome_html_formatter::{
    HtmlFormatOptions,
    context::{IndentScriptAndStyle, WhitespaceSensitivity},
    format_node,
};
use biome_html_parser::parse_html_with_cache;
use biome_html_syntax::{HtmlElement, HtmlLanguage, HtmlRoot, HtmlSyntaxNode};
use biome_js_parser::{JsParserOptions, parse_js_with_offset_and_cache};
use biome_js_syntax::JsFileSource;
use biome_parser::AnyParse;
use biome_rowan::{AstNode, AstNodeList, NodeCache};
use camino::Utf8Path;
use tracing::debug_span;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HtmlFormatterSettings {
    pub enabled: Option<HtmlFormatterEnabled>,
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub attribute_position: Option<AttributePosition>,
    pub bracket_same_line: Option<BracketSameLine>,
    pub whitespace_sensitivity: Option<WhitespaceSensitivity>,
    pub indent_script_and_style: Option<IndentScriptAndStyle>,
    pub self_close_void_elements: Option<SelfCloseVoidElements>,
}

impl From<HtmlFormatterConfiguration> for HtmlFormatterSettings {
    fn from(config: HtmlFormatterConfiguration) -> Self {
        Self {
            enabled: config.enabled,
            line_ending: config.line_ending,
            line_width: config.line_width,
            indent_width: config.indent_width,
            indent_style: config.indent_style,
            attribute_position: config.attribute_position,
            bracket_same_line: config.bracket_same_line,
            whitespace_sensitivity: config.whitespace_sensitivity,
            indent_script_and_style: config.indent_script_and_style,
            self_close_void_elements: config.self_close_void_elements,
        }
    }
}

impl ServiceLanguage for HtmlLanguage {
    type FormatterSettings = HtmlFormatterSettings;
    type LinterSettings = ();
    type FormatOptions = HtmlFormatOptions;
    type ParserSettings = ();
    type EnvironmentSettings = ();
    type AssistSettings = ();

    fn lookup_settings(
        languages: &crate::settings::LanguageListSettings,
    ) -> &crate::settings::LanguageSettings<Self> {
        &languages.html
    }

    fn resolve_format_options(
        global: &crate::settings::FormatSettings,
        overrides: &crate::settings::OverrideSettings,
        language: &Self::FormatterSettings,
        path: &biome_fs::BiomePath,
        file_source: &super::DocumentFileSource,
    ) -> Self::FormatOptions {
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
        let attribute_position = language
            .attribute_position
            .or(global.attribute_position)
            .unwrap_or_default();
        let bracket_same_line = language
            .bracket_same_line
            .or(global.bracket_same_line)
            .unwrap_or_default();
        let whitespace_sensitivity = language.whitespace_sensitivity.unwrap_or_default();
        let indent_script_and_style = language.indent_script_and_style.unwrap_or_default();
        let self_close_void_elements = language.self_close_void_elements.unwrap_or_default();

        let mut options =
            HtmlFormatOptions::new(file_source.to_html_file_source().unwrap_or_default())
                .with_indent_style(indent_style)
                .with_indent_width(indent_width)
                .with_line_width(line_width)
                .with_line_ending(line_ending)
                .with_attribute_position(attribute_position)
                .with_bracket_same_line(bracket_same_line)
                .with_whitespace_sensitivity(whitespace_sensitivity)
                .with_indent_script_and_style(indent_script_and_style)
                .with_self_close_void_elements(self_close_void_elements);

        overrides.apply_override_html_format_options(path, &mut options);

        options
    }

    fn resolve_analyzer_options(
        _global: &Settings,
        _language: &Self::LinterSettings,
        _environment: Option<&Self::EnvironmentSettings>,
        path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        AnalyzerOptions::default()
            .with_file_path(path.as_path())
            .with_suppression_reason(suppression_reason)
    }

    fn formatter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        let overrides_activity =
            settings
                .override_settings
                .patterns
                .iter()
                .rev()
                .find_map(|pattern| {
                    check_override_feature_activity(
                        pattern.languages.html.formatter.enabled,
                        pattern.formatter.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.html.formatter.enabled,
                settings.formatter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    fn assist_enabled_for_file_path(_settings: &Settings, _path: &Utf8Path) -> bool {
        false
    }

    fn linter_enabled_for_file_path(_settings: &Settings, _path: &Utf8Path) -> bool {
        false
    }

    fn resolve_environment(_settings: &Settings) -> Option<&Self::EnvironmentSettings> {
        None
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct HtmlFileHandler;

impl ExtensionHandler for HtmlFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            enabled_for_path: EnabledForPath {
                formatter: Some(formatter_enabled),
                linter: Some(linter_enabled),
                assist: Some(assist_enabled),
                search: Some(search_enabled),
            },
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: Some(debug_formatter_ir),
                debug_type_info: None,
                debug_registered_types: None,
                debug_semantic_model: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: None,
                rename: None,
                fix_all: Some(fix_all),
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

fn formatter_enabled(path: &Utf8Path, settings: &Settings) -> bool {
    settings.formatter_enabled_for_file_path::<HtmlLanguage>(path)
}

fn linter_enabled(path: &Utf8Path, settings: &Settings) -> bool {
    settings.linter_enabled_for_file_path::<HtmlLanguage>(path)
}

fn assist_enabled(path: &Utf8Path, settings: &Settings) -> bool {
    settings.assist_enabled_for_file_path::<HtmlLanguage>(path)
}

fn search_enabled(_path: &Utf8Path, _settings: &Settings) -> bool {
    true
}

fn parse(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    let html_file_source = file_source.to_html_file_source().unwrap_or_default();
    let parse = parse_html_with_cache(text, html_file_source, cache);

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

/// Extracts embedded JavaScript content from HTML script elements.
///
/// This function walks the HTML syntax tree to find all `<script>` elements
/// and extracts their content for offset-aware JavaScript parsing.
///
/// # Arguments
/// * `html_root` - The parsed HTML syntax tree
/// * `source_text` - The original HTML source text
/// * `cache` - Node cache for performance optimization
///
/// # Returns
/// A vector of `EmbeddedJsContent` containing parsed JavaScript with correct offsets
pub(crate) fn extract_embedded_scripts(
    html_root: &HtmlRoot,
    cache: &mut NodeCache,
) -> Vec<EmbeddedJsContent> {
    let mut scripts = Vec::new();

    // Walk through all HTML elements looking for script tags
    for element in html_root.syntax().descendants() {
        let Some(list) = extract_embedded_script(element.clone(), cache) else {
            continue;
        };
        scripts.extend(list);
    }
    scripts
}

fn extract_embedded_script(
    element: HtmlSyntaxNode,
    cache: &mut NodeCache,
) -> Option<Vec<EmbeddedJsContent>> {
    let html_element = HtmlElement::cast(element)?;
    let opening_element = html_element.opening_element().ok()?;
    let name = opening_element.name().ok()?;
    let name_text = name.value_token().ok()?;

    if name_text.text_trimmed() == "script" {
        Some(
            html_element
                .children()
                .iter()
                .filter_map(|child| child.as_any_html_content().cloned())
                .filter_map(|child| child.as_html_content().cloned())
                .filter_map(|child| {
                    let content = child.value_token().ok()?;
                    let parse = parse_js_with_offset_and_cache(
                        content.text(),
                        content.text_range().start(),
                        JsFileSource::js_script(),
                        JsParserOptions::default(),
                        cache,
                    );

                    Some(EmbeddedJsContent {
                        parse: parse.into(),
                        element_range: child.range(),
                        content_range: content.text_range(),
                        content_offset: content.text_range().start(),
                    })
                })
                .collect::<Vec<_>>(),
        )
    } else {
        None
    }
}

/// Extracts embedded CSS content from HTML style elements.
pub(crate) fn parse_embedded_styles(
    html_root: &HtmlRoot,
    cache: &mut NodeCache,
) -> Vec<EmbeddedCssContent> {
    let mut styles = Vec::new();

    // Walk through all HTML elements looking for style tags
    for element in html_root.syntax().descendants() {
        let Some(list) = parse_embedded_style(element.clone(), cache) else {
            continue;
        };
        styles.extend(list);
    }

    styles
}

fn parse_embedded_style(
    element: HtmlSyntaxNode,
    cache: &mut NodeCache,
) -> Option<Vec<EmbeddedCssContent>> {
    let html_element = HtmlElement::cast(element)?;
    let opening_element = html_element.opening_element().ok()?;
    let name = opening_element.name().ok()?;
    let name_text = name.value_token().ok()?;

    if name_text.text_trimmed() == "style" {
        Some(
            html_element
                .children()
                .iter()
                .filter_map(|child| child.as_any_html_content().cloned())
                .filter_map(|child| child.as_html_content().cloned())
                .filter_map(|child| {
                    let content = child.value_token().ok()?;
                    let parse = parse_css_with_offset_and_cache(
                        content.text(),
                        content.text_range().start(),
                        cache,
                        CssParserOptions::default(),
                    );

                    Some(EmbeddedCssContent {
                        parse: parse.into(),
                        element_range: child.range(),
                        content_range: content.text_range(),
                        content_offset: content.text_range().start(),
                    })
                })
                .collect::<Vec<_>>(),
        )
    } else {
        None
    }
}

fn debug_syntax_tree(_biome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: HtmlSyntaxNode = parse.syntax();
    let tree: HtmlRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<HtmlLanguage>(path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<HtmlLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

#[tracing::instrument(level = "debug", skip(params))]
fn lint(params: LintParams) -> LintResults {
    let _ = debug_span!("Linting HTML file", path =? params.path, language =? params.language)
        .entered();
    let diagnostics = params.parse.into_diagnostics();

    let diagnostic_count = diagnostics.len() as u32;
    let skipped_diagnostics = diagnostic_count.saturating_sub(diagnostics.len() as u32);
    let errors = diagnostics
        .iter()
        .filter(|diag| diag.severity() <= Severity::Error)
        .count();

    LintResults {
        diagnostics,
        errors,
        skipped_diagnostics,
    }
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    // We don't have analyzer rules yet
    let tree: HtmlRoot = params.parse.tree();
    let code = if params.should_format {
        format_node(
            params
                .settings
                .format_options::<HtmlLanguage>(params.biome_path, &params.document_file_source),
            tree.syntax(),
        )?
        .print()?
        .into_code()
    } else {
        tree.syntax().to_string()
    };
    Ok(FixFileResult {
        code,
        skipped_suggested_fixes: 0,
        actions: vec![],
        errors: 0,
    })
}
