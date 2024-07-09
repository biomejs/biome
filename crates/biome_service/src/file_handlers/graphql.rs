use super::{
    is_diagnostic_error, CodeActionsParams, DocumentFileSource, ExtensionHandler, FixAllParams,
    LintParams, LintResults, ParseResult, SearchCapabilities,
};
use crate::file_handlers::DebugCapabilities;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FormatterCapabilities, ParserCapabilities,
};
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, LinterSettings, OverrideSettings,
    ServiceLanguage, Settings, WorkspaceSettingsHandle,
};
use crate::workspace::{
    CodeAction, FixAction, FixFileMode, FixFileResult, GetSyntaxTreeResult, PullActionsResult,
};
use crate::WorkspaceError;
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never,
    RuleCategoriesBuilder, RuleCategory, RuleError,
};
use biome_diagnostics::{category, Applicability, Diagnostic, DiagnosticExt, Severity};
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
use std::borrow::Cow;
use tracing::{debug_span, error, info, trace, trace_span};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GraphqlFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub quote_style: Option<QuoteStyle>,
    pub bracket_spacing: Option<BracketSpacing>,
    pub enabled: Option<bool>,
}

impl Default for GraphqlFormatterSettings {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            indent_style: Default::default(),
            indent_width: Default::default(),
            line_ending: Default::default(),
            line_width: Default::default(),
            quote_style: Default::default(),
            bracket_spacing: Default::default(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GraphqlLinterSettings {
    pub enabled: Option<bool>,
}

impl Default for GraphqlLinterSettings {
    fn default() -> Self {
        Self {
            enabled: Some(false),
        }
    }
}

impl ServiceLanguage for GraphqlLanguage {
    type FormatterSettings = GraphqlFormatterSettings;
    type LinterSettings = GraphqlLinterSettings;
    type OrganizeImportsSettings = ();
    type FormatOptions = GraphqlFormatOptions;
    type ParserSettings = ();
    type EnvironmentSettings = ();

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
    ) -> AnalyzerOptions {
        AnalyzerOptions {
            configuration: AnalyzerConfiguration::default(),
            file_path: path.to_path_buf(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct GraphqlFileHandler;

impl ExtensionHandler for GraphqlFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
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
                organize_imports: None,
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

fn parse(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: Option<&Settings>,
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

    tracing::debug!("Format with the following options: \n{}", options);

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

    let range = tree.text_range();
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
    debug_span!("Linting GraphQL file", path =? params.path, language =? params.language).in_scope(
        move || {
            let workspace_settings = &params.workspace;
            let analyzer_options = workspace_settings
                .analyzer_options::<GraphqlLanguage>(params.path, &params.language);
            let tree = params.parse.tree();
            let mut diagnostics = params.parse.into_diagnostics();

            let has_only_filter = !params.only.is_empty();
            let mut rules = None;

            let enabled_rules = if let Some(settings) = params.workspace.settings() {
                // Compute final rules (taking `overrides` into account)
                rules = settings.as_rules(params.path.as_path());

                if has_only_filter {
                    params
                        .only
                        .into_iter()
                        .map(|selector| selector.into())
                        .collect::<Vec<_>>()
                } else {
                    rules
                        .as_ref()
                        .map(|rules| rules.as_enabled_rules())
                        .unwrap_or_default()
                        .into_iter()
                        .collect::<Vec<_>>()
                }
            } else {
                vec![]
            };

            let disabled_rules = params
                .skip
                .into_iter()
                .map(|selector| selector.into())
                .collect::<Vec<_>>();

            let filter = AnalysisFilter {
                categories: params.categories,
                enabled_rules: Some(enabled_rules.as_slice()),
                disabled_rules: &disabled_rules,
                range: None,
            };

            // Do not report unused suppression comment diagnostics if:
            // - it is a syntax-only analyzer pass, or
            // - if a single rule is run.
            let ignores_suppression_comment =
                !filter.categories.contains(RuleCategory::Lint) || has_only_filter;

            let mut diagnostic_count = diagnostics.len() as u32;
            let mut errors = diagnostics
                .iter()
                .filter(|diag| diag.severity() <= Severity::Error)
                .count();

            info!("Analyze file {}", params.path.display());
            let (_, analyze_diagnostics) = analyze(&tree, filter, &analyzer_options, |signal| {
                if let Some(mut diagnostic) = signal.diagnostic() {
                    // Do not report unused suppression comment diagnostics if this is a syntax-only analyzer pass
                    if ignores_suppression_comment
                        && diagnostic.category() == Some(category!("suppressions/unused"))
                    {
                        return ControlFlow::<Never>::Continue(());
                    }

                    diagnostic_count += 1;

                    // We do now check if the severity of the diagnostics should be changed.
                    // The configuration allows to change the severity of the diagnostics emitted by rules.
                    let severity = diagnostic
                        .category()
                        .filter(|category| category.name().starts_with("lint/"))
                        .map_or_else(
                            || diagnostic.severity(),
                            |category| {
                                rules
                                    .as_ref()
                                    .and_then(|rules| rules.get_severity_from_code(category))
                                    .unwrap_or(Severity::Warning)
                            },
                        );

                    if severity >= Severity::Error {
                        errors += 1;
                    }

                    if diagnostic_count <= params.max_diagnostics {
                        for action in signal.actions() {
                            if !action.is_suppression() {
                                diagnostic = diagnostic.add_code_suggestion(action.into());
                            }
                        }

                        let error = diagnostic.with_severity(severity);

                        diagnostics.push(biome_diagnostics::serde::Diagnostic::new(error));
                    }
                }

                ControlFlow::<Never>::Continue(())
            });

            diagnostics.extend(
                analyze_diagnostics
                    .into_iter()
                    .map(biome_diagnostics::serde::Diagnostic::new)
                    .collect::<Vec<_>>(),
            );
            let skipped_diagnostics = diagnostic_count.saturating_sub(diagnostics.len() as u32);

            LintResults {
                diagnostics,
                errors,
                skipped_diagnostics,
            }
        },
    )
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parse,
        range,
        workspace,
        path,
        manifest: _,
        language,
        settings,
    } = params;
    debug_span!("Code actions GraphQL", range =? range, path =? path).in_scope(move || {
        let tree = parse.tree();
        trace_span!("Parsed file", tree =? tree).in_scope(move || {
            let rules = settings.as_rules(params.path.as_path());
            let filter = rules
                .as_ref()
                .map(|rules| rules.as_enabled_rules())
                .unwrap_or_default()
                .into_iter()
                .collect::<Vec<_>>();

            let mut filter = AnalysisFilter::from_enabled_rules(filter.as_slice());

            let mut categories = RuleCategoriesBuilder::default().with_syntax().with_lint();
            if settings.organize_imports.enabled {
                categories = categories.with_action();
            }
            filter.categories = categories.build();
            filter.range = Some(range);

            let analyzer_options = workspace.analyzer_options::<GraphqlLanguage>(path, &language);

            let Some(_) = language.to_graphql_file_source() else {
                error!("Could not determine the file source of the file");
                return PullActionsResult { actions: vec![] };
            };

            trace!("GraphQL runs the analyzer");
            let mut actions = Vec::new();

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
        })
    })
}

/// If applies all the safe fixes to the given syntax tree.
pub(crate) fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let FixAllParams {
        parse,
        fix_file_mode,
        workspace,
        should_format: _, // we don't have a formatter yet
        biome_path,
        manifest: _,
        document_file_source,
    } = params;

    let settings = workspace.settings();
    let Some(settings) = settings else {
        let tree: GraphqlRoot = parse.tree();

        return Ok(FixFileResult {
            actions: vec![],
            errors: 0,
            skipped_suggested_fixes: 0,
            code: tree.syntax().to_string(),
        });
    };

    let mut tree: GraphqlRoot = parse.tree();
    let mut actions = Vec::new();

    // Compute final rules (taking `overrides` into account)
    let rules = settings.as_rules(params.biome_path.as_path());
    let rule_filter_list = rules
        .as_ref()
        .map(|rules| rules.as_enabled_rules())
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>();
    let filter = AnalysisFilter::from_enabled_rules(rule_filter_list.as_slice());

    let mut skipped_suggested_fixes = 0;
    let mut errors: u16 = 0;
    let analyzer_options =
        workspace.analyzer_options::<GraphqlLanguage>(biome_path, &document_file_source);
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

                match fix_file_mode {
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
