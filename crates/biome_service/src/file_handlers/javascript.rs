use super::{
    AnalyzerCapabilities, AnalyzerVisitorBuilder, CodeActionsParams, DebugCapabilities,
    DiagnosticsAndActionsParams, EnabledForPath, ExtensionHandler, FormatEmbedNode,
    FormatterCapabilities, LintParams, LintResults, ParseEmbedResult, ParseResult,
    ParserCapabilities, ProcessDiagnosticsAndActions, ProcessFixAll, ProcessLint,
    SearchCapabilities, UpdateSnippetsNodes, search,
};
use crate::configuration::to_analyzer_rules;
use crate::diagnostics::extension_error;
use crate::file_handlers::FixAllParams;
use crate::settings::{
    OverrideSettings, Settings, SettingsWithEditor, check_feature_activity,
    check_override_feature_activity,
};
use crate::workspace::document::services::embedded_bindings::EmbeddedBuilder;
use crate::workspace::{DocumentFileSource, EmbeddedSnippet, PullDiagnosticsAndActionsResult};
use crate::{
    WorkspaceError,
    settings::{FormatSettings, LanguageListSettings, LanguageSettings, ServiceLanguage},
    workspace::{CodeAction, FixFileResult, GetSyntaxTreeResult, PullActionsResult, RenameResult},
};
use biome_analyze::options::{PreferredIndentation, PreferredQuote};
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never, QueryMatch,
    RuleCategoriesBuilder, RuleFilter,
};
use biome_configuration::javascript::{
    JsAssistConfiguration, JsAssistEnabled, JsFormatterConfiguration, JsFormatterEnabled,
    JsGritMetavariable, JsLinterConfiguration, JsLinterEnabled, JsParserConfiguration,
    JsxEverywhere, JsxRuntime, UnsafeParameterDecoratorsEnabled,
};
use biome_css_parser::parse_css_with_offset_and_cache;
use biome_css_syntax::{CssFileSource, CssLanguage, EmbeddingKind};
use biome_formatter::prelude::{Document, Interned, LineMode, Tag};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, Expand, FormatElement, FormatError,
    IndentStyle, IndentWidth, LineEnding, LineWidth, Printed, QuoteStyle, TrailingNewline,
};
use biome_fs::BiomePath;
use biome_graphql_parser::parse_graphql_with_offset_and_cache;
use biome_graphql_syntax::{GraphqlFileSource, GraphqlLanguage};
use biome_js_analyze::utils::rename::{RenameError, RenameSymbolExtensions};
use biome_js_analyze::{
    ControlFlowGraph, JsAnalyzerServices, analyze, analyze_with_inspect_matcher,
};
use biome_js_factory::make::ident;
use biome_js_formatter::context::trailing_commas::TrailingCommas;
use biome_js_formatter::context::{
    ArrowParentheses, JsFormatOptions, OperatorLinebreak, QuoteProperties, Semicolons,
};
use biome_js_formatter::format_node;
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::{
    AnyJsExpression, AnyJsRoot, AnyJsTemplateElement, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsClassDeclaration, JsClassExpression, JsFileSource, JsFunctionDeclaration,
    JsLanguage, JsSyntaxNode, JsTemplateChunkElement, JsTemplateExpression, JsVariableDeclarator,
    TextRange, TextSize, TokenAtOffset,
};
use biome_js_type_info::{GlobalsResolver, ScopeId, TypeData, TypeResolver};
use biome_module_graph::ModuleGraph;
use biome_parser::AnyParse;
use biome_rowan::{
    AstNode, AstNodeList, BatchMutation, BatchMutationExt, Direction, NodeCache, SendNode,
    WalkEvent,
};
use camino::Utf8Path;
use either::Either;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;
use tracing::{debug, debug_span, error, instrument, trace_span};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsFormatterSettings {
    pub quote_style: Option<QuoteStyle>,
    pub jsx_quote_style: Option<QuoteStyle>,
    pub quote_properties: Option<QuoteProperties>,
    pub trailing_commas: Option<TrailingCommas>,
    pub semicolons: Option<Semicolons>,
    pub arrow_parentheses: Option<ArrowParentheses>,
    pub bracket_spacing: Option<BracketSpacing>,
    pub bracket_same_line: Option<BracketSameLine>,
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub enabled: Option<JsFormatterEnabled>,
    pub attribute_position: Option<AttributePosition>,
    pub expand: Option<Expand>,
    pub operator_linebreak: Option<OperatorLinebreak>,
    pub trailing_newline: Option<TrailingNewline>,
}

