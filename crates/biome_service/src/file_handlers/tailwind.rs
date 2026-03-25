use super::{
    AnalyzerCapabilities, AnalyzerVisitorBuilder, Capabilities, CodeActionsParams, FixAllParams,
    LintParams, LintResults, ProcessFixAll, ProcessLint,
};
use crate::WorkspaceError;
use crate::configuration::to_analyzer_rules;
use crate::settings::Settings;
use biome_analyze::{AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never};
use biome_formatter::SimpleFormatContext;
use biome_rowan::AstNode;
use biome_tailwind_analyze::analyze;
use biome_tailwind_syntax::TwRoot;
use either::Either;
use std::borrow::Cow;
use tracing::debug_span;

pub(crate) fn capabilities() -> Capabilities {
    Capabilities {
        analyzer: AnalyzerCapabilities {
            lint: Some(lint),
            code_actions: Some(code_actions),
            fix_all: Some(fix_all),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub(crate) fn lint(params: LintParams) -> LintResults {
    let _ = debug_span!("Linting Tailwind snippet", path =? params.path).entered();
    let analyzer_options = analyzer_options(
        params.settings.as_ref(),
        params.path,
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
        language: _,
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

    let analyzer_options = analyzer_options(settings.as_ref(), path, suppression_reason.as_deref());
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
    let mut tree: TwRoot = params.parse.tree();

    let rules = params
        .settings
        .as_ref()
        .as_linter_rules(params.biome_path.as_path());
    let analyzer_options = analyzer_options(
        params.settings.as_ref(),
        params.biome_path,
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

        let result = process_fix_all.process_action(action, |root| {
            tree = match TwRoot::cast(root) {
                Some(tree) => tree,
                None => return None,
            };

            Some(tree.syntax().text_range_with_trivia().len().into())
        })?;

        if result.is_none() {
            return process_fix_all.finish::<_, SimpleFormatContext>(
                || Ok(Either::Right(tree.syntax().text_trimmed().to_string())),
                params.embeds_initial_indent,
            );
        }
    }
}

pub(crate) fn analyzer_options(
    settings: &Settings,
    path: &biome_fs::BiomePath,
    suppression_reason: Option<&str>,
) -> AnalyzerOptions {
    let configuration =
        AnalyzerConfiguration::default().with_rules(to_analyzer_rules(settings, path.as_path()));
    AnalyzerOptions::default()
        .with_file_path(path.as_path())
        .with_configuration(configuration)
        .with_suppression_reason(suppression_reason)
}
