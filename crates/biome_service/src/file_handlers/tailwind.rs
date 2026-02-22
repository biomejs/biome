use super::{
    AnalyzerCapabilities, AnalyzerVisitorBuilder, Capabilities, CodeActionsParams,
    DocumentFileSource, EnabledForPath, ExtensionHandler, FixAllParams, LintParams, LintResults,
    ParserCapabilities, ProcessFixAll, ProcessLint,
};
use crate::WorkspaceError;
use crate::configuration::to_analyzer_rules;
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, OverrideSettings, ServiceLanguage,
    Settings, SettingsWithEditor, check_feature_activity, check_override_feature_activity,
};
use biome_analyze::{AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never};
use biome_configuration::bool::Bool;
use biome_formatter::printer::PrinterOptions;
use biome_formatter::{
    IndentStyle, IndentWidth, LineEnding, LineWidth, SimpleFormatContext, TrailingNewline,
};
use biome_fs::BiomePath;
use biome_rowan::AstNode;
use biome_tailwind_analyze::analyze;
use biome_tailwind_syntax::{TailwindLanguage, TwRoot};
use camino::Utf8Path;
use either::Either;
use std::borrow::Cow;
use std::fmt::Display;
use tracing::debug_span;

/// A stub format options type for Tailwind — Tailwind has no formatter, but
/// `ServiceLanguage::FormatOptions` requires `biome_formatter::FormatOptions`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TailwindFormatOptions;

impl biome_formatter::FormatOptions for TailwindFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        IndentStyle::default()
    }

    fn indent_width(&self) -> IndentWidth {
        IndentWidth::default()
    }

    fn line_width(&self) -> LineWidth {
        LineWidth::default()
    }

    fn line_ending(&self) -> LineEnding {
        LineEnding::default()
    }

    fn trailing_newline(&self) -> TrailingNewline {
        TrailingNewline::default()
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::default()
    }
}

impl Display for TailwindFormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TailwindFormatOptions")
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TailwindLinterSettings {
    pub enabled: Option<Bool<true>>,
}

impl ServiceLanguage for TailwindLanguage {
    type FormatterSettings = ();
    type LinterSettings = TailwindLinterSettings;
    type AssistSettings = ();
    type FormatOptions = TailwindFormatOptions;
    type ParserSettings = ();
    type ParserOptions = ();
    type EnvironmentSettings = ();

    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self> {
        &languages.tailwind
    }

    fn resolve_environment(_settings: &Settings) -> Option<&Self::EnvironmentSettings> {
        None
    }

    fn resolve_parse_options(
        _overrides: &OverrideSettings,
        _language: &Self::ParserSettings,
        _path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> Self::ParserOptions {
    }

    fn resolve_format_options(
        _global: &FormatSettings,
        _overrides: &OverrideSettings,
        _language: &Self::FormatterSettings,
        _path: &BiomePath,
        _document_file_source: &DocumentFileSource,
    ) -> Self::FormatOptions {
        TailwindFormatOptions
    }

    fn resolve_analyzer_options(
        global: &Settings,
        _language: &Self::LinterSettings,
        _environment: Option<&Self::EnvironmentSettings>,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        let configuration =
            AnalyzerConfiguration::default().with_rules(to_analyzer_rules(global, path.as_path()));
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
                        pattern.languages.tailwind.linter.enabled,
                        pattern.linter.enabled,
                    )
                    .filter(|_| pattern.is_file_included(path))
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.tailwind.linter.enabled,
                settings.linter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    fn formatter_enabled_for_file_path(_settings: &Settings, _path: &Utf8Path) -> bool {
        // Tailwind has no formatter
        false
    }

    fn assist_enabled_for_file_path(_settings: &Settings, _path: &Utf8Path) -> bool {
        // Tailwind has no assist actions yet
        false
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct TailwindFileHandler;

impl ExtensionHandler for TailwindFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            enabled_for_path: EnabledForPath {
                linter: Some(linter_enabled),
                ..Default::default()
            },
            parser: ParserCapabilities {
                parse: None,
                parse_embedded_nodes: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                fix_all: Some(fix_all),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

fn linter_enabled(path: &Utf8Path, settings: &SettingsWithEditor) -> bool {
    settings.linter_enabled_for_file_path::<TailwindLanguage>(path)
}

fn lint(params: LintParams) -> LintResults {
    let _ = debug_span!("Linting Tailwind snippet", path =? params.path).entered();
    let workspace_settings = &params.settings;
    let analyzer_options = workspace_settings.analyzer_options::<TailwindLanguage>(
        params.path,
        &params.language,
        params.suppression_reason.as_deref(),
    );
    let tree: TwRoot = params.parse.tree();

    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.settings.as_ref(), analyzer_options)
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

    let (_, analyze_diagnostics) = analyze(&tree, filter, &analyzer_options, |signal| {
        process_lint.process_signal(signal)
    });

    process_lint.into_result(
        params
            .parse
            .into_serde_diagnostics(params.diagnostic_offset),
        analyze_diagnostics,
    )
}

pub(crate) fn code_actions(params: CodeActionsParams) -> crate::workspace::PullActionsResult {
    use crate::workspace::CodeAction;

    let CodeActionsParams {
        parse,
        range,
        settings,
        path,
        module_graph: _,
        project_layout,
        language,
        only,
        skip,
        suppression_reason,
        enabled_rules: rules,
        plugins: _,
        categories,
        action_offset,
        document_services: _,
    } = params;
    let _ = debug_span!("Code actions Tailwind", range =? range, path =? path).entered();
    let tree: TwRoot = parse.tree();

    let analyzer_options = settings.analyzer_options::<TailwindLanguage>(
        path,
        &language,
        suppression_reason.as_deref(),
    );
    let mut actions = Vec::new();
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(settings.as_ref(), analyzer_options)
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

    analyze(&tree, filter, &analyzer_options, |signal| {
        actions.extend(signal.actions().into_code_action_iter().map(|item| {
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
    });

    crate::workspace::PullActionsResult { actions }
}

pub(crate) fn fix_all(
    params: FixAllParams,
) -> Result<crate::workspace::FixFileResult, WorkspaceError> {
    let tree: TwRoot = params.parse.tree();

    let rules = params
        .settings
        .as_ref()
        .as_linter_rules(params.biome_path.as_path());
    let analyzer_options = params.settings.analyzer_options::<TailwindLanguage>(
        params.biome_path,
        &params.document_file_source,
        params.suppression_reason.as_deref(),
    );
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.settings.as_ref(), analyzer_options)
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

    let syntax_len: u32 = tree.syntax().text_range_with_trivia().len().into();
    let mut process_fix_all = ProcessFixAll::new(&params, rules, syntax_len);
    loop {
        let (action, _) = analyze(&tree, filter, &analyzer_options, |signal| {
            process_fix_all.process_signal(signal)
        });

        // Tailwind has no formatter; fix_all just rewrites the text.
        let result = process_fix_all.process_action(action, |_root| {
            // Tailwind CST mutations aren't supported (no fix actions defined).
            // Return None to stop the loop.
            None
        })?;

        if result.is_none() {
            return process_fix_all.finish::<_, SimpleFormatContext>(|| {
                Ok(Either::Right(tree.syntax().text_trimmed().to_string()))
            });
        }
    }
}
