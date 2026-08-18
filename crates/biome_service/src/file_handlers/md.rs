use super::{
    AnalyzerCapabilities, AnalyzerVisitorBuilder, AnalyzerVisitorResult, Capabilities,
    CodeActionsParams, DebugCapabilities, DocumentFileSource, EditorCapabilities, EnabledForPath,
    ExtensionHandler, FixAllParams, FixedFileResult, FormatterCapabilities, LintParams,
    LintResults, ParseResult, ParserCapabilities, ProcessFixAll, ProcessLint, SearchCapabilities,
};
use crate::WorkspaceError;
use crate::configuration::to_analyzer_rules_by_indices;
use crate::db::WorkspaceDb;
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, OverrideSettings, ServiceLanguage,
    Settings, SettingsIdentity, SettingsWithEditor, check_feature_activity,
    check_override_feature_activity, finalize_analyzer_options,
};
use crate::workspace::{CodeAction, FixFileMode, GetSyntaxTreeResult, PullActionsResult};
use biome_analyze::{
    ActionFilter, AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never,
};
use biome_configuration::analyzer::assist::AssistEnabled;
use biome_configuration::markdown::{
    MarkdownFormatterConfiguration, MarkdownFormatterEnabled, MarkdownLinterEnabled,
    MarkdownParseFrontmatter, MarkdownParserConfiguration,
};
use biome_db::AnyParsedSource;
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth, Printed, TrailingNewline};
use biome_fs::BiomePath;
use biome_markdown_analyze::analyze;
use biome_markdown_formatter::context::{MdFormatOptions, ProseWrap};
use biome_markdown_formatter::format_node;
use biome_markdown_parser::{MarkdownParserOptions, parse_markdown_with_cache};
use biome_markdown_syntax::{MarkdownLanguage, MarkdownSyntaxNode, MdRoot};
use biome_parser::NodeParse;
use biome_rowan::{AstNode, NodeCache};
use camino::Utf8Path;
use std::borrow::Cow;
use tracing::{debug, debug_span, error};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MarkdownFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub trailing_newline: Option<TrailingNewline>,
    pub enabled: Option<MarkdownFormatterEnabled>,
    pub prose_wrap: Option<ProseWrap>,
}

