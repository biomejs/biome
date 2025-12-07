use super::{
    AnalyzerCapabilities, AnalyzerVisitorBuilder, Capabilities, CodeActionsParams,
    DebugCapabilities, DocumentFileSource, EnabledForPath, ExtensionHandler, FixAllParams,
    FormatEmbedNode, FormatterCapabilities, LintParams, LintResults, ParseEmbedResult, ParseResult,
    ParserCapabilities, ProcessFixAll, ProcessLint, SearchCapabilities, UpdateSnippetsNodes,
};
use crate::configuration::to_analyzer_rules;
use crate::settings::{OverrideSettings, check_feature_activity, check_override_feature_activity};
use crate::workspace::{CodeAction, CssDocumentServices, DocumentServices, EmbeddedSnippet};
use crate::workspace::{FixFileResult, PullActionsResult};
use crate::{
    WorkspaceError,
    settings::{ServiceLanguage, Settings},
    workspace::GetSyntaxTreeResult,
};
use biome_analyze::{AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never};
use biome_configuration::html::{
    HtmlAssistConfiguration, HtmlAssistEnabled, HtmlFormatterConfiguration, HtmlFormatterEnabled,
    HtmlLinterConfiguration, HtmlLinterEnabled, HtmlParseInterpolation, HtmlParserConfiguration,
};
use biome_css_parser::parse_css_with_offset_and_cache;
use biome_css_syntax::{CssFileSource, CssLanguage};
use biome_formatter::format_element::{Interned, LineMode};
use biome_formatter::prelude::{Document, Tag};
use biome_formatter::{
    AttributePosition, BracketSameLine, FormatElement, IndentStyle, IndentWidth, LineEnding,
    LineWidth, Printed,
};
use biome_fs::BiomePath;
use biome_html_analyze::analyze;
use biome_html_factory::make::ident;
use biome_html_formatter::context::SelfCloseVoidElements;
use biome_html_formatter::{
    HtmlFormatOptions,
    context::{IndentScriptAndStyle, WhitespaceSensitivity},
    format_node,
};
use biome_html_parser::{HtmlParseOptions, parse_html_with_cache};
use biome_html_syntax::element_ext::AnyEmbeddedContent;
use biome_html_syntax::{
    AstroEmbeddedContent, HtmlElement, HtmlLanguage, HtmlRoot, HtmlSyntaxNode,
};
use biome_js_parser::parse_js_with_offset_and_cache;
use biome_js_syntax::{EmbeddingKind, JsFileSource, JsLanguage};
use biome_json_parser::parse_json_with_offset_and_cache;
use biome_json_syntax::{JsonFileSource, JsonLanguage};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, AstNodeList, BatchMutation, NodeCache, SendNode};
use camino::Utf8Path;
use either::Either;
use std::borrow::Cow;
use std::fmt::Debug;
use tracing::{debug_span, error, instrument, trace_span};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HtmlParserSettings {
    pub interpolation: Option<HtmlParseInterpolation>,
}

impl From<HtmlParserConfiguration> for HtmlParserSettings {
    fn from(configuration: HtmlParserConfiguration) -> Self {
        Self {
            interpolation: configuration.interpolation,
        }
    }
}

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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HtmlLinterSettings {
    pub enabled: Option<HtmlLinterEnabled>,
}

