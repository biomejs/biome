use super::{
    AnalyzerCapabilities, AnalyzerVisitorBuilder, CodeActionsParams, DebugCapabilities,
    EnabledForPath, ExtensionHandler, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, ProcessLint, SearchCapabilities, search,
};
use crate::configuration::to_analyzer_rules;
use crate::diagnostics::extension_error;
use crate::file_handlers::{FixAllParams, is_diagnostic_error};
use crate::settings::{
    OverrideSettings, Settings, check_feature_activity, check_override_feature_activity,
};
use crate::utils::growth_guard::GrowthGuard;
use crate::workspace::DocumentFileSource;
use crate::{
    WorkspaceError,
    settings::{FormatSettings, LanguageListSettings, LanguageSettings, ServiceLanguage},
    workspace::{
        CodeAction, FixAction, FixFileMode, FixFileResult, GetSyntaxTreeResult, PullActionsResult,
        RenameResult,
    },
};
use biome_analyze::options::{PreferredIndentation, PreferredQuote};
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never, QueryMatch,
    RuleCategoriesBuilder, RuleError, RuleFilter,
};
use biome_configuration::javascript::{
    JsAssistConfiguration, JsAssistEnabled, JsFormatterConfiguration, JsFormatterEnabled,
    JsGritMetavariable, JsLinterConfiguration, JsLinterEnabled, JsParserConfiguration,
    JsxEverywhere, JsxRuntime, UnsafeParameterDecoratorsEnabled,
};
use biome_diagnostics::Applicability;
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, Expand, FormatError, IndentStyle,
    IndentWidth, LineEnding, LineWidth, Printed, QuoteStyle,
};
use biome_fs::BiomePath;
use biome_js_analyze::utils::rename::{RenameError, RenameSymbolExtensions};
use biome_js_analyze::{
    ControlFlowGraph, JsAnalyzerServices, analyze, analyze_with_inspect_matcher,
};
use biome_js_formatter::context::trailing_commas::TrailingCommas;
use biome_js_formatter::context::{
    ArrowParentheses, JsFormatOptions, OperatorLinebreak, QuoteProperties, Semicolons,
};
use biome_js_formatter::format_node;
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::{
    AnyJsRoot, JsClassDeclaration, JsClassExpression, JsFileSource, JsFunctionDeclaration,
    JsLanguage, JsSyntaxNode, JsVariableDeclarator, TextRange, TextSize, TokenAtOffset,
};
use biome_js_type_info::{GlobalsResolver, ScopeId, TypeData, TypeResolver};
use biome_module_graph::ModuleGraph;
use biome_parser::AnyParse;
use biome_rowan::{AstNode, BatchMutationExt, Direction, NodeCache, WalkEvent};
use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use tracing::{debug, debug_span, error, trace_span};

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
                parse_embedded_nodes: None,
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
                update_snippets: None,
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
                format_embedded: None,
            },
            search: SearchCapabilities {
                search: Some(search),
            },
        }
    }
}

pub fn formatter_enabled(path: &Utf8Path, settings: &Settings) -> bool {
    settings.formatter_enabled_for_file_path::<JsLanguage>(path)
}

pub fn linter_enabled(path: &Utf8Path, settings: &Settings) -> bool {
    settings.linter_enabled_for_file_path::<JsLanguage>(path)
}

pub fn assist_enabled(path: &Utf8Path, settings: &Settings) -> bool {
    settings.assist_enabled_for_file_path::<JsLanguage>(path)
}

pub fn search_enabled(_path: &Utf8Path, _settings: &Settings) -> bool {
    true
}

