use super::{
    AnalyzerCapabilities, AnalyzerVisitorBuilder, Capabilities, CodeActionsParams,
    DebugCapabilities, DocumentFileSource, EnabledForPath, ExtensionHandler, FixAllParams,
    FormatEmbedNode, FormatterCapabilities, LintParams, LintResults, ParseEmbedResult, ParseResult,
    ParserCapabilities, ProcessFixAll, ProcessLint, SearchCapabilities, UpdateSnippetsNodes,
};
use crate::configuration::to_analyzer_rules;
use crate::embed::registry::{EmbedDetectorsRegistry, EmbedMatch};
use crate::embed::types::{EmbedCandidate, EmbedContent, GuestLanguage, HostLanguage};
use crate::settings::{
    OverrideSettings, SettingsWithEditor, check_feature_activity, check_override_feature_activity,
};
use crate::workspace::document::AnyEmbeddedSnippet;
use crate::workspace::document::services::embedded_bindings::EmbeddedBuilder;
use crate::workspace::{
    CodeAction, CssDocumentServices, DocumentServices, EmbeddedSnippet, JsDocumentServices,
};
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
use biome_css_parser::{CssModulesKind, parse_css_with_offset_and_cache};
use biome_css_syntax::{
    CssFileSource, CssLanguage, EmbeddingHtmlKind, EmbeddingKind as CssEmbeddingKind,
    EmbeddingStyleApplicability,
};
use biome_formatter::format_element::{Interned, LineMode};
use biome_formatter::prelude::{Document, Tag};
use biome_formatter::{
    AttributePosition, BracketSameLine, FormatElement, IndentStyle, IndentWidth, LineEnding,
    LineWidth, Printed, TrailingNewline,
};
use biome_fs::BiomePath;
use biome_html_analyze::{HtmlAnalyzerServices, analyze};
use biome_html_factory::make::ident;
use biome_html_formatter::context::SelfCloseVoidElements;
use biome_html_formatter::{
    HtmlFormatOptions,
    context::{IndentScriptAndStyle, WhitespaceSensitivity},
    format_node,
};
use biome_html_parser::{HtmlParserOptions, parse_html_with_cache};
use biome_html_syntax::element_ext::AnyEmbeddedContent;
use biome_html_syntax::{
    AnyAstroDirective, AnySvelteDirective, AstroEmbeddedContent, HtmlAttribute,
    HtmlAttributeInitializerClause, HtmlDoubleTextExpression, HtmlElement, HtmlFileSource,
    HtmlLanguage, HtmlRoot, HtmlSingleTextExpression, HtmlSyntaxNode, HtmlTextExpression,
    HtmlTextExpressions, HtmlVariant, SvelteAwaitBlock, SvelteEachBlock, SvelteIfBlock,
    SvelteKeyBlock, VueDirective, VueVBindShorthandDirective, VueVOnShorthandDirective,
    VueVSlotShorthandDirective,
};
use biome_js_parser::parse_js_with_offset_and_cache;
use biome_js_syntax::{EmbeddingKind, JsFileSource, JsLanguage};
use biome_json_parser::parse_json_with_offset_and_cache;
use biome_json_syntax::{JsonFileSource, JsonLanguage};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, AstNodeList, BatchMutation, NodeCache, SendNode, TextSize};
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
    pub trailing_newline: Option<TrailingNewline>,
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
            trailing_newline: config.trailing_newline,
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
        let trailing_newline = language.trailing_newline.unwrap_or_default();

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
                .with_self_close_void_elements(self_close_void_elements)
                .with_trailing_newline(trailing_newline);

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

    type ParserOptions = HtmlParserOptions;

    fn resolve_parse_options(
        overrides: &OverrideSettings,
        language: &Self::ParserSettings,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> Self::ParserOptions {
        let html_file_source = file_source.to_html_file_source().unwrap_or_default();
        let mut options = HtmlParserOptions::from(&html_file_source);
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

fn formatter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.formatter_enabled_for_file_path::<HtmlLanguage>(path)
}

fn linter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.linter_enabled_for_file_path::<HtmlLanguage>(path)
}

fn assist_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.assist_enabled_for_file_path::<HtmlLanguage>(path)
}

fn search_enabled(_path: &Utf8Path, _settings: &SettingsWithEditor) -> bool {
    true
}

