mod go_to;

use super::{
    AnalyzerCapabilities, AnalyzerVisitorBuilder, AnalyzerVisitorResult, CodeActionsParams,
    DebugCapabilities, DiagnosticsAndActionsParams, EditorCapabilities, EnabledForPath,
    ExtensionHandler, FixedFileResult, FormatterCapabilities, LintParams, LintResults,
    ParseEmbedResult, ParseEmbeddedParams, ParseResult, ParserCapabilities,
    ProcessDiagnosticsAndActions, ProcessFixAll, ProcessLint, SearchCapabilities,
    UpdateSnippetsNodes, format_on_type_noop, matches_on_type_char,
};
use crate::configuration::to_analyzer_rules;
#[cfg(feature = "js_embeds")]
use crate::embed::EmbedContent;
#[cfg(feature = "js_embeds")]
use crate::embed::js::{
    EmbedCandidate, EmbedDetectorsRegistry, EmbedMatch, GuestLanguage, TemplateTagKind,
};
use crate::file_handlers::FixAllParams;
use crate::file_handlers::javascript::go_to::{resolve_binding, resolve_definition};
use crate::settings::{
    OverrideSettings, Settings, SettingsWithEditor, check_feature_activity,
    check_override_feature_activity,
};
use crate::workspace::{FixFileMode, SearchQuery};
use crate::workspace::{PatternId, PullDiagnosticsAndActionsResult};
use crate::{
    WorkspaceError,
    settings::{FormatSettings, LanguageListSettings, LanguageSettings, ServiceLanguage},
    workspace::{CodeAction, GetSyntaxTreeResult, PullActionsResult, RenameResult},
};
use biome_analyze::ActionFilter;
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
#[cfg(feature = "js_embeds")]
use biome_css_parser::parse_css_with_offset_and_cache;
#[cfg(feature = "js_embeds")]
use biome_css_syntax::CssLanguage;
use biome_db::AnyParsedSource;
#[cfg(feature = "js_embeds")]
use biome_formatter::FormatElement;
#[cfg(feature = "js_embeds")]
use biome_formatter::prelude::{Document, Interned, LineMode, Tag};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, DelimiterSpacing, Expand, FormatError,
    IndentStyle, IndentWidth, LineEnding, LineWidth, Printed, QuoteStyle, TrailingNewline,
};
use biome_fs::BiomePath;
#[cfg(all(feature = "js_embeds", feature = "lang_graphql"))]
use biome_graphql_parser::parse_graphql_with_offset_and_cache;
#[cfg(all(feature = "js_embeds", feature = "lang_graphql"))]
use biome_graphql_syntax::GraphqlLanguage;
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
use biome_js_semantic::{
    SVELTE_RUNES, SemanticModel, SemanticModelOptions, js_semantic_model, semantic_model,
};
#[cfg(feature = "js_embeds")]
use biome_js_syntax::{
    AnyJsExpression, AnyJsTemplateElement, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsTemplateExpression,
};
#[cfg(feature = "type_inference")]
use biome_js_syntax::{
    AnyJsExpression as TypeInfoExpression, JsClassDeclaration, JsClassExpression,
    JsFunctionDeclaration, JsVariableDeclarator,
};
use biome_js_syntax::{
    AnyJsRoot, JsLanguage, JsSyntaxNode, JsTemplateChunkElement, TextRange, TextSize, TokenAtOffset,
};
#[cfg(feature = "type_inference")]
use biome_js_type_info::{GlobalsResolver, RawTypeCollector, ScopeId, TypeData, TypeId, TypeStore};
#[cfg(feature = "js_embeds")]
use biome_languages::CssFileSource;
#[cfg(all(feature = "js_embeds", feature = "lang_graphql"))]
use biome_languages::GraphqlFileSource;
#[cfg(feature = "js_embeds")]
use biome_languages::css::CssEmbeddingKind;
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
#[cfg(feature = "module_graph")]
use biome_module_graph::ModuleDb;
#[cfg(feature = "js_embeds")]
use biome_parser::AnyParse;
use biome_project_layout::ProjectLayout;
#[cfg(feature = "js_embeds")]
use biome_rowan::AstNodeList;
use biome_rowan::SyntaxKind;
#[cfg(feature = "type_inference")]
use biome_rowan::WalkEvent;
use biome_rowan::{AstNode, BatchMutation, BatchMutationExt, Direction, NodeCache, SendNode};
use biome_workspace_db::WorkspaceDb;
use camino::Utf8Path;
#[cfg(feature = "js_embeds")]
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;
#[cfg(feature = "module_graph")]
use std::rc::Rc;
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
    pub delimiter_spacing: Option<DelimiterSpacing>,
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
            delimiter_spacing: value.delimiter_spacing,
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
        .with_delimiter_spacing(
            language
                .delimiter_spacing
                .or(global.delimiter_spacing)
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

        if let Ok(source_type) = JsFileSource::try_from(path.as_path()) {
            if source_type.as_embedding_kind().is_vue() {
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
            } else if source_type.as_embedding_kind().is_astro() {
                globals.extend(["Astro"].map(Into::into));
            } else if source_type.as_embedding_kind().is_svelte() {
                // Svelte 5 runes
                globals.extend(SVELTE_RUNES.iter().copied().map(Into::into));
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
            editors: EditorCapabilities {
                resolve_binding: Some(resolve_binding),
                resolve_definition: Some(resolve_definition),
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

#[cfg(not(feature = "js_embeds"))]
fn parse_embedded_nodes(_params: ParseEmbeddedParams) -> ParseEmbedResult {
    ParseEmbedResult::default()
}

#[cfg(feature = "js_embeds")]
fn parse_embedded_nodes(params: ParseEmbeddedParams) -> ParseEmbedResult {
    let ParseEmbeddedParams {
        any_parse,
        path,
        file_source,
        settings,
        node_cache,
    } = params;
    if !settings
        .as_ref()
        .experimental_js_embedded_snippets_enabled()
    {
        return ParseEmbedResult { nodes: vec![] };
    }

    let js_root: AnyJsRoot = any_parse.tree();

    let nodes = js_root
        .syntax()
        .descendants()
        .filter_map(JsTemplateExpression::cast)
        .filter_map(|expr| {
            let candidate = build_js_template_candidate(&expr)?;
            let embed_match = EmbedDetectorsRegistry::detect_match(&candidate, file_source)?;
            let (snippet, content, doc_source) =
                parse_js_matched_embed(&candidate, &embed_match, node_cache, path, settings)?;
            Some((snippet, content, doc_source))
        })
        .collect();

    ParseEmbedResult { nodes }
}

/// Build an `EmbedCandidate::TaggedTemplate` from a `JsTemplateExpression`.
///
/// Returns `None` if:
/// - The template has interpolations (not supported yet)
/// - The tag can't be classified (unknown pattern)
#[cfg(feature = "js_embeds")]
fn build_js_template_candidate(expr: &JsTemplateExpression) -> Option<EmbedCandidate> {
    // TODO: Interpolations are not supported yet.
    if expr.elements().len() != 1 {
        return None;
    }

    let Some(AnyJsTemplateElement::JsTemplateChunkElement(chunk)) = expr.elements().first() else {
        return None;
    };

    let tag_kind = template_expression_to_template_tag(expr)?;

    let content_token = chunk.template_chunk_token().ok()?;
    Some(EmbedCandidate::TaggedTemplate {
        tag: tag_kind,
        content: EmbedContent {
            element_range: chunk.range(),
            content_range: content_token.text_range(),
            content_offset: content_token.text_range().start(),
            text: content_token.token_text(),
        },
    })
}

/// Classify a template expression's tag into a `TemplateTagKind`.
///
/// Handles:
/// - `css\`\`` → Identifier("css")
/// - `gql\`\`` / `graphql\`\`` → Identifier("gql") / Identifier("graphql")
/// - `styled.div\`\`` → MemberExpression { object: "styled", property: "div" }
/// - `styled(Comp)\`\`` → CallExpression { callee: "styled" }
/// - `graphql(\`\`)` → CallExpression { callee: "graphql" } (template as argument)
#[cfg(feature = "js_embeds")]
fn template_expression_to_template_tag(expr: &JsTemplateExpression) -> Option<TemplateTagKind> {
    if let Some(tag) = expr.tag() {
        match tag {
            // css``, gql``, graphql``
            AnyJsExpression::JsIdentifierExpression(ident) => {
                let name = ident.name().ok()?;
                Some(TemplateTagKind::Identifier(
                    name.value_token().ok()?.token_text_trimmed(),
                ))
            }
            // styled.div``
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let object = match member.object().ok()? {
                    AnyJsExpression::JsIdentifierExpression(ident) => {
                        ident.name().ok()?.value_token().ok()?.token_text_trimmed()
                    }
                    _ => return None,
                };
                let property = member
                    .member()
                    .ok()?
                    .value_token()
                    .ok()?
                    .token_text_trimmed();
                Some(TemplateTagKind::MemberExpression { object, property })
            }
            // styled(Component)``
            AnyJsExpression::JsCallExpression(call) => {
                let callee = match call.callee().ok()? {
                    AnyJsExpression::JsIdentifierExpression(ident) => {
                        ident.name().ok()?.value_token().ok()?.token_text_trimmed()
                    }
                    _ => return None,
                };
                Some(TemplateTagKind::CallExpression { callee })
            }
            _ => None,
        }
    } else {
        // No tag — check if template is an argument to a call expression
        // e.g. graphql(`query { ... }`)
        let list = expr.parent::<JsCallArgumentList>()?;
        let args = list.parent::<JsCallArguments>()?;
        let call = args.parent::<JsCallExpression>()?;
        let callee = match call.callee().ok()? {
            AnyJsExpression::JsIdentifierExpression(ident) => {
                ident.name().ok()?.value_token().ok()?.token_text_trimmed()
            }
            _ => return None,
        };
        Some(TemplateTagKind::CallExpression { callee })
    }
}

/// Parse an embed site that the JS registry matched.
///
/// Note: The old code returned `EmbeddedSnippet<JsLanguage>` for ALL languages
/// (CSS, GraphQL), not the actual language type. This is because `AnyEmbeddedSnippet`
/// erases the language via `AnyParse`, and the JS handler stores everything as
/// `AnyEmbeddedSnippet::Js`. We preserve this behavior for compatibility.
#[cfg(feature = "js_embeds")]
fn parse_js_matched_embed(
    candidate: &EmbedCandidate,
    embed_match: &EmbedMatch,
    cache: &mut NodeCache,
    biome_path: &BiomePath,
    settings: &SettingsWithEditor,
) -> Option<(AnyParse, EmbedContent, DocumentFileSource)> {
    let content = candidate.content();

    match embed_match.guest {
        GuestLanguage::Css => {
            let file_source = DocumentFileSource::Css(
                CssFileSource::css().with_embedding_kind(CssEmbeddingKind::Styled),
            );
            let options = settings.parse_options::<CssLanguage>(biome_path, &file_source);
            let parse = parse_css_with_offset_and_cache(
                content.text.text(),
                file_source.to_css_file_source().unwrap_or_default(),
                content.content_offset,
                cache,
                options,
            );

            Some((parse.into(), content.clone(), file_source))
        }

        #[cfg(feature = "lang_graphql")]
        GuestLanguage::GraphQL => {
            let file_source = DocumentFileSource::Graphql(GraphqlFileSource::graphql());
            let parse = parse_graphql_with_offset_and_cache(
                content.text.text(),
                content.content_offset,
                cache,
            );

            Some((parse.into(), content, file_source))
        }
    }
}

fn debug_syntax_tree(
    _biome_path: &BiomePath,
    parse: AnyParsedSource,
    workspace_db: WorkspaceDb,
) -> GetSyntaxTreeResult {
    let syntax: JsSyntaxNode = parse.syntax(&workspace_db);
    let tree: AnyJsRoot = parse.tree(&workspace_db);
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_control_flow(
    parse: AnyParsedSource,
    cursor: TextSize,
    workspace_db: WorkspaceDb,
) -> String {
    let mut control_flow_graph = None;

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("correctness", "noUnreachable")]),
        ..AnalysisFilter::default()
    };
    let options = AnalyzerOptions::default();

    analyze_with_inspect_matcher(
        &parse.tree(&workspace_db),
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
    parse: AnyParsedSource,
    settings: &SettingsWithEditor,
    workspace_db: WorkspaceDb,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(path, document_file_source);

    let tree = parse.syntax(&workspace_db);
    let formatted = format_node(options, &tree, Vec::new())?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

fn debug_type_info(
    parse: AnyParsedSource,
    workspace_db: WorkspaceDb,
) -> Result<String, WorkspaceError> {
    #[cfg(feature = "type_inference")]
    {
        let tree: AnyJsRoot = parse.tree(&workspace_db);
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
                            &TypeData::from_js_function_declaration(
                                &mut resolver,
                                scope_id,
                                &function,
                            )
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
                            &TypeData::from_js_class_expression(
                                &mut resolver,
                                scope_id,
                                &expression,
                            )
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

    #[cfg(not(feature = "type_inference"))]
    {
        let _ = (parse, workspace_db);
        Err(WorkspaceError::feature_not_enabled())
    }
}

fn debug_registered_types(
    _path: &BiomePath,
    parse: AnyParsedSource,
    workspace_db: WorkspaceDb,
) -> Result<String, WorkspaceError> {
    #[cfg(feature = "type_inference")]
    {
        let tree: AnyJsRoot = parse.tree(&workspace_db);
        let mut result = String::new();
        let preorder = tree.syntax().preorder();

        let mut resolver = DebugTypeCollector::default();
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

        for (i, ty) in resolver.types.as_references().iter().enumerate() {
            result.push_str(&format!("\nTypeId({i}) => {ty}\n"));
        }

        Ok(result)
    }

    #[cfg(not(feature = "type_inference"))]
    {
        let _ = (_path, parse, workspace_db);
        Err(WorkspaceError::feature_not_enabled())
    }
}

#[cfg(feature = "type_inference")]
#[derive(Default)]
struct DebugTypeCollector {
    types: TypeStore,
}

#[cfg(feature = "type_inference")]
impl RawTypeCollector for DebugTypeCollector {
    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types.find(type_data)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.types.get_by_id(id)
    }

    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        self.types.insert_cow(type_data)
    }

    fn resolve_expression(
        &mut self,
        scope_id: ScopeId,
        expression: &TypeInfoExpression,
    ) -> Cow<'_, TypeData> {
        Cow::Owned(TypeData::from_any_js_expression(self, scope_id, expression))
    }
}

fn debug_semantic_model(
    _path: &BiomePath,
    parse: AnyParsedSource,
    workspace_db: WorkspaceDb,
) -> Result<String, WorkspaceError> {
    let model = js_semantic_model(&workspace_db, &parse);
    Ok(model.to_string())
}

fn js_analyzer_services<'a>(
    root: &'a AnyJsRoot,
    workspace_db: &WorkspaceDb,
    #[cfg(feature = "module_graph")] module_db: Rc<dyn ModuleDb>,
    project_layout: Arc<ProjectLayout>,
    source_type: JsFileSource,
) -> JsAnalyzerServices<'a> {
    #[cfg(feature = "module_graph")]
    let services = {
        let _ = root;
        JsAnalyzerServices::from((module_db, project_layout, source_type))
    };
    #[cfg(not(feature = "module_graph"))]
    let services = JsAnalyzerServices::from(root)
        .with_project_layout(project_layout)
        .with_source_type(source_type);

    services.with_language_db(workspace_db.rc_language_db())
}

