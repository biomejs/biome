use super::{
    AnalyzerCapabilities, AnalyzerVisitorBuilder, Capabilities, CodeActionsParams,
    DebugCapabilities, DocumentFileSource, EnabledForPath, ExtensionHandler, FixAllParams,
    FormatEmbedNode, FormatterCapabilities, LintParams, LintResults, ParseEmbedResult, ParseResult,
    ParserCapabilities, ProcessFixAll, ProcessLint, SearchCapabilities, UpdateSnippetsNodes,
};
use crate::configuration::to_analyzer_rules;
use crate::settings::{
    OverrideSettings, SettingsWithEditor, check_feature_activity, check_override_feature_activity,
};
use crate::workspace::document::services::embedded_bindings::EmbeddedBuilder;
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
use biome_css_parser::{CssModulesKind, parse_css_with_offset_and_cache};
use biome_css_syntax::{CssFileSource, CssLanguage};
use biome_formatter::format_element::{Interned, LineMode};
use biome_formatter::prelude::{Document, Tag};
use biome_formatter::{
    AttributePosition, BracketSameLine, FormatElement, IndentStyle, IndentWidth, LineEnding,
    LineWidth, Printed, TrailingNewline,
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
    AnySvelteDirective, AstroEmbeddedContent, HtmlAttributeInitializerClause,
    HtmlDoubleTextExpression, HtmlElement, HtmlFileSource, HtmlLanguage, HtmlRoot,
    HtmlSingleTextExpression, HtmlSyntaxNode, HtmlTextExpression, HtmlTextExpressions, HtmlVariant,
    SvelteAwaitBlock, SvelteEachBlock, SvelteIfBlock, SvelteKeyBlock, VueDirective,
    VueVBindShorthandDirective, VueVOnShorthandDirective, VueVSlotShorthandDirective,
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

    match file_source.variant() {
        HtmlVariant::Standard(text_expression) => {
            for element in html_root.syntax().descendants() {
                if let Some(html_element) = HtmlElement::cast_ref(&element) {
                    if let Some(script_type) = html_element.get_script_type() {
                        if script_type.is_javascript() {
                            let result = parse_embedded_script(
                                html_element.clone(),
                                cache,
                                biome_path,
                                &file_source,
                                settings,
                                builder,
                            );
                            if let Some((content, file_source)) = result {
                                nodes.push((content.into(), file_source.into()));
                            }
                        } else if script_type.is_json() {
                            let result = parse_embedded_json(
                                html_element.clone(),
                                cache,
                                biome_path,
                                settings,
                            );
                            if let Some((content, file_source)) = result {
                                nodes.push((content.into(), file_source));
                            }
                        }
                    } else if html_element.is_style_tag() {
                        let result = parse_embedded_style(
                            html_element.clone(),
                            cache,
                            biome_path,
                            &file_source,
                            settings,
                        );
                        if let Some((content, services, file_source)) = result {
                            nodes.push(((content, services).into(), file_source));
                        }
                    }
                }

                match text_expression {
                    HtmlTextExpressions::Single => {
                        if let Some(text_expression) = HtmlSingleTextExpression::cast_ref(&element)
                        {
                            let result = parse_single_text_expression(
                                text_expression,
                                cache,
                                biome_path,
                                settings,
                            );
                            if let Some((content, file_source)) = result {
                                nodes.push((content.into(), file_source));
                            }
                        }
                    }

                    HtmlTextExpressions::Double => {
                        if let Some(text_expression) = HtmlDoubleTextExpression::cast_ref(&element)
                        {
                            let result = parse_double_text_expression(
                                text_expression,
                                cache,
                                biome_path,
                                settings,
                            );
                            if let Some((content, file_source)) = result {
                                nodes.push((content.into(), file_source));
                            }
                        }
                    }
                    HtmlTextExpressions::None => {}
                }
            }
        }

        HtmlVariant::Astro => {
            for element in html_root.syntax().descendants() {
                if let Some(astro_embedded_content) = AstroEmbeddedContent::cast_ref(&element) {
                    let result = parse_astro_embedded_script(
                        astro_embedded_content.clone(),
                        cache,
                        biome_path,
                        settings,
                        builder,
                    );
                    if let Some((content, file_source)) = result {
                        nodes.push((content.into(), file_source));
                    }
                }

                if let Some(text_expression) = HtmlSingleTextExpression::cast_ref(&element) {
                    let result =
                        parse_astro_text_expression(text_expression, cache, biome_path, settings);
                    if let Some((content, file_source)) = result {
                        nodes.push((content.into(), file_source));
                    }
                }

                if let Some(html_element) = HtmlElement::cast_ref(&element) {
                    if let Some(script_type) = html_element.get_script_type() {
                        if script_type.is_javascript() {
                            let result = parse_embedded_script(
                                html_element.clone(),
                                cache,
                                biome_path,
                                &file_source,
                                settings,
                                builder,
                            );
                            if let Some((content, file_source)) = result {
                                nodes.push((content.into(), file_source.into()));
                            }
                        } else if script_type.is_json() {
                            let result = parse_embedded_json(
                                html_element.clone(),
                                cache,
                                biome_path,
                                settings,
                            );
                            if let Some((content, file_source)) = result {
                                nodes.push((content.into(), file_source));
                            }
                        }
                    } else if html_element.is_style_tag() {
                        let result = parse_embedded_style(
                            html_element.clone(),
                            cache,
                            biome_path,
                            &file_source,
                            settings,
                        );
                        if let Some((content, services, file_source)) = result {
                            nodes.push(((content, services).into(), file_source));
                        }
                    }
                }
            }
        }
        HtmlVariant::Vue => {
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

            let mut embedded_file_source = JsFileSource::js_module();
            for element in elements {
                if let Some(script_type) = element.get_script_type() {
                    if script_type.is_javascript() {
                        let result = parse_embedded_script(
                            element.clone(),
                            cache,
                            biome_path,
                            &file_source,
                            settings,
                            builder,
                        );
                        if let Some((content, file_source)) = result {
                            embedded_file_source = file_source;
                            nodes.push((content.into(), file_source.into()));
                        }
                    } else if script_type.is_json() {
                        let result =
                            parse_embedded_json(element.clone(), cache, biome_path, settings);
                        if let Some((content, file_source)) = result {
                            nodes.push((content.into(), file_source));
                        }
                    }
                } else if element.is_style_tag() {
                    let result = parse_embedded_style(
                        element.clone(),
                        cache,
                        biome_path,
                        &file_source,
                        settings,
                    );
                    if let Some((content, services, file_source)) = result {
                        nodes.push(((content, services).into(), file_source));
                    }
                }
            }

            for snippet in snippet_expressions {
                let result = parse_vue_text_expression(
                    snippet,
                    cache,
                    biome_path,
                    settings,
                    embedded_file_source,
                );

                if let Some((content, file_source)) = result {
                    nodes.push((content.into(), file_source));
                }
            }

            // Parse Vue directive attributes (v-on, v-bind, v-if, etc.)
            for element in html_root.syntax().descendants() {
                // Handle @click shorthand (VueVOnShorthandDirective)
                if let Some(directive) = VueVOnShorthandDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                {
                    let file_source =
                        embedded_file_source.with_embedding_kind(EmbeddingKind::Vue {
                            setup: false,
                            is_source: false,
                            event_handler: true,
                        });
                    if let Some((content, doc_source)) = parse_directive_string_value(
                        &initializer,
                        cache,
                        biome_path,
                        settings,
                        file_source,
                    ) {
                        nodes.push((content.into(), doc_source));
                    }
                }

                // Handle :prop shorthand (VueVBindShorthandDirective)
                if let Some(directive) = VueVBindShorthandDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                {
                    let file_source =
                        embedded_file_source.with_embedding_kind(EmbeddingKind::Vue {
                            setup: false,
                            is_source: false,
                            event_handler: false,
                        });
                    if let Some((content, doc_source)) = parse_directive_string_value(
                        &initializer,
                        cache,
                        biome_path,
                        settings,
                        file_source,
                    ) {
                        nodes.push((content.into(), doc_source));
                    }
                }

                // Handle #slot shorthand (VueVSlotShorthandDirective)
                if let Some(directive) = VueVSlotShorthandDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                {
                    let file_source =
                        embedded_file_source.with_embedding_kind(EmbeddingKind::Vue {
                            setup: false,
                            is_source: false,
                            event_handler: false,
                        });
                    if let Some((content, doc_source)) = parse_directive_string_value(
                        &initializer,
                        cache,
                        biome_path,
                        settings,
                        file_source,
                    ) {
                        nodes.push((content.into(), doc_source));
                    }
                }

                // Handle full directives (v-on:, v-bind:, v-if, v-show, etc.)
                if let Some(directive) = VueDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                {
                    let is_v_on = directive
                        .name_token()
                        .map(|t| t.text_trimmed() == "v-on")
                        .unwrap_or(false);
                    let file_source =
                        embedded_file_source.with_embedding_kind(EmbeddingKind::Vue {
                            setup: false,
                            is_source: false,
                            event_handler: is_v_on,
                        });
                    if let Some((content, doc_source)) = parse_directive_string_value(
                        &initializer,
                        cache,
                        biome_path,
                        settings,
                        file_source,
                    ) {
                        nodes.push((content.into(), doc_source));
                    }
                }
            }
        }
        HtmlVariant::Svelte => {
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

            let mut embedded_file_source = JsFileSource::js_module();
            for element in elements {
                if let Some(script_type) = element.get_script_type() {
                    if script_type.is_javascript() {
                        let result = parse_embedded_script(
                            element.clone(),
                            cache,
                            biome_path,
                            &file_source,
                            settings,
                            builder,
                        );
                        if let Some((content, file_source)) = result {
                            embedded_file_source = file_source;
                            nodes.push((content.into(), file_source.into()));
                        }
                    } else if script_type.is_json() {
                        let result =
                            parse_embedded_json(element.clone(), cache, biome_path, settings);
                        if let Some((content, file_source)) = result {
                            nodes.push((content.into(), file_source));
                        }
                    }
                } else if element.is_style_tag() {
                    let result = parse_embedded_style(
                        element.clone(),
                        cache,
                        biome_path,
                        &file_source,
                        settings,
                    );
                    if let Some((content, services, file_source)) = result {
                        nodes.push(((content, services).into(), file_source));
                    }
                }
            }

            for snippet in snippet_expressions {
                let result = parse_svelte_text_expression(
                    snippet,
                    cache,
                    biome_path,
                    settings,
                    embedded_file_source,
                );

                if let Some((content, file_source)) = result {
                    nodes.push((content.into(), file_source));
                }
            }

            // Parse Svelte control flow block expressions ({#if}, {#each}, {#await}, {#key})
            for element in html_root.syntax().descendants() {
                let file_source = embedded_file_source
                    .with_embedding_kind(EmbeddingKind::Svelte { is_source: false });

                // Handle {#if expression}
                if let Some(if_block) = SvelteIfBlock::cast_ref(&element)
                    && let Ok(opening_block) = if_block.opening_block()
                    && let Ok(expression) = opening_block.expression()
                    && let Some((content, doc_source)) =
                        parse_text_expression(expression, cache, biome_path, settings, file_source)
                {
                    nodes.push((content.into(), doc_source));
                }

                // Handle {:else if expression}
                if let Some(if_block) = SvelteIfBlock::cast_ref(&element) {
                    for else_if_clause in if_block.else_if_clauses() {
                        if let Ok(expression) = else_if_clause.expression()
                            && let Some((content, doc_source)) = parse_text_expression(
                                expression,
                                cache,
                                biome_path,
                                settings,
                                file_source,
                            )
                        {
                            nodes.push((content.into(), doc_source));
                        }
                    }
                }

                // Handle {#each expression as item}
                if let Some(each_block) = SvelteEachBlock::cast_ref(&element)
                    && let Ok(opening_block) = each_block.opening_block()
                {
                    if let Ok(expression) = opening_block.list()
                        && let Some((content, doc_source)) = parse_text_expression(
                            expression,
                            cache,
                            biome_path,
                            settings,
                            file_source,
                        )
                    {
                        nodes.push((content.into(), doc_source));
                    }

                    if let Some(item) = opening_block.item()
                        && let Some(item) = item.as_svelte_each_as_keyed_item()
                        && let Some(key) = item.key()
                        && let Ok(key_expression) = key.expression()
                        && let Some((content, doc_source)) = parse_text_expression(
                            key_expression,
                            cache,
                            biome_path,
                            settings,
                            file_source,
                        )
                    {
                        nodes.push((content.into(), doc_source));
                    }
                }

                // Handle {#await expression}
                if let Some(await_block) = SvelteAwaitBlock::cast_ref(&element)
                    && let Ok(opening_block) = await_block.opening_block()
                    && let Ok(expression) = opening_block.expression()
                    && let Some((content, doc_source)) =
                        parse_text_expression(expression, cache, biome_path, settings, file_source)
                {
                    nodes.push((content.into(), doc_source));
                }

                // Handle {#key expression}
                if let Some(key_block) = SvelteKeyBlock::cast_ref(&element)
                    && let Ok(opening_block) = key_block.opening_block()
                    && let Ok(expression) = opening_block.expression()
                    && let Some((content, doc_source)) =
                        parse_text_expression(expression, cache, biome_path, settings, file_source)
                {
                    nodes.push((content.into(), doc_source));
                }
            }

            // Parse Svelte directive attributes (bind:, class:, use:, etc.)
            // Note: on: event handlers are legacy Svelte 3/4 syntax and not supported.
            // Svelte 5 runes mode uses regular attributes for event handlers.
            for element in html_root.syntax().descendants() {
                // Handle special Svelte directives (bind:, class:, etc.)
                if let Some(directive) = AnySvelteDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                {
                    let file_source = embedded_file_source
                        .with_embedding_kind(EmbeddingKind::Svelte { is_source: false });
                    if let Some((content, doc_source)) = parse_directive_text_expression(
                        &initializer,
                        cache,
                        biome_path,
                        settings,
                        file_source,
                    ) {
                        nodes.push((content.into(), doc_source));
                    }
                }
            }
        }
    }

    ParseEmbedResult { nodes }
}