fn parse(
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    settings: &SettingsWithEditor,
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
    settings: &SettingsWithEditor,
    cache: &mut NodeCache,
    builder: &mut EmbeddedBuilder,
) -> ParseEmbedResult {
    let mut nodes = Vec::new();
    let html_root: HtmlRoot = root.tree();
    let Some(file_source) = file_source.to_html_file_source() else {
        return ParseEmbedResult::default();
    };

    let doc_file_source = DocumentFileSource::Html(file_source);

    let mut ctx = EmbedParseContext {
        cache,
        biome_path,
        host_file_source: &file_source,
        settings,
        builder,
    };

    match file_source.variant() {
        HtmlVariant::Standard(text_expression) => {
            for element in html_root.syntax().descendants() {
                // Element-level embeds via registry
                if let Some(html_element) = HtmlElement::cast_ref(&element)
                    && let Some(candidate) = build_html_candidate(&html_element)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) =
                        parse_matched_embed(&candidate, &embed_match, &mut ctx, None)
                {
                    nodes.push(parsed.node);
                }

                // Text expressions via registry
                match text_expression {
                    HtmlTextExpressions::Single => {
                        if let Some(text_expression) = HtmlSingleTextExpression::cast_ref(&element)
                            && let Ok(expression) = text_expression.expression()
                            && let Some(candidate) = build_text_expression_candidate(&expression)
                            && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                                HostLanguage::Html,
                                &candidate,
                                &doc_file_source,
                            )
                            && let Some(parsed) =
                                parse_matched_embed(&candidate, &embed_match, &mut ctx, None)
                        {
                            nodes.push(parsed.node);
                        }
                    }

                    HtmlTextExpressions::Double => {
                        if let Some(text_expression) = HtmlDoubleTextExpression::cast_ref(&element)
                            && let Ok(expression) = text_expression.expression()
                            && let Some(candidate) = build_text_expression_candidate(&expression)
                            && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                                HostLanguage::Html,
                                &candidate,
                                &doc_file_source,
                            )
                            && let Some(parsed) =
                                parse_matched_embed(&candidate, &embed_match, &mut ctx, None)
                        {
                            nodes.push(parsed.node);
                        }
                    }
                    HtmlTextExpressions::None => {}
                }
            }
        }

        HtmlVariant::Astro => {
            for element in html_root.syntax().descendants() {
                // Astro frontmatter → registry
                if let Some(astro_content) = AstroEmbeddedContent::cast_ref(&element)
                    && let Some(candidate) = build_astro_frontmatter_candidate(&astro_content)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) =
                        parse_matched_embed(&candidate, &embed_match, &mut ctx, None)
                {
                    nodes.push(parsed.node);
                }

                // Text expressions via registry
                if let Some(text_expression) = HtmlSingleTextExpression::cast_ref(&element)
                    && let Ok(expression) = text_expression.expression()
                    && let Some(candidate) = build_text_expression_candidate(&expression)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) =
                        parse_matched_embed(&candidate, &embed_match, &mut ctx, None)
                {
                    nodes.push(parsed.node);
                }

                // HTML elements (script/style) → registry
                if let Some(html_element) = HtmlElement::cast_ref(&element)
                    && let Some(candidate) = build_html_candidate(&html_element)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) =
                        parse_matched_embed(&candidate, &embed_match, &mut ctx, None)
                {
                    nodes.push(parsed.node);
                }

                // Astro directives: class:list={...}, define:vars={...}, etc.
                if let Some(directive) = AnyAstroDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                    && let Some(candidate) = build_attribute_expression_candidate(&initializer)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) =
                        parse_matched_embed(&candidate, &embed_match, &mut ctx, None)
                {
                    nodes.push(parsed.node);
                }

                // Plain HTML attributes with expression values: class={expr}, id={expr}, etc.
                if let Some(attr) = HtmlAttribute::cast_ref(&element)
                    && let Some(initializer) = attr.initializer()
                    && let Some(candidate) = build_attribute_expression_candidate(&initializer)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) =
                        parse_matched_embed(&candidate, &embed_match, &mut ctx, None)
                {
                    nodes.push(parsed.node);
                }
            }
        }
        HtmlVariant::Vue => {
            // Two-pass: collect elements + expressions, then process
            let mut elements = vec![];
            let mut snippet_expressions = vec![];
            for element in html_root.syntax().descendants() {
                if let Some(text_expression) = HtmlDoubleTextExpression::cast_ref(&element) {
                    snippet_expressions.push(text_expression);
                }

                if let Some(element) = HtmlElement::cast_ref(&element) {
                    elements.push(element);
                }
            }

            // Pass 1: elements via registry, collecting JS file sources
            let mut embedded_file_source = JsFileSource::js_module();
            for element in elements {
                if let Some(candidate) = build_html_candidate(&element)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) =
                        parse_matched_embed(&candidate, &embed_match, &mut ctx, None)
                {
                    if let Some(js_fs) = parsed.js_file_source {
                        embedded_file_source = merge_js_file_source(embedded_file_source, js_fs);
                    }
                    nodes.push(parsed.node);
                }
            }

            // Pass 2: text expressions via registry using merged embedded_file_source
            for snippet in snippet_expressions {
                if let Ok(expression) = snippet.expression()
                    && let Some(candidate) = build_text_expression_candidate(&expression)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) = parse_matched_embed(
                        &candidate,
                        &embed_match,
                        &mut ctx,
                        Some(embedded_file_source),
                    )
                {
                    nodes.push(parsed.node);
                }
            }

            // Pass 3: directive attributes via registry using merged embedded_file_source
            for element in html_root.syntax().descendants() {
                // Handle @click shorthand (VueVOnShorthandDirective)
                if let Some(directive) = VueVOnShorthandDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                    && let Some(candidate) = build_vue_directive_candidate(&initializer, true)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) = parse_matched_embed(
                        &candidate,
                        &embed_match,
                        &mut ctx,
                        Some(embedded_file_source),
                    )
                {
                    nodes.push(parsed.node);
                }

                // Handle :prop shorthand (VueVBindShorthandDirective)
                if let Some(directive) = VueVBindShorthandDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                    && let Some(candidate) = build_vue_directive_candidate(&initializer, false)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) = parse_matched_embed(
                        &candidate,
                        &embed_match,
                        &mut ctx,
                        Some(embedded_file_source),
                    )
                {
                    nodes.push(parsed.node);
                }

                // Handle #slot shorthand (VueVSlotShorthandDirective)
                if let Some(directive) = VueVSlotShorthandDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                    && let Some(candidate) = build_vue_directive_candidate(&initializer, false)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) = parse_matched_embed(
                        &candidate,
                        &embed_match,
                        &mut ctx,
                        Some(embedded_file_source),
                    )
                {
                    nodes.push(parsed.node);
                }

                // Handle full directives (v-on:, v-bind:, v-if, v-show, etc.)
                if let Some(directive) = VueDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                {
                    let is_v_on = directive
                        .name_token()
                        .map(|t| t.text_trimmed() == "v-on")
                        .unwrap_or(false);
                    if let Some(candidate) = build_vue_directive_candidate(&initializer, is_v_on)
                        && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                            HostLanguage::Html,
                            &candidate,
                            &doc_file_source,
                        )
                        && let Some(parsed) = parse_matched_embed(
                            &candidate,
                            &embed_match,
                            &mut ctx,
                            Some(embedded_file_source),
                        )
                    {
                        nodes.push(parsed.node);
                    }
                }
            }
        }
        HtmlVariant::Svelte => {
            // Two-pass: collect elements + expressions, then process
            let mut elements = vec![];
            let mut snippet_expressions = vec![];
            for element in html_root.syntax().descendants() {
                if let Some(text_expression) = HtmlSingleTextExpression::cast_ref(&element) {
                    snippet_expressions.push(text_expression);
                }

                if let Some(element) = HtmlElement::cast_ref(&element) {
                    elements.push(element);
                }
            }

            // Pass 1: elements via registry, collecting JS file sources
            let mut embedded_file_source = JsFileSource::js_module();
            for element in elements {
                if let Some(candidate) = build_html_candidate(&element)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) =
                        parse_matched_embed(&candidate, &embed_match, &mut ctx, None)
                {
                    if let Some(js_fs) = parsed.js_file_source {
                        embedded_file_source = merge_js_file_source(embedded_file_source, js_fs);
                    }
                    nodes.push(parsed.node);
                }
            }

            // Pass 2: text expressions via registry using merged embedded_file_source
            for snippet in snippet_expressions {
                if let Ok(expression) = snippet.expression()
                    && let Some(candidate) = build_text_expression_candidate(&expression)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) = parse_matched_embed(
                        &candidate,
                        &embed_match,
                        &mut ctx,
                        Some(embedded_file_source),
                    )
                {
                    nodes.push(parsed.node);
                }
            }

            // Pass 3: control flow blocks via registry
            for element in html_root.syntax().descendants() {
                // Handle {#if expression}
                if let Some(if_block) = SvelteIfBlock::cast_ref(&element)
                    && let Ok(opening_block) = if_block.opening_block()
                    && let Ok(expression) = opening_block.expression()
                    && let Some(candidate) = build_text_expression_candidate(&expression)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) = parse_matched_embed(
                        &candidate,
                        &embed_match,
                        &mut ctx,
                        Some(embedded_file_source),
                    )
                {
                    nodes.push(parsed.node);
                }

                // Handle {:else if expression}
                if let Some(if_block) = SvelteIfBlock::cast_ref(&element) {
                    for else_if_clause in if_block.else_if_clauses() {
                        if let Ok(expression) = else_if_clause.expression()
                            && let Some(candidate) = build_text_expression_candidate(&expression)
                            && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                                HostLanguage::Html,
                                &candidate,
                                &doc_file_source,
                            )
                            && let Some(parsed) = parse_matched_embed(
                                &candidate,
                                &embed_match,
                                &mut ctx,
                                Some(embedded_file_source),
                            )
                        {
                            nodes.push(parsed.node);
                        }
                    }
                }

                // Handle {#each expression as item}
                if let Some(each_block) = SvelteEachBlock::cast_ref(&element)
                    && let Ok(opening_block) = each_block.opening_block()
                {
                    if let Ok(expression) = opening_block.list()
                        && let Some(candidate) = build_text_expression_candidate(&expression)
                        && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                            HostLanguage::Html,
                            &candidate,
                            &doc_file_source,
                        )
                        && let Some(parsed) = parse_matched_embed(
                            &candidate,
                            &embed_match,
                            &mut ctx,
                            Some(embedded_file_source),
                        )
                    {
                        nodes.push(parsed.node);
                    }

                    if let Some(item) = opening_block.item()
                        && let Some(item) = item.as_svelte_each_as_keyed_item()
                        && let Some(key) = item.key()
                        && let Ok(key_expression) = key.expression()
                        && let Some(candidate) = build_text_expression_candidate(&key_expression)
                        && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                            HostLanguage::Html,
                            &candidate,
                            &doc_file_source,
                        )
                        && let Some(parsed) = parse_matched_embed(
                            &candidate,
                            &embed_match,
                            &mut ctx,
                            Some(embedded_file_source),
                        )
                    {
                        nodes.push(parsed.node);
                    }
                }

                // Handle {#await expression}
                if let Some(await_block) = SvelteAwaitBlock::cast_ref(&element)
                    && let Ok(opening_block) = await_block.opening_block()
                    && let Ok(expression) = opening_block.expression()
                    && let Some(candidate) = build_text_expression_candidate(&expression)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) = parse_matched_embed(
                        &candidate,
                        &embed_match,
                        &mut ctx,
                        Some(embedded_file_source),
                    )
                {
                    nodes.push(parsed.node);
                }

                // Handle {#key expression}
                if let Some(key_block) = SvelteKeyBlock::cast_ref(&element)
                    && let Ok(opening_block) = key_block.opening_block()
                    && let Ok(expression) = opening_block.expression()
                    && let Some(candidate) = build_text_expression_candidate(&expression)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) = parse_matched_embed(
                        &candidate,
                        &embed_match,
                        &mut ctx,
                        Some(embedded_file_source),
                    )
                {
                    nodes.push(parsed.node);
                }
            }

            // Pass 4: directive attributes and attributes which initializer is a text expression
            for element in html_root.syntax().descendants() {
                // Handle special Svelte directives (bind:, class:, etc.)
                if let Some(directive) = AnySvelteDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                    && let Some(candidate) = build_svelte_directive_candidate(&initializer)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) = parse_matched_embed(
                        &candidate,
                        &embed_match,
                        &mut ctx,
                        Some(embedded_file_source),
                    )
                {
                    nodes.push(parsed.node);
                }

                if let Some(attr) = HtmlAttribute::cast_ref(&element)
                    && let Some(initializer) = attr.initializer()
                    && let Some(candidate) = build_attribute_expression_candidate(&initializer)
                    && let Some(embed_match) = EmbedDetectorsRegistry::detect_match(
                        HostLanguage::Html,
                        &candidate,
                        &doc_file_source,
                    )
                    && let Some(parsed) = parse_matched_embed(
                        &candidate,
                        &embed_match,
                        &mut ctx,
                        Some(embedded_file_source),
                    )
                {
                    nodes.push(parsed.node);
                }
            }
        }
    }

    ParseEmbedResult { nodes }
}