impl From<JsFormatterConfiguration> for JsFormatterSettings {
    fn from(value: JsFormatterConfiguration) -> Self {
        Self {
            quote_style: value.quote_style,
            jsx_quote_style: value.jsx_quote_style,
            quote_properties: value.quote_properties,
            trailing_commas: value.trailing_commas,
            semicolons: value.semicolons,
            arrow_parentheses: value.arrow_parentheses,
            bracket_same_line: value.bracket_same_line,
            enabled: value.enabled,
            line_width: value.line_width,
            bracket_spacing: value.bracket_spacing,
            attribute_position: value.attribute_position,
            indent_width: value.indent_width,
            indent_style: value.indent_style,
            line_ending: value.line_ending,
            expand: value.expand,
            operator_linebreak: value.operator_linebreak,
            trailing_newline: value.trailing_newline,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsParserSettings {
    pub parse_class_parameter_decorators: Option<UnsafeParameterDecoratorsEnabled>,
    pub grit_metavariables: Option<JsGritMetavariable>,
    pub jsx_everywhere: Option<JsxEverywhere>,
}

impl From<JsParserConfiguration> for JsParserSettings {
    fn from(value: JsParserConfiguration) -> Self {
        Self {
            parse_class_parameter_decorators: value.unsafe_parameter_decorators_enabled,
            grit_metavariables: value.grit_metavariables,
            jsx_everywhere: value.jsx_everywhere,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsLinterSettings {
    pub enabled: Option<JsLinterEnabled>,
    pub suppression_reason: Option<String>,
}

impl From<JsLinterConfiguration> for JsLinterSettings {
    fn from(value: JsLinterConfiguration) -> Self {
        Self {
            enabled: value.enabled,
            suppression_reason: None,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsAssistSettings {
    pub enabled: Option<JsAssistEnabled>,
    pub suppression_reason: Option<String>,
}

impl From<JsAssistConfiguration> for JsAssistSettings {
    fn from(value: JsAssistConfiguration) -> Self {
        Self {
            enabled: value.enabled,
            suppression_reason: None,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsEnvironmentSettings {
    pub jsx_runtime: Option<JsxRuntime>,
}

impl From<JsxRuntime> for JsEnvironmentSettings {
    fn from(jsx_runtime: JsxRuntime) -> Self {
        Self {
            jsx_runtime: Some(jsx_runtime),
        }
    }
}

impl ServiceLanguage for JsLanguage {
    type FormatterSettings = JsFormatterSettings;
    type LinterSettings = JsLinterSettings;
    type AssistSettings = JsAssistSettings;
    type FormatOptions = JsFormatOptions;
    type ParserSettings = JsParserSettings;
    type ParserOptions = JsParserOptions;

    type EnvironmentSettings = JsEnvironmentSettings;

    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self> {
        &languages.javascript
    }

    fn resolve_environment(global: &Settings) -> Option<&Self::EnvironmentSettings> {
        Some(&global.languages.javascript.environment)
    }

    fn resolve_parse_options(
        overrides: &OverrideSettings,
        language: &Self::ParserSettings,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> Self::ParserOptions {
        let mut options = JsParserOptions {
            grit_metavariables: false,
            parse_class_parameter_decorators: language
                .parse_class_parameter_decorators
                .unwrap_or_default()
                .into(),
        };

        overrides.apply_override_js_parser_options(path, &mut options);

        options
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &JsFormatterSettings,
        path: &BiomePath,
        document_file_source: &DocumentFileSource,
    ) -> JsFormatOptions {
        let options = JsFormatOptions::new(
            document_file_source
                .to_js_file_source()
                .or(JsFileSource::try_from(path.as_path()).ok())
                .unwrap_or_default(),
        )
        .with_indent_style(
            language
                .indent_style
                .or(global.indent_style)
                .unwrap_or_default(),
        )
        .with_indent_width(
            language
                .indent_width
                .or(global.indent_width)
                .unwrap_or_default(),
        )
        .with_line_width(
            language
                .line_width
                .or(global.line_width)
                .unwrap_or_default(),
        )
        .with_line_ending(
            language
                .line_ending
                .or(global.line_ending)
                .unwrap_or_default(),
        )
        .with_quote_style(language.quote_style.unwrap_or_default())
        .with_jsx_quote_style(language.jsx_quote_style.unwrap_or_default())
        .with_quote_properties(language.quote_properties.unwrap_or_default())
        .with_trailing_commas(language.trailing_commas.unwrap_or_default())
        .with_semicolons(language.semicolons.unwrap_or_default())
        .with_arrow_parentheses(language.arrow_parentheses.unwrap_or_default())
        .with_bracket_spacing(
            language
                .bracket_spacing
                .or(global.bracket_spacing)
                .unwrap_or_default(),
        )
        .with_bracket_same_line(
            language
                .bracket_same_line
                .or(global.bracket_same_line)
                .unwrap_or_default(),
        )
        .with_attribute_position(
            language
                .attribute_position
                .or(global.attribute_position)
                .unwrap_or_default(),
        )
        .with_expand(language.expand.or(global.expand).unwrap_or_default())
        .with_trailing_newline(
            language
                .trailing_newline
                .or(global.trailing_newline)
                .unwrap_or_default(),
        )
        .with_operator_linebreak(language.operator_linebreak.unwrap_or_default());

        overrides.override_js_format_options(path, options)
    }

    fn resolve_analyzer_options(
        global: &Settings,
        _language: &Self::LinterSettings,
        environment: Option<&Self::EnvironmentSettings>,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        let preferred_quote = global
            .languages
            .javascript
            .formatter
            .quote_style
            .map(|quote_style: QuoteStyle| match quote_style {
                QuoteStyle::Single => PreferredQuote::Single,
                QuoteStyle::Double => PreferredQuote::Double,
            })
            .unwrap_or_default();
        let preferred_jsx_quote = global
            .languages
            .javascript
            .formatter
            .jsx_quote_style
            .map(|quote_style: QuoteStyle| match quote_style {
                QuoteStyle::Single => PreferredQuote::Single,
                QuoteStyle::Double => PreferredQuote::Double,
            })
            .unwrap_or_default();
        let preferred_indentation = {
            let indent_style = global
                .languages
                .javascript
                .formatter
                .indent_style
                .unwrap_or_else(|| global.formatter.indent_style.unwrap_or_default());
            match indent_style {
                IndentStyle::Tab => PreferredIndentation::Tab,
                IndentStyle::Space => PreferredIndentation::Spaces(
                    global
                        .languages
                        .javascript
                        .formatter
                        .indent_width
                        .unwrap_or_else(|| global.formatter.indent_width.unwrap_or_default())
                        .value(),
                ),
            }
        };

        let mut configuration = AnalyzerConfiguration::default();
        let mut globals = Vec::new();
        let overrides = &global.override_settings;
        let jsx_runtime = match overrides.override_jsx_runtime(
            path,
            environment
                .and_then(|env| env.jsx_runtime)
                .unwrap_or_default(),
        ) {
            // In the future, we may wish to map an `Auto` variant to a concrete
            // analyzer value for easy access by the analyzer.
            JsxRuntime::Transparent => biome_analyze::options::JsxRuntime::Transparent,
            JsxRuntime::ReactClassic => biome_analyze::options::JsxRuntime::ReactClassic,
        };
        configuration = configuration.with_jsx_runtime(jsx_runtime);

        globals.extend(overrides.override_js_globals(path, &global.languages.javascript.globals));

        if let Some(filename) = path.file_name() {
            if filename.ends_with(".vue") {
                globals.extend(
                    [
                        "defineEmits",
                        "defineExpose",
                        "defineModel",
                        "defineOptions",
                        "defineProps",
                        "defineSlots",
                        "withDefaults",
                    ]
                    .map(Into::into),
                );
            } else if filename.ends_with(".astro") {
                globals.extend(["Astro"].map(Into::into));
            } else if filename.ends_with(".svelte")
                || filename.ends_with(".svelte.js")
                || filename.ends_with(".svelte.ts")
                || filename.ends_with(".svelte.test.ts")
                || filename.ends_with(".svelte.test.js")
                || filename.ends_with(".svelte.spec.ts")
                || filename.ends_with(".svelte.spec.js")
            {
                // Svelte 5 runes
                globals.extend(
                    [
                        "$bindable",
                        "$derived",
                        "$effect",
                        "$host",
                        "$inspect",
                        "$props",
                        "$state",
                    ]
                    .map(Into::into),
                );
            }
        }

        let configuration = configuration
            .with_rules(to_analyzer_rules(global, path.as_path()))
            .with_globals(globals)
            .with_preferred_quote(preferred_quote)
            .with_preferred_jsx_quote(preferred_jsx_quote)
            .with_preferred_indentation(preferred_indentation);

        AnalyzerOptions::default()
            .with_file_path(path.as_path())
            .with_configuration(configuration)
            .with_suppression_reason(suppression_reason)
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
                        pattern.languages.javascript.linter.enabled,
                        pattern.linter.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.javascript.linter.enabled,
                settings.linter.enabled,
            ))
            .unwrap_or_default()
            .into()
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
                        pattern.languages.javascript.formatter.enabled,
                        pattern.formatter.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.javascript.formatter.enabled,
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
                        pattern.languages.javascript.assist.enabled,
                        pattern.assist.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.javascript.assist.enabled,
                settings.assist.enabled,
            ))
            .unwrap_or_default()
            .into()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct JsFileHandler;

impl ExtensionHandler for JsFileHandler {
    fn capabilities(&self) -> super::Capabilities {
        super::Capabilities {
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
                debug_control_flow: Some(debug_control_flow),
                debug_formatter_ir: Some(debug_formatter_ir),
                debug_type_info: Some(debug_type_info),
                debug_registered_types: Some(debug_registered_types),
                debug_semantic_model: Some(debug_semantic_model),
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                fix_all: Some(fix_all),
                rename: Some(rename),
                update_snippets: Some(update_snippets),
                pull_diagnostics_and_actions: Some(pull_diagnostics_and_actions),
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
                format_embedded: Some(format_embedded),
            },
            search: SearchCapabilities {
                search: Some(search),
            },
        }
    }
}

pub fn formatter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.formatter_enabled_for_file_path::<JsLanguage>(path)
}

pub fn linter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.linter_enabled_for_file_path::<JsLanguage>(path)
}

pub fn assist_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.assist_enabled_for_file_path::<JsLanguage>(path)
}

pub fn search_enabled(_path: &Utf8Path, _settings: &SettingsWithEditor) -> bool {
    true
}

fn parse(
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    settings: &SettingsWithEditor,
    cache: &mut NodeCache,
) -> ParseResult {
    let options = settings.parse_options::<JsLanguage>(biome_path, &file_source);

    let file_source = file_source.to_js_file_source().unwrap_or_default();
    let parse = biome_js_parser::parse_js_with_cache(text, file_source, options, cache);
    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source.into()),
    }
}

fn parse_embedded_nodes(
    root: &AnyParse,
    biome_path: &BiomePath,
    _file_source: &DocumentFileSource,
    settings: &SettingsWithEditor,
    cache: &mut NodeCache,
    _builder: &mut EmbeddedBuilder,
) -> ParseEmbedResult {
    if !settings
        .as_ref()
        .experimental_js_embedded_snippets_enabled()
    {
        return ParseEmbedResult { nodes: vec![] };
    }

    let mut nodes = Vec::new();
    let js_root: AnyJsRoot = root.tree();

    // Walk through all JS elements looking for template expressions
    for node in js_root.syntax().descendants() {
        let Some(expr) = JsTemplateExpression::cast_ref(&node) else {
            continue;
        };

        if let Some((content, file_source)) =
            parse_template_expression(expr, cache, biome_path, settings)
        {
            nodes.push((content.into(), file_source))
        }
    }

    ParseEmbedResult { nodes }
}

fn parse_template_expression(
    expr: JsTemplateExpression,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &SettingsWithEditor,
) -> Option<(EmbeddedSnippet<JsLanguage>, DocumentFileSource)> {
    // TODO: Interpolations are not supported yet.
    if expr.elements().len() != 1 {
        return None;
    }

    let Some(AnyJsTemplateElement::JsTemplateChunkElement(chunk)) = expr.elements().first() else {
        return None;
    };

    let tag = expr.tag();
    if is_styled_tag(tag.as_ref()) {
        let file_source = DocumentFileSource::Css(
            CssFileSource::css().with_embedding_kind(EmbeddingKind::Styled),
        );
        let options = settings.parse_options::<CssLanguage>(biome_path, &file_source);
        let content = chunk.template_chunk_token().ok()?;
        let parse = parse_css_with_offset_and_cache(
            content.text(),
            file_source.to_css_file_source().unwrap_or_default(),
            content.text_range().start(),
            cache,
            options,
        );

        let snippet = EmbeddedSnippet::new(
            parse.into(),
            chunk.range(),
            content.text_range(),
            content.text_range().start(),
        );

        Some((snippet, file_source))
    } else if is_graphql_tag(tag.as_ref(), &expr) {
        let file_source = DocumentFileSource::Graphql(GraphqlFileSource::graphql());
        let content = chunk.template_chunk_token().ok()?;
        let parse = parse_graphql_with_offset_and_cache(
            content.text(),
            content.text_range().start(),
            cache,
        );

        let snippet = EmbeddedSnippet::new(
            parse.into(),
            chunk.range(),
            content.text_range(),
            content.text_range().start(),
        );

        Some((snippet, file_source))
    } else {
        None
    }
}

fn is_styled_tag(tag: Option<&AnyJsExpression>) -> bool {
    // css``
    if let Some(AnyJsExpression::JsIdentifierExpression(ident)) = tag
        && ident.name().is_ok_and(|name| name.has_name("css"))
    {
        return true;
    }

    // styled.div``
    if let Some(AnyJsExpression::JsStaticMemberExpression(expr)) = tag
        && let Ok(AnyJsExpression::JsIdentifierExpression(ident)) = expr.object()
        && ident.name().is_ok_and(|name| name.has_name("styled"))
    {
        return true;
    }

    // styled(Component)``
    if let Some(AnyJsExpression::JsCallExpression(expr)) = tag
        && let Ok(AnyJsExpression::JsIdentifierExpression(ident)) = expr.callee()
        && ident.name().is_ok_and(|name| name.has_name("styled"))
    {
        return true;
    }

    false
}

fn is_graphql_tag(tag: Option<&AnyJsExpression>, template: &JsTemplateExpression) -> bool {
    // gql`` or graphql``
    if let Some(AnyJsExpression::JsIdentifierExpression(ident)) = tag
        && ident
            .name()
            .is_ok_and(|name| name.has_name("gql") || name.has_name("graphql"))
    {
        return true;
    }

    // graphql(``)
    if let Some(list) = template.parent::<JsCallArgumentList>()
        && let Some(args) = list.parent::<JsCallArguments>()
        && let Some(call) = args.parent::<JsCallExpression>()
        && let Ok(AnyJsExpression::JsIdentifierExpression(ident)) = call.callee()
        && ident.name().is_ok_and(|name| name.has_name("graphql"))
    {
        return true;
    }

    false
}

fn debug_syntax_tree(_rome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: JsSyntaxNode = parse.syntax();
    let tree: AnyJsRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_control_flow(parse: AnyParse, cursor: TextSize) -> String {
    let mut control_flow_graph = None;

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("correctness", "noUnreachable")]),
        ..AnalysisFilter::default()
    };
    let options = AnalyzerOptions::default();

    analyze_with_inspect_matcher(
        &parse.tree(),
        filter,
        |match_params| {
            let cfg = match match_params.query.downcast_ref::<ControlFlowGraph>() {
                Some(cfg) => cfg,
                _ => return,
            };

            let range = cfg.text_range();
            if !range.contains(cursor) {
                return;
            }

            match &control_flow_graph {
                None => {
                    control_flow_graph = Some((cfg.graph.to_string(), range));
                }
                Some((_, prev_range)) => {
                    if range.len() < prev_range.len() {
                        control_flow_graph = Some((cfg.graph.to_string(), range));
                    }
                }
            }
        },
        &options,
        &[],
        Default::default(),
        |_| ControlFlow::<Never>::Continue(()),
    );

    control_flow_graph.map(|(cfg, _)| cfg).unwrap_or_default()
}

fn debug_formatter_ir(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree, false)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

fn debug_type_info(
    path: &BiomePath,
    parse: Option<AnyParse>,
    graph: Arc<ModuleGraph>,
) -> Result<String, WorkspaceError> {
    let Some(parse) = parse else {
        let result = graph.js_module_info_for_path(path);
        return match result {
            None => Ok(String::new()),
            // TODO: print correct type info
            Some(module_info) => {
                let mut result = String::new();
                for ty in module_info.types() {
                    result.push_str(format!("{ty}\n").as_str());
                }
                Ok(result)
            }
        };
    };
    let tree: AnyJsRoot = parse.tree();
    let mut result = String::new();
    let preorder = tree.syntax().preorder();

    let mut resolver = GlobalsResolver::default();
    let scope_id = ScopeId::GLOBAL;

    for event in preorder {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(node) = JsVariableDeclarator::cast_ref(&node) {
                    if let Some(ty) =
                        TypeData::from_js_variable_declarator(&mut resolver, scope_id, &node)
                    {
                        result.push_str(&ty.to_string());
                        result.push('\n');
                    }
                } else if let Some(function) = JsFunctionDeclaration::cast_ref(&node) {
                    result.push_str(
                        &TypeData::from_js_function_declaration(&mut resolver, scope_id, &function)
                            .to_string(),
                    );
                    result.push('\n');
                } else if let Some(class) = JsClassDeclaration::cast_ref(&node) {
                    result.push_str(
                        &TypeData::from_js_class_declaration(&mut resolver, scope_id, &class)
                            .to_string(),
                    );
                    result.push('\n');
                } else if let Some(expression) = JsClassExpression::cast_ref(&node) {
                    result.push_str(
                        &TypeData::from_js_class_expression(&mut resolver, scope_id, &expression)
                            .to_string(),
                    );
                    result.push('\n');
                }
            }
            WalkEvent::Leave(_) => {}
        }
    }

    Ok(result)
}

fn debug_registered_types(_path: &BiomePath, parse: AnyParse) -> Result<String, WorkspaceError> {
    let tree: AnyJsRoot = parse.tree();
    let mut result = String::new();
    let preorder = tree.syntax().preorder();

    let mut resolver = GlobalsResolver::default();
    let scope_id = ScopeId::GLOBAL;

    for event in preorder {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(node) = JsVariableDeclarator::cast_ref(&node) {
                    TypeData::from_js_variable_declarator(&mut resolver, scope_id, &node);
                } else if let Some(function) = JsFunctionDeclaration::cast_ref(&node) {
                    TypeData::from_js_function_declaration(&mut resolver, scope_id, &function);
                } else if let Some(class) = JsClassDeclaration::cast_ref(&node) {
                    TypeData::from_js_class_declaration(&mut resolver, scope_id, &class);
                } else if let Some(expression) = JsClassExpression::cast_ref(&node) {
                    TypeData::from_js_class_expression(&mut resolver, scope_id, &expression);
                }
            }
            WalkEvent::Leave(_) => {}
        }
    }

    for (i, ty) in resolver.registered_types().iter().enumerate() {
        result.push_str(&format!("\nTypeId({i}) => {ty}\n"));
    }

    Ok(result)
}