pub(crate) fn parse_astro_embedded_script(
    element: AstroEmbeddedContent,
    cache: &mut NodeCache,
    path: &BiomePath,
    settings: &SettingsWithEditor,
    builder: &mut EmbeddedBuilder,
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
    builder.visit_js_source_snippet(&parse.tree());

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
    html_file_source: &HtmlFileSource,
    settings: &SettingsWithEditor,
    builder: &mut EmbeddedBuilder,
) -> Option<(EmbeddedSnippet<JsLanguage>, JsFileSource)> {
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
                file_source =
                    file_source.with_embedding_kind(EmbeddingKind::Svelte { is_source: true });
            } else if html_file_source.is_vue() {
                file_source = file_source.with_embedding_kind(EmbeddingKind::Vue {
                    setup: element.is_script_with_setup_attribute(),
                    is_source: true,
                    event_handler: false,
                });
            }
            file_source
        } else if html_file_source.is_astro() {
            // Astro script tags are parsed as regular TypeScript/JavaScript modules
            // They should not use EmbeddingKind::Astro because they are source code,
            // not template expressions
            JsFileSource::ts()
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

            builder.visit_js_source_snippet(&parse.tree());

            Some(EmbeddedSnippet::new(
                parse.into(),
                child.range(),
                content.text_range(),
                content.text_range().start(),
            ))
        })?;
        Some((embedded_content, file_source))
    } else {
        None
    }
}