fn parse(
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    let options = settings.parse_options::<JsLanguage>(biome_path, &file_source);

    let file_source = file_source.to_js_file_source().unwrap_or_default();
    let parse = biome_js_parser::parse_js_with_cache(text, file_source, options, cache);
    ParseResult {
        any_parse: parse.into(),
        language: None,
    }
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
    settings: &Settings,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

fn debug_type_info(
    path: &BiomePath,
    parse: Option<AnyParse>,
    graph: Arc<ModuleGraph>,
) -> Result<String, WorkspaceError> {
    let Some(parse) = parse else {
        let result = graph.module_info_for_path(path);
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
    let services =
        JsAnalyzerServices::from((params.module_graph, params.project_layout, file_source));
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
    } = params;
    let _ = debug_span!("Code actions JavaScript", range =? range, path =? path).entered();
    let tree = parse.tree();
    let _ = trace_span!("Parsed file").entered();
    let analyzer_options =
        settings.analyzer_options::<JsLanguage>(path, &language, suppression_reason.as_deref());
    let mut actions = Vec::new();
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(settings, analyzer_options)
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

    let services = JsAnalyzerServices::from((module_graph, project_layout, source_type));

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
    let rules = params.settings.as_linter_rules(params.biome_path.as_path());
    let analyzer_options = params.settings.analyzer_options::<JsLanguage>(
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

    let Some(file_source) = params
        .document_file_source
        .to_js_file_source()
        .or(JsFileSource::try_from(params.biome_path.as_path()).ok())
    else {
        return Err(extension_error(params.biome_path));
    };

    let mut actions = Vec::new();
    let mut skipped_suggested_fixes = 0;
    let mut errors: u16 = 0;
    // For detecting runaway edit growth
    let mut growth_guard = GrowthGuard::new(tree.syntax().text_range_with_trivia().len().into());

    loop {
        let services = JsAnalyzerServices::from((
            params.module_graph.clone(),
            params.project_layout.clone(),
            file_source,
        ));

        let (action, _) = analyze(
            &tree,
            filter,
            &analyzer_options,
            &params.plugins,
            services,
            |signal| {
                let current_diagnostic = signal.diagnostic();

                if let Some(diagnostic) = current_diagnostic.as_ref()
                    && is_diagnostic_error(diagnostic, rules.as_deref())
                {
                    errors += 1;
                }

                for action in signal.actions() {
                    match params.fix_file_mode {
                        FixFileMode::ApplySuppressions => {
                            if action.is_suppression() {
                                return ControlFlow::Break(action);
                            }
                        }
                        FixFileMode::SafeFixes => {
                            // suppression actions should not be part of safe fixes
                            if action.is_suppression() {
                                continue;
                            }
                            if action.applicability == Applicability::MaybeIncorrect {
                                skipped_suggested_fixes += 1;
                            }
                            if action.applicability == Applicability::Always {
                                errors = errors.saturating_sub(1);
                                return ControlFlow::Break(action);
                            }
                        }
                        FixFileMode::SafeAndUnsafeFixes => {
                            if action.is_suppression() {
                                continue;
                            }
                            if matches!(
                                action.applicability,
                                Applicability::Always | Applicability::MaybeIncorrect
                            ) {
                                errors = errors.saturating_sub(1);
                                return ControlFlow::Break(action);
                            }
                        }
                    }
                }

                ControlFlow::Continue(())
            },
        );

        match action {
            Some(action) => {
                if let (root, Some((range, _))) =
                    action.mutation.commit_with_text_range_and_edit(true)
                {
                    tree = match AnyJsRoot::cast(root) {
                        Some(tree) => tree,
                        None => {
                            return Err(WorkspaceError::RuleError(
                                RuleError::ReplacedRootWithNonRootError {
                                    rule_name: action.rule_name.map(|(group, rule)| {
                                        (Cow::Borrowed(group), Cow::Borrowed(rule))
                                    }),
                                },
                            ));
                        }
                    };
                    actions.push(FixAction {
                        rule_name: action
                            .rule_name
                            .map(|(group, rule)| (Cow::Borrowed(group), Cow::Borrowed(rule))),
                        range,
                    });

                    // Check for runaway edit growth
                    let curr_len: u32 = tree.syntax().text_range_with_trivia().len().into();
                    if !growth_guard.check(curr_len) {
                        // In order to provide a useful diagnostic, we want to flag the rules that caused the conflict.
                        // We can do this by inspecting the last few fixes that were applied.
                        // We limit it to the last 10 fixes. If there is a chain of conflicting fixes longer than that, something is **really** fucked up.

                        let mut seen_rules = HashSet::new();
                        for action in actions.iter().rev().take(10) {
                            if let Some((group, rule)) = action.rule_name.as_ref() {
                                seen_rules.insert((group.clone(), rule.clone()));
                            }
                        }

                        return Err(WorkspaceError::RuleError(
                            RuleError::ConflictingRuleFixesError {
                                rules: seen_rules.into_iter().collect(),
                            },
                        ));
                    }
                }
            }
            None => {
                let code = if params.should_format {
                    format_node(
                        params.settings.format_options::<JsLanguage>(
                            params.biome_path,
                            &params.document_file_source,
                        ),
                        tree.syntax(),
                    )?
                    .print()?
                    .into_code()
                } else {
                    tree.syntax().to_string()
                };
                return Ok(FixFileResult {
                    code,
                    skipped_suggested_fixes,
                    actions,
                    errors: errors.into(),
                });
            }
        }
    }
}

pub(crate) fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);
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

#[tracing::instrument(level = "debug", skip(parse, settings, document_file_source))]
pub(crate) fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
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
    settings: &Settings,
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