fn debug_semantic_model(_path: &BiomePath, parse: AnyParse) -> Result<String, WorkspaceError> {
    let tree: AnyJsRoot = parse.tree();
    let model = semantic_model(&tree, SemanticModelOptions::default());
    Ok(model.to_string())
}

pub(crate) fn lint(params: LintParams) -> LintResults {
    let _ =
        debug_span!("Linting JavaScript file", path =? params.path, language =? params.language)
            .entered();
    let Some(file_source) = params
        .language
        .to_js_file_source()
        .or(JsFileSource::try_from(params.path.as_path()).ok())
    else {
        return LintResults {
            errors: 0,
            diagnostics: Vec::new(),
            skipped_diagnostics: 0,
        };
    };
    let tree = params.parse.tree();
    let analyzer_options = params.settings.analyzer_options::<JsLanguage>(
        params.path,
        &params.language,
        params.suppression_reason.as_deref(),
    );
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

    // Use snippet services (for embedded JS) if present, else document services.
    let effective_services = params.snippet_services.unwrap_or(params.document_services);
    let semantic_model = effective_services
        .as_js_services()
        .and_then(|s| s.semantic_model.clone());

    let mut services = JsAnalyzerServices::from((
        params.module_graph,
        params.project_layout,
        file_source,
        semantic_model,
    ));

    if let Some(embedded_bindings) = params.document_services.embedded_bindings() {
        services.set_embedded_bindings(embedded_bindings.bindings)
    }

    if let Some(value_refs) = params.document_services.embedded_value_references() {
        services.set_embedded_value_references(value_refs.references)
    }
    let (_, analyze_diagnostics) = analyze(
        &tree,
        filter,
        &analyzer_options,
        &params.plugins,
        services,
        |signal| process_lint.process_signal(signal),
    );

    process_lint.into_result(
        params
            .parse
            .into_serde_diagnostics(params.diagnostic_offset),
        analyze_diagnostics,
    )
}