/// Parses embedded style, but it skips it if it contains SASS language
pub(crate) fn parse_embedded_style(
    element: HtmlElement,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    html_file_source: &HtmlFileSource,
    settings: &SettingsWithEditor,
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

        let file_source = if html_file_source.is_html() {
            DocumentFileSource::Css(CssFileSource::css())
        } else {
            DocumentFileSource::Css(CssFileSource::new_css_modules())
        };
        let (snippet, services) = element.children().iter().next().and_then(|child| {
            let child = child.as_any_html_content()?;
            let child = child.as_html_embedded_content()?;
            let mut options = settings.parse_options::<CssLanguage>(biome_path, &file_source);
            if html_file_source.is_vue() {
                options.css_modules = CssModulesKind::Vue
            } else if !html_file_source.is_html() {
                options.css_modules = CssModulesKind::Classic
            }
            let content = child.value_token().ok()?;
            let parse = parse_css_with_offset_and_cache(
                content.text(),
                file_source.to_css_file_source().unwrap_or_default(),
                content.text_range().start(),
                cache,
                options,
            );

            let mut services = CssDocumentServices::default();
            if settings.as_ref().is_linter_enabled() || settings.as_ref().is_assist_enabled() {
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
    settings: &SettingsWithEditor,
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

pub(crate) fn parse_text_expression(
    expression: HtmlTextExpression,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &SettingsWithEditor,
    file_source: JsFileSource,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    let content = expression.html_literal_token().ok()?;
    let document_file_source = DocumentFileSource::Js(file_source);
    let options = settings.parse_options::<JsLanguage>(biome_path, &document_file_source);
    let parse = parse_js_with_offset_and_cache(
        content.text(),
        content.text_range().start(),
        file_source,
        options,
        cache,
    );

    let snippet = EmbeddedSnippet::new(
        parse.into(),
        expression.range(),
        content.text_range(),
        content.text_range().start(),
    );
    Some((snippet, document_file_source))
}

/// Parses Svelte single text expressions `{ expression }`
pub(crate) fn parse_svelte_text_expression(
    element: HtmlSingleTextExpression,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &SettingsWithEditor,
    file_source: JsFileSource,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    let expression = element.expression().ok()?;
    let file_source = file_source.with_embedding_kind(EmbeddingKind::Svelte { is_source: false });
    parse_text_expression(expression, cache, biome_path, settings, file_source)
}

pub(crate) fn parse_double_text_expression(
    element: HtmlDoubleTextExpression,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &SettingsWithEditor,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    let expression = element.expression().ok()?;
    let file_source = JsFileSource::js_module();
    parse_text_expression(expression, cache, biome_path, settings, file_source)
}

pub(crate) fn parse_single_text_expression(
    element: HtmlSingleTextExpression,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &SettingsWithEditor,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    let expression = element.expression().ok()?;
    let file_source = JsFileSource::js_module();
    parse_text_expression(expression, cache, biome_path, settings, file_source)
}

/// Parses Astro single text expressions `{ expression }`
pub(crate) fn parse_astro_text_expression(
    element: HtmlSingleTextExpression,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &SettingsWithEditor,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    let expression = element.expression().ok()?;
    let content = expression.html_literal_token().ok()?;
    // Astro is kinda weird in its JSX-like expressions. They are JS, but they contain HTML, not JSX.
    // That's because Astro doesn't parse what's inside the expressions, actually. In fact, their arrow function callbacks
    // don't have curly brackets.
    //
    // As for now, we use the TSX parser, hoping it won't bite us back in the future.
    let file_source =
        JsFileSource::tsx().with_embedding_kind(EmbeddingKind::Astro { frontmatter: false });
    let document_file_source = DocumentFileSource::Js(file_source);
    let options = settings.parse_options::<JsLanguage>(biome_path, &document_file_source);
    let parse = parse_js_with_offset_and_cache(
        content.text(),
        content.text_range().start(),
        file_source,
        options,
        cache,
    );
    let snippet = EmbeddedSnippet::new(
        parse.into(),
        expression.range(),
        content.text_range(),
        content.text_range().start(),
    );
    Some((snippet, document_file_source))
}

/// Parses Vue double text expressions `{{ expression }}`
pub(crate) fn parse_vue_text_expression(
    element: HtmlDoubleTextExpression,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &SettingsWithEditor,
    js_file_source: JsFileSource,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    let expression = element.expression().ok()?;
    let file_source = js_file_source.with_embedding_kind(EmbeddingKind::Vue {
        setup: false,
        is_source: false,
        event_handler: false,
    });
    parse_text_expression(expression, cache, biome_path, settings, file_source)
}

/// Parses a directive attribute's string value as JavaScript
///
/// Extracts the JavaScript expression from a Vue/Svelte directive attribute value
/// (e.g., `@click="handler()"` -> `handler()`) and parses it as an embedded JavaScript snippet.
///
/// The function:
/// 1. Extracts the attribute initializer clause (`="value"`)
/// 2. Gets the HTML string node from the initializer
/// 3. Uses `inner_string_text()` to extract the content without quotes
/// 4. Parses the content as JavaScript with the correct offset
/// 5. Returns an `EmbeddedSnippet` with proper range information
fn parse_directive_string_value(
    value: &HtmlAttributeInitializerClause,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &SettingsWithEditor,
    file_source: JsFileSource,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    // Get the HTML string from the initializer (e.g., `"handler()"`)
    let value_node = value.value().ok()?;
    let html_string = value_node.as_html_string()?;

    // Get the token and extract inner text (without quotes)
    let content_token = html_string.value_token().ok()?;
    let inner_text = html_string.inner_string_text().ok()?;
    let text = inner_text.text();

    // Calculate offset: start of token + 1 (for opening quote)
    let token_range = content_token.text_trimmed_range();
    let inner_offset = token_range.start() + TextSize::from(1);

    // Parse as JavaScript
    let document_file_source = DocumentFileSource::Js(file_source);
    let options = settings.parse_options::<JsLanguage>(biome_path, &document_file_source);
    let parse = parse_js_with_offset_and_cache(text, inner_offset, file_source, options, cache);

    // Create snippet with proper ranges
    let snippet = EmbeddedSnippet::new(
        parse.into(),
        value.range(), // Full attribute range
        token_range,   // Token range (string with quotes)
        inner_offset,  // Offset where JS starts (after opening quote)
    );

    Some((snippet, document_file_source))
}

/// Parses a Svelte directive attribute's text expression value as JavaScript
///
/// Extracts the JavaScript expression from a Svelte directive attribute value
/// (e.g., `on:click={handler}` -> `handler`) and parses it as an embedded JavaScript snippet.
///
/// Unlike Vue which uses quoted strings, Svelte uses curly braces with text expressions.
fn parse_directive_text_expression(
    value: &HtmlAttributeInitializerClause,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &SettingsWithEditor,
    file_source: JsFileSource,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    // Get the text expression from the initializer (e.g., `{handler}`)
    let value_node = value.value().ok()?;
    let text_expression = value_node.as_html_attribute_single_text_expression()?;

    // Extract the expression inside the curly braces
    let expression = text_expression.expression().ok()?;

    // Parse as JavaScript using the same logic as Svelte text expressions
    let document_file_source = DocumentFileSource::Js(file_source);
    parse_text_expression(expression, cache, biome_path, settings, file_source)
        .map(|(snippet, _)| (snippet, document_file_source))
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
    let (_, analyze_diagnostics) =
        analyze(&tree, filter, &analyzer_options, source_type, |signal| {
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
    let Some(source_type) = language.to_html_file_source() else {
        error!("Could not determine the HTML file source of the file");
        return PullActionsResult {
            actions: Vec::new(),
        };
    };
    let analyzer_options =
        settings.analyzer_options::<HtmlLanguage>(path, &language, suppression_reason.as_deref());
    let mut actions = Vec::new();
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(settings.as_ref(), analyzer_options)
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

    analyze(&tree, filter, &analyzer_options, source_type, |signal| {
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
    let rules = params
        .settings
        .as_ref()
        .as_linter_rules(params.biome_path.as_path());
    let analyzer_options = params.settings.analyzer_options::<HtmlLanguage>(
        params.biome_path,
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
        let (action, _) = analyze(&tree, filter, &analyzer_options, source_type, |signal| {
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
                        // NOTE: this is important that stays false. In this instance, the formatting of embedded
                        // nodes has already happened, because the workspace during fix_all() process the embedded nodes
                        // first, and then the root document. This means the embedded nodes don't need to be formatted and can
                        // be printed verbatim by the formatter.
                        false,
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
