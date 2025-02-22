use super::{
    is_diagnostic_error, AnalyzerVisitorBuilder, CodeActionsParams, DocumentFileSource,
    EnabledForPath, ExtensionHandler, FixAllParams, LintParams, LintResults, ParseResult,
    ProcessLint, SearchCapabilities,
};
use crate::file_handlers::DebugCapabilities;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FormatterCapabilities, ParserCapabilities,
};
use crate::settings::{
    check_feature_activity, check_override_feature_activity, FormatSettings, LanguageListSettings,
    LanguageSettings, LinterSettings, OverrideSettings, ServiceLanguage, Settings,
    WorkspaceSettingsHandle,
};
use crate::workspace::{
    CodeAction, FixAction, FixFileMode, FixFileResult, GetSyntaxTreeResult, PullActionsResult,
};
use crate::WorkspaceError;
use biome_analyze::{
    AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleCategoriesBuilder, RuleError,
};
use biome_configuration::graphql::{
    GraphqlAssistConfiguration, GraphqlAssistEnabled, GraphqlFormatterConfiguration,
    GraphqlFormatterEnabled, GraphqlLinterConfiguration, GraphqlLinterEnabled,
};
use biome_diagnostics::Applicability;
use biome_formatter::{
    BracketSpacing, FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed,
    QuoteStyle,
};
use biome_fs::BiomePath;
use biome_graphql_analyze::analyze;
use biome_graphql_formatter::context::GraphqlFormatOptions;
use biome_graphql_formatter::format_node;
use biome_graphql_parser::parse_graphql_with_cache;
use biome_graphql_syntax::{GraphqlLanguage, GraphqlRoot, GraphqlSyntaxNode, TextRange, TextSize};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, NodeCache, TokenAtOffset};
use camino::Utf8Path;
use std::borrow::Cow;
use tracing::{debug_span, error, info, trace_span};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GraphqlFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub quote_style: Option<QuoteStyle>,
    pub bracket_spacing: Option<BracketSpacing>,
    pub enabled: Option<GraphqlFormatterEnabled>,
}