impl From<MarkdownFormatterConfiguration> for MarkdownFormatterSettings {
    fn from(configuration: MarkdownFormatterConfiguration) -> Self {
        Self {
            line_ending: configuration.line_ending,
            line_width: configuration.line_width,
            indent_width: configuration.indent_width,
            indent_style: configuration.indent_style,
            enabled: configuration.enabled,
            trailing_newline: configuration.trailing_newline,
            prose_wrap: configuration.prose_wrap,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MarkdownParserSettings {
    pub frontmatter: Option<MarkdownParseFrontmatter>,
}

impl From<MarkdownParserConfiguration> for MarkdownParserSettings {
    fn from(configuration: MarkdownParserConfiguration) -> Self {
        Self {
            frontmatter: configuration.frontmatter,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MarkdownLinterSettings {
    pub enabled: Option<MarkdownLinterEnabled>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MarkdownAssistSettings {
    pub enabled: Option<AssistEnabled>,
}

impl ServiceLanguage for MarkdownLanguage {
    type FormatterSettings = MarkdownFormatterSettings;
    type LinterSettings = MarkdownLinterSettings;
    type AssistSettings = MarkdownAssistSettings;
    type FormatOptions = MdFormatOptions;
    type ParserSettings = MarkdownParserSettings;
    type ParserOptions = MarkdownParserOptions;
    type EnvironmentSettings = ();

    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.markdown
    }

    fn resolve_environment(_settings: &Settings) -> Option<&Self::EnvironmentSettings> {
        None
    }

    fn resolve_parse_options(
        _overrides: &OverrideSettings,
        language: &Self::ParserSettings,
        _path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> Self::ParserOptions {
        MarkdownParserOptions::default()
            .with_frontmatter(language.frontmatter.unwrap_or_default().into())
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        override_indices: &[usize],
        _file_source: &DocumentFileSource,
    ) -> Self::FormatOptions {
        // TODO: apply markdown overrides once markdown override settings are introduced.
        let _ = (overrides, override_indices);

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
        let trailing_newline = language
            .trailing_newline
            .or(global.trailing_newline)
            .unwrap_or_default();
        let prose_wrap = language.prose_wrap.unwrap_or_default();
        MdFormatOptions::new()
            .with_indent_style(indent_style)
            .with_indent_width(indent_width)
            .with_line_width(line_width)
            .with_line_ending(line_ending)
            .with_trailing_newline(trailing_newline)
            .with_prose_wrap(prose_wrap)
    }

    fn resolve_analyzer_options(
        global: &Settings,
        _language: &Self::LinterSettings,
        _environment: Option<&Self::EnvironmentSettings>,
        override_indices: &[usize],
        _file_source: &DocumentFileSource,
    ) -> AnalyzerOptions {
        let configuration = AnalyzerConfiguration::default()
            .with_rules(to_analyzer_rules_by_indices(global, override_indices));

        AnalyzerOptions::default().with_configuration(configuration)
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
                        pattern.languages.markdown.linter.enabled,
                        pattern.linter.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.markdown.linter.enabled,
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
                        pattern.languages.markdown.formatter.enabled,
                        pattern.formatter.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.markdown.formatter.enabled,
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
                        pattern.languages.markdown.assist.enabled,
                        pattern.assist.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.markdown.assist.enabled,
                settings.assist.enabled,
            ))
            .unwrap_or_default()
            .into()
    }
}

#[salsa::interned]
struct MarkdownFormatOptionsInput {
    #[returns(ref)]
    settings: SettingsIdentity,
    #[returns(ref)]
    override_indices: Box<[usize]>,
    #[returns(ref)]
    file_source: DocumentFileSource,
}

#[salsa::tracked(returns(clone))]
fn resolved_markdown_format_options<'db>(
    db: &'db dyn salsa::Database,
    input: MarkdownFormatOptionsInput<'db>,
) -> MdFormatOptions {
    input
        .settings(db)
        .as_ref()
        .format_options::<MarkdownLanguage>(input.override_indices(db), input.file_source(db))
}

#[salsa::interned]
struct MarkdownAnalyzerOptionsInput {
    #[returns(ref)]
    settings: SettingsIdentity,
    #[returns(ref)]
    override_indices: Box<[usize]>,
    #[returns(ref)]
    file_source: DocumentFileSource,
}

#[salsa::tracked(returns(clone))]
fn resolved_markdown_analyzer_options<'db>(
    db: &'db dyn salsa::Database,
    input: MarkdownAnalyzerOptionsInput<'db>,
) -> AnalyzerOptions {
    input
        .settings(db)
        .as_ref()
        .analyzer_options::<MarkdownLanguage>(input.override_indices(db), input.file_source(db))
}

pub(in crate::file_handlers) fn resolve_format_options(
    _path: &BiomePath,
    source: &DocumentFileSource,
    settings: &SettingsWithEditor,
    workspace_db: &WorkspaceDb,
) -> MdFormatOptions {
    let query = settings.query();
    if query.inline_settings().is_some() {
        return settings.format_options::<MarkdownLanguage>(source);
    }
    let selected_settings = query
        .selection()
        .selected_settings(workspace_db, query.project());
    let query_db = workspace_db.settings_query_db();
    let input = MarkdownFormatOptionsInput::new(
        &query_db,
        selected_settings,
        query.override_indices(),
        *source,
    );
    resolved_markdown_format_options(&query_db, input)
}

fn resolve_analyzer_options(
    path: &BiomePath,
    working_directory: Option<&Utf8Path>,
    source: &DocumentFileSource,
    suppression_reason: Option<&str>,
    settings: &SettingsWithEditor,
    workspace_db: &WorkspaceDb,
) -> AnalyzerOptions {
    let query = settings.query();
    let options = if query.inline_settings().is_some() {
        settings.analyzer_options::<MarkdownLanguage>(source)
    } else {
        let selected_settings = query
            .selection()
            .selected_settings(workspace_db, query.project());
        let query_db = workspace_db.settings_query_db();
        let input = MarkdownAnalyzerOptionsInput::new(
            &query_db,
            selected_settings,
            query.override_indices(),
            *source,
        );
        resolved_markdown_analyzer_options(&query_db, input)
    };
    finalize_analyzer_options(options, path, working_directory, suppression_reason)
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct MarkdownFileHandler;

impl ExtensionHandler for MarkdownFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            enabled_for_path: EnabledForPath {
                formatter: Some(formatter_enabled),
                linter: Some(linter_enabled),
                assist: Some(assist_enabled),
                search: None,
            },
            parser: ParserCapabilities {
                parse: Some(parse),
                parse_embedded_nodes: None,
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
                fix_all: Some(fix_all),
                rename: None,
                update_snippets: None,
                pull_diagnostics_and_actions: None,
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: None,
                format_on_type: None,
                format_embedded: None,
            },
            search: SearchCapabilities { search: None },
            editors: EditorCapabilities {
                resolve_binding: None,
                resolve_definition: None,
            },
        }
    }
}

fn formatter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.formatter_enabled_for_file_path::<MarkdownLanguage>(path)
}

fn linter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.linter_enabled_for_file_path::<MarkdownLanguage>(path)
}

fn assist_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.assist_enabled_for_file_path::<MarkdownLanguage>(path)
}

fn parse(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    settings: &SettingsWithEditor,
    cache: &mut NodeCache,
) -> ParseResult {
    let options = settings.parse_options::<MarkdownLanguage>(_biome_path, &file_source);
    let parse = parse_markdown_with_cache(text, cache, options);
    let any_parse =
        NodeParse::new(parse.syntax().as_send().unwrap(), parse.into_diagnostics()).into();

    ParseResult {
        any_parse,
        language: Some(file_source),
    }
}