/// Build an `EmbedCandidate::Element` from an `HtmlElement`.
/// Returns `None` if the element has no embedded content or has multiple children (error).
fn build_html_candidate(element: &HtmlElement) -> Option<EmbedCandidate> {
    // Multiple children is likely an error — skip
    if element.children().len() > 1 {
        return None;
    }

    let tag_name = element.tag_name()?;

    let attributes: Vec<_> = element
        .opening_element()
        .ok()
        .into_iter()
        .flat_map(|opening| opening.attributes())
        .filter_map(|attr| {
            let html_attr = attr.as_html_attribute()?;
            let name = html_attr
                .name()
                .ok()?
                .value_token()
                .ok()?
                .token_text_trimmed();
            let value = html_attr
                .initializer()
                .and_then(|init| init.value().ok())
                .and_then(|v| v.as_html_string().cloned())
                .and_then(|s| s.inner_string_text().ok());
            Some((name, value))
        })
        .collect();

    // Extract content from HtmlEmbeddedContent child
    let content_child = element.children().iter().next().and_then(|child| {
        let child = child.as_any_html_content()?;
        child.as_html_embedded_content().cloned()
    })?;
    let value_token = content_child.value_token().ok()?;

    Some(EmbedCandidate::Element {
        tag_name,
        attributes,
        content: EmbedContent {
            element_range: content_child.range(),
            content_range: value_token.text_range(),
            content_offset: value_token.text_range().start(),
            // Use full token text (including trivia) to match the untrimmed content_offset.
            // The parser needs text and offset to be consistent.
            text: value_token.token_text(),
        },
    })
}