impl From<HtmlLinterConfiguration> for HtmlLinterSettings {
    fn from(configuration: HtmlLinterConfiguration) -> Self {
        Self {
            enabled: configuration.enabled,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HtmlAssistSettings {
    pub enabled: Option<HtmlAssistEnabled>,
}

impl From<HtmlAssistConfiguration> for HtmlAssistSettings {
    fn from(configuration: HtmlAssistConfiguration) -> Self {
        Self {
            enabled: configuration.enabled,
        }
    }
}

impl ServiceLanguage for HtmlLanguage {
    type FormatterSettings = HtmlFormatterSettings;
    type LinterSettings = HtmlLinterSettings;
    type FormatOptions = HtmlFormatOptions;
    type ParserSettings = HtmlParserSettings;
    type EnvironmentSettings = ();
    type AssistSettings = HtmlAssistSettings;

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
        global: &Settings,
        _language: &Self::LinterSettings,
        _environment: Option<&Self::EnvironmentSettings>,
        path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        let configuration =
            AnalyzerConfiguration::default().with_rules(to_analyzer_rules(global, path.as_path()));

        AnalyzerOptions::default()
            .with_file_path(path.as_path())
            .with_configuration(configuration)
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

    fn assist_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        let overrides_activity =
            settings
                .override_settings
                .patterns
                .iter()
                .rev()
                .find_map(|pattern| {
                    check_override_feature_activity(
                        pattern.languages.html.assist.enabled,
                        pattern.assist.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.html.assist.enabled,
                settings.assist.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    fn linter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        let overrides_activity =
            settings
                .override_settings
                .patterns
                .iter()
                .rev()
                .find_map(|pattern| {
                    check_override_feature_activity(
                        pattern.languages.html.linter.enabled,
                        pattern.linter.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });
        overrides_activity
            .or(check_feature_activity(
                settings.languages.html.linter.enabled,
                settings.linter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    fn resolve_environment(_settings: &Settings) -> Option<&Self::EnvironmentSettings> {
        None
    }

    type ParserOptions = HtmlParseOptions;

    fn resolve_parse_options(
        overrides: &OverrideSettings,
        language: &Self::ParserSettings,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> Self::ParserOptions {
        let html_file_source = file_source.to_html_file_source().unwrap_or_default();
        let mut options = HtmlParseOptions::from(&html_file_source);
        if language.interpolation.unwrap_or_default().into() && html_file_source.is_html() {
            options = options.with_double_text_expression();
        }

        overrides.apply_override_html_parser_options(path, &mut options);

        options
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
            parser: ParserCapabilities {
                parse: Some(parse),
                parse_embedded_nodes: Some(parse_embedded_nodes),
            },
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
                code_actions: Some(code_actions),
                rename: None,
                fix_all: Some(fix_all),
                update_snippets: Some(update_snippets),
                pull_diagnostics_and_actions: None,
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: None,
                format_on_type: None,
                format_embedded: Some(format_embedded),
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
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    let options = settings.parse_options::<HtmlLanguage>(biome_path, &file_source);
    let parse = parse_html_with_cache(text, cache, options);

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

fn parse_embedded_nodes(
    root: &AnyParse,
    biome_path: &BiomePath,
    file_source: &DocumentFileSource,
    settings: &Settings,
    cache: &mut NodeCache,
) -> ParseEmbedResult {
    let mut nodes = Vec::new();
    let html_root: HtmlRoot = root.tree();

    // Walk through all HTML elements looking for script tags and style tags
    for element in html_root.syntax().descendants() {
        if let Some(astro_embedded_content) = AstroEmbeddedContent::cast_ref(&element) {
            let result = parse_astro_embedded_script(
                astro_embedded_content.clone(),
                cache,
                biome_path,
                settings,
            );
            if let Some((content, file_source)) = result {
                nodes.push((content.into(), file_source));
            }
        }

        let Some(element) = HtmlElement::cast_ref(&element) else {
            continue;
        };

        if let Some(script_type) = element.get_script_type() {
            if script_type.is_javascript() {
                let result = parse_embedded_script(
                    element.clone(),
                    cache,
                    biome_path,
                    file_source,
                    settings,
                );
                if let Some((content, file_source)) = result {
                    nodes.push((content.into(), file_source));
                }
            } else if script_type.is_json() {
                let result = parse_embedded_json(element.clone(), cache, biome_path, settings);
                if let Some((content, file_source)) = result {
                    nodes.push((content.into(), file_source));
                }
            }
        } else if element.is_style_tag() {
            let result = parse_embedded_style(element.clone(), cache, biome_path, settings);
            if let Some((content, services, file_source)) = result {
                nodes.push(((content, services).into(), file_source));
            }
        }
    }
    ParseEmbedResult { nodes }
}

pub(crate) fn parse_astro_embedded_script(
    element: AstroEmbeddedContent,
    cache: &mut NodeCache,
    path: &BiomePath,
    settings: &Settings,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    let content = element.content_token()?;
    let file_source =
        JsFileSource::ts().with_embedding_kind(EmbeddingKind::Astro { frontmatter: true });
    let document_file_source = DocumentFileSource::Js(file_source);
    let options = settings.parse_options::<JsLanguage>(path, &document_file_source);
    let parse = parse_js_with_offset_and_cache(
        content.text(),
        content.text_range().start(),
        file_source,
        options,
        cache,
    );

    Some((
        EmbeddedSnippet::new(
            parse.into(),
            element.range(),
            content.text_trimmed_range(),
            content.text_range().start(),
        ),
        document_file_source,
    ))
}

pub(crate) fn parse_embedded_script(
    element: HtmlElement,
    cache: &mut NodeCache,
    path: &BiomePath,
    html_file_source: &DocumentFileSource,
    settings: &Settings,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    let html_file_source = html_file_source.to_html_file_source()?;
    if element.is_javascript_tag() {
        let file_source = if html_file_source.is_svelte() || html_file_source.is_vue() {
            let mut file_source = if element.is_typescript_lang() {
                JsFileSource::ts()
            } else if element.is_jsx_lang() {
                JsFileSource::jsx()
            } else if element.is_tsx_lang() {
                JsFileSource::tsx()
            } else {
                JsFileSource::js_module()
            };
            if html_file_source.is_svelte() {
                file_source = file_source.with_embedding_kind(EmbeddingKind::Svelte);
            } else if html_file_source.is_vue() {
                file_source = file_source.with_embedding_kind(EmbeddingKind::Vue);
            }
            file_source
        } else if html_file_source.is_astro() {
            JsFileSource::ts().with_embedding_kind(EmbeddingKind::Astro { frontmatter: false })
        } else {
            let is_module = element.is_javascript_module().unwrap_or_default();
            if is_module {
                JsFileSource::js_module()
            } else {
                JsFileSource::js_script()
            }
        };

        let document_file_source = DocumentFileSource::Js(file_source);

        // This is likely an error
        if element.children().len() > 1 {
            return None;
        }

        let embedded_content = element.children().iter().next().and_then(|child| {
            let child = child.as_any_html_content()?;
            let child = child.as_html_embedded_content()?;
            let content = child.value_token().ok()?;
            let options = settings.parse_options::<JsLanguage>(path, &document_file_source);
            let parse = parse_js_with_offset_and_cache(
                content.text(),
                content.text_range().start(),
                file_source,
                options,
                cache,
            );

            Some(EmbeddedSnippet::new(
                parse.into(),
                child.range(),
                content.text_range(),
                content.text_range().start(),
            ))
        })?;
        Some((embedded_content, file_source.into()))
    } else {
        None
    }
}

/// Parses embedded style, but it skips it if it contains SASS language
pub(crate) fn parse_embedded_style(
    element: HtmlElement,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &Settings,
) -> Option<(
    EmbeddedSnippet<CssLanguage>,
    DocumentServices,
    DocumentFileSource,
)> {
    if element.is_style_tag() {
        // This is probably an error
        if element.children().len() > 1 {
            return None;
        }

        // We don't support SASS
        if element.is_sass_lang() {
            return None;
        }

        let file_source = DocumentFileSource::Css(CssFileSource::css());
        let (snippet, services) = element.children().iter().next().and_then(|child| {
            let child = child.as_any_html_content()?;
            let child = child.as_html_embedded_content()?;
            let options = settings.parse_options::<CssLanguage>(biome_path, &file_source);
            let content = child.value_token().ok()?;
            let parse = parse_css_with_offset_and_cache(
                content.text(),
                content.text_range().start(),
                cache,
                options,
            );

            let mut services = CssDocumentServices::default();
            if settings.is_linter_enabled() || settings.is_assist_enabled() {
                services = services.with_css_semantic_model(&parse.tree())
            }
            Some((
                EmbeddedSnippet::new(
                    parse.into(),
                    child.range(),
                    content.text_range(),
                    content.text_range().start(),
                ),
                services.into(),
            ))
        })?;
        Some((snippet, services, file_source))
    } else {
        None
    }
}

pub(crate) fn parse_embedded_json(
    element: HtmlElement,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &Settings,
) -> Option<(EmbeddedSnippet<JsonLanguage>, DocumentFileSource)> {
    // This is probably an error
    if element.children().len() > 1 {
        return None;
    }

    let file_source = DocumentFileSource::Json(JsonFileSource::json());
    let script_children = element.children().iter().next().and_then(|child| {
        let child = child.as_any_html_content()?;
        let child = child.as_html_embedded_content()?;
        let content = child.value_token().ok()?;
        let options = settings.parse_options::<JsonLanguage>(biome_path, &file_source);
        let parse = parse_json_with_offset_and_cache(
            content.text(),
            content.text_range().start(),
            cache,
            options,
        );

        Some(EmbeddedSnippet::new(
            parse.into(),
            child.range(),
            content.text_range(),
            content.text_range().start(),
        ))
    })?;
    Some((script_children, file_source))
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
    let formatted = format_node(options, &tree, false)?;

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
    let formatted = format_node(options, &tree, true)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

fn format_embedded(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
    embedded_nodes: Vec<FormatEmbedNode>,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<HtmlLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let indent_script_and_style = options.indent_script_and_style().value();
    let mut formatted = format_node(options, &tree, true)?;
    formatted.format_embedded(move |range| {
        let mut iter = embedded_nodes.iter();
        let node = iter.find(|node| node.range == range)?;

        let wrap_document = |document: Document, should_indent: bool| {
            if indent_script_and_style && should_indent {
                let elements = vec![
                    FormatElement::Line(LineMode::Hard),
                    FormatElement::Tag(Tag::StartIndent),
                    FormatElement::Line(LineMode::Hard),
                    FormatElement::Interned(Interned::new(document.into_elements())),
                    FormatElement::Tag(Tag::EndIndent),
                ];

                Document::new(elements)
            } else {
                let elements = vec![
                    FormatElement::Line(LineMode::Hard),
                    FormatElement::Interned(Interned::new(document.into_elements())),
                ];
                Document::new(elements)
            }
        };

        match node.source {
            DocumentFileSource::Js(file_source) => {
                let js_options = settings.format_options::<JsLanguage>(biome_path, &node.source);
                let node = node.node.clone().embedded_syntax::<JsLanguage>().clone();
                let formatted =
                    biome_js_formatter::format_node_with_offset(js_options, &node).ok()?;

                Some(wrap_document(
                    formatted.into_document(),
                    !file_source.as_embedding_kind().is_astro_frontmatter(),
                ))
            }
            DocumentFileSource::Json(_) => {
                let json_options =
                    settings.format_options::<JsonLanguage>(biome_path, &node.source);
                let node = node.node.clone().embedded_syntax::<JsonLanguage>().clone();
                let formatted =
                    biome_json_formatter::format_node_with_offset(json_options, &node).ok()?;
                Some(wrap_document(formatted.into_document(), true))
            }
            DocumentFileSource::Css(_) => {
                let css_options = settings.format_options::<CssLanguage>(biome_path, &node.source);
                let node = node.node.clone().embedded_syntax::<CssLanguage>();
                let formatted =
                    biome_css_formatter::format_node_with_offset(css_options, &node).ok()?;
                Some(wrap_document(formatted.into_document(), true))
            }
            _ => None,
        }
    });

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

#[tracing::instrument(level = "debug", skip(params))]
fn lint(params: LintParams) -> LintResults {
    let workspace_settings = &params.settings;
    let analyzer_options = workspace_settings.analyzer_options::<HtmlLanguage>(
        params.path,
        &params.language,
        params.suppression_reason.as_deref(),
    );
    let tree = params.parse.tree();

    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.settings, analyzer_options)
            .with_only(params.only)
            .with_skip(params.skip)
            .with_path(params.path.as_path())
            .with_enabled_selectors(params.enabled_selectors)
            .with_project_layout(params.project_layout.clone())
            .finish();

    let filter = AnalysisFilter {
        categories: params.categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    let mut process_lint = ProcessLint::new(&params);

    let (_, analyze_diagnostics) = analyze(&tree, filter, &analyzer_options, |signal| {
        process_lint.process_signal(signal)
    });

    process_lint.into_result(
        params
            .parse
            .into_serde_diagnostics(params.diagnostic_offset),
        analyze_diagnostics,
    )
}

pub(crate) fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parse,
        range,
        settings,
        path,
        module_graph: _,
        project_layout,
        language,
        only,
        skip,
        suppression_reason,
        enabled_rules: rules,
        plugins: _,
        categories,
        action_offset,
        document_services: _,
    } = params;
    let _ = debug_span!("Code actions HTML", range =? range, path =? path).entered();
    let tree = parse.tree();
    let _ = trace_span!("Parsed file", tree =? tree).entered();
    let Some(_) = language.to_html_file_source() else {
        error!("Could not determine the HTML file source of the file");
        return PullActionsResult {
            actions: Vec::new(),
        };
    };
    let analyzer_options =
        settings.analyzer_options::<HtmlLanguage>(path, &language, suppression_reason.as_deref());
    let mut actions = Vec::new();
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(settings, analyzer_options)
            .with_only(only)
            .with_skip(skip)
            .with_path(path.as_path())
            .with_enabled_selectors(rules)
            .with_project_layout(project_layout)
            .finish();

    let filter = AnalysisFilter {
        categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range,
    };

    analyze(&tree, filter, &analyzer_options, |signal| {
        actions.extend(signal.actions().into_code_action_iter().map(|item| {
            CodeAction {
                category: item.category.clone(),
                rule_name: item
                    .rule_name
                    .map(|(group, name)| (Cow::Borrowed(group), Cow::Borrowed(name))),
                suggestion: item.suggestion,
                offset: action_offset,
            }
        }));

        ControlFlow::<Never>::Continue(())
    });

    PullActionsResult { actions }
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let mut tree: HtmlRoot = params.parse.tree();

    // Compute final rules (taking `overrides` into account)
    let rules = params.settings.as_linter_rules(params.biome_path.as_path());
    let analyzer_options = params.settings.analyzer_options::<HtmlLanguage>(
        params.biome_path,
        &params.document_file_source,
        params.suppression_reason.as_deref(),
    );
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.settings, analyzer_options)
            .with_only(params.only)
            .with_skip(params.skip)
            .with_path(params.biome_path.as_path())
            .with_enabled_selectors(params.enabled_rules)
            .with_project_layout(params.project_layout.clone())
            .finish();

    let filter = AnalysisFilter {
        categories: params.rule_categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    let mut process_fix_all = ProcessFixAll::new(
        &params,
        rules,
        tree.syntax().text_range_with_trivia().len().into(),
    );

    loop {
        let (action, _) = analyze(&tree, filter, &analyzer_options, |signal| {
            process_fix_all.process_signal(signal)
        });

        let result = process_fix_all.process_action(action, |root| {
            tree = match HtmlRoot::cast(root) {
                Some(tree) => tree,
                None => return None,
            };
            Some(tree.syntax().text_range_with_trivia().len().into())
        })?;

        if result.is_none() {
            return process_fix_all.finish(|| {
                Ok(if params.should_format {
                    Either::Left(format_node(
                        params.settings.format_options::<HtmlLanguage>(
                            params.biome_path,
                            &params.document_file_source,
                        ),
                        tree.syntax(),
                        true,
                    ))
                } else {
                    Either::Right(tree.syntax().to_string())
                })
            });
        }
    }
}

#[instrument(level = "debug", skip_all)]
pub(crate) fn update_snippets(
    root: AnyParse,
    new_snippets: Vec<UpdateSnippetsNodes>,
) -> Result<SendNode, WorkspaceError> {
    let tree: HtmlRoot = root.tree();
    let mut mutation = BatchMutation::new(tree.syntax().clone());
    let iterator = tree
        .syntax()
        .descendants()
        .filter_map(AnyEmbeddedContent::cast);

    for element in iterator {
        let Some(snippet) = new_snippets
            .iter()
            .find(|snippet| snippet.range == element.range())
        else {
            continue;
        };

        if let Some(value_token) = element.value_token() {
            let new_token = ident(snippet.new_code.as_str());
            mutation.replace_token(value_token, new_token);
        }
    }

    let root = mutation.commit();

    Ok(root.as_send().unwrap())
}
