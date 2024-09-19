mod keywords;
mod lint;
pub mod options;
mod registry;
mod services;
mod suppression_action;
mod utils;

pub use crate::registry::visit_registry;
use crate::suppression_action::CssSuppressionAction;
use biome_analyze::{
    AnalysisFilter, AnalyzerOptions, AnalyzerSignal, ControlFlow, LanguageRoot, MatchQueryParams,
    MetadataRegistry, RuleRegistry, SuppressionKind,
};
use biome_css_syntax::CssLanguage;
use biome_diagnostics::{category, Error};
use biome_suppression::{parse_suppression_comment, SuppressionDiagnostic};
use std::ops::Deref;
use std::sync::LazyLock;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_registry(&mut metadata);
    metadata
});

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action
pub fn analyze<'a, F, B>(
    root: &LanguageRoot<CssLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<CssLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(root, filter, |_| {}, options, emit_signal)
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action.
/// Additionally, this function takes a `inspect_matcher` function that can be
/// used to inspect the "query matches" emitted by the analyzer before they are
/// processed by the lint rules registry
pub fn analyze_with_inspect_matcher<'a, V, F, B>(
    root: &LanguageRoot<CssLanguage>,
    filter: AnalysisFilter,
    inspect_matcher: V,
    options: &'a AnalyzerOptions,
    mut emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    V: FnMut(&MatchQueryParams<CssLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<CssLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    fn parse_linter_suppression_comment(
        text: &str,
    ) -> Vec<Result<SuppressionKind, SuppressionDiagnostic>> {
        let mut result = Vec::new();

        for comment in parse_suppression_comment(text) {
            let categories = match comment {
                Ok(comment) => {
                    if comment.is_legacy {
                        result.push(Ok(SuppressionKind::Deprecated));
                    }
                    comment.categories
                }
                Err(err) => {
                    result.push(Err(err));
                    continue;
                }
            };

            for (key, value) in categories {
                if key == category!("lint") {
                    if let Some(value) = value {
                        result.push(Ok(SuppressionKind::MaybeLegacy(value)));
                    } else {
                        result.push(Ok(SuppressionKind::Everything));
                    }
                } else {
                    let category = key.name();
                    if let Some(rule) = category.strip_prefix("lint/") {
                        result.push(Ok(SuppressionKind::Rule(rule)));
                    }
                }
            }
        }

        result
    }

    let mut registry = RuleRegistry::builder(&filter, root);
    visit_registry(&mut registry);

    let (registry, services, diagnostics, visitors) = registry.build();

    // Bail if we can't parse a rule option
    if !diagnostics.is_empty() {
        return (None, diagnostics);
    }

    let mut analyzer = biome_analyze::Analyzer::new(
        METADATA.deref(),
        biome_analyze::InspectMatcher::new(registry, inspect_matcher),
        parse_linter_suppression_comment,
        Box::new(CssSuppressionAction),
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
    use biome_analyze::{AnalyzerOptions, Never, RuleFilter};
    use biome_console::fmt::{Formatter, Termcolor};
    use biome_console::{markup, Markup};
    use biome_css_parser::{parse_css, CssParserOptions};
    use biome_css_syntax::TextRange;
    use biome_diagnostics::termcolor::NoColor;
    use biome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic, Severity};
    use std::slice;

    use crate::{analyze, AnalysisFilter, ControlFlow};

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

        const SOURCE: &str = r#"
        /* valid */
        a:hover {}
        :not(p) {}
        a:before { }
        input:not([type='submit'])
        :root { }
        :--heading { }
        :popover-open {}
        .test::-webkit-scrollbar-button:horizontal:decrement {}
        @page :first { }

        /* invalid */
        a:unknown { }
        a:pseudo-class { }
        body:not(div):noot(span) {}
        :first { }
        @page :blank:unknown { }
        "#;

        let parsed = parse_css(SOURCE, CssParserOptions::default());

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
