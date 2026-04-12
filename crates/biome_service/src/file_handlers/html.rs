mod parse_embedded_nodes;

use super::{
    AnalyzerCapabilities, AnalyzerVisitorBuilder, AnalyzerVisitorResult, Capabilities,
    CodeActionsParams, DebugCapabilities, DocumentFileSource, EnabledForPath, ExtensionHandler,
    FixAllParams, FormatEmbedNode, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, ProcessFixAll, ProcessLint, SearchCapabilities, UpdateSnippetsNodes,
};
use crate::configuration::to_analyzer_rules;
use crate::file_handlers::html::parse_embedded_nodes::parse_embedded_nodes;
use crate::settings::{
    OverrideSettings, SettingsWithEditor, check_feature_activity, check_override_feature_activity,
};
use crate::workspace::CodeAction;
use crate::workspace::FixFileMode;
use crate::workspace::document::AnyEmbeddedSnippet;
use crate::workspace::document::services::embedded_bindings::EmbeddedBuilder;
use crate::workspace::{FixFileResult, PullActionsResult};
use crate::{
    WorkspaceError,
    settings::{ServiceLanguage, Settings},
    workspace::GetSyntaxTreeResult,
};
use biome_analyze::{
    ActionFilter, AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never,
};
use biome_configuration::html::{
    HtmlAssistConfiguration, HtmlAssistEnabled, HtmlFormatterConfiguration, HtmlFormatterEnabled,
    HtmlLinterConfiguration, HtmlLinterEnabled, HtmlParseInterpolation, HtmlParserConfiguration,
};
use biome_css_syntax::CssLanguage;
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
use biome_html_syntax::{HtmlFileSource, HtmlLanguage, HtmlRoot, HtmlSyntaxNode};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_json_syntax::JsonLanguage;
use biome_parser::AnyParse;
use biome_rowan::{AstNode, BatchMutation, NodeCache, SendNode};
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

/// Result of parsing a matched embed.
struct ParsedEmbed {
    /// The parsed snippet + file source, ready to push to `nodes`.
    node: (AnyEmbeddedSnippet, DocumentFileSource),
    /// If JS was parsed, the resolved JsFileSource (for `embedded_file_source` capture).
    js_file_source: Option<JsFileSource>,
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

    let AnalyzerVisitorResult {
        enabled_rules,
        disabled_rules,
        analyzer_options,
        ..
    } = AnalyzerVisitorBuilder::new(params.settings.as_ref(), analyzer_options)
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
        compute_actions,
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
    let AnalyzerVisitorResult {
        enabled_rules,
        disabled_rules,
        analyzer_options,
        ..
    } = AnalyzerVisitorBuilder::new(settings.as_ref(), analyzer_options)
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
            if compute_actions {
                actions.extend(
                    signal
                        .actions(ActionFilter::all())
                        .into_code_action_iter()
                        .map(|item| CodeAction {
                            category: item.category.clone(),
                            rule_name: item
                                .rule_name
                                .map(|(group, name)| (Cow::Borrowed(group), Cow::Borrowed(name))),
                            applicability: Some(item.suggestion.applicability),
                            suggestion: Some(item.suggestion),
                            offset: action_offset,
                        }),
                );
            } else {
                actions.extend(signal.actions_metadata().into_iter().map(|meta| {
                    CodeAction {
                        category: meta.category,
                        rule_name: meta
                            .rule_name
                            .map(|(g, r)| (Cow::Borrowed(g), Cow::Borrowed(r))),
                        applicability: Some(meta.applicability),
                        suggestion: None,
                        offset: action_offset,
                    }
                }));
            }

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
    let AnalyzerVisitorResult {
        enabled_rules,
        disabled_rules,
        analyzer_options,
        fixable_rules,
    } = AnalyzerVisitorBuilder::new(params.settings.as_ref(), analyzer_options)
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

