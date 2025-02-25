use super::{
    is_diagnostic_error, AnalyzerVisitorBuilder, CodeActionsParams, DocumentFileSource,
    EnabledForPath, ExtensionHandler, ParseResult, ProcessLint, SearchCapabilities,
};
use crate::configuration::to_analyzer_rules;
use crate::file_handlers::DebugCapabilities;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FixAllParams, FormatterCapabilities, LintParams,
    LintResults, ParserCapabilities,
};
use crate::settings::{
    check_feature_activity, check_override_feature_activity, FormatSettings, LanguageListSettings,
    LanguageSettings, LinterSettings, OverrideSettings, ServiceLanguage, Settings,
    WorkspaceSettingsHandle,
};
use crate::workspace::{
    CodeAction, FixAction, FixFileMode, FixFileResult, GetSyntaxTreeResult, PullActionsResult,
};
use crate::{extension_error, WorkspaceError};
use biome_analyze::options::PreferredQuote;
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never,
    RuleCategoriesBuilder, RuleError,
};
use biome_configuration::json::{
    JsonAllowCommentsEnabled, JsonAllowTrailingCommasEnabled, JsonAssistConfiguration,
    JsonAssistEnabled, JsonFormatterConfiguration, JsonFormatterEnabled, JsonLinterConfiguration,
    JsonLinterEnabled, JsonParserConfiguration,
};
use biome_configuration::Configuration;
use biome_deserialize::json::deserialize_from_json_ast;
use biome_diagnostics::Applicability;
use biome_formatter::{
    BracketSpacing, FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, ObjectWrap,
    Printed,
};
use biome_fs::{BiomePath, ConfigName};
use biome_json_analyze::analyze;
use biome_json_formatter::context::{Expand, JsonFormatOptions, TrailingCommas};
use biome_json_formatter::format_node;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::{JsonFileSource, JsonLanguage, JsonRoot, JsonSyntaxNode};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, NodeCache};
use biome_rowan::{TextRange, TextSize, TokenAtOffset};
use camino::Utf8Path;
use std::borrow::Cow;
use tracing::{debug_span, error, instrument};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsonFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub trailing_commas: Option<TrailingCommas>,
    pub expand: Option<Expand>,
    pub bracket_spacing: Option<BracketSpacing>,
    pub object_wrap: Option<ObjectWrap>,
    pub enabled: Option<JsonFormatterEnabled>,
}