fn js_analyzer_services_for_fix<'a>(
    root: &'a AnyJsRoot,
    semantic_model: &'a SemanticModel,
    params: &FixAllParams,
    source_type: JsFileSource,
) -> JsAnalyzerServices<'a> {
    #[cfg(feature = "module_graph")]
    {
        js_analyzer_services(
            root,
            &params.workspace_db,
            params.module_db.clone(),
            params.project_layout.clone(),
            source_type,
        )
        .with_semantic_model(semantic_model)
    }

    #[cfg(not(feature = "module_graph"))]
    js_analyzer_services(
        root,
        &params.workspace_db,
        #[cfg(feature = "module_graph")]
        params.module_db.clone(),
        params.project_layout.clone(),
        source_type,
    )
    .with_semantic_model(semantic_model)
}

pub(crate) fn lint(params: LintParams) -> LintResults {
    let _ =
        debug_span!("Linting JavaScript file", path =? params.path, language =? params.language)
            .entered();

    let Some(files_source) = params.language.to_js_file_source() else {
        return LintResults::default();
    };

    let tree = params.parsed_source.tree(&params.workspace_db);
    let analyzer_options = params.settings.analyzer_options::<JsLanguage>(
        params.path,
        params.working_directory,
        &params.language,
        params.suppression_reason.as_deref(),
    );
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
        .with_cache(params.analyzer_cache)
        .finish();

    let filter = AnalysisFilter {
        categories: params.categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    let mut process_lint = ProcessLint::new(&params);

    let semantic_model = match &params.parsed_source {
        super::ParsedOrigin::Workspace(source) => {
            js_semantic_model(&params.workspace_db, source).clone()
        }
        super::ParsedOrigin::Interned { .. } => {
            semantic_model(&tree, SemanticModelOptions::from(&files_source))
        }
    };
    let services = js_analyzer_services(
        &tree,
        &params.workspace_db,
        #[cfg(feature = "module_graph")]
        params.module_db.clone(),
        params.project_layout.clone(),
        files_source,
    );
    let services = services.with_semantic_model(&semantic_model);

    let (_, analyze_diagnostics) = analyze(
        &tree,
        filter,
        &analyzer_options,
        &params.plugins,
        services,
        |signal| process_lint.process_signal(signal),
    );

    process_lint.into_result(
        params.parsed_source.serde_diagnostics(&params.workspace_db),
        analyze_diagnostics,
    )
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parsed_source,
        range,
        settings,
        path,
        workspace_db,
        project_layout,
        language,
        only,
        skip,
        suppression_reason,
        enabled_rules: rules,
        plugins,
        categories,
        working_directory,
        compute_actions,
        analyzer_cache,
    } = params;
    let _ = debug_span!("Code actions JavaScript", range =? range, path =? path).entered();
    let tree = parsed_source.tree(&workspace_db);
    let _ = trace_span!("Parsed file").entered();
    let analyzer_options = settings.analyzer_options::<JsLanguage>(
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
        .with_cache(analyzer_cache)
        .finish();
    let filter = AnalysisFilter {
        categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range,
    };

    let source_type = workspace_db
        .source_from_index(parsed_source.document_file_index(&workspace_db))
        .map_or(JsFileSource::try_from(path.as_path()).ok(), |file_source| {
            file_source.to_js_file_source()
        });
    let Some(source_type) = source_type else {
        error!("Could not determine the file source of the file");
        return PullActionsResult {
            actions: Vec::new(),
        };
    };
    let semantic_model = js_semantic_model(&workspace_db, &parsed_source);
    let action_offset = parsed_source.diagnostic_offset(&workspace_db);
    let services = js_analyzer_services(
        &tree,
        &workspace_db,
        #[cfg(feature = "module_graph")]
        workspace_db.rc_module_db(),
        project_layout,
        source_type,
    )
    .with_semantic_model(semantic_model);

    debug!("Javascript runs the analyzer");
    analyze(
        &tree,
        filter,
        &analyzer_options,
        &plugins,
        services,
        |signal| {
            if compute_actions {
                actions.extend(
                    signal
                        .actions(ActionFilter::all())
                        .into_code_action_iter()
                        .map(|item| {
                            debug!("Pulled action category {:?}", item.category);
                            CodeAction {
                                category: item.category.clone(),
                                rule_name: item.rule_name.map(|(group, name)| {
                                    (Cow::Borrowed(group), Cow::Borrowed(name))
                                }),
                                applicability: Some(item.suggestion.applicability),
                                suggestion: Some(item.suggestion),
                                offset: action_offset,
                            }
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

/// If applies all the safe fixes to the given syntax tree.
pub(crate) fn fix_all(params: FixAllParams) -> Result<Option<FixedFileResult>, WorkspaceError> {
    let mut tree: AnyJsRoot = params.parsed_source.tree(&params.workspace_db);

    // Compute final rules (taking `overrides` into account)
    let rules = params
        .settings
        .as_ref()
        .as_linter_rules(params.biome_path.as_path());
    let analyzer_options = params.settings.analyzer_options::<JsLanguage>(
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

    let source_type = params
        .document_file_source
        .to_js_file_source()
        .or(JsFileSource::try_from(params.biome_path.as_path()).ok());
    let Some(file_source) = source_type else {
        error!("Could not determine the file source of the file");
        return Ok(None);
    };

    let mut process_fix_all = ProcessFixAll::new(
        &params,
        rules,
        tree.syntax().text_range_with_trivia().len().into(),
    );

    if matches!(params.fix_file_mode, FixFileMode::ApplySuppressions) {
        // Suppressions apply to all rules -- keep original single-phase loop
        loop {
            let semantic_model = semantic_model(&tree, SemanticModelOptions::from(&file_source));
            let services =
                js_analyzer_services_for_fix(&tree, &semantic_model, &params, file_source);

            let mut pending_actions = Vec::new();

            let (_, _) = analyze(
                &tree,
                filter,
                &analyzer_options,
                &params.plugins,
                services,
                |signal| {
                    if params.collect_final_diagnostics {
                        process_fix_all.collect_signal(signal, &mut pending_actions)
                    } else {
                        process_fix_all.collect_signal_fixes_only(signal, &mut pending_actions)
                    }
                },
            );

            let result = process_fix_all.process_batch_actions(pending_actions, |root| {
                tree = match AnyJsRoot::cast(root) {
                    Some(tree) => tree,
                    None => return None,
                };
                Some(tree.syntax().text_range_with_trivia().len().into())
            })?;

            if result.is_none() {
                return Ok(Some(
                    process_fix_all.finish(tree.syntax().as_send().unwrap()),
                ));
            }
        }
    }

    // Two-phase fix-all: Phase 1 runs only fixable rules, Phase 2 runs all rules for diagnostics.

    // Phase 1: fix loop with fixable-only rules
    let fixable_filter = AnalysisFilter {
        categories: params.rule_categories,
        enabled_rules: Some(fixable_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };
    loop {
        let semantic_model = semantic_model(&tree, SemanticModelOptions::from(&file_source));
        let services = js_analyzer_services_for_fix(&tree, &semantic_model, &params, file_source);

        let mut pending_actions = Vec::new();

        let (_, _) = analyze(
            &tree,
            fixable_filter,
            &analyzer_options,
            &params.plugins,
            services,
            |signal| process_fix_all.collect_signal_fixes_only(signal, &mut pending_actions),
        );

        let mut plugin_text_edit = None;
        pending_actions.retain_mut(|action| {
            if let Some(text_edit) = action.text_edit.take() {
                plugin_text_edit.get_or_insert(text_edit);
                false
            } else {
                true
            }
        });
        let result = process_fix_all.process_batch_actions(pending_actions, |root| {
            tree = match AnyJsRoot::cast(root) {
                Some(tree) => tree,
                None => return None,
            };
            Some(tree.syntax().text_range_with_trivia().len().into())
        })?;

        if result.is_none() {
            if let Some(plugin_text_edit) = plugin_text_edit {
                let new_text = {
                    let current_text = tree.syntax().to_string();
                    process_fix_all.apply_plugin_text_edit(plugin_text_edit, &current_text)?
                };
                if let Some(new_text) = new_text {
                    let options = params.settings.parse_options::<JsLanguage>(
                        params.biome_path,
                        &params.document_file_source,
                    );
                    let parse = biome_js_parser::parse(&new_text, file_source, options);
                    tree = parse.tree();
                    continue;
                }
            }

            break;
        }
    }

    // Phase 2: run all rules on the fixed tree for final diagnostics
    if params.collect_final_diagnostics {
        let semantic_model = semantic_model(&tree, SemanticModelOptions::from(&file_source));
        let services = js_analyzer_services_for_fix(&tree, &semantic_model, &params, file_source);

        let (_, _) = analyze(
            &tree,
            filter,
            &analyzer_options,
            &params.plugins,
            services,
            |signal| process_fix_all.collect_diagnostic_only(signal),
        );
    }

    Ok(Some(
        process_fix_all.finish(tree.syntax().as_send().unwrap()),
    ))
}

pub(crate) fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: super::ParsedOrigin,
    settings: &SettingsWithEditor,
    workspace_db: WorkspaceDb,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);
    debug!("{:?}", &options);
    let tree = parse.syntax(&workspace_db);
    let formatted = format_node(options, &tree, Vec::new())?;
    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => {
            error!("The file {} couldn't be formatted", biome_path.as_str());
            Err(WorkspaceError::FormatError(error.into()))
        }
    }
}

#[tracing::instrument(
    level = "debug",
    skip(parse, settings, document_file_source, workspace_db)
)]
pub(crate) fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParsedSource,
    settings: &SettingsWithEditor,
    range: TextRange,
    workspace_db: WorkspaceDb,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);
    debug!("{:?}", &options);
    let tree = parse.syntax(&workspace_db);
    let printed = biome_js_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

#[tracing::instrument(
    level = "debug",
    skip(parse, settings, document_file_source, workspace_db)
)]
pub(crate) fn format_on_type(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParsedSource,
    settings: &SettingsWithEditor,
    offset: TextSize,
    workspace_db: WorkspaceDb,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(path, document_file_source);
    debug!("{:?}", &options);
    let tree = parse.syntax(&workspace_db);

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

    if token.text_trimmed_range().end() != offset {
        return Ok(format_on_type_noop(offset));
    }

    if !matches_on_type_char(token.text_trimmed()) {
        return Ok(format_on_type_noop(offset));
    }

    let root_node = match token.parent() {
        Some(node) => node,
        None => panic!("found a token with no parent"),
    };

    if root_node
        .ancestors()
        .any(|node: JsSyntaxNode| node.kind().is_bogus())
    {
        return Ok(format_on_type_noop(offset));
    }

    let printed = biome_js_formatter::format_range(options, &tree, root_node.text_trimmed_range())?;
    Ok(printed)
}

fn format_embedded(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: super::ParsedOrigin,
    settings: &SettingsWithEditor,
    embedded_nodes: Vec<super::ParsedSnippetOrigin>,
    workspace_db: WorkspaceDb,
) -> Result<Printed, WorkspaceError> {
    #[cfg(feature = "js_embeds")]
    {
        let tree = parse.syntax(&workspace_db);
        let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);

        // Hand the snippet ranges to the formatter, so it only emits embedded
        // tags for chunks that were actually parsed as embedded languages.
        let snippets: FxHashMap<TextRange, super::ParsedSnippetOrigin> = embedded_nodes
            .into_iter()
            .map(|snippet| (snippet.content_range(&workspace_db), snippet))
            .collect();
        let mut formatted = format_node(options, &tree, snippets.keys().copied().collect())?;

        formatted.format_embedded(move |range| {
            let snippet = snippets.get(&range)?;
            let snippet_file_source = snippet.file_source(&workspace_db)?;

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

            match snippet_file_source {
                DocumentFileSource::Css(_) => {
                    let css_options =
                        settings.format_options::<CssLanguage>(biome_path, &snippet_file_source);
                    let node = snippet
                        .parsed_origin()
                        .parse(&workspace_db)
                        .embedded_syntax::<CssLanguage>();
                    let formatted =
                        biome_css_formatter::format_node_with_offset(css_options, &node).ok()?;
                    Some(wrap_document(formatted.into_document()))
                }
                #[cfg(feature = "lang_graphql")]
                DocumentFileSource::Graphql(_) => {
                    let graphql_options = settings
                        .format_options::<GraphqlLanguage>(biome_path, &snippet_file_source);
                    let node = snippet
                        .parsed_origin()
                        .parse(&workspace_db)
                        .embedded_syntax::<GraphqlLanguage>();
                    let formatted =
                        biome_graphql_formatter::format_node_with_offset(graphql_options, &node)
                            .ok()?;
                    Some(wrap_document(formatted.into_document()))
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

    #[cfg(not(feature = "js_embeds"))]
    {
        let _ = (
            biome_path,
            document_file_source,
            parse,
            settings,
            embedded_nodes,
            workspace_db,
        );
        panic!("formatting embedded JavaScript snippets requires the `js_embeds` feature")
    }
}

pub(crate) fn pull_diagnostics_and_actions(
    params: DiagnosticsAndActionsParams,
) -> PullDiagnosticsAndActionsResult {
    let DiagnosticsAndActionsParams {
        parsed_source,
        settings,
        language,
        path,
        only,
        skip,
        categories,
        workspace_db,
        project_layout,
        suppression_reason,
        enabled_selectors,
        plugins,
        working_directory,
    } = params;
    let tree = parsed_source.tree(&workspace_db);
    let analyzer_options = settings.analyzer_options::<JsLanguage>(
        path,
        working_directory,
        &language,
        suppression_reason.as_deref(),
    );
    let AnalyzerVisitorResult {
        enabled_rules,
        disabled_rules,
        analyzer_options,
        ..
    } = AnalyzerVisitorBuilder::new(settings.as_ref(), analyzer_options)
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
    let diagnostic_offset = parsed_source.diagnostic_offset(&workspace_db);
    let source_type = workspace_db
        .source_from_index(parsed_source.document_file_index(&workspace_db))
        .map_or(JsFileSource::try_from(path.as_path()).ok(), |file_source| {
            file_source.to_js_file_source()
        });
    let Some(source_type) = source_type else {
        error!("Could not determine the file source of the file");
        return PullDiagnosticsAndActionsResult {
            diagnostics: Vec::new(),
        };
    };
    let semantic_model = js_semantic_model(&workspace_db, &parsed_source);
    let services = js_analyzer_services(
        &tree,
        &workspace_db,
        #[cfg(feature = "module_graph")]
        workspace_db.rc_module_db(),
        project_layout,
        source_type,
    )
    .with_semantic_model(semantic_model);
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
    path: &BiomePath,
    parse: AnyParsedSource,
    symbol_at: TextSize,
    new_name: String,
    workspace_db: WorkspaceDb,
) -> Result<RenameResult, WorkspaceError> {
    let root = parse.tree(&workspace_db);
    let source_type = JsFileSource::try_from(path.as_path()).unwrap_or_default();
    let model = semantic_model(&root, SemanticModelOptions::from(&source_type));

    if let Some(node) = parse
        .syntax(&workspace_db)
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
    root: super::ParsedOrigin,
    workspace_db: WorkspaceDb,
    new_snippets: Vec<UpdateSnippetsNodes>,
) -> Result<SendNode, WorkspaceError> {
    let tree: AnyJsRoot = root.tree(&workspace_db);
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

fn search(
    path: &BiomePath,
    document: &DocumentFileSource,
    parsed: AnyParsedSource,
    provider: &dyn SearchQuery,
    settings: &SettingsWithEditor,
    pattern_id: PatternId,
    workspace_db: WorkspaceDb,
) -> Result<Vec<TextRange>, WorkspaceError> {
    let any_parse = parsed.any_parse(&workspace_db);
    provider.search(path, document, any_parse.clone(), settings, pattern_id)
}