/// Build an `EmbedCandidate::Frontmatter` from Astro's `---` block.
fn build_astro_frontmatter_candidate(element: &AstroEmbeddedContent) -> Option<EmbedCandidate> {
    let content_token = element.content_token()?;

    Some(EmbedCandidate::Frontmatter {
        content: EmbedContent {
            element_range: element.range(),
            content_range: content_token.text_trimmed_range(),
            content_offset: content_token.text_range().start(),
            // Use full token text (including trivia) to match the untrimmed content_offset.
            // The parser needs text and offset to be consistent.
            text: content_token.token_text(),
        },
    })
}

/// Build an `EmbedCandidate::TextExpression` from an `HtmlTextExpression`.
///
/// This is the inner expression node (the JS code inside `{ }` or `{{ }}`).
/// The caller extracts it from the outer wrapper (`HtmlSingleTextExpression`,
/// `HtmlDoubleTextExpression`, or control flow block).
fn build_text_expression_candidate(expression: &HtmlTextExpression) -> Option<EmbedCandidate> {
    let content_token = expression.html_literal_token().ok()?;
    Some(EmbedCandidate::TextExpression {
        content: EmbedContent {
            element_range: expression.range(),
            content_range: content_token.text_range(),
            content_offset: content_token.text_range().start(),
            text: content_token.token_text(),
        },
    })
}

