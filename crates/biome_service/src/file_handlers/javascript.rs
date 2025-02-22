use super::{
    search, AnalyzerCapabilities, AnalyzerVisitorBuilder, CodeActionsParams, DebugCapabilities,
    EnabledForPath, ExtensionHandler, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, ProcessLint, SearchCapabilities,
};
use crate::configuration::to_analyzer_rules;
use crate::diagnostics::extension_error;
use crate::file_handlers::{is_diagnostic_error, FixAllParams};
use crate::settings::{
    check_feature_activity, check_override_feature_activity, LinterSettings, OverrideSettings,
    Settings,
};
use crate::workspace::DocumentFileSource;
use crate::{
    settings::{
        FormatSettings, LanguageListSettings, LanguageSettings, ServiceLanguage,
        WorkspaceSettingsHandle,
    },
    workspace::{
        CodeAction, FixAction, FixFileMode, FixFileResult, GetSyntaxTreeResult, PullActionsResult,
        RenameResult,
    },
    WorkspaceError,
};
use biome_analyze::options::PreferredQuote;
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
    AttributePosition, BracketSameLine, BracketSpacing, FormatError, IndentStyle, IndentWidth,
    LineEnding, LineWidth, ObjectWrap, Printed, QuoteStyle,
};
use biome_fs::BiomePath;
use biome_js_analyze::utils::rename::{RenameError, RenameSymbolExtensions};
use biome_js_analyze::{
    analyze, analyze_with_inspect_matcher, ControlFlowGraph, JsAnalyzerServices,
};
use biome_js_formatter::context::trailing_commas::TrailingCommas;
use biome_js_formatter::context::{ArrowParentheses, JsFormatOptions, QuoteProperties, Semicolons};
use biome_js_formatter::format_node;
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{semantic_model, SemanticModelOptions};
use biome_js_syntax::{
    AnyJsRoot, JsFileSource, JsLanguage, JsSyntaxNode, LanguageVariant, TextRange, TextSize,
    TokenAtOffset,
};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, BatchMutationExt, Direction, NodeCache};
use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;
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
    pub object_wrap: Option<ObjectWrap>,
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
            object_wrap: value.object_wrap,
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
    type FormatOptions = JsFormatOptions;
    type ParserSettings = JsParserSettings;
    type EnvironmentSettings = JsEnvironmentSettings;
    type AssistSettings = JsAssistSettings;

    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self> {
        &languages.javascript
    }

    fn resolve_format_options(
        global: Option<&FormatSettings>,
        overrides: Option<&OverrideSettings>,
        language: Option<&JsFormatterSettings>,
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
                .and_then(|l| l.indent_style)
                .or(global.and_then(|g| g.indent_style))
                .unwrap_or_default(),
        )
        .with_indent_width(
            language
                .and_then(|l| l.indent_width)
                .or(global.and_then(|g| g.indent_width))
                .unwrap_or_default(),
        )
        .with_line_width(
            language
                .and_then(|l| l.line_width)
                .or(global.and_then(|g| g.line_width))
                .unwrap_or_default(),
        )
        .with_line_ending(
            language
                .and_then(|l| l.line_ending)
                .or(global.and_then(|g| g.line_ending))
                .unwrap_or_default(),
        )
        .with_quote_style(language.and_then(|l| l.quote_style).unwrap_or_default())
        .with_jsx_quote_style(language.and_then(|l| l.jsx_quote_style).unwrap_or_default())
        .with_quote_properties(
            language
                .and_then(|l| l.quote_properties)
                .unwrap_or_default(),
        )
        .with_trailing_commas(language.and_then(|l| l.trailing_commas).unwrap_or_default())
        .with_semicolons(language.and_then(|l| l.semicolons).unwrap_or_default())
        .with_arrow_parentheses(
            language
                .and_then(|l| l.arrow_parentheses)
                .unwrap_or_default(),
        )
        .with_bracket_spacing(
            language
                .and_then(|l| l.bracket_spacing)
                .or(global.and_then(|g| g.bracket_spacing))
                .unwrap_or_default(),
        )
        .with_bracket_same_line(
            language
                .and_then(|l| l.bracket_same_line)
                .or(global.and_then(|g| g.bracket_same_line))
                .unwrap_or_default(),
        )
        .with_attribute_position(
            language
                .and_then(|l| l.attribute_position)
                .or(global.and_then(|g| g.attribute_position))
                .unwrap_or_default(),
        )
        .with_object_wrap(
            language
                .and_then(|l| l.object_wrap)
                .or(global.and_then(|g| g.object_wrap))
                .unwrap_or_default(),
        );

        if let Some(overrides) = overrides {
            overrides.override_js_format_options(path, options)
        } else {
            options
        }
    }

    fn resolve_analyzer_options(
        global: Option<&Settings>,
        _linter: Option<&LinterSettings>,
        overrides: Option<&OverrideSettings>,
        _language: Option<&Self::LinterSettings>,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        let preferred_quote =
            global
                .and_then(|global| {
                    global.languages.javascript.formatter.quote_style.map(
                        |quote_style: QuoteStyle| {
                            if quote_style == QuoteStyle::Single {
                                PreferredQuote::Single
                            } else {
                                PreferredQuote::Double
                            }
                        },
                    )
                })
                .unwrap_or_default();
        let preferred_jsx_quote = global
            .and_then(|global| {
                global.languages.javascript.formatter.jsx_quote_style.map(
                    |quote_style: QuoteStyle| {
                        if quote_style == QuoteStyle::Single {
                            PreferredQuote::Single
                        } else {
                            PreferredQuote::Double
                        }
                    },
                )
            })
            .unwrap_or_default();

        let mut configuration = AnalyzerConfiguration::default();
        let mut globals = Vec::new();

        if let (Some(overrides), Some(global)) = (overrides, global) {
            let jsx_runtime = match overrides.override_jsx_runtime(
                path,
                global
                    .languages
                    .javascript
                    .environment
                    .jsx_runtime
                    .unwrap_or_default(),
            ) {
                // In the future, we may wish to map an `Auto` variant to a concrete
                // analyzer value for easy access by the analyzer.
                JsxRuntime::Transparent => biome_analyze::options::JsxRuntime::Transparent,
                JsxRuntime::ReactClassic => biome_analyze::options::JsxRuntime::ReactClassic,
            };
            configuration = configuration.with_jsx_runtime(jsx_runtime);

            globals.extend(
                overrides
                    .override_js_globals(path, &global.languages.javascript.globals)
                    .into_iter()
                    .collect::<Vec<_>>(),
            );
        }

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
            .with_rules(
                global
                    .map(|g| to_analyzer_rules(g, path.as_path()))
                    .unwrap_or_default(),
            )
            .with_globals(globals)
            .with_preferred_quote(preferred_quote)
            .with_preferred_jsx_quote(preferred_jsx_quote);

        AnalyzerOptions::default()
            .with_file_path(path.as_path())
            .with_configuration(configuration)
            .with_suppression_reason(suppression_reason)
    }

    fn formatter_enabled_for_file_path(settings: Option<&Settings>, path: &Utf8Path) -> bool {
        settings
            .and_then(|settings| {
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

                overrides_activity.or(check_feature_activity(
                    settings.languages.javascript.formatter.enabled,
                    settings.formatter.enabled,
                ))
            })
            .unwrap_or_default()
            .into()
    }

    fn assist_enabled_for_file_path(settings: Option<&Settings>, path: &Utf8Path) -> bool {
        settings
            .and_then(|settings| {
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

                overrides_activity.or(check_feature_activity(
                    settings.languages.javascript.assist.enabled,
                    settings.assist.enabled,
                ))
            })
            .unwrap_or_default()
            .into()
    }

    fn linter_enabled_for_file_path(settings: Option<&Settings>, path: &Utf8Path) -> bool {
        settings
            .and_then(|settings| {
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

                overrides_activity.or(check_feature_activity(
                    settings.languages.javascript.linter.enabled,
                    settings.linter.enabled,
                ))
            })
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
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: Some(debug_control_flow),
                debug_formatter_ir: Some(debug_formatter_ir),
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                fix_all: Some(fix_all),
                rename: Some(rename),
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
            },
            search: SearchCapabilities {
                search: Some(search),
            },
        }
    }
}