    if matches!(params.fix_file_mode, FixFileMode::ApplySuppressions) {
        loop {
            let mut pending_actions = Vec::new();
            let html_services = HtmlAnalyzerServices {
                module_graph: Some(params.module_graph.clone()),
                project_layout: Some(params.project_layout.clone()),
            };

            let (_, _) = analyze(
                &tree,
                filter,
                &analyzer_options,
                source_type,
                html_services,
                |signal| process_fix_all.collect_signal(signal, &mut pending_actions),
            );

            let result = process_fix_all.process_batch_actions(pending_actions, |root| {
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

    // Phase 1: fix loop with fixable-only rules
    let fixable_filter = AnalysisFilter {
        categories: params.rule_categories,
        enabled_rules: Some(fixable_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    loop {
        let mut pending_actions = Vec::new();
        let html_services = HtmlAnalyzerServices {
            module_graph: Some(params.module_graph.clone()),
            project_layout: Some(params.project_layout.clone()),
        };

        let (_, _) = analyze(
            &tree,
            fixable_filter,
            &analyzer_options,
            source_type,
            html_services,
            |signal| process_fix_all.collect_signal_fixes_only(signal, &mut pending_actions),
        );

        let result = process_fix_all.process_batch_actions(pending_actions, |root| {
            tree = match HtmlRoot::cast(root) {
                Some(tree) => tree,
                None => return None,
            };
            Some(tree.syntax().text_range_with_trivia().len().into())
        })?;

        if result.is_none() {
            break;
        }
    }

    // Phase 2: all rules for final diagnostics
    {
        let html_services = HtmlAnalyzerServices {
            module_graph: Some(params.module_graph.clone()),
            project_layout: Some(params.project_layout.clone()),
        };
        let (_, _) = analyze(
            &tree,
            filter,
            &analyzer_options,
            source_type,
            html_services,
            |signal| process_fix_all.collect_diagnostic_only(signal),
        );
    }

    process_fix_all.finish(
        || {
            Ok(if params.should_format {
                Either::Left(format_node(
                    params.settings.format_options::<HtmlLanguage>(
                        params.biome_path,
                        &params.document_file_source,
                    ),
                    tree.syntax(),
                    false,
                ))
            } else {
                Either::Right(tree.syntax().to_string())
            })
        },
        params.embeds_initial_indent,
    )
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
            let new_token_text = if snippet.needs_reindent {
                // The formatted code doesn't carry the host's nesting
                // indentation. Re-apply it to every line so the embed
                // lines up with its surroundings.
                let old_text = value_token.text_trimmed();
                let leading_trivia = read_leading_trivia(old_text);
                let trailing_trivia = read_trailing_trivia(old_text);
                let indent_prefix = content_indent_prefix(&leading_trivia);
                let reindented = reindent_embedded_code(snippet.new_code.trim(), indent_prefix);
                format!("{}{}{}", leading_trivia, reindented, trailing_trivia)
            } else {
                snippet.new_code.clone()
            };

            mutation.replace_token(value_token, ident(&new_token_text));
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

/// Returns the indent the host was using for an embed's content, taken
/// from the whitespace after the last newline in its leading trivia.
///
/// For example, `"\n\t\t\t"` yields `"\t\t\t"`. Used to re-indent
/// replacement code coming from an embedded formatter, which always
/// returns code indented from column zero.
fn content_indent_prefix(leading_trivia: &str) -> &str {
    match leading_trivia.rfind('\n') {
        Some(pos) => &leading_trivia[pos + 1..],
        None => leading_trivia,
    }
}

/// Prefixes every line of `code` after the first with `indent`. Empty
/// lines are left alone so no trailing whitespace sneaks in.
fn reindent_embedded_code(code: &str, indent: &str) -> String {
    if indent.is_empty() {
        return code.to_string();
    }
    let mut out = String::new();
    for (i, line) in code.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
            if !line.is_empty() {
                out.push_str(indent);
            }
        }
        out.push_str(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{content_indent_prefix, reindent_embedded_code};

    #[test]
    fn content_indent_prefix_reads_indent_after_last_newline() {
        assert_eq!(content_indent_prefix("\n\t\t\t"), "\t\t\t");
        assert_eq!(content_indent_prefix("\n  "), "  ");
        assert_eq!(content_indent_prefix("\n\n\t"), "\t");
    }

    #[test]
    fn content_indent_prefix_with_no_newline_returns_whole_trivia() {
        assert_eq!(content_indent_prefix(""), "");
        assert_eq!(content_indent_prefix("   "), "   ");
    }

    #[test]
    fn reindent_embedded_code_prefixes_every_line_after_the_first() {
        assert_eq!(
            reindent_embedded_code("p {\n\tcolor: red;\n}", "\t\t\t"),
            "p {\n\t\t\t\tcolor: red;\n\t\t\t}"
        );
    }

    #[test]
    fn reindent_embedded_code_is_a_noop_when_indent_is_empty() {
        assert_eq!(reindent_embedded_code("a\nb\nc", ""), "a\nb\nc");
    }

    #[test]
    fn reindent_embedded_code_leaves_single_line_input_unchanged() {
        assert_eq!(reindent_embedded_code("oneline", "\t\t"), "oneline");
    }

    #[test]
    fn reindent_embedded_code_does_not_indent_empty_lines() {
        assert_eq!(reindent_embedded_code("a\n\nb", "  "), "a\n\n  b");
    }
}
