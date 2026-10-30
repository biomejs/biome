#![deny(clippy::use_self)]

mod assist;
mod lint;
mod registry;
mod suppression;
mod suppression_action;

pub use crate::registry::visit_registry;
pub use crate::suppression::MarkdownSuppression;
use crate::suppression_action::MarkdownSuppressionAction;
use biome_analyze::{
    AnalysisFilter, AnalyzerOptions, AnalyzerSignal, ControlFlow, EmbeddedSignalInspector,
    LanguageRoot, MatchQueryParams, MetadataRegistry, RuleAction, RuleRegistry, SnippetAnalyzer,
};
use biome_diagnostics::Error;
use biome_languages::MdFileSource;
use biome_markdown_syntax::MarkdownLanguage;
use std::ops::Deref;
use std::sync::LazyLock;

pub(crate) type MarkdownRuleAction = RuleAction<MarkdownLanguage>;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_registry(&mut metadata);
    metadata
});

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action.
/// Rules read `file_source` through `RuleContext::source_type`.
pub fn analyze<'a, F, B>(
    root: &LanguageRoot<MarkdownLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    file_source: MdFileSource,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<MarkdownLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(root, filter, |_| {}, options, file_source, emit_signal)
}

/// Analyzes Markdown and embedded snippets together. Ignore comments in
/// Markdown can also apply to findings in the snippets.
pub fn analyze_with_snippets<'a, F, B, Output>(
    root: &LanguageRoot<MarkdownLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    file_source: MdFileSource,
    snippets: &mut [Box<dyn SnippetAnalyzer<B, Output = Output> + '_>],
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<MarkdownLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_optional_snippets(
        AnalyzerParams {
            root,
            filter,
            options,
            file_source,
            snippet_inspector: None,
            snippets: Some(snippets),
        },
        |_| {},
        emit_signal,
    )
}

/// Analyzes Markdown embedded in another file, honoring ignore comments in both.
pub fn analyze_snippet<'a, F, B>(
    root: &LanguageRoot<MarkdownLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    file_source: MdFileSource,
    inspector: EmbeddedSignalInspector<'_, '_>,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<MarkdownLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_optional_snippets::<_, _, B, ()>(
        AnalyzerParams {
            root,
            filter,
            options,
            file_source,
            snippet_inspector: Some(inspector),
            snippets: None,
        },
        |_| {},
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
    root: &LanguageRoot<MarkdownLanguage>,
    filter: AnalysisFilter,
    inspect_matcher: V,
    options: &'a AnalyzerOptions,
    file_source: MdFileSource,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    V: FnMut(&MatchQueryParams<MarkdownLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<MarkdownLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_optional_snippets::<_, _, B, ()>(
        AnalyzerParams {
            root,
            filter,
            options,
            file_source,
            snippet_inspector: None,
            snippets: None,
        },
        inspect_matcher,
        emit_signal,
    )
}

struct AnalyzerParams<'a, 'guest, 'registry, 'snippets, 'analyzer, B, Output> {
    root: &'a LanguageRoot<MarkdownLanguage>,
    filter: AnalysisFilter<'a>,
    options: &'a AnalyzerOptions,
    file_source: MdFileSource,
    /// Checks ignore comments of the file that contains `root`, when `root`
    /// is a snippet.
    snippet_inspector: Option<EmbeddedSignalInspector<'guest, 'registry>>,
    /// Snippets embedded in `root`, analyzed after it.
    snippets: Option<&'snippets mut [Box<dyn SnippetAnalyzer<B, Output = Output> + 'analyzer>]>,
}

fn analyze_with_optional_snippets<'a, V, F, B, Output>(
    params: AnalyzerParams<'a, '_, '_, '_, '_, B, Output>,
    inspect_matcher: V,
    mut emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    V: FnMut(&MatchQueryParams<MarkdownLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<MarkdownLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    let AnalyzerParams {
        root,
        filter,
        options,
        file_source,
        snippet_inspector,
        snippets,
    } = params;
    let mut registry = RuleRegistry::builder(&filter, root);
    visit_registry(&mut registry);

    let (registry, mut services, diagnostics, visitors) = registry.build();

    // Bail if we can't parse a rule option
    if !diagnostics.is_empty() {
        return (None, diagnostics);
    }

    services.insert_service(file_source);

    let mut analyzer = biome_analyze::Analyzer::new(
        METADATA.deref(),
        biome_analyze::InspectMatcher::new(registry, inspect_matcher),
        Box::new(MarkdownSuppression::new(root)),
        Box::new(MarkdownSuppressionAction),
        &mut emit_signal,
    );

    for ((phase, _), visitor) in visitors {
        analyzer.add_visitor(phase, visitor);
    }

    let ctx = biome_analyze::AnalyzerContext {
        root: root.clone(),
        range: filter.range,
        services,
        options,
    };
    let result = match (snippet_inspector, snippets) {
        (Some(inspector), _) => analyzer.run_snippet(ctx, inspector),
        (None, Some(snippets)) => analyzer.run_with_snippets(ctx, snippets),
        (None, None) => analyzer.run(ctx),
    };

    (result, diagnostics)
}

#[cfg(test)]
mod tests {
    use crate::analyze;
    use biome_analyze::{
        ActionFilter, AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter,
    };
    use biome_console::fmt::{Formatter, Termcolor};
    use biome_console::{Markup, markup};
    use biome_diagnostics::termcolor::NoColor;
    use biome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic, Severity};
    use biome_languages::MdFileSource;
    use biome_markdown_parser::{MarkdownParserOptions, parse_markdown};
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

        let parsed = parse_markdown(SOURCE, MarkdownParserOptions::default());

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
            MdFileSource::markdown(),
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

                for action in signal.actions(ActionFilter::all()) {
                    let new_code = action.mutation.commit();
                    eprintln!("{new_code}");
                }

                ControlFlow::<Never>::Continue(())
            },
        );

        assert_eq!(error_ranges.as_slice(), &[]);
    }
}