pub fn formatter_enabled(path: &Utf8Path, handle: &WorkspaceSettingsHandle) -> bool {
    handle.formatter_enabled_for_file_path::<JsLanguage>(path)
}

pub fn linter_enabled(path: &Utf8Path, handle: &WorkspaceSettingsHandle) -> bool {
    handle.linter_enabled_for_file_path::<JsLanguage>(path)
}

pub fn assist_enabled(path: &Utf8Path, handle: &WorkspaceSettingsHandle) -> bool {
    handle.assist_enabled_for_file_path::<JsLanguage>(path)
}

pub fn search_enabled(_path: &Utf8Path, _handle: &WorkspaceSettingsHandle) -> bool {
    true
}

fn parse(
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    handle: WorkspaceSettingsHandle,
    cache: &mut NodeCache,
) -> ParseResult {
    let settings = handle.settings();
    let mut options = JsParserOptions {
        grit_metavariables: false,
        parse_class_parameter_decorators: settings.is_some_and(|settings| {
            settings
                .languages
                .javascript
                .parser
                .parse_class_parameter_decorators
                .unwrap_or_default()
                .into()
        }),
    };
    let jsx_everywhere = settings.is_some_and(|settings| {
        settings
            .languages
            .javascript
            .parser
            .jsx_everywhere
            .unwrap_or_default()
            .into()
    });
    if let Some(settings) = settings {
        options = settings
            .override_settings
            .to_override_js_parser_options(biome_path, options);
    }

    let mut file_source = file_source.to_js_file_source().unwrap_or_default();
    if jsx_everywhere {
        file_source = file_source.with_variant(LanguageVariant::Jsx);
    }
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
    settings: WorkspaceSettingsHandle,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
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
    let analyzer_options = params.workspace.analyzer_options::<JsLanguage>(
        params.path,
        &params.language,
        params.suppression_reason.as_deref(),
    );
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.workspace.settings(), analyzer_options)
            .with_only(&params.only)
            .with_skip(&params.skip)
            .with_path(params.path.as_path())
            .with_enabled_rules(&params.enabled_rules)
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
        JsAnalyzerServices::from((params.dependency_graph, params.project_layout, file_source));
    let (_, analyze_diagnostics) = analyze(
        &tree,
        filter,
        &analyzer_options,
        &params.plugins,
        services,
        |signal| process_lint.process_signal(signal),
    );

    process_lint.into_result(params.parse.into_diagnostics(), analyze_diagnostics)
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parse,
        range,
        workspace,
        path,
        dependency_graph,
        project_layout,
        language,
        only,
        skip,
        suppression_reason,
        enabled_rules: rules,
        plugins,
    } = params;
    let _ = debug_span!("Code actions JavaScript", range =? range, path =? path).entered();
    let tree = parse.tree();
    let _ = trace_span!("Parsed file").entered();
    let analyzer_options =
        workspace.analyzer_options::<JsLanguage>(path, &language, suppression_reason.as_deref());
    let mut actions = Vec::new();
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.workspace.settings(), analyzer_options)
            .with_only(&only)
            .with_skip(&skip)
            .with_path(path.as_path())
            .with_enabled_rules(&rules)
            .with_project_layout(project_layout.clone())
            .finish();

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default()
            .with_syntax()
            .with_lint()
            .with_assist()
            .build(),
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

    let services = JsAnalyzerServices::from((dependency_graph, project_layout, source_type));

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
    let Some(settings) = params.workspace.settings() else {
        return Ok(FixFileResult {
            actions: Vec::new(),
            errors: 0,
            skipped_suggested_fixes: 0,
            code: tree.syntax().to_string(),
        });
    };

    // Compute final rules (taking `overrides` into account)
    let rules = settings.as_linter_rules(params.biome_path.as_path());
    let analyzer_options = params.workspace.analyzer_options::<JsLanguage>(
        params.biome_path,
        &params.document_file_source,
        params.suppression_reason.as_deref(),
    );
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.workspace.settings(), analyzer_options)
            .with_only(&params.only)
            .with_skip(&params.skip)
            .with_path(params.biome_path.as_path())
            .with_enabled_rules(&params.enabled_rules)
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

    loop {
        let services = JsAnalyzerServices::from((
            params.dependency_graph.clone(),
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

                if let Some(diagnostic) = current_diagnostic.as_ref() {
                    if is_diagnostic_error(diagnostic, rules.as_deref()) {
                        errors += 1;
                    }
                }

                for action in signal.actions() {
                    match params.fix_file_mode {
                        FixFileMode::ApplySuppressions => {
                            if action.is_suppression() {
                                return ControlFlow::Break(action);
                            }
                        }
                        FixFileMode::SafeFixes => {
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
                }
            }
            None => {
                let code = if params.should_format {
                    format_node(
                        params.workspace.format_options::<JsLanguage>(
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
    settings: WorkspaceSettingsHandle,
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
    settings: WorkspaceSettingsHandle,
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
    settings: WorkspaceSettingsHandle,
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
                    let (range, indels) = batch.as_text_range_and_edit().unwrap_or_default();
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
