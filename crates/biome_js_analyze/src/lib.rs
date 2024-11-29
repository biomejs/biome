#![warn(clippy::needless_pass_by_value)]

use crate::suppression_action::JsSuppressionAction;
use biome_analyze::{
    to_analyzer_suppressions, AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions,
    AnalyzerPlugin, AnalyzerSignal, AnalyzerSuppression, ControlFlow, InspectMatcher, LanguageRoot,
    MatchQueryParams, MetadataRegistry, RuleAction, RuleRegistry,
};
use biome_aria::AriaRoles;
use biome_diagnostics::Error as DiagnosticError;
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_project::PackageJson;
use biome_rowan::TextRange;
use biome_suppression::{parse_suppression_comment, SuppressionDiagnostic};
use std::ops::Deref;
use std::sync::{Arc, LazyLock};

pub mod assist;
mod ast_utils;
pub mod globals;
pub mod lint;
pub mod options;
mod react;
mod registry;
mod services;
mod suppression_action;
mod syntax;
pub mod utils;

pub use crate::registry::visit_registry;
pub use crate::services::control_flow::ControlFlowGraph;

pub(crate) type JsRuleAction = RuleAction<JsLanguage>;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_registry(&mut metadata);
    metadata
});

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action.
/// Additionally, this function takes a `inspect_matcher` function that can be
/// used to inspect the "query matches" emitted by the analyzer before they are
/// processed by the lint rules registry
#[allow(clippy::too_many_arguments)]
pub fn analyze_with_inspect_matcher<'a, V, F, B>(
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    inspect_matcher: V,
    options: &'a AnalyzerOptions,
    plugins: Vec<Box<dyn AnalyzerPlugin>>,
    source_type: JsFileSource,
    manifest: Option<PackageJson>,
    mut emit_signal: F,
) -> (Option<B>, Vec<DiagnosticError>)
where
    V: FnMut(&MatchQueryParams<JsLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    fn parse_linter_suppression_comment(
        text: &str,
        piece_range: TextRange,
    ) -> Vec<Result<AnalyzerSuppression, SuppressionDiagnostic>> {
        let mut result = Vec::new();

        for comment in parse_suppression_comment(text) {
            let suppression = match comment {
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

    let mut analyzer = Analyzer::new(
        METADATA.deref(),
        InspectMatcher::new(registry, inspect_matcher),
        parse_linter_suppression_comment,
        Box::new(JsSuppressionAction),
        &mut emit_signal,
    );

    for plugin in plugins {
        if plugin.supports_js() {
            analyzer.add_plugin(plugin);
        }
    }

    for ((phase, _), visitor) in visitors {
        analyzer.add_visitor(phase, visitor);
    }

    services.insert_service(Arc::new(AriaRoles));
    services.insert_service(Arc::new(manifest));
    services.insert_service(source_type);
    (
        analyzer.run(AnalyzerContext {
            root: root.clone(),
            range: filter.range,
            services,
            options,
        }),
        diagnostics,
    )
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action
pub fn analyze<'a, F, B>(
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    plugins: Vec<Box<dyn AnalyzerPlugin>>,
    source_type: JsFileSource,
    manifest: Option<PackageJson>,
    emit_signal: F,
) -> (Option<B>, Vec<DiagnosticError>)
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(
        root,
        filter,
        |_| {},
        options,
        plugins,
        source_type,
        manifest,
        emit_signal,
    )
}

#[cfg(test)]
mod tests {
    use biome_analyze::{AnalyzerOptions, Never, RuleCategoriesBuilder, RuleFilter};
    use biome_diagnostics::category;
    use biome_diagnostics::{print_diagnostic_to_string, Diagnostic, DiagnosticExt, Severity};
    use biome_js_parser::{parse, JsParserOptions};
    use biome_js_syntax::{JsFileSource, TextRange, TextSize};
    use biome_project::{Dependencies, PackageJson};
    use std::slice;

    use crate::{analyze, AnalysisFilter, ControlFlow};

    // #[ignore]
    #[test]
    fn quick_test() {
        const SOURCE: &str = r#"

        /**
* biome-ignore lint/style/useConst: reason
 */


let foo = 2;
let bar = 33;
        "#;

        let parsed = parse(SOURCE, JsFileSource::tsx(), JsParserOptions::default());

        let mut error_ranges: Vec<TextRange> = Vec::new();
        let options = AnalyzerOptions::default();
        let rule_filter = RuleFilter::Rule("style", "useConst");

        let mut dependencies = Dependencies::default();
        dependencies.add("buffer", "latest");
        analyze(
            &parsed.tree(),
            AnalysisFilter {
                enabled_rules: Some(slice::from_ref(&rule_filter)),
                ..AnalysisFilter::default()
            },
            &options,
            Vec::new(),
            JsFileSource::tsx(),
            Some(PackageJson {
                dependencies,
                ..Default::default()
            }),
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    error_ranges.push(diag.location().span.unwrap());
                    let error = diag
                        .with_severity(Severity::Warning)
                        .with_file_path("dummyFile")
                        .with_file_source_code(SOURCE);
                    let text = print_diagnostic_to_string(&error);
                    eprintln!("{text}");
                }

                for action in signal.actions() {
                    let new_code = action.mutation.commit();
                    eprintln!("new code!!!");
                    eprintln!("{new_code}");
                }

                ControlFlow::<Never>::Continue(())
            },
        );

        // assert_eq!(error_ranges.as_slice(), &[]);
    }

    #[test]
    fn quick_test_suppression() {
        const SOURCE: &str = "
            function checkSuppressions1(a, b) {
                // biome-ignore lint/suspicious:whole group
                p == f;
                // biome-ignore lint/suspicious/noDoubleEquals: single rule
                j == k;
            }
        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            AnalysisFilter::default(),
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let error = diag
                        .with_severity(Severity::Warning)
                        .with_file_path("example.js")
                        .with_file_source_code(SOURCE);

                    let code = error.category().unwrap();
                    if code == category!("lint/suspicious/noDoubleEquals") {
                        let text = print_diagnostic_to_string(&error);
                        eprintln!("{text}");
                        panic!("unexpected diagnostic");
                    }
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }

    #[test]
    fn suppression() {
        const SOURCE: &str = "
            function checkSuppressions1(a, b) {
                a == b;
                // biome-ignore lint/suspicious:whole group
                p == f;
                // biome-ignore lint/suspicious/noDoubleEquals: single rule
                j == k;
                /* biome-ignore lint/style/useWhile: multiple block comments */ /* biome-ignore lint/suspicious/noDoubleEquals: multiple block comments */
                o == m;
                // biome-ignore lint/style/useWhile: multiple line comments
                // biome-ignore lint/suspicious/noDoubleEquals: multiple line comments
                d == x;
                z == v;
            }

            // biome-ignore lint/suspicious/noDoubleEquals: do not suppress warning for the whole function
            function checkSuppressions2(a, b) {
                a == b;
            }

            function checkSuppressions3(a, b) {
                a == b;
                // biome-ignore lint/suspicious: whole group
                a == b;
                // biome-ignore lint/suspicious/noDoubleEquals: single rule
                a == b;
                /* biome-ignore lint/style/useWhile: multiple block comments */ /* biome-ignore lint(suspicious/noDoubleEquals): multiple block comments */
                a == b;
                // biome-ignore lint/style/useWhile: multiple line comments
                // biome-ignore lint/suspicious/noDoubleEquals: multiple line comments
                a == b;
                a == b;
            }

            // biome-ignore lint(suspicious/noDoubleEquals): do not suppress warning for the whole function
            function checkSuppressions4(a, b) {
                a == b;
            }

            function checkSuppressions5() {
                // biome-ignore format explanation
                // biome-ignore format(:
                // biome-ignore (value): explanation
                // biome-ignore unknown: explanation
            }
        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let mut lint_ranges: Vec<TextRange> = Vec::new();
        let mut parse_ranges: Vec<TextRange> = Vec::new();

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            AnalysisFilter::default(),
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let span = diag.get_span();
                    let error = diag
                        .with_severity(Severity::Warning)
                        .with_file_path("example.js")
                        .with_file_source_code(SOURCE);

                    let code = error.category().unwrap();
                    if code == category!("lint/suspicious/noDoubleEquals") {
                        let text = print_diagnostic_to_string(&error);
                        eprintln!("{text}");
                        lint_ranges.push(span.unwrap());
                    }

                    if code == category!("suppressions/parse") {
                        parse_ranges.push(span.unwrap());
                    }
                }

                ControlFlow::<Never>::Continue(())
            },
        );
        assert_eq!(
            lint_ranges.as_slice(),
            &[
                TextRange::new(TextSize::from(67), TextSize::from(69)),
                TextRange::new(TextSize::from(641), TextSize::from(643)),
                TextRange::new(TextSize::from(835), TextSize::from(837)),
                TextRange::new(TextSize::from(922), TextSize::from(924)),
                TextRange::new(TextSize::from(1498), TextSize::from(1500)),
                TextRange::new(TextSize::from(1693), TextSize::from(1695)),
            ]
        );

        assert_eq!(
            parse_ranges.as_slice(),
            &[
                TextRange::new(TextSize::from(1797), TextSize::from(1808)),
                TextRange::new(TextSize::from(1848), TextSize::from(1849)),
                TextRange::new(TextSize::from(1882), TextSize::from(1883)),
                TextRange::new(TextSize::from(1935), TextSize::from(1942)),
            ]
        );
    }

    #[test]
    fn suppression_syntax() {
        const SOURCE: &str = "
            // biome-ignore lint/suspicious/noDoubleEquals: single rule
            a == b;
        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_syntax().build(),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let code = diag.category().unwrap();
                    if code != category!("suppressions/unused") {
                        panic!("unexpected diagnostic {code:?}");
                    }
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }

    #[test]
    fn top_level_suppression_simple() {
        const SOURCE: &str = "
/**
* biome-ignore-all lint/style/useConst: reason
*/


let foo = 2;
let bar = 33;
        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let error = diag
                        .with_file_path("dummyFile")
                        .with_file_source_code(SOURCE);
                    let text = print_diagnostic_to_string(&error);
                    eprintln!("{text}");
                    panic!("Unexpected diagnostic");
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }

    #[test]
    fn top_level_suppression_multiple() {
        const SOURCE: &str = "
/**
* biome-ignore-all lint/style/useConst: reason
*/

/**
* biome-ignore-all lint/suspicious/noDebugger: reason2
*/


let foo = 2;
let bar = 33;
debugger;
        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let error = diag
                        .with_file_path("dummyFile")
                        .with_file_source_code(SOURCE);
                    let text = print_diagnostic_to_string(&error);
                    eprintln!("{text}");
                    panic!("Unexpected diagnostic");
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }

    #[test]
    fn top_level_suppression_all() {
        const SOURCE: &str = "
/**
* biome-ignore-all lint: reason
*/

let foo = 2;
let bar = 33;
debugger;
        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let error = diag
                        .with_file_path("dummyFile")
                        .with_file_source_code(SOURCE);
                    let text = print_diagnostic_to_string(&error);
                    eprintln!("{text}");
                    panic!("Unexpected diagnostic");
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }

    #[test]
    fn top_level_suppression_multiple2() {
        const SOURCE: &str = "
/**
* biome-ignore-all lint/style/useConst: reason
* biome-ignore-all lint/suspicious/noDebugger: reason2
*/


let foo = 2;
let bar = 33;
debugger;
        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let error = diag
                        .with_file_path("dummyFile")
                        .with_file_source_code(SOURCE);
                    let text = print_diagnostic_to_string(&error);
                    eprintln!("{text}");
                    panic!("Unexpected diagnostic");
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }

    #[test]
    fn top_level_suppression_with_unused() {
        const SOURCE: &str = "
/**
* biome-ignore-all lint/style/useConst: reason
*/


let foo = 2;
/**
* biome-ignore lint/style/useConst: reason
*/
let bar = 33;
        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let code = diag.category().unwrap();
                    if code != category!("suppressions/unused") {
                        panic!("unexpected diagnostic {code:?}");
                    }
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }

    #[test]
    fn top_level_suppression_with_block_comment() {
        const SOURCE: &str = "
/*
* Top level comment here. It could be a banner or a license comment
* MIT
*/
/**
* biome-ignore-all lint/style/useConst: reason
*/

let foo = 2;
let bar = 33;
        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let error = diag
                        .with_file_path("dummyFile")
                        .with_file_source_code(SOURCE);
                    let text = print_diagnostic_to_string(&error);
                    eprintln!("{text}");
                    panic!("Unexpected diagnostic");
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }

    #[test]
    fn suppression_range_should_report_after_end() {
        const SOURCE: &str = "
// biome-ignore-start lint/suspicious/noDoubleEquals: single rule
// biome-ignore-start lint/style/useConst: single rule
a == b;
let c;
// biome-ignore-end lint/suspicious/noDoubleEquals: single rule
a == b;
let c;

        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            enabled_rules: Some(&[
                RuleFilter::Rule("suspicious", "noDoubleEquals"),
                RuleFilter::Rule("style", "useConst"),
            ]),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let code = diag.category().unwrap();
                    if code != category!("lint/suspicious/noDoubleEquals") {
                        panic!("unexpected diagnostic {code:?}");
                    }
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }

    #[test]
    fn suppression_range_should_report_after_end_v2() {
        const SOURCE: &str = "
// biome-ignore-start lint/suspicious/noDoubleEquals: single rule
// biome-ignore-start lint/suspicious/noDebugger: single rule
a == b;
debugger;
// biome-ignore-end lint/suspicious/noDoubleEquals: single rule
a === b;
debugger;
// biome-ignore-end lint/suspicious/noDebugger: single rule
debugger;

        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            enabled_rules: Some(&[
                RuleFilter::Rule("suspicious", "noDoubleEquals"),
                RuleFilter::Rule("suspicious", "noDebugger"),
            ]),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        let mut has_diagnostics = false;
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    has_diagnostics = true;
                    let code = diag.category().unwrap();
                    if code != category!("lint/suspicious/noDebugger") {
                        panic!("unexpected diagnostic {code:?}");
                    }
                }

                ControlFlow::<Never>::Continue(())
            },
        );
        assert!(has_diagnostics, "must have diagnostics");
    }

    #[test]
    fn suppression_range_should_not_report_after_end() {
        const SOURCE: &str = "
// biome-ignore-start lint/suspicious/noDoubleEquals: single rule
// biome-ignore-start lint/style/useConst: single rule
a == b;
let c;
// biome-ignore-end lint/suspicious/noDoubleEquals: single rule
a === b;
let f;
// biome-ignore-end lint/style/useConst: single rule
let d;

        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            enabled_rules: Some(&[
                RuleFilter::Rule("suspicious", "noDoubleEquals"),
                RuleFilter::Rule("style", "useConst"),
            ]),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let error = diag
                        .with_file_path("dummyFile")
                        .with_file_source_code(SOURCE);
                    let text = print_diagnostic_to_string(&error);
                    eprintln!("{text}");
                    panic!("Unexpected diagnostic");
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }

    #[test]
    fn unused_range_suppression() {
        const SOURCE: &str = "
// biome-ignore-all lint/suspicious/noDoubleEquals: single rule
a == b;
// biome-ignore-start lint/suspicious/noDoubleEquals: single rule
a == b;
a == b;
// biome-ignore-end lint/suspicious/noDoubleEquals: single rule

        ";

        let parsed = parse(
            SOURCE,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let filter = AnalysisFilter {
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            enabled_rules: Some(&[RuleFilter::Rule("suspicious", "noDoubleEquals")]),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        let mut has_diagnostics = false;
        analyze(
            &parsed.tree(),
            filter,
            &options,
            Vec::new(),
            JsFileSource::js_module(),
            None,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    has_diagnostics = true;
                    let code = diag.category().unwrap();
                    if code != category!("suppressions/unused") {
                        panic!("unexpected diagnostic {code:?}");
                    }
                }

                ControlFlow::<Never>::Continue(())
            },
        );
        assert!(has_diagnostics, "must have diagnostics");
    }
}