#[tracing::instrument(level = "debug", skip(params))]
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
        plugins,
        categories,
        action_offset,
        document_services,
    } = params;
    let _ = debug_span!("Code actions JavaScript", range =? range, path =? path).entered();
    let tree = parse.tree();
    let _ = trace_span!("Parsed file").entered();
    let analyzer_options =
        settings.analyzer_options::<JsLanguage>(path, &language, suppression_reason.as_deref());
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

    let Some(source_type) = language.to_js_file_source() else {
        error!("Could not determine the file source of the file");
        return PullActionsResult {
            actions: Vec::new(),
        };
    };
    let semantic_model = document_services
        .as_js_services()
        .and_then(|s| s.semantic_model.clone());
    let services =
        JsAnalyzerServices::from((module_graph, project_layout, source_type, semantic_model));

    debug!("Javascript runs the analyzer");
    analyze(
        &tree,
        filter,
        &analyzer_options,
        &plugins,
        services,
        |signal| {
            actions.extend(signal.actions().into_code_action_iter().map(|item| {
                debug!("Pulled action category {:?}", item.category);
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

/// If applies all the safe fixes to the given syntax tree.
pub(crate) fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let mut tree: AnyJsRoot = params.parse.tree();

    // Compute final rules (taking `overrides` into account)
    let rules = params
        .settings
        .as_ref()
        .as_linter_rules(params.biome_path.as_path());
    let analyzer_options = params.settings.analyzer_options::<JsLanguage>(
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

    let Some(file_source) = params
        .document_file_source
        .to_js_file_source()
        .or(JsFileSource::try_from(params.biome_path.as_path()).ok())
    else {
        return Err(extension_error(params.biome_path));
    };

    let mut process_fix_all = ProcessFixAll::new(
        &params,
        rules,
        tree.syntax().text_range_with_trivia().len().into(),
    );

    loop {
        let mut services = JsAnalyzerServices::from((
            params.module_graph.clone(),
            params.project_layout.clone(),
            file_source,
        ));

        if let Some(embedded_bindings) = params.document_services.embedded_bindings() {
            services.set_embedded_bindings(embedded_bindings.bindings)
        }

        if let Some(value_refs) = params.document_services.embedded_value_references() {
            services.set_embedded_value_references(value_refs.references)
        }

        let (action, _) = analyze(
            &tree,
            filter,
            &analyzer_options,
            &params.plugins,
            services,
            |signal| process_fix_all.process_signal(signal),
        );

        let result = process_fix_all.process_action(action, |root| {
            tree = match AnyJsRoot::cast(root) {
                Some(tree) => tree,
                None => return None,
            };
            Some(tree.syntax().text_range_with_trivia().len().into())
        })?;

        if result.is_none() {
            return process_fix_all.finish(|| {
                Ok(if params.should_format {
                    Either::Left(format_node(
                        params.settings.format_options::<JsLanguage>(
                            params.biome_path,
                            &params.document_file_source,
                        ),
                        tree.syntax(),
                        false,
                    ))
                } else {
                    Either::Right(tree.syntax().to_string())
                })
            });
        }
    }
}

pub(crate) fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);
    debug!("{:?}", &options);
    let tree = parse.syntax();
    let formatted = format_node(options, &tree, false)?;
    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => {
            error!("The file {} couldn't be formatted", biome_path.as_str());
            Err(WorkspaceError::FormatError(error.into()))
        }
    }
}

#[tracing::instrument(level = "debug", skip(parse, settings, document_file_source))]
pub(crate) fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);
    debug!("{:?}", &options);
    let tree = parse.syntax();
    let printed = biome_js_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

#[tracing::instrument(level = "debug", skip(parse, settings, document_file_source))]
pub(crate) fn format_on_type(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(path, document_file_source);
    debug!("{:?}", &options);
    let tree = parse.syntax();

    let range = tree.text_range_with_trivia();
    if offset < range.start() || offset > range.end() {
        return Err(WorkspaceError::FormatError(FormatError::RangeError {
            input: TextRange::at(offset, TextSize::from(0)),
            tree: range,
        }));
    }

    let token = match tree.token_at_offset(offset) {
        // File is empty, do nothing
        TokenAtOffset::None => panic!("empty file"),
        TokenAtOffset::Single(token) => token,
        // The cursor should be right after the closing character that was just typed,
        // select the previous token as the correct one
        TokenAtOffset::Between(token, _) => token,
    };

    let root_node = match token.parent() {
        Some(node) => node,
        None => panic!("found a token with no parent"),
    };

    let printed = biome_js_formatter::format_sub_tree(options, &root_node)?;
    Ok(printed)
}

fn format_embedded(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
    embedded_nodes: Vec<FormatEmbedNode>,
) -> Result<Printed, WorkspaceError> {
    let tree = parse.syntax();
    let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);
    let mut formatted = format_node(options, &tree, true)?;

    formatted.format_embedded(move |range| {
        let mut iter = embedded_nodes.iter();
        let node = iter.find(|node| node.range == range)?;

        let wrap_document = |document: Document| {
            // TODO: Option to disable indent here?
            let elements = vec![
                FormatElement::Line(LineMode::Hard),
                FormatElement::Tag(Tag::StartIndent),
                FormatElement::Line(LineMode::Hard),
                FormatElement::Interned(Interned::new(document.into_elements())),
                FormatElement::Tag(Tag::EndIndent),
            ];
            Document::new(elements)
        };

        match node.source {
            DocumentFileSource::Css(_) => {
                let css_options = settings.format_options::<CssLanguage>(biome_path, &node.source);
                let node = node.node.clone().embedded_syntax::<CssLanguage>();
                let formatted =
                    biome_css_formatter::format_node_with_offset(css_options, &node).ok()?;
                Some(wrap_document(formatted.into_document()))
            }
            DocumentFileSource::Graphql(_) => {
                let graphql_options =
                    settings.format_options::<GraphqlLanguage>(biome_path, &node.source);
                let node = node.node.clone().embedded_syntax::<GraphqlLanguage>();
                let formatted =
                    biome_graphql_formatter::format_node_with_offset(graphql_options, &node)
                        .ok()?;
                Some(wrap_document(formatted.into_document()))
            }
            _ => None,
        }
    });

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

pub(crate) fn pull_diagnostics_and_actions(
    params: DiagnosticsAndActionsParams,
) -> PullDiagnosticsAndActionsResult {
    let DiagnosticsAndActionsParams {
        parse,
        settings,
        language,
        path,
        only,
        skip,
        categories,
        module_graph,
        project_layout,
        suppression_reason,
        enabled_selectors,
        plugins,
        diagnostic_offset,
        document_services,
    } = params;
    let tree = parse.tree();
    let analyzer_options =
        settings.analyzer_options::<JsLanguage>(path, &language, suppression_reason.as_deref());
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(settings.as_ref(), analyzer_options)
            .with_only(only)
            .with_skip(skip)
            .with_path(path.as_path())
            .with_enabled_selectors(enabled_selectors)
            .with_project_layout(project_layout.clone())
            .finish();
    let filter = AnalysisFilter {
        categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    let Some(source_type) = language.to_js_file_source() else {
        error!("Could not determine the file source of the file");
        return PullDiagnosticsAndActionsResult {
            diagnostics: Vec::new(),
        };
    };
    let semantic_model = document_services
        .as_js_services()
        .and_then(|s| s.semantic_model.clone());
    let services =
        JsAnalyzerServices::from((module_graph, project_layout, source_type, semantic_model));
    let mut process_pull_diagnostics_and_actions =
        ProcessDiagnosticsAndActions::new(diagnostic_offset);
    analyze(
        &tree,
        filter,
        &analyzer_options,
        &plugins,
        services,
        |signal| process_pull_diagnostics_and_actions.process_signal(signal),
    );

    process_pull_diagnostics_and_actions.finish()
}

fn rename(
    _rome_path: &BiomePath,
    parse: AnyParse,
    symbol_at: TextSize,
    new_name: String,
) -> Result<RenameResult, WorkspaceError> {
    let root = parse.tree();
    let model = semantic_model(&root, SemanticModelOptions::default());

    if let Some(node) = parse
        .syntax()
        .descendants_tokens(Direction::Next)
        .find(|token| token.text_range().contains(symbol_at))
        .and_then(|token| token.parent())
    {
        let original_name = node.text_trimmed();
        let range = node.text_range_with_trivia();
        match node.try_into() {
            Ok(node) => {
                let mut batch = root.begin();
                let result = batch.rename_any_renamable_node(&model, &node, &new_name);
                if !result {
                    Err(WorkspaceError::RenameError(RenameError::CannotBeRenamed {
                        original_name: original_name.to_string(),
                        original_range: range,
                        new_name,
                    }))
                } else {
                    let (range, indels) = batch.to_text_range_and_edit().unwrap_or_default();
                    Ok(RenameResult { range, indels })
                }
            }
            Err(err) => Err(WorkspaceError::RenameError(err)),
        }
    } else {
        Err(WorkspaceError::RenameError(
            RenameError::CannotFindDeclaration(new_name),
        ))
    }
}

#[instrument(level = "debug", skip_all)]
fn update_snippets(
    root: AnyParse,
    new_snippets: Vec<UpdateSnippetsNodes>,
) -> Result<SendNode, WorkspaceError> {
    let tree: AnyJsRoot = root.tree();
    let mut mutation = BatchMutation::new(tree.syntax().clone());
    let iterator = tree
        .syntax()
        .descendants()
        .filter_map(JsTemplateChunkElement::cast);

    for element in iterator {
        let Some(snippet) = new_snippets
            .iter()
            .find(|snippet| snippet.range == element.range())
        else {
            continue;
        };

        if let Ok(value_token) = element.template_chunk_token() {
            let new_token = ident(snippet.new_code.as_str());
            mutation.replace_token(value_token, new_token);
        }
    }

    let root = mutation.commit();

    Ok(root.as_send().unwrap())
}
