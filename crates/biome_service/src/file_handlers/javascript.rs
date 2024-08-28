use super::{
    search, AnalyzerCapabilities, AnalyzerVisitorBuilder, CodeActionsParams, DebugCapabilities,
    ExtensionHandler, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, SearchCapabilities,
};
use crate::configuration::to_analyzer_rules;
use crate::diagnostics::extension_error;
use crate::file_handlers::{is_diagnostic_error, FixAllParams};
use crate::settings::{LinterSettings, OverrideSettings, Settings};
use crate::workspace::{DocumentFileSource, OrganizeImportsResult};
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
    RuleCategoriesBuilder, RuleCategory, RuleError, RuleFilter,
};
use biome_configuration::javascript::JsxRuntime;
use biome_diagnostics::{category, Applicability, Diagnostic, DiagnosticExt, Severity};
use biome_formatter::{
    AttributePosition, BracketSpacing, FormatError, IndentStyle, IndentWidth, LineEnding,
    LineWidth, Printed, QuoteStyle,
};
use biome_fs::BiomePath;
use biome_js_analyze::utils::rename::{RenameError, RenameSymbolExtensions};
use biome_js_analyze::{analyze, analyze_with_inspect_matcher, ControlFlowGraph};
use biome_js_formatter::context::trailing_commas::TrailingCommas;
use biome_js_formatter::context::{
    ArrowParentheses, BracketSameLine, JsFormatOptions, QuoteProperties, Semicolons,
};
use biome_js_formatter::format_node;
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{semantic_model, SemanticModelOptions};
use biome_js_syntax::{
    AnyJsRoot, JsFileSource, JsLanguage, JsSyntaxNode, TextRange, TextSize, TokenAtOffset,
};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, BatchMutationExt, Direction, NodeCache};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;
use tracing::{debug, debug_span, error, info, trace, trace_span};

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
    pub enabled: Option<bool>,
    pub attribute_position: Option<AttributePosition>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsParserSettings {
    pub parse_class_parameter_decorators: bool,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsLinterSettings {
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsOrganizeImportsSettings {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsEnvironmentSettings {
    pub jsx_runtime: JsxRuntime,
}

impl From<JsxRuntime> for JsEnvironmentSettings {
    fn from(jsx_runtime: JsxRuntime) -> Self {
        Self { jsx_runtime }
    }
}

impl ServiceLanguage for JsLanguage {
    type FormatterSettings = JsFormatterSettings;
    type LinterSettings = JsLinterSettings;
    type FormatOptions = JsFormatOptions;
    type OrganizeImportsSettings = JsOrganizeImportsSettings;
    type ParserSettings = JsParserSettings;
    type EnvironmentSettings = JsEnvironmentSettings;

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
                .unwrap_or_default(),
        )
        .with_attribute_position(
            language
                .and_then(|l| l.attribute_position)
                .or(global.and_then(|g| g.attribute_position))
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

        let mut jsx_runtime = None;
        let mut globals = Vec::new();

        if let (Some(overrides), Some(global)) = (overrides, global) {
            jsx_runtime = Some(
                match overrides
                    .override_jsx_runtime(path, global.languages.javascript.environment.jsx_runtime)
                {
                    // In the future, we may wish to map an `Auto` variant to a concrete
                    // analyzer value for easy access by the analyzer.
                    JsxRuntime::Transparent => biome_analyze::options::JsxRuntime::Transparent,
                    JsxRuntime::ReactClassic => biome_analyze::options::JsxRuntime::ReactClassic,
                },
            );

            globals.extend(
                overrides
                    .override_js_globals(path, &global.languages.javascript.globals)
                    .into_iter()
                    .collect::<Vec<_>>(),
            );
        }

        if let Some(filename) = path.file_name().map(|filename| filename.as_encoded_bytes()) {
            if filename.ends_with(b".vue") {
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
                    .map(str::to_string),
                );
            } else if filename.ends_with(b".astro") {
                globals.extend(["Astro"].map(str::to_string));
            } else if filename.ends_with(b".svelte")
                || filename.ends_with(b".svelte.js")
                || filename.ends_with(b".svelte.ts")
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
                    .map(str::to_string),
                );
            }
        }

        let configuration = AnalyzerConfiguration {
            rules: global
                .map(|g| to_analyzer_rules(g, path.as_path()))
                .unwrap_or_default(),
            globals,
            preferred_quote,
            jsx_runtime,
        };

        AnalyzerOptions {
            configuration,
            file_path: path.to_path_buf(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct JsFileHandler;

impl ExtensionHandler for JsFileHandler {
    fn capabilities(&self) -> super::Capabilities {
        super::Capabilities {
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
                organize_imports: Some(organize_imports),
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

fn parse(
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    settings: Option<&Settings>,
    cache: &mut NodeCache,
) -> ParseResult {
    let mut options = JsParserOptions {
        grit_metavariables: false,
        parse_class_parameter_decorators: settings
            .map(|settings| {
                settings
                    .languages
                    .javascript
                    .parser
                    .parse_class_parameter_decorators
            })
            .unwrap_or_default(),
    };
    if let Some(settings) = settings {
        options = settings
            .override_settings
            .to_override_js_parser_options(biome_path, options);
    }

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
        JsFileSource::default(),
        None,
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
    debug_span!("Linting JavaScript file", path =? params.path, language =? params.language)
        .in_scope(move || {
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
            let analyzer_options = &params
                .workspace
                .analyzer_options::<JsLanguage>(params.path, &params.language);

            let rules = params
                .workspace
                .settings()
                .as_ref()
                .and_then(|settings| settings.as_linter_rules(params.path.as_path()));

            let (enabled_rules, disabled_rules) =
                AnalyzerVisitorBuilder::new(params.workspace.settings())
                    .with_syntax_rules()
                    .with_linter_rules(&params.only, &params.skip, params.path.as_path())
                    .with_assists_rules(&params.only, &params.skip, params.path.as_path())
                    .finish();

            let filter = AnalysisFilter {
                categories: params.categories,
                enabled_rules: Some(enabled_rules.as_slice()),
                disabled_rules: &disabled_rules,
                range: None,
            };

            let ignores_suppression_comment =
                !filter.categories.contains(RuleCategory::Lint) || !params.only.is_empty();

            let mut diagnostics = params.parse.into_diagnostics();
            let mut diagnostic_count = diagnostics.len() as u32;
            let mut errors = diagnostics
                .iter()
                .filter(|diag| diag.severity() <= Severity::Error)
                .count();

            info!("Analyze file {}", params.path.display());
            let (_, analyze_diagnostics) = analyze(
                &tree,
                filter,
                analyzer_options,
                file_source,
                params.manifest,
                |signal| {
                    if let Some(mut diagnostic) = signal.diagnostic() {
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
                },
            );

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
        })
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parse,
        range,
        workspace,
        path,
        manifest,
        language,
        only,
        skip,
    } = params;
    debug_span!("Code actions JavaScript", range =? range, path =? path).in_scope(move || {
        let tree = parse.tree();
        trace_span!("Parsed file", tree =? tree).in_scope(move || {
            let analyzer_options = workspace.analyzer_options::<JsLanguage>(path, &language);
            let mut actions = Vec::new();
            let (enabled_rules, disabled_rules) =
                AnalyzerVisitorBuilder::new(params.workspace.settings())
                    .with_syntax_rules()
                    .with_linter_rules(&only, &skip, params.path.as_path())
                    .with_assists_rules(&only, &skip, params.path.as_path())
                    .finish();

            let filter = AnalysisFilter {
                categories: RuleCategoriesBuilder::default()
                    .with_syntax()
                    .with_lint()
                    .with_action()
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

            trace!("Javascript runs the analyzer");
            analyze(
                &tree,
                filter,
                &analyzer_options,
                source_type,
                manifest,
                |signal| {
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
                },
            );

            PullActionsResult { actions }
        })
    })
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

    let (enabled_rules, disabled_rules) = AnalyzerVisitorBuilder::new(params.workspace.settings())
        .with_syntax_rules()
        .with_linter_rules(&params.only, &params.skip, params.biome_path.as_path())
        .with_assists_rules(&params.only, &params.skip, params.biome_path.as_path())
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
    let analyzer_options = params
        .workspace
        .analyzer_options::<JsLanguage>(params.biome_path, &params.document_file_source);
    loop {
        let (action, _) = analyze(
            &tree,
            filter,
            &analyzer_options,
            file_source,
            params.manifest.clone(),
            |signal| {
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

#[tracing::instrument(level = "trace", skip(parse, settings))]
pub(crate) fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);

    debug!("Options used for format: \n{}", options);

    let tree = parse.syntax();
    info!("Format file {}", biome_path.display());
    let formatted = format_node(options, &tree)?;
    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => {
            error!("The file {} couldn't be formatted", biome_path.display());
            Err(WorkspaceError::FormatError(error.into()))
        }
    }
}

#[tracing::instrument(level = "trace", skip(parse, settings))]
pub(crate) fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let printed = biome_js_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

#[tracing::instrument(level = "trace", skip(parse, settings))]
pub(crate) fn format_on_type(
    path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(path, document_file_source);

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
        let range = node.text_range();
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

pub(crate) fn organize_imports(parse: AnyParse) -> Result<OrganizeImportsResult, WorkspaceError> {
    let mut tree: AnyJsRoot = parse.tree();

    let filter = AnalysisFilter {
        enabled_rules: Some(&[RuleFilter::Rule("source", "organizeImports")]),
        categories: RuleCategoriesBuilder::default().with_action().build(),
        ..AnalysisFilter::default()
    };

    let (action, _) = analyze(
        &tree,
        filter,
        &AnalyzerOptions::default(),
        JsFileSource::default(),
        None,
        |signal| {
            for action in signal.actions() {
                if action.is_suppression() {
                    continue;
                }

                return ControlFlow::Break(action);
            }
            ControlFlow::Continue(())
        },
    );

    if let Some(action) = action {
        tree = match AnyJsRoot::cast(action.mutation.commit()) {
            Some(tree) => tree,
            None => {
                return Err(WorkspaceError::RuleError(
                    RuleError::ReplacedRootWithNonRootError {
                        rule_name: action
                            .rule_name
                            .map(|(group, rule)| (Cow::Borrowed(group), Cow::Borrowed(rule))),
                    },
                ));
            }
        };

        Ok(OrganizeImportsResult {
            code: tree.syntax().to_string(),
        })
    } else {
        Ok(OrganizeImportsResult {
            code: tree.syntax().to_string(),
        })
    }
}
