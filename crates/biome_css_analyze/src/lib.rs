mod assist;
mod keywords;
mod lint;
pub mod options;
mod order;
mod registry;
mod services;
mod suppression_action;
mod utils;

pub use crate::registry::visit_registry;
use crate::suppression_action::CssSuppressionAction;
use biome_analyze::{
    to_analyzer_suppressions, AnalysisFilter, AnalyzerOptions, AnalyzerPluginSlice, AnalyzerSignal,
    AnalyzerSuppression, ControlFlow, LanguageRoot, MatchQueryParams, MetadataRegistry, RuleAction,
    RuleRegistry,
};
use biome_css_syntax::{CssLanguage, TextRange};
use biome_diagnostics::Error;
use biome_suppression::{parse_suppression_comment, SuppressionDiagnostic};
use std::ops::Deref;
use std::sync::LazyLock;

pub(crate) type CssRuleAction = RuleAction<CssLanguage>;

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
    plugins: AnalyzerPluginSlice<'a>,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<CssLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(root, filter, |_| {}, options, plugins, emit_signal)
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
    plugins: AnalyzerPluginSlice<'a>,
    mut emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    V: FnMut(&MatchQueryParams<CssLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<CssLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    fn parse_linter_suppression_comment(
        text: &str,
        piece_range: TextRange,
    ) -> Vec<Result<AnalyzerSuppression, SuppressionDiagnostic>> {
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

    for plugin in plugins {
        if plugin.supports_css() {
            analyzer.add_plugin(plugin.clone());
        }
    }

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
    use biome_analyze::{AnalyzerOptions, Never, RuleCategoriesBuilder, RuleFilter};
    use biome_console::fmt::{Formatter, Termcolor};
    use biome_console::{markup, Markup};
    use biome_css_parser::{parse_css, CssParserOptions};
    use biome_css_syntax::TextRange;
    use biome_diagnostics::termcolor::NoColor;
    use biome_diagnostics::{
        category, print_diagnostic_to_string, Diagnostic, DiagnosticExt, PrintDiagnostic, Severity,
    };
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
            &[],
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

    #[test]
    fn top_level_suppression_simple() {
        const SOURCE: &str = "
/**
* biome-ignore lint/suspicious/noEmptyBlock: reason
*/

#foo {}
#bar {}
        ";

        let parsed = parse_css(SOURCE, CssParserOptions::default());

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_syntax().build(),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(&parsed.tree(), filter, &options, &[], |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        });
    }

    #[test]
    fn top_level_suppression_multiple() {
        const SOURCE: &str = "
/**
* biome-ignore lint/suspicious/noEmptyBlock: reason
*/

/**
* biome-ignore lint/correctness/noUnknownProperty: reason2
*/


#foo {}
a {
    colr: blue;
}
        ";

        let parsed = parse_css(SOURCE, CssParserOptions::default());

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_syntax().build(),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(&parsed.tree(), filter, &options, &[], |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        });
    }

    #[test]
    fn top_level_suppression_multiple2() {
        const SOURCE: &str = "
/**
* biome-ignore lint/suspicious/noEmptyBlock: reason
* biome-ignore lint/correctness/noUnknownProperty: reason2
*/

#foo {}
a {
    colr: blue;
}
        ";

        let parsed = parse_css(SOURCE, CssParserOptions::default());

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_syntax().build(),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(&parsed.tree(), filter, &options, &[], |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        });
    }

    #[test]
    fn top_level_suppression_with_unused() {
        const SOURCE: &str = "
/**
*/

#foo {}
// biome-ignore lint/suspicious/noEmptyBlock: reason
#bar {}
        ";

        let parsed = parse_css(SOURCE, CssParserOptions::default());

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_syntax().build(),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(&parsed.tree(), filter, &options, &[], |signal| {
            if let Some(diag) = signal.diagnostic() {
                let code = diag.category().unwrap();
                if code != category!("suppressions/unused") {
                    panic!("unexpected diagnostic {code:?}");
                }
            }

            ControlFlow::<Never>::Continue(())
        });
    }
}