fn debug_syntax_tree(
    _biome_path: &BiomePath,
    parse: AnyParsedSource,
    workspace_db: WorkspaceDb,
) -> GetSyntaxTreeResult {
    let syntax: MarkdownSyntaxNode = parse.syntax(&workspace_db);
    let tree: MdRoot = parse.tree(&workspace_db);
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParsedSource,
    settings: &SettingsWithEditor,
    workspace_db: WorkspaceDb,
) -> Result<String, WorkspaceError> {
    let options = resolve_format_options(biome_path, document_file_source, settings, &workspace_db);

    let tree = parse.syntax(&workspace_db);
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

pub(crate) fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: super::ParsedOrigin,
    settings: &SettingsWithEditor,
    workspace_db: WorkspaceDb,
) -> Result<Printed, WorkspaceError> {
    let options = resolve_format_options(biome_path, document_file_source, settings, &workspace_db);
    debug!("{:?}", &options);
    let tree = parse.syntax(&workspace_db);
    let formatted = format_node(options, &tree)?;
    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => {
            error!("The file {} couldn't be formatted", biome_path.as_str());
            Err(WorkspaceError::FormatError(error.into()))
        }
    }
}

fn lint(params: LintParams) -> LintResults {
    let _ = debug_span!("Linting Markdown file", path =? params.path, language =? params.language)
        .entered();
    let root: MdRoot = params.parsed_source.tree(&params.workspace_db);

    let analyzer_options = resolve_analyzer_options(
        params.path,
        params.working_directory,
        &params.language,
        params.suppression_reason.as_deref(),
        params.settings,
        &params.workspace_db,
    );

    let AnalyzerVisitorResult {
        enabled_rules,
        disabled_rules,
        analyzer_options,
        ..
    } = AnalyzerVisitorBuilder::new(params.settings, &params.workspace_db, analyzer_options)
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

    let (_, analyze_diagnostics) = analyze(&root, filter, &analyzer_options, |signal| {
        process_lint.process_signal(signal)
    });

    let diagnostics = params.parsed_source.serde_diagnostics(&params.workspace_db);

    process_lint.into_result(diagnostics, analyze_diagnostics)
}

fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parsed_source,
        range,
        settings,
        path,
        workspace_db,
        project_layout,
        language,
        skip,
        only,
        enabled_rules: rules,
        suppression_reason,
        plugins: _,
        categories,
        working_directory,
        compute_actions,
    } = params;

    let _ = debug_span!("Code actions JSON",  range =? range, path =? path).entered();
    let tree: MdRoot = parsed_source.tree(&workspace_db);
    let analyzer_options = resolve_analyzer_options(
        path,
        working_directory,
        &language,
        suppression_reason.as_deref(),
        settings,
        &workspace_db,
    );
    let mut actions = Vec::new();
    let AnalyzerVisitorResult {
        enabled_rules,
        disabled_rules,
        analyzer_options,
        ..
    } = AnalyzerVisitorBuilder::new(settings, &workspace_db, analyzer_options)
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

    let action_offset = parsed_source.diagnostic_offset(&workspace_db);
    analyze(&tree, filter, &analyzer_options, |signal| {
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
    });

    PullActionsResult { actions }
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn fix_all(params: FixAllParams) -> Result<Option<FixedFileResult>, WorkspaceError> {
    let mut tree: MdRoot = params.parsed_source.tree(&params.workspace_db);

    let analyzer_options = resolve_analyzer_options(
        params.biome_path,
        params.working_directory,
        &params.document_file_source,
        params.suppression_reason.as_deref(),
        params.settings,
        &params.workspace_db,
    );
    let AnalyzerVisitorResult {
        enabled_rules,
        disabled_rules,
        analyzer_options,
        fixable_rules,
    } = AnalyzerVisitorBuilder::new(params.settings, &params.workspace_db, analyzer_options)
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

    let mut process_fix_all =
        ProcessFixAll::new(&params, tree.syntax().text_range_with_trivia().len().into());

    if matches!(params.fix_file_mode, FixFileMode::ApplySuppressions) {
        loop {
            let mut pending_actions = Vec::new();

            let (_, _) = analyze(&tree, filter, &analyzer_options, |signal| {
                if params.collect_final_diagnostics {
                    process_fix_all.collect_signal(signal, &mut pending_actions)
                } else {
                    process_fix_all.collect_signal_fixes_only(signal, &mut pending_actions)
                }
            });

            let result = process_fix_all.process_batch_actions(pending_actions, |root| {
                tree = match MdRoot::cast(root) {
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

    // Phase 1: fix loop with fixable-only rules
    let fixable_filter = AnalysisFilter {
        categories: params.rule_categories,
        enabled_rules: Some(fixable_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    loop {
        let mut pending_actions = Vec::new();

        let (_, _) = analyze(&tree, fixable_filter, &analyzer_options, |signal| {
            process_fix_all.collect_signal_fixes_only(signal, &mut pending_actions)
        });

        let result = process_fix_all.process_batch_actions(pending_actions, |root| {
            tree = match MdRoot::cast(root) {
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
    if params.collect_final_diagnostics {
        let (_, _) = analyze(&tree, filter, &analyzer_options, |signal| {
            process_fix_all.collect_diagnostic_only(signal)
        });
    }

    Ok(Some(
        process_fix_all.finish(tree.syntax().as_send().unwrap()),
    ))
}
