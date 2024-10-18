use super::{
    is_diagnostic_error, AnalyzerVisitorBuilder, CodeActionsParams, ExtensionHandler, FixAllParams,
    LintParams, LintResults, ParseResult, SearchCapabilities,
};
use crate::configuration::to_analyzer_rules;
use crate::file_handlers::DebugCapabilities;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FormatterCapabilities, ParserCapabilities,
};
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, LinterSettings, OverrideSettings,
    ServiceLanguage, Settings, WorkspaceSettingsHandle,
};
use crate::workspace::{
    CodeAction, DocumentFileSource, FixAction, FixFileMode, FixFileResult, GetSyntaxTreeResult,
    OrganizeImportsResult, PullActionsResult,
};
use crate::WorkspaceError;
use biome_analyze::options::PreferredQuote;
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never,
    RuleCategoriesBuilder, RuleCategory, RuleError,
};
use biome_css_analyze::analyze;
use biome_css_formatter::context::CssFormatOptions;
use biome_css_formatter::format_node;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::{CssLanguage, CssRoot, CssSyntaxNode};
use biome_diagnostics::{category, Applicability, Diagnostic, DiagnosticExt, Severity};
use biome_formatter::{
    FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed, QuoteStyle,
};
use biome_fs::BiomePath;
use biome_parser::AnyParse;
use biome_rowan::{AstNode, NodeCache};
use biome_rowan::{TextRange, TextSize, TokenAtOffset};
use std::borrow::Cow;
use tracing::{debug_span, error, info, trace_span};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub quote_style: Option<QuoteStyle>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssLinterSettings {
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssParserSettings {
    pub allow_wrong_line_comments: Option<bool>,
    pub css_modules: Option<bool>,
}

impl ServiceLanguage for CssLanguage {
    type FormatterSettings = CssFormatterSettings;
    type LinterSettings = CssLinterSettings;
    type OrganizeImportsSettings = ();
    type FormatOptions = CssFormatOptions;
    type ParserSettings = CssParserSettings;
    type EnvironmentSettings = ();

    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.css
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

        let options = CssFormatOptions::new(
            document_file_source
                .to_css_file_source()
                .unwrap_or_default(),
        )
        .with_indent_style(indent_style)
        .with_indent_width(indent_width)
        .with_line_width(line_width)
        .with_line_ending(line_ending)
        .with_quote_style(language.and_then(|l| l.quote_style).unwrap_or_default());
        if let Some(overrides) = overrides {
            overrides.to_override_css_format_options(path, options)
        } else {
            options
        }
    }

    fn resolve_analyzer_options(
        global: Option<&Settings>,
        _linter: Option<&LinterSettings>,
        _overrides: Option<&OverrideSettings>,
        _language: Option<&Self::LinterSettings>,
        file_path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> AnalyzerOptions {
        let preferred_quote = global
            .and_then(|global| {
                global
                    .languages
                    .css
                    .formatter
                    .quote_style
                    .map(|quote_style: QuoteStyle| {
                        if quote_style == QuoteStyle::Single {
                            PreferredQuote::Single
                        } else {
                            PreferredQuote::Double
                        }
                    })
            })
            .unwrap_or_default();

        let configuration = AnalyzerConfiguration {
            rules: global
                .map(|g| to_analyzer_rules(g, file_path.as_path()))
                .unwrap_or_default(),
            globals: Vec::new(),
            preferred_quote,
            jsx_runtime: None,
        };

        AnalyzerOptions {
            configuration,
            file_path: file_path.to_path_buf(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct CssFileHandler;

impl ExtensionHandler for CssFileHandler {
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
                organize_imports: Some(organize_imports),
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
    biome_path: &BiomePath,
    _file_source: DocumentFileSource,
    text: &str,
    settings: Option<&Settings>,
    cache: &mut NodeCache,
) -> ParseResult {
    let mut options = CssParserOptions {
        allow_wrong_line_comments: settings
            .and_then(|s| s.languages.css.parser.allow_wrong_line_comments)
            .unwrap_or_default(),
        css_modules: settings
            .and_then(|s| s.languages.css.parser.css_modules)
            .unwrap_or_default(),
        grit_metavariables: false,
    };
    if let Some(settings) = settings {
        options = settings
            .override_settings
            .to_override_css_parser_options(biome_path, options);
    }
    let parse = biome_css_parser::parse_css_with_cache(text, cache, options);
    ParseResult {
        any_parse: parse.into(),
        language: None,
    }
}

fn debug_syntax_tree(_rome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: CssSyntaxNode = parse.syntax();
    let tree: CssRoot = parse.tree();
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
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

#[tracing::instrument(level = "debug", skip(parse))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

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
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let printed = biome_css_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: WorkspaceSettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

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

    let printed = biome_css_formatter::format_sub_tree(options, &root_node)?;
    Ok(printed)
}

fn lint(params: LintParams) -> LintResults {
    debug_span!("Linting CSS file", path =? params.path, language =? params.language).in_scope(
        move || {
            let workspace_settings = &params.workspace;
            let analyzer_options =
                workspace_settings.analyzer_options::<CssLanguage>(params.path, &params.language);
            let tree = params.parse.tree();

            let has_only_filter = !params.only.is_empty();
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
            let mut diagnostics = params.parse.into_diagnostics();

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

fn organize_imports(parse: AnyParse) -> Result<OrganizeImportsResult, WorkspaceError> {
    Ok(OrganizeImportsResult {
        code: parse.syntax::<CssLanguage>().to_string(),
    })
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
        only,
        skip,
    } = params;
    debug_span!("Code actions CSS", range =? range, path =? path).in_scope(move || {
        let tree = parse.tree();
        trace_span!("Parsed file", tree =? tree).in_scope(move || {
            let Some(_) = language.to_css_file_source() else {
                error!("Could not determine the file source of the file");
                return PullActionsResult {
                    actions: Vec::new(),
                };
            };

            let analyzer_options = workspace.analyzer_options::<CssLanguage>(path, &language);
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

            info!("CSS runs the analyzer");

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
    let mut tree: CssRoot = params.parse.tree();
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
    let analyzer_options = params
        .workspace
        .analyzer_options::<CssLanguage>(params.biome_path, &params.document_file_source);
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
                        // TODO: to implement
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
                    tree = match CssRoot::cast(root) {
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
                        params.workspace.format_options::<CssLanguage>(
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

#[cfg(test)]
mod test {
    use super::*;
    use biome_css_syntax::CssFileSource;

    #[test]
    fn inherit_global_format_settings() {
        let format_options = CssLanguage::resolve_format_options(
            Some(&FormatSettings::default()),
            None,
            None,
            &BiomePath::new(""),
            &DocumentFileSource::Css(CssFileSource::css()),
        );
        assert_eq!(
            format_options,
            CssFormatOptions::default()
                .with_indent_style(IndentStyle::default())
                .with_indent_width(IndentWidth::default())
                .with_line_ending(LineEnding::default())
                .with_line_width(LineWidth::default())
        );
    }
}