/// Build an `EmbedCandidate::Directive` from a Vue directive initializer clause.
///
/// Vue directives use quoted string values (`@click="handler()"`).
/// The JS content is the inner text without quotes, offset by +1 for the opening quote.
fn build_vue_directive_candidate(
    initializer: &HtmlAttributeInitializerClause,
    is_event_handler: bool,
) -> Option<EmbedCandidate> {
    let value_node = initializer.value().ok()?;
    let html_string = value_node.as_html_string()?;
    let content_token = html_string.value_token().ok()?;
    let inner_text = html_string.inner_string_text().ok()?;
    let token_range = content_token.text_trimmed_range();
    let inner_offset = token_range.start() + TextSize::from(1);

    Some(EmbedCandidate::Directive {
        content: EmbedContent {
            element_range: initializer.range(),
            content_range: token_range,
            content_offset: inner_offset,
            text: inner_text,
        },
        is_event_handler,
    })
}

/// Build an `EmbedCandidate::Directive` from a Svelte directive initializer clause.
///
/// Svelte directives use curly brace text expressions (`on:click={handler}`).
/// The JS content is the literal token inside the expression node.
fn build_svelte_directive_candidate(
    initializer: &HtmlAttributeInitializerClause,
) -> Option<EmbedCandidate> {
    build_attribute_expression_candidate(initializer)
}

/// Build an `EmbedCandidate::Directive` from an initializer clause containing
/// a curly brace text expression (`attr={expr}`).
///
/// Used by both Astro and Svelte attribute expression extraction.
/// Returns `None` if the initializer does not contain a text expression.
fn build_attribute_expression_candidate(
    initializer: &HtmlAttributeInitializerClause,
) -> Option<EmbedCandidate> {
    let value_node = initializer.value().ok()?;
    let text_expression = value_node.as_html_attribute_single_text_expression()?;
    let expression = text_expression.expression().ok()?;
    let content_token = expression.html_literal_token().ok()?;

    Some(EmbedCandidate::Directive {
        content: EmbedContent {
            element_range: expression.range(),
            content_range: content_token.text_range(),
            content_offset: content_token.text_range().start(),
            text: content_token.token_text(),
        },
        is_event_handler: false,
    })
}

/// Result of parsing a matched embed.
struct ParsedEmbed {
    /// The parsed snippet + file source, ready to push to `nodes`.
    node: (AnyEmbeddedSnippet, DocumentFileSource),
    /// If JS was parsed, the resolved JsFileSource (for `embedded_file_source` capture).
    js_file_source: Option<JsFileSource>,
}

/// Merge two `JsFileSource` values by picking the most permissive one.
///
/// Vue and Svelte files can have multiple `<script>` tags with different
/// `lang` attributes. The merged result is used as the base file source
/// for text expressions and directives, so it must be able to parse any
/// syntax that might appear in the template.
///
/// Hierarchy: Tsx > Ts > Jsx > JsModule > JsScript.
fn merge_js_file_source(a: JsFileSource, b: JsFileSource) -> JsFileSource {
    let ts = a.is_typescript() || b.is_typescript();
    let jsx = a.is_jsx() || b.is_jsx();
    match (ts, jsx) {
        (true, true) => JsFileSource::tsx(),
        (true, false) => JsFileSource::ts(),
        (false, true) => JsFileSource::jsx(),
        (false, false) => JsFileSource::js_module(),
    }
}

/// Shared parsing context passed to `parse_matched_embed`.
/// Groups the arguments that stay constant across all embed parses within a
/// single `parse_embedded_nodes` invocation.
struct EmbedParseContext<'a, 'b> {
    cache: &'a mut NodeCache,
    biome_path: &'a BiomePath,
    host_file_source: &'a HtmlFileSource,
    settings: &'a SettingsWithEditor<'b>,
    builder: &'a mut EmbeddedBuilder,
}