impl From<GraphqlFormatterConfiguration> for GraphqlFormatterSettings {
    fn from(configuration: GraphqlFormatterConfiguration) -> Self {
        Self {
            line_ending: configuration.line_ending,
            line_width: configuration.line_width,
            indent_width: configuration.indent_width,
            indent_style: configuration.indent_style,
            quote_style: configuration.quote_style,
            bracket_spacing: configuration.bracket_spacing,
            enabled: configuration.enabled,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GraphqlLinterSettings {
    pub enabled: Option<GraphqlLinterEnabled>,
}

impl From<GraphqlLinterConfiguration> for GraphqlLinterSettings {
    fn from(configuration: GraphqlLinterConfiguration) -> Self {
        Self {
            enabled: configuration.enabled,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GraphqlAssistSettings {
    pub enabled: Option<GraphqlAssistEnabled>,
}

impl From<GraphqlAssistConfiguration> for GraphqlAssistSettings {
    fn from(configuration: GraphqlAssistConfiguration) -> Self {
        Self {
            enabled: configuration.enabled,
        }
    }
}

impl ServiceLanguage for GraphqlLanguage {
    type FormatterSettings = GraphqlFormatterSettings;
    type LinterSettings = GraphqlLinterSettings;
    type FormatOptions = GraphqlFormatOptions;
    type ParserSettings = ();
    type EnvironmentSettings = ();
    type AssistSettings = GraphqlAssistSettings;

    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.graphql
    }

    fn resolve_format_options(
        global: Option<&FormatSettings>,
        overrides: Option<&OverrideSettings>,
        language: Option<&Self::FormatterSettings>,
        path: &BiomePath,
        document_file_source: &DocumentFileSource,
    ) -> Self::FormatOptions {
        let indent_style = language
            .and_then(|l| l.indent_style)
            .or(global.and_then(|g| g.indent_style))
            .unwrap_or_default();
        let line_width = language
            .and_then(|l| l.line_width)
            .or(global.and_then(|g| g.line_width))
            .unwrap_or_default();
        let indent_width = language
            .and_then(|l| l.indent_width)
            .or(global.and_then(|g| g.indent_width))
            .unwrap_or_default();

        let line_ending = language
            .and_then(|l| l.line_ending)
            .or(global.and_then(|g| g.line_ending))
            .unwrap_or_default();

        let bracket_spacing = language
            .and_then(|l| l.bracket_spacing)
            .or(global.and_then(|g| g.bracket_spacing))
            .unwrap_or_default();

        let options = GraphqlFormatOptions::new(
            document_file_source
                .to_graphql_file_source()
                .unwrap_or_default(),
        )
        .with_indent_style(indent_style)
        .with_indent_width(indent_width)
        .with_line_width(line_width)
        .with_line_ending(line_ending)
        .with_bracket_spacing(bracket_spacing)
        .with_quote_style(language.and_then(|l| l.quote_style).unwrap_or_default());
        if let Some(overrides) = overrides {
            overrides.to_override_graphql_format_options(path, options)
        } else {
            options
        }
    }

    fn resolve_analyzer_options(
        _global: Option<&Settings>,
        _linter: Option<&LinterSettings>,
        _overrides: Option<&OverrideSettings>,
        _language: Option<&Self::LinterSettings>,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        AnalyzerOptions::default()
            .with_file_path(path.as_path())
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
                                pattern.languages.graphql.formatter.enabled,
                                pattern.formatter.enabled,
                            )
                            .filter(|_| {
                                // Then check whether the path satisfies
                                pattern.is_file_included(path)
                            })
                        });

                overrides_activity.or(check_feature_activity(
                    settings.languages.graphql.formatter.enabled,
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
                                pattern.languages.graphql.assist.enabled,
                                pattern.assist.enabled,
                            )
                            .filter(|_| {
                                // Then check whether the path satisfies
                                pattern.is_file_included(path)
                            })
                        });

                overrides_activity.or(check_feature_activity(
                    settings.languages.graphql.assist.enabled,
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
                                pattern.languages.graphql.linter.enabled,
                                pattern.linter.enabled,
                            )
                            .filter(|_| {
                                // Then check whether the path satisfies
                                pattern.is_file_included(path)
                            })
                        });

                overrides_activity.or(check_feature_activity(
                    settings.languages.graphql.linter.enabled,
                    settings.linter.enabled,
                ))
            })
            .unwrap_or_default()
            .into()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct GraphqlFileHandler;

impl ExtensionHandler for GraphqlFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            enabled_for_path: EnabledForPath {
                formatter: Some(formatter_enabled),
                assist: Some(assist_enabled),
                linter: Some(linter_enabled),
                search: Some(search_enabled),
            },
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: Some(debug_formatter_ir),
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                rename: None,
                fix_all: Some(fix_all),
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
            },
            search: SearchCapabilities { search: None },
        }
    }
}

fn formatter_enabled(path: &Utf8Path, handle: &WorkspaceSettingsHandle) -> bool {
    handle.formatter_enabled_for_file_path::<GraphqlLanguage>(path)
}

fn linter_enabled(path: &Utf8Path, handle: &WorkspaceSettingsHandle) -> bool {
    handle.linter_enabled_for_file_path::<GraphqlLanguage>(path)
}

fn assist_enabled(path: &Utf8Path, handle: &WorkspaceSettingsHandle) -> bool {
    handle.assist_enabled_for_file_path::<GraphqlLanguage>(path)
}

fn search_enabled(_path: &Utf8Path, _handle: &WorkspaceSettingsHandle) -> bool {
    true
}

