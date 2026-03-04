#![deny(clippy::use_self)]

mod a11y;
mod assist;
mod lint;
pub mod options;
mod registry;
mod services;
mod suppression_action;

pub use crate::registry::visit_registry;
pub use crate::services::module_graph::{HtmlModuleGraph, HtmlModuleGraphService};
use crate::suppression_action::HtmlSuppressionAction;

/// Services available to HTML lint rules.
#[derive(Debug, Default)]
pub struct HtmlAnalyzerServices {
    pub module_graph: Option<Arc<ModuleGraph>>,
    pub project_layout: Option<Arc<ProjectLayout>>,
}
use biome_analyze::{
    AnalysisFilter, AnalyzerOptions, AnalyzerSignal, AnalyzerSuppression, ControlFlow,
    LanguageRoot, MatchQueryParams, MetadataRegistry, RuleAction, RuleRegistry,
    to_analyzer_suppressions,
};
use biome_deserialize::TextRange;
use biome_diagnostics::Error;
use biome_html_syntax::{HtmlFileSource, HtmlLanguage};
use biome_module_graph::ModuleGraph;
use biome_project_layout::ProjectLayout;
use biome_suppression::{SuppressionDiagnostic, parse_suppression_comment};
use std::ops::Deref;
use std::sync::{Arc, LazyLock};

pub(crate) type HtmlRuleAction = RuleAction<HtmlLanguage>;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_registry(&mut metadata);
    metadata
});

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action
pub fn analyze<'a, F, B>(
    root: &LanguageRoot<HtmlLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    source_type: HtmlFileSource,
    html_services: HtmlAnalyzerServices,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<HtmlLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(
        root,
        filter,
        |_| {},
        options,
        source_type,
        html_services,
        emit_signal,
    )
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action.
/// Additionally, this function takes a `inspect_matcher` function that can be
/// used to inspect the "query matches" emitted by the analyzer before they are
/// processed by the lint rules registry
pub fn analyze_with_inspect_matcher<'a, V, F, B>(
    root: &LanguageRoot<HtmlLanguage>,
    filter: AnalysisFilter,
    inspect_matcher: V,
    options: &'a AnalyzerOptions,
    source_type: HtmlFileSource,
    html_services: HtmlAnalyzerServices,
    mut emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    V: FnMut(&MatchQueryParams<HtmlLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<HtmlLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    fn parse_linter_suppression_comment(
        text: &str,
        piece_range: TextRange,
    ) -> Vec<Result<AnalyzerSuppression<'_>, SuppressionDiagnostic>> {
        let mut result = Vec::new();

        for suppression in parse_suppression_comment(text) {
            let suppression = match suppression {
                Ok(suppression) => suppression,
                Err(err) => {
                    result.push(Err(err));
                    continue;
                }
            };

            let analyzer_suppressions: Vec<_> = to_analyzer_suppressions(suppression, piece_range)
                .into_iter()
                .map(Ok)
                .collect();

            result.extend(analyzer_suppressions)
        }

        result
    }

    let mut registry = RuleRegistry::builder(&filter, root);
    visit_registry(&mut registry);

    let (registry, mut services, diagnostics, visitors) = registry.build();

    // Bail if we can't parse a rule option
    if !diagnostics.is_empty() {
        return (None, diagnostics);
    }

    services.insert_service(source_type);
    if let Some(module_graph) = html_services.module_graph {
        services.insert_service(module_graph);
    }
    if let Some(project_layout) = html_services.project_layout {
        services.insert_service(project_layout);
    }

    let mut analyzer = biome_analyze::Analyzer::new(
        METADATA.deref(),
        biome_analyze::InspectMatcher::new(registry, inspect_matcher),
        parse_linter_suppression_comment,
        Box::new(HtmlSuppressionAction),
        &mut emit_signal,
    );

    for ((phase, _), visitor) in visitors {
        analyzer.add_visitor(phase, visitor);
    }

    (
        analyzer.run(biome_analyze::AnalyzerContext {
            root: root.clone(),
            range: filter.range,
            services,
            options,
        }),
        diagnostics,
    )
}

#[cfg(test)]
mod tests {
    use crate::analyze;
    use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
    use biome_console::fmt::{Formatter, Termcolor};
    use biome_console::{Markup, markup};
    use biome_diagnostics::termcolor::NoColor;
    use biome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic, Severity};
    use biome_html_parser::parse_html;
    use biome_html_syntax::HtmlFileSource;
    use biome_rowan::TextRange;
    use std::slice;

    #[ignore]
    #[test]
    fn quick_test() {
        fn markup_to_string(markup: Markup) -> String {
            let mut buffer = Vec::new();
            let mut write = Termcolor(NoColor::new(&mut buffer));
            let mut fmt = Formatter::new(&mut write);
            fmt.write_markup(markup).unwrap();

            String::from_utf8(buffer).unwrap()
        }

        const SOURCE: &str = r#" "#;

        let parsed = parse_html(SOURCE, Default::default());

        let mut error_ranges: Vec<TextRange> = Vec::new();
        let rule_filter = RuleFilter::Rule("nursery", "noUnknownPseudoClass");
        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            AnalysisFilter {
                enabled_rules: Some(slice::from_ref(&rule_filter)),
                ..AnalysisFilter::default()
            },
            &options,
            HtmlFileSource::html(),
            crate::HtmlAnalyzerServices::default(),
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    error_ranges.push(diag.location().span.unwrap());
                    let error = diag
                        .with_severity(Severity::Warning)
                        .with_file_path("ahahah")
                        .with_file_source_code(SOURCE);
                    let text = markup_to_string(markup! {
                        {PrintDiagnostic::verbose(&error)}
                    });
                    eprintln!("{text}");
                }

                for action in signal.actions() {
                    let new_code = action.mutation.commit();
                    eprintln!("{new_code}");
                }

                ControlFlow::<Never>::Continue(())
            },
        );

        assert_eq!(error_ranges.as_slice(), &[]);
    }
}