/// Parse an embedded code fragment using the parser for the matched guest language.
fn parse_matched_embed(
    candidate: &EmbedCandidate,
    embed_match: &EmbedMatch,
    ctx: &mut EmbedParseContext,
    embedded_file_source: Option<JsFileSource>,
) -> Option<ParsedEmbed> {
    let content = candidate.content();

    match embed_match.guest {
        GuestLanguage::JsModule
        | GuestLanguage::JsScript
        | GuestLanguage::Jsx
        | GuestLanguage::Ts
        | GuestLanguage::Tsx => {
            // Determine base JsFileSource from guest language
            let mut js_source = match embed_match.guest {
                GuestLanguage::JsModule => JsFileSource::js_module(),
                GuestLanguage::JsScript => JsFileSource::js_script(),
                GuestLanguage::Jsx => JsFileSource::jsx(),
                GuestLanguage::Ts => JsFileSource::ts(),
                GuestLanguage::Tsx => JsFileSource::tsx(),
                _ => unreachable!(),
            };

            // Configure EmbeddingKind based on framework + candidate type
            let is_source_level = match candidate {
                EmbedCandidate::Frontmatter { .. } => {
                    js_source =
                        js_source.with_embedding_kind(EmbeddingKind::Astro { frontmatter: true });
                    true
                }
                EmbedCandidate::Element { .. } => {
                    if ctx.host_file_source.is_svelte() {
                        js_source = js_source
                            .with_embedding_kind(EmbeddingKind::Svelte { is_source: true });
                    } else if ctx.host_file_source.is_vue() {
                        js_source = js_source.with_embedding_kind(EmbeddingKind::Vue {
                            setup: candidate.has_attribute("setup"),
                            is_source: true,
                            event_handler: false,
                            allow_statements: true,
                        });
                    }
                    // Astro <script> tags and plain HTML: no EmbeddingKind
                    true
                }
                EmbedCandidate::TextExpression { .. } => {
                    // Use embedded_file_source as base if available (Vue/Svelte pass 2+)
                    if let Some(efs) = embedded_file_source {
                        js_source = efs;
                    }
                    if ctx.host_file_source.is_astro() {
                        js_source = js_source
                            .with_embedding_kind(EmbeddingKind::Astro { frontmatter: false });
                    } else if ctx.host_file_source.is_svelte() {
                        js_source = js_source
                            .with_embedding_kind(EmbeddingKind::Svelte { is_source: false });
                    } else if ctx.host_file_source.is_vue() {
                        js_source = js_source.with_embedding_kind(EmbeddingKind::Vue {
                            setup: false,
                            is_source: false,
                            event_handler: false,
                            allow_statements: false,
                        });
                    }
                    false
                }
                EmbedCandidate::Directive {
                    is_event_handler, ..
                } => {
                    // Use embedded_file_source as base if available (Vue/Svelte pass 2+)
                    if let Some(efs) = embedded_file_source {
                        js_source = efs;
                    }
                    match ctx.host_file_source.variant() {
                        HtmlVariant::Standard(_) => {}
                        HtmlVariant::Astro => {
                            js_source = js_source
                                .with_embedding_kind(EmbeddingKind::Astro { frontmatter: false });
                        }
                        HtmlVariant::Vue => {
                            js_source = js_source.with_embedding_kind(EmbeddingKind::Vue {
                                setup: false,
                                is_source: false,
                                event_handler: *is_event_handler,
                                allow_statements: false,
                            });
                        }
                        HtmlVariant::Svelte => {
                            js_source = js_source
                                .with_embedding_kind(EmbeddingKind::Svelte { is_source: false });
                        }
                    }

                    false
                }
                _ => false,
            };

            let doc_source = DocumentFileSource::Js(js_source);
            let options = ctx
                .settings
                .parse_options::<JsLanguage>(ctx.biome_path, &doc_source);
            let parse = parse_js_with_offset_and_cache(
                content.text.text(),
                content.content_offset,
                js_source,
                options,
                ctx.cache,
            );

            // Only visit source-level snippets for binding tracking
            if is_source_level {
                ctx.builder.visit_js_source_snippet(&parse.tree());
            }

            let snippet: EmbeddedSnippet<JsLanguage> = EmbeddedSnippet::new(
                parse.into(),
                content.element_range,
                content.content_range,
                content.content_offset,
            );

            // Source-level embeds get full services; expression-level don't
            let js_services = if is_source_level
                && (ctx.settings.as_ref().is_linter_enabled()
                    || ctx.settings.as_ref().is_assist_enabled())
            {
                JsDocumentServices::default()
                    .with_js_semantic_model(&snippet.tree())
                    .into()
            } else {
                DocumentServices::none()
            };

            Some(ParsedEmbed {
                node: ((snippet, js_services).into(), doc_source),
                // Only source-level embeds contribute to embedded_file_source capture
                js_file_source: if is_source_level {
                    Some(js_source)
                } else {
                    None
                },
            })
        }

        GuestLanguage::Css => {
            let css_source = if ctx.host_file_source.is_html() {
                CssFileSource::css()
                    .with_embedding_kind(CssEmbeddingKind::Html(EmbeddingHtmlKind::Html))
            } else if ctx.host_file_source.is_vue() {
                // Vue: <style scoped> and <style module> are component-local.
                // Plain <style> (no attribute) leaks into the global scope.
                let applicability =
                    if candidate.has_attribute("scoped") || candidate.has_attribute("module") {
                        EmbeddingStyleApplicability::Local
                    } else {
                        EmbeddingStyleApplicability::Global
                    };
                CssFileSource::new_css_modules().with_embedding_kind(CssEmbeddingKind::Html(
                    EmbeddingHtmlKind::Vue { applicability },
                ))
            } else if ctx.host_file_source.is_astro() {
                // Astro: <style is:global> is global; plain <style> is local
                let applicability = if candidate.has_attribute("is:global") {
                    EmbeddingStyleApplicability::Global
                } else {
                    EmbeddingStyleApplicability::Local
                };
                CssFileSource::new_css_modules().with_embedding_kind(CssEmbeddingKind::Html(
                    EmbeddingHtmlKind::Astro { applicability },
                ))
            } else if ctx.host_file_source.is_svelte() {
                // Svelte: plain <style> is local (global classes via :global()
                // are detected at the selector level by the CSS semantic model)
                CssFileSource::new_css_modules().with_embedding_kind(CssEmbeddingKind::Html(
                    EmbeddingHtmlKind::Svelte {
                        applicability: EmbeddingStyleApplicability::Local,
                    },
                ))
            } else {
                CssFileSource::new_css_modules()
            };
            let doc_source = DocumentFileSource::Css(css_source);
            let mut options = ctx
                .settings
                .parse_options::<CssLanguage>(ctx.biome_path, &doc_source);
            if ctx.host_file_source.is_vue() {
                options.css_modules = CssModulesKind::Vue;
            } else if !ctx.host_file_source.is_html() {
                options.css_modules = CssModulesKind::Classic;
            }
            let parse = parse_css_with_offset_and_cache(
                content.text.text(),
                css_source,
                content.content_offset,
                ctx.cache,
                options,
            );

            let mut services = CssDocumentServices::default();
            if ctx.settings.as_ref().is_linter_enabled()
                || ctx.settings.as_ref().is_assist_enabled()
            {
                services = services.with_css_semantic_model(&parse.tree());
            }

            let snippet: EmbeddedSnippet<CssLanguage> = EmbeddedSnippet::new(
                parse.into(),
                content.element_range,
                content.content_range,
                content.content_offset,
            );

            Some(ParsedEmbed {
                node: ((snippet, services.into()).into(), doc_source),
                js_file_source: None,
            })
        }

        GuestLanguage::Json => {
            let doc_source = DocumentFileSource::Json(JsonFileSource::json());
            let options = ctx
                .settings
                .parse_options::<JsonLanguage>(ctx.biome_path, &doc_source);
            let parse = parse_json_with_offset_and_cache(
                content.text.text(),
                content.content_offset,
                ctx.cache,
                options,
            );

            let snippet: EmbeddedSnippet<JsonLanguage> = EmbeddedSnippet::new(
                parse.into(),
                content.element_range,
                content.content_range,
                content.content_offset,
            );

            Some(ParsedEmbed {
                node: (snippet.into(), doc_source),
                js_file_source: None,
            })
        }

        GuestLanguage::GraphQL => {
            // GraphQL embeds are only used by the JS handler, not HTML
            None
        }
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
    settings: &SettingsWithEditor,
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
    settings: &SettingsWithEditor,
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
    settings: &SettingsWithEditor,
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

    // Propagate expand flags again after inserting embedded content,
    // so that groups inside the embedded documents properly expand.
    formatted.propagate_expand();

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
        params.working_directory,
        &params.language,
        params.suppression_reason.as_deref(),
    );
    let tree = params.parse.tree();

    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.settings.as_ref(), analyzer_options)
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

    let source_type = params.language.to_html_file_source().unwrap_or_default();
    let html_services = HtmlAnalyzerServices {
        module_graph: Some(params.module_graph.clone()),
        project_layout: Some(params.project_layout.clone()),
    };
    let (_, analyze_diagnostics) = analyze(
        &tree,
        filter,
        &analyzer_options,
        source_type,
        html_services,
        |signal| process_lint.process_signal(signal),
    );

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
        module_graph,
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
        working_directory,
    } = params;
    let _ = debug_span!("Code actions HTML", range =? range, path =? path).entered();
    let tree = parse.tree();
    let _ = trace_span!("Parsed file", tree =? tree).entered();
    let Some(source_type) = language.to_html_file_source() else {
        error!("Could not determine the HTML file source of the file");
        return PullActionsResult {
            actions: Vec::new(),
        };
    };
    let analyzer_options = settings.analyzer_options::<HtmlLanguage>(
        path,
        working_directory,
        &language,
        suppression_reason.as_deref(),
    );
    let mut actions = Vec::new();
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(settings.as_ref(), analyzer_options)
            .with_only(only)
            .with_skip(skip)
            .with_path(path.as_path())
            .with_enabled_selectors(rules)
            .with_project_layout(project_layout.clone())
            .finish();

    let filter = AnalysisFilter {
        categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range,
    };

    let html_services = HtmlAnalyzerServices {
        module_graph: Some(module_graph),
        project_layout: Some(project_layout),
    };
    analyze(
        &tree,
        filter,
        &analyzer_options,
        source_type,
        html_services,
        |signal| {
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
        },
    );

    PullActionsResult { actions }
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let mut tree: HtmlRoot = params.parse.tree();

    // Compute final rules (taking `overrides` into account)
    let rules = params
        .settings
        .as_ref()
        .as_linter_rules(params.biome_path.as_path());
    let analyzer_options = params.settings.analyzer_options::<HtmlLanguage>(
        params.biome_path,
        params.working_directory,
        &params.document_file_source,
        params.suppression_reason.as_deref(),
    );
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.settings.as_ref(), analyzer_options)
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

    let source_type = params
        .document_file_source
        .to_html_file_source()
        .unwrap_or_default();
    loop {
        let html_services = HtmlAnalyzerServices {
            module_graph: Some(params.module_graph.clone()),
            project_layout: Some(params.project_layout.clone()),
        };
        let (action, _) = analyze(
            &tree,
            filter,
            &analyzer_options,
            source_type,
            html_services,
            |signal| process_fix_all.process_signal(signal),
        );
        let result = process_fix_all.process_action(action, |root| {
            tree = match HtmlRoot::cast(root) {
                Some(tree) => tree,
                None => return None,
            };
            Some(tree.syntax().text_range_with_trivia().len().into())
        })?;

        if result.is_none() {
            return process_fix_all.finish(
                || {
                    Ok(if params.should_format {
                        Either::Left(format_node(
                            params.settings.format_options::<HtmlLanguage>(
                                params.biome_path,
                                &params.document_file_source,
                            ),
                            tree.syntax(),
                            // NOTE: this is important that stays false. In this instance, the formatting of embedded
                            // nodes has already happened, because the workspace during fix_all() process the embedded nodes
                            // first, and then the root document. This means the embedded nodes don't need to be formatted and can
                            // be printed verbatim by the formatter.
                            false,
                        ))
                    } else {
                        Either::Right(tree.syntax().to_string())
                    })
                },
                params.embeds_initial_indent,
            );
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
            let leading_trivia = read_leading_trivia(value_token.text_trimmed());
            let trailing_trivia = read_trailing_trivia(value_token.text_trimmed());
            let new_token = ident(&format!(
                "{}{}{}",
                leading_trivia,
                snippet.new_code.trim(), // trim to avoid duplicating trivia
                trailing_trivia
            ));
            mutation.replace_token(value_token, new_token);
        }
    }

    let root = mutation.commit();

    Ok(root.as_send().unwrap())
}