fn parse(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: WorkspaceSettingsHandle,
    cache: &mut NodeCache,
) -> ParseResult {
    let parse = parse_graphql_with_cache(text, cache);

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

fn debug_syntax_tree(_rome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: GraphqlSyntaxNode = parse.syntax();
    let tree: GraphqlRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<GraphqlLanguage>(biome_path, document_file_source);

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
    settings: WorkspaceSettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<GraphqlLanguage>(biome_path, document_file_source);

    tracing::debug!("Format with the following options: {:?}", options);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<GraphqlLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let printed = biome_graphql_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<GraphqlLanguage>(biome_path, document_file_source);

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

    let printed = biome_graphql_formatter::format_sub_tree(options, &root_node)?;
    Ok(printed)
}

fn lint(params: LintParams) -> LintResults {
    let _ = debug_span!("Linting GraphQL file", path =? params.path, language =? params.language)
        .entered();
    let workspace_settings = &params.workspace;
    let analyzer_options = workspace_settings.analyzer_options::<GraphqlLanguage>(
        params.path,
        &params.language,
        params.suppression_reason.as_deref(),
    );
    let tree = params.parse.tree();

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

    let (_, analyze_diagnostics) = analyze(&tree, filter, &analyzer_options, |signal| {
        process_lint.process_signal(signal)
    });

    process_lint.into_result(params.parse.into_diagnostics(), analyze_diagnostics)
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parse,
        range,
        workspace,
        path,
        dependency_graph: _,
        project_layout,
        language,
        only,
        skip,
        suppression_reason,
        enabled_rules: rules,
        plugins: _,
    } = params;
    let _ = debug_span!("Code actions GraphQL", range =? range, path =? path).entered();
    let tree = parse.tree();
    let _ = trace_span!("Parsed file", tree =? tree).entered();
    let Some(_) = language.to_graphql_file_source() else {
        error!("Could not determine the file source of the file");
        return PullActionsResult {
            actions: Vec::new(),
        };
    };

    let analyzer_options = workspace.analyzer_options::<GraphqlLanguage>(
        path,
        &language,
        suppression_reason.as_deref(),
    );
    let mut actions = Vec::new();
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.workspace.settings(), analyzer_options)
            .with_only(&only)
            .with_skip(&skip)
            .with_path(path.as_path())
            .with_enabled_rules(&rules)
            .with_project_layout(project_layout)
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

    info!("GraphQL runs the analyzer");

    analyze(&tree, filter, &analyzer_options, |signal| {
        actions.extend(signal.actions().into_code_action_iter().map(|item| {
            CodeAction {
                category: item.category.clone(),
                rule_name: item
                    .rule_name
                    .map(|(group, name)| (Cow::Borrowed(group), Cow::Borrowed(name))),
                suggestion: item.suggestion,
            }
        }));

        ControlFlow::<Never>::Continue(())
    });

    PullActionsResult { actions }
}

/// If applies all the safe fixes to the given syntax tree.
pub(crate) fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let mut tree: GraphqlRoot = params.parse.tree();
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
    let analyzer_options = params.workspace.analyzer_options::<GraphqlLanguage>(
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
            .with_project_layout(params.project_layout)
            .finish();

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default()
            .with_syntax()
            .with_lint()
            .build(),
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    let mut actions = Vec::new();
    let mut skipped_suggested_fixes = 0;
    let mut errors: u16 = 0;

    loop {
        let (action, _) = analyze(&tree, filter, &analyzer_options, |signal| {
            let current_diagnostic = signal.diagnostic();

            if let Some(diagnostic) = current_diagnostic.as_ref() {
                if is_diagnostic_error(diagnostic, rules.as_deref()) {
                    errors += 1;
                }
            }

            for action in signal.actions() {
                // suppression actions should not be part of the fixes (safe or suggested)
                if action.is_suppression() {
                    continue;
                }

                match params.fix_file_mode {
                    FixFileMode::SafeFixes => {
                        if action.applicability == Applicability::MaybeIncorrect {
                            skipped_suggested_fixes += 1;
                        }
                        if action.applicability == Applicability::Always {
                            errors = errors.saturating_sub(1);
                            return ControlFlow::Break(action);
                        }
                    }
                    FixFileMode::SafeAndUnsafeFixes => {
                        if matches!(
                            action.applicability,
                            Applicability::Always | Applicability::MaybeIncorrect
                        ) {
                            errors = errors.saturating_sub(1);
                            return ControlFlow::Break(action);
                        }
                    }
                    FixFileMode::ApplySuppressions => {
                        // TODO: implement once a GraphQL suppression action is available
                    }
                }
            }

            ControlFlow::Continue(())
        });

        match action {
            Some(action) => {
                if let (root, Some((range, _))) =
                    action.mutation.commit_with_text_range_and_edit(true)
                {
                    tree = match GraphqlRoot::cast(root) {
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
                // we don't have a formatter yet
                // let code = if should_format {
                //     format_node(
                //         workspace.format_options::<GraphqlLanguage>(biome_path, &document_file_source),
                //         tree.syntax(),
                //     )?
                //         .print()?
                //         .into_code()
                // } else {
                let code = tree.syntax().to_string();
                // };
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
