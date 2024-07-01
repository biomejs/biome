#![warn(clippy::needless_pass_by_value)]

use crate::suppression_action::JsSuppressionAction;
use biome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerSignal, ControlFlow,
    InspectMatcher, LanguageRoot, MatchQueryParams, MetadataRegistry, RuleAction, RuleRegistry,
    SuppressionKind,
};
use biome_aria::{AriaProperties, AriaRoles};
use biome_diagnostics::{category, Error as DiagnosticError};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_project::PackageJson;
use biome_suppression::{parse_suppression_comment, SuppressionDiagnostic};
use std::sync::Arc;

mod assists;
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

/// Return the static [MetadataRegistry] for the JS analyzer rules
pub fn metadata() -> &'static MetadataRegistry {
    lazy_static::lazy_static! {
        static ref METADATA: MetadataRegistry = {
            let mut metadata = MetadataRegistry::default();
            visit_registry(&mut metadata);
            metadata
        };
    }

    &METADATA
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action.
/// Additionally, this function takes a `inspect_matcher` function that can be
/// used to inspect the "query matches" emitted by the analyzer before they are
/// processed by the lint rules registry
pub fn analyze_with_inspect_matcher<'a, V, F, B>(
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    inspect_matcher: V,
    options: &'a AnalyzerOptions,
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
                        if let Some(instance) = value {
                            result.push(Ok(SuppressionKind::RuleInstance(rule, instance)));
                        } else {
                            result.push(Ok(SuppressionKind::Rule(rule)));
                        }
                    }
                }
            }
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
        metadata(),
        InspectMatcher::new(registry, inspect_matcher),
        parse_linter_suppression_comment,
        Box::new(JsSuppressionAction),
        &mut emit_signal,
    );

    for ((phase, _), visitor) in visitors {
        analyzer.add_visitor(phase, visitor);
    }

    services.insert_service(Arc::new(AriaRoles));
    services.insert_service(Arc::new(AriaProperties));
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
        source_type,
        manifest,
        emit_signal,
    )
}

#[cfg(test)]
mod tests {
    use biome_analyze::options::RuleOptions;
    use biome_analyze::{AnalyzerOptions, Never, RuleCategoriesBuilder, RuleFilter, RuleKey};
    use biome_console::fmt::{Formatter, Termcolor};
    use biome_console::{markup, Markup};
    use biome_diagnostics::category;
    use biome_diagnostics::termcolor::NoColor;
    use biome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic, Severity};
    use biome_js_parser::{parse, JsParserOptions};
    use biome_js_syntax::{JsFileSource, TextRange, TextSize};
    use biome_project::{Dependencies, PackageJson};
    use std::slice;

    use crate::lint::correctness::use_exhaustive_dependencies::{Hook, HooksOptions};
    use crate::react::hooks::StableHookResult;
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

        const SOURCE: &str = r#"import buffer from "buffer"; "#;

        let parsed = parse(SOURCE, JsFileSource::tsx(), JsParserOptions::default());

        let mut error_ranges: Vec<TextRange> = Vec::new();
        let mut options = AnalyzerOptions::default();
        let hook = Hook {
            name: "myEffect".to_string(),
            closure_index: Some(0),
            dependencies_index: Some(1),
            stable_result: StableHookResult::None,
        };
        let rule_filter = RuleFilter::Rule("style", "useNodejsImportProtocol");

        options.configuration.rules.push_rule(
            RuleKey::new("nursery", "useHookAtTopLevel"),
            RuleOptions::new(HooksOptions { hooks: vec![hook] }, None),
        );

        let mut dependencies = Dependencies::default();
        dependencies.add("buffer", "latest");
        analyze(
            &parsed.tree(),
            AnalysisFilter {
                enabled_rules: Some(slice::from_ref(&rule_filter)),
                ..AnalysisFilter::default()
            },
            &options,
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
                    let text = markup_to_string(markup! {
                        {PrintDiagnostic::verbose(&error)}
                    });
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
    fn suppression() {
        const SOURCE: &str = "
            function checkSuppressions1(a, b) {
                a == b;
                // biome-ignore lint/suspicious:whole group
                a == b;
                // biome-ignore lint/suspicious/noDoubleEquals: single rule
                a == b;
                /* biome-ignore lint/style/useWhile: multiple block comments */ /* biome-ignore lint/suspicious/noDoubleEquals: multiple block comments */
                a == b;
                // biome-ignore lint/style/useWhile: multiple line comments
                // biome-ignore lint/suspicious/noDoubleEquals: multiple line comments
                a == b;
                a == b;
            }

            // biome-ignore lint/suspicious/noDoubleEquals: do not suppress warning for the whole function
            function checkSuppressions2(a, b) {
                a == b;
            }

            function checkSuppressions3(a, b) {
                a == b;
                // rome-ignore lint/suspicious: whole group
                a == b;
                // rome-ignore lint/suspicious/noDoubleEquals: single rule
                a == b;
                /* rome-ignore lint/style/useWhile: multiple block comments */ /* rome-ignore lint(suspicious/noDoubleEquals): multiple block comments */
                a == b;
                // rome-ignore lint/style/useWhile: multiple line comments
                // rome-ignore lint/suspicious/noDoubleEquals: multiple line comments
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
        let mut warn_ranges: Vec<TextRange> = Vec::new();

        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            AnalysisFilter::default(),
            &options,
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
                        lint_ranges.push(span.unwrap());
                    }

                    if code == category!("suppressions/parse") {
                        parse_ranges.push(span.unwrap());
                    }

                    if code == category!("suppressions/deprecatedSuppressionComment") {
                        warn_ranges.push(span.unwrap());
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
                TextRange::new(TextSize::from(1492), TextSize::from(1494)),
                TextRange::new(TextSize::from(1687), TextSize::from(1689)),
            ]
        );

        assert_eq!(
            parse_ranges.as_slice(),
            &[
                TextRange::new(TextSize::from(1791), TextSize::from(1802)),
                TextRange::new(TextSize::from(1842), TextSize::from(1843)),
                TextRange::new(TextSize::from(1876), TextSize::from(1877)),
                TextRange::new(TextSize::from(1929), TextSize::from(1936)),
            ]
        );

        assert_eq!(
            warn_ranges.as_slice(),
            &[
                TextRange::new(TextSize::from(944), TextSize::from(987)),
                TextRange::new(TextSize::from(1028), TextSize::from(1086)),
                TextRange::new(TextSize::from(1127), TextSize::from(1189)),
                TextRange::new(TextSize::from(1190), TextSize::from(1264)),
                TextRange::new(TextSize::from(1305), TextSize::from(1363)),
                TextRange::new(TextSize::from(1380), TextSize::from(1449)),
                TextRange::new(TextSize::from(1525), TextSize::from(1620)),
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
}