/// Extracts all leading whitespace (spaces, tabs, newlines, carriage returns) from a string.
///
/// This function iterates through the string bytes to find where the actual content starts.
/// For HTML embedded content tokens, whitespace is part of the token text itself, not stored as trivia.
///
/// # Arguments
/// * `value` - The string to extract leading trivia from
///
/// # Returns
/// A `Cow<'_, str>` containing the leading whitespace. If the entire string is whitespace,
/// returns the entire string. If there's no leading whitespace, returns an empty string.
///
/// # Examples
/// ```ignore
/// assert_eq!(read_leading_trivia("\n\tconsole.log('Hi');"), "\n\t");
/// assert_eq!(read_leading_trivia("console.log('Hi');"), "");
/// assert_eq!(read_leading_trivia("   "), "   ");
/// ```
fn read_leading_trivia(value: &str) -> Cow<'_, str> {
    let bytes = value.as_bytes();
    let count = bytes
        .iter()
        .take_while(|&&b| matches!(b, b' ' | b'\t' | b'\n' | b'\r'))
        .count();

    if count > 0 {
        Cow::Borrowed(&value[..count])
    } else {
        Cow::Borrowed("")
    }
}

/// Extracts all trailing whitespace (spaces, tabs, newlines, carriage returns) from a string.
///
/// This function iterates backward through the string bytes to find where the actual content ends.
/// For HTML embedded content tokens, whitespace is part of the token text itself, not stored as trivia.
///
/// # Arguments
/// * `value` - The string to extract trailing trivia from
///
/// # Returns
/// A `Cow<'_, str>` containing the trailing whitespace. If the entire string is whitespace,
/// returns an empty string (because leading trivia would have consumed it all). If there's no
/// trailing whitespace, returns an empty string.
///
/// # Examples
/// ```ignore
/// assert_eq!(read_trailing_trivia("console.log('Hi');\n"), "\n");
/// assert_eq!(read_trailing_trivia("console.log('Hi');"), "");
/// assert_eq!(read_trailing_trivia("   "), "");
/// ```
fn read_trailing_trivia(value: &str) -> Cow<'_, str> {
    let bytes = value.as_bytes();
    let count = bytes
        .iter()
        .rev()
        .take_while(|&&b| matches!(b, b' ' | b'\t' | b'\n' | b'\r'))
        .count();

    if count > 0 {
        Cow::Borrowed(&value[value.len() - count..])
    } else {
        Cow::Borrowed("")
    }
}
