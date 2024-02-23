use super::{
    AnalyzerCapabilities, CodeActionsParams, DebugCapabilities, ExtensionHandler,
    FormatterCapabilities, LintParams, LintResults, Mime, ParseResult, ParserCapabilities,
};
use crate::configuration::to_analyzer_rules;
use crate::diagnostics::extension_error;
use crate::file_handlers::{is_diagnostic_error, FixAllParams, Language as LanguageId};
use crate::settings::OverrideSettings;
use crate::workspace::OrganizeImportsResult;
use crate::{
    settings::{FormatSettings, Language, LanguageListSettings, LanguageSettings, SettingsHandle},
    workspace::{
        CodeAction, FixAction, FixFileMode, FixFileResult, GetSyntaxTreeResult, PullActionsResult,
        RenameResult,
    },
    WorkspaceError,
};
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, GroupCategory, Never,
    QueryMatch, RegistryVisitor, RuleCategories, RuleCategory, RuleFilter, RuleGroup,
};
use biome_diagnostics::{category, Applicability, Diagnostic, DiagnosticExt, Severity};
use biome_formatter::{
    AttributePosition, FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed,
    QuoteStyle,
};
use biome_fs::BiomePath;
use biome_js_analyze::utils::rename::{RenameError, RenameSymbolExtensions};
use biome_js_analyze::{
    analyze, analyze_with_inspect_matcher, visit_registry, ControlFlowGraph, RuleError,
};
use biome_js_formatter::context::trailing_comma::TrailingComma;
use biome_js_formatter::context::{
    ArrowParentheses, BracketSameLine, BracketSpacing, JsFormatOptions, QuoteProperties, Semicolons,
};
use biome_js_formatter::format_node;
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{semantic_model, SemanticModelOptions};
use biome_js_syntax::{
    AnyJsRoot, JsFileSource, JsLanguage, JsSyntaxNode, TextRange, TextSize, TokenAtOffset,
};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, BatchMutationExt, Direction, NodeCache};
use std::borrow::Cow;
use std::fmt::Debug;
use std::path::PathBuf;
use tracing::{debug, debug_span, error, info, trace, trace_span};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsFormatterSettings {
    pub quote_style: Option<QuoteStyle>,
    pub jsx_quote_style: Option<QuoteStyle>,
    pub quote_properties: Option<QuoteProperties>,
    pub trailing_comma: Option<TrailingComma>,
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
    pub globals: Vec<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JsOrganizeImportsSettings {}

impl Language for JsLanguage {
    type FormatterSettings = JsFormatterSettings;
    type LinterSettings = JsLinterSettings;
    type FormatOptions = JsFormatOptions;
    type OrganizeImportsSettings = JsOrganizeImportsSettings;
    type ParserSettings = JsParserSettings;

    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self> {
        &languages.javascript
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &JsFormatterSettings,
        path: &BiomePath,
    ) -> JsFormatOptions {
        let options = JsFormatOptions::new(path.as_path().try_into().unwrap_or_default())
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
            .with_trailing_comma(language.trailing_comma.unwrap_or_default())
            .with_semicolons(language.semicolons.unwrap_or_default())
            .with_arrow_parentheses(language.arrow_parentheses.unwrap_or_default())
            .with_bracket_spacing(language.bracket_spacing.unwrap_or_default())
            .with_bracket_same_line(language.bracket_same_line.unwrap_or_default())
            .with_attribute_position(
                language
                    .attribute_position
                    .or(global.attribute_position)
                    .unwrap_or_default(),
            );

        overrides.override_js_format_options(path, options)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct JsFileHandler;

impl ExtensionHandler for JsFileHandler {
    fn language(&self) -> super::Language {
        super::Language::JavaScript
    }

    fn mime(&self) -> Mime {
        Mime::Javascript
    }

    fn may_use_tabs(&self) -> bool {
        true
    }

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
        }
    }
}

fn parse(
    biome_path: &BiomePath,
    language_hint: LanguageId,
    text: &str,
    settings: SettingsHandle,
    cache: &mut NodeCache,
) -> ParseResult {
    let source_type =
        JsFileSource::try_from(biome_path.as_path()).unwrap_or_else(|_| match language_hint {
            LanguageId::JavaScriptReact => JsFileSource::jsx(),
            LanguageId::TypeScript => JsFileSource::ts(),
            LanguageId::TypeScriptReact => JsFileSource::tsx(),
            _ => JsFileSource::js_module(),
        });
    let parser_settings = &settings.as_ref().languages.javascript.parser;
    let overrides = &settings.as_ref().override_settings;
    let options = overrides.override_js_parser_options(
        biome_path,
        JsParserOptions {
            parse_class_parameter_decorators: parser_settings.parse_class_parameter_decorators,
        },
    );
    let parse = biome_js_parser::parse_js_with_cache(text, source_type, options, cache);
    let root = parse.syntax();
    let diagnostics = parse.into_diagnostics();
    ParseResult {
        any_parse: AnyParse::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        ),
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
        categories: RuleCategories::LINT,
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
    biome_path: &BiomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

pub(crate) fn lint(params: LintParams) -> LintResults {
    debug_span!("Linting JavaScript file", path =? params.path, language =? params.language)
        .in_scope(move || {
            let settings = params.settings.as_ref();
            let Some(file_source) = params
                .language
                .as_js_file_source()
                .or(JsFileSource::try_from(params.path.as_path()).ok())
            else {
                return LintResults {
                    errors: 0,
                    diagnostics: vec![],
                    skipped_diagnostics: 0,
                };
            };
            let tree = params.parse.tree();
            let mut diagnostics = params.parse.into_diagnostics();
            let analyzer_options =
                compute_analyzer_options(&params.settings, PathBuf::from(params.path.as_path()));

            // Compute final rules (taking `overrides` into account)
            let rules = settings.as_rules(params.path.as_path());
            let mut rule_filter_list = rules
                .as_ref()
                .map(|rules| rules.as_enabled_rules())
                .unwrap_or_default()
                .into_iter()
                .collect::<Vec<_>>();
            if settings.organize_imports.enabled && !params.categories.is_syntax() {
                rule_filter_list.push(RuleFilter::Rule("correctness", "organizeImports"));
            }

            rule_filter_list.push(RuleFilter::Rule(
                "correctness",
                "noDuplicatePrivateClassMembers",
            ));
            rule_filter_list.push(RuleFilter::Rule("correctness", "noInitializerWithDefinite"));
            rule_filter_list.push(RuleFilter::Rule("correctness", "noSuperWithoutExtends"));
            rule_filter_list.push(RuleFilter::Rule("nursery", "noSuperWithoutExtends"));

            let mut filter = AnalysisFilter::from_enabled_rules(Some(rule_filter_list.as_slice()));
            filter.categories = params.categories;

            let mut diagnostic_count = diagnostics.len() as u32;
            let mut errors = diagnostics
                .iter()
                .filter(|diag| diag.severity() <= Severity::Error)
                .count();

            let has_lint = filter.categories.contains(RuleCategories::LINT);

            info!("Analyze file {}", params.path.display());
            let (_, analyze_diagnostics) = analyze(
                &tree,
                filter,
                &analyzer_options,
                file_source,
                params.manifest,
                |signal| {
                    if let Some(mut diagnostic) = signal.diagnostic() {
                        // Do not report unused suppression comment diagnostics if this is a syntax-only analyzer pass
                        if !has_lint
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
                            .map(|category| {
                                rules
                                    .as_ref()
                                    .and_then(|rules| rules.get_severity_from_code(category))
                                    .unwrap_or(Severity::Warning)
                            })
                            .unwrap_or_else(|| diagnostic.severity());

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

struct ActionsVisitor<'a> {
    enabled_rules: Vec<RuleFilter<'a>>,
}

impl RegistryVisitor<JsLanguage> for ActionsVisitor<'_> {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_group<G: RuleGroup<Language = JsLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule + 'static,
        R::Query: biome_analyze::Queryable<Language = JsLanguage>,
        <R::Query as biome_analyze::Queryable>::Output: Clone,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ));
    }
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parse,
        range,
        rules,
        settings,
        path,
        manifest,
        language,
    } = params;
    debug_span!("Code actions JavaScript", range =? range, path =? path).in_scope(move || {
        let tree = parse.tree();
        trace_span!("Parsed file", tree =? tree).in_scope(move || {
            let mut actions = Vec::new();
            let mut enabled_rules = vec![];
            if settings.as_ref().organize_imports.enabled {
                enabled_rules.push(RuleFilter::Rule("correctness", "organizeImports"));
            }
            if let Some(rules) = rules {
                let rules = rules.as_enabled_rules().into_iter().collect();

                // The rules in the assist category do not have configuration entries,
                // always add them all to the enabled rules list
                let mut visitor = ActionsVisitor {
                    enabled_rules: rules,
                };
                visit_registry(&mut visitor);

                enabled_rules.extend(visitor.enabled_rules);
            }

            let mut filter = if !enabled_rules.is_empty() {
                AnalysisFilter::from_enabled_rules(Some(enabled_rules.as_slice()))
            } else {
                AnalysisFilter::default()
            };

            filter.categories = RuleCategories::SYNTAX | RuleCategories::LINT;
            if settings.as_ref().organize_imports.enabled {
                filter.categories |= RuleCategories::ACTION;
            }
            filter.range = Some(range);

            let analyzer_options =
                compute_analyzer_options(&settings, PathBuf::from(path.as_path()));

            let Some(source_type) = language.as_js_file_source() else {
                error!("Could not determine the file source of the file");
                return PullActionsResult { actions: vec![] };
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
///
/// If `indent_style` is [Some], it means that the formatting should be applied at the end
pub(crate) fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let FixAllParams {
        parse,
        rules,
        fix_file_mode,
        settings,
        should_format,
        biome_path,
        mut filter,
        manifest,
        language,
    } = params;

    let Some(file_source) = language
        .as_js_file_source()
        .or(JsFileSource::try_from(biome_path.as_path()).ok())
    else {
        return Err(extension_error(biome_path));
    };
    let mut tree: AnyJsRoot = parse.tree();
    let mut actions = Vec::new();

    filter.categories = RuleCategories::SYNTAX | RuleCategories::LINT;

    let mut skipped_suggested_fixes = 0;
    let mut errors: u16 = 0;
    let analyzer_options = compute_analyzer_options(&settings, PathBuf::from(biome_path.as_path()));
    loop {
        let (action, _) = analyze(
            &tree,
            filter,
            &analyzer_options,
            file_source,
            manifest.clone(),
            |signal| {
                let current_diagnostic = signal.diagnostic();

                if let Some(diagnostic) = current_diagnostic.as_ref() {
                    if is_diagnostic_error(diagnostic, rules) {
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
            },
        );

        match action {
            Some(action) => {
                if let Some((range, _)) = action.mutation.as_text_edits() {
                    tree = match AnyJsRoot::cast(action.mutation.commit()) {
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
                let code = if should_format {
                    format_node(
                        settings.format_options::<JsLanguage>(biome_path),
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
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path);

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
    parse: AnyParse,
    settings: SettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path);

    let tree = parse.syntax();
    let printed = biome_js_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

#[tracing::instrument(level = "trace", skip(parse, settings))]
pub(crate) fn format_on_type(
    biome_path: &BiomePath,
    parse: AnyParse,
    settings: SettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(biome_path);

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
                let result = batch.rename_any_renamable_node(&model, node, &new_name);
                if !result {
                    Err(WorkspaceError::RenameError(RenameError::CannotBeRenamed {
                        original_name: original_name.to_string(),
                        original_range: range,
                        new_name,
                    }))
                } else {
                    let (range, indels) = batch.as_text_edits().unwrap_or_default();
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
        enabled_rules: Some(&[RuleFilter::Rule("correctness", "organizeImports")]),
        categories: RuleCategories::ACTION,
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

fn compute_analyzer_options(settings: &SettingsHandle, file_path: PathBuf) -> AnalyzerOptions {
    let settings = settings.as_ref();
    let configuration = AnalyzerConfiguration {
        rules: to_analyzer_rules(settings, file_path.as_path()),
        globals: settings
            .override_settings
            .override_js_globals(
                &BiomePath::new(file_path.as_path()),
                &settings.languages.javascript.globals,
            )
            .into_iter()
            .collect(),
    };

    AnalyzerOptions {
        configuration,
        file_path,
    }
}