impl From<JsonFormatterConfiguration> for JsonFormatterSettings {
    fn from(configuration: JsonFormatterConfiguration) -> Self {
        Self {
            line_ending: configuration.line_ending,
            line_width: configuration.line_width,
            indent_width: configuration.indent_width,
            indent_style: configuration.indent_style,
            trailing_commas: configuration.trailing_commas,
            expand: configuration.expand,
            bracket_spacing: configuration.bracket_spacing,
            object_wrap: configuration.object_wrap,
            enabled: configuration.enabled,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsonParserSettings {
    pub allow_comments: Option<JsonAllowCommentsEnabled>,
    pub allow_trailing_commas: Option<JsonAllowTrailingCommasEnabled>,
}

impl From<JsonParserConfiguration> for JsonParserSettings {
    fn from(configuration: JsonParserConfiguration) -> Self {
        Self {
            allow_comments: configuration.allow_comments,
            allow_trailing_commas: configuration.allow_trailing_commas,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsonLinterSettings {
    pub enabled: Option<JsonLinterEnabled>,
}

impl From<JsonLinterConfiguration> for JsonLinterSettings {
    fn from(configuration: JsonLinterConfiguration) -> Self {
        Self {
            enabled: configuration.enabled,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsonAssistSettings {
    pub enabled: Option<JsonAssistEnabled>,
}

impl From<JsonAssistConfiguration> for JsonAssistSettings {
    fn from(configuration: JsonAssistConfiguration) -> Self {
        Self {
            enabled: configuration.enabled,
        }
    }
}

impl ServiceLanguage for JsonLanguage {
    type FormatterSettings = JsonFormatterSettings;
    type LinterSettings = JsonLinterSettings;
    type FormatOptions = JsonFormatOptions;
    type ParserSettings = JsonParserSettings;
    type AssistSettings = JsonAssistSettings;
    type EnvironmentSettings = ();

    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.json
    }

    fn resolve_format_options(
        global: Option<&FormatSettings>,
        overrides: Option<&OverrideSettings>,
        language: Option<&JsonFormatterSettings>,
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

        // ensure it never formats biome.json into a form it can't parse
        let trailing_commas = if matches!(path.file_name(), Some("biome.json")) {
            TrailingCommas::None
        } else {
            language.and_then(|l| l.trailing_commas).unwrap_or_default()
        };

        let expand_lists = language.and_then(|l| l.expand).unwrap_or_else(|| {
            if path.file_name() == Some("package.json") {
                Expand::Always
            } else {
                Expand::default()
            }
        });

        let bracket_spacing = language
            .and_then(|l| l.bracket_spacing)
            .or(global.and_then(|g| g.bracket_spacing))
            .unwrap_or_default();

        let object_wrap = language
            .and_then(|l| l.object_wrap)
            .or(global.and_then(|g| g.object_wrap))
            .unwrap_or_default();

        let file_source = document_file_source
            .to_json_file_source()
            .unwrap_or_default();

        let options = JsonFormatOptions::new(file_source)
            .with_line_ending(line_ending)
            .with_indent_style(indent_style)
            .with_indent_width(indent_width)
            .with_line_width(line_width)
            .with_trailing_commas(trailing_commas)
            .with_expand(expand_lists)
            .with_bracket_spacing(bracket_spacing)
            .with_object_wrap(object_wrap);

        if let Some(overrides) = overrides {
            overrides.to_override_json_format_options(path, options)
        } else {
            options
        }
    }

    fn resolve_analyzer_options(
        global: Option<&Settings>,
        _linter: Option<&LinterSettings>,
        _overrides: Option<&OverrideSettings>,
        _language: Option<&Self::LinterSettings>,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        let configuration = AnalyzerConfiguration::default()
            .with_rules(
                global
                    .map(|g| to_analyzer_rules(g, path.as_path()))
                    .unwrap_or_default(),
            )
            .with_preferred_quote(PreferredQuote::Double);
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
                                pattern.languages.json.formatter.enabled,
                                pattern.formatter.enabled,
                            )
                            .filter(|_| {
                                // Then check whether the path satisfies
                                pattern.is_file_included(path)
                            })
                        });

                overrides_activity.or(check_feature_activity(
                    settings.languages.json.formatter.enabled,
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
                                pattern.languages.json.assist.enabled,
                                pattern.assist.enabled,
                            )
                            .filter(|_| {
                                // Then check whether the path satisfies
                                pattern.is_file_included(path)
                            })
                        });

                overrides_activity.or(check_feature_activity(
                    settings.languages.json.assist.enabled,
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
                                pattern.languages.json.linter.enabled,
                                pattern.linter.enabled,
                            )
                            .filter(|_| {
                                // Then check whether the path satisfies
                                pattern.is_file_included(path)
                            })
                        });

                overrides_activity.or(check_feature_activity(
                    settings.languages.json.linter.enabled,
                    settings.linter.enabled,
                ))
            })
            .unwrap_or_default()
            .into()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct JsonFileHandler;

impl ExtensionHandler for JsonFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            enabled_for_path: EnabledForPath {
                formatter: Some(formatter_enabled),
                search: Some(search_enabled),
                assist: Some(assist_enabled),
                linter: Some(linter_enabled),
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
    handle.formatter_enabled_for_file_path::<JsonLanguage>(path)
}

fn linter_enabled(path: &Utf8Path, handle: &WorkspaceSettingsHandle) -> bool {
    handle.linter_enabled_for_file_path::<JsonLanguage>(path)
}

fn assist_enabled(path: &Utf8Path, handle: &WorkspaceSettingsHandle) -> bool {
    handle.assist_enabled_for_file_path::<JsonLanguage>(path)
}

fn search_enabled(_path: &Utf8Path, _handle: &WorkspaceSettingsHandle) -> bool {
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
    let options = if biome_path.ends_with(ConfigName::biome_jsonc()) {
        JsonParserOptions::default()
            .with_allow_comments()
            .with_allow_trailing_commas()
    } else {
        let parser = settings.map(|s| &s.languages.json.parser);
        let overrides = settings.map(|s| &s.override_settings);
        let optional_json_file_source = file_source.to_json_file_source();
        let options = JsonParserOptions {
            allow_comments: parser.and_then(|p| p.allow_comments).map_or_else(
                || optional_json_file_source.is_some_and(|x| x.allow_comments()),
                |value| value.value(),
            ),
            allow_trailing_commas: parser.and_then(|p| p.allow_trailing_commas).map_or_else(
                || optional_json_file_source.is_some_and(|x| x.allow_trailing_commas()),
                |value| value.value(),
            ),
        };
        if let Some(overrides) = overrides {
            overrides.to_override_json_parser_options(biome_path, options)
        } else {
            options
        }
    };

    let parse = biome_json_parser::parse_json_with_cache(text, cache, options);

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

fn debug_syntax_tree(_rome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: JsonSyntaxNode = parse.syntax();
    let tree: JsonRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

#[tracing::instrument(level = "debug", skip(parse, settings))]
fn format(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(path, document_file_source);

    tracing::debug!("Format with the following options: {:?}", options);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

fn format_range(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(path, document_file_source);

    let tree = parse.syntax();
    let printed = biome_json_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(path, document_file_source);

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

    let printed = biome_json_formatter::format_sub_tree(options, &root_node)?;
    Ok(printed)
}

fn lint(params: LintParams) -> LintResults {
    let _ = debug_span!("Linting JSON file", path =? params.path, language =? params.language)
        .entered();
    let Some(file_source) = params
        .language
        .to_json_file_source()
        .or(JsonFileSource::try_from(params.path.as_path()).ok())
    else {
        return LintResults {
            errors: 0,
            diagnostics: vec![],
            skipped_diagnostics: 0,
        };
    };
    let root: JsonRoot = params.parse.tree();

    let analyzer_options = params.workspace.analyzer_options::<JsonLanguage>(
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

    let (_, analyze_diagnostics) =
        analyze(&root, filter, &analyzer_options, file_source, |signal| {
            process_lint.process_signal(signal)
        });

    let mut diagnostics = params.parse.into_diagnostics();
    // if we're parsing the `biome.json` file, we deserialize it, so we can emit diagnostics for
    // malformed configuration
    if params.path.ends_with(ConfigName::biome_json())
        || params.path.ends_with(ConfigName::biome_jsonc())
    {
        let deserialized = deserialize_from_json_ast::<Configuration>(&root, "");
        diagnostics.extend(
            deserialized
                .into_diagnostics()
                .into_iter()
                .map(biome_diagnostics::serde::Diagnostic::new)
                .collect::<Vec<_>>(),
        );
    }

    process_lint.into_result(diagnostics, analyze_diagnostics)
}

fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parse,
        range,
        workspace,
        path,
        dependency_graph: _,
        project_layout,
        language,
        skip,
        only,
        enabled_rules: rules,
        suppression_reason,
        plugins: _,
    } = params;

    let _ = debug_span!("Code actions JSON",  range =? range, path =? path).entered();
    let tree: JsonRoot = parse.tree();
    let analyzer_options = workspace.analyzer_options::<JsonLanguage>(
        params.path,
        &params.language,
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

    let Some(file_source) = language.to_json_file_source() else {
        error!("Could not determine the file source of the file");
        return PullActionsResult { actions: vec![] };
    };

    analyze(&tree, filter, &analyzer_options, file_source, |signal| {
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

#[instrument(level = "debug", skip(params))]
fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let mut tree: JsonRoot = params.parse.tree();
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
    let analyzer_options = params.workspace.analyzer_options::<JsonLanguage>(
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
            .with_assist()
            .build(),
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    let Some(file_source) = params
        .document_file_source
        .to_json_file_source()
        .or(JsonFileSource::try_from(params.biome_path.as_path()).ok())
    else {
        return Err(extension_error(params.biome_path));
    };

    let mut actions = Vec::new();
    let mut skipped_suggested_fixes = 0;
    let mut errors: u16 = 0;

    loop {
        let (action, _) = analyze(&tree, filter, &analyzer_options, file_source, |signal| {
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
                        // TODO: implement once a JSON suppression action is available
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
                    tree = match JsonRoot::cast(root) {
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
                        params.workspace.format_options::<JsonLanguage>(
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
