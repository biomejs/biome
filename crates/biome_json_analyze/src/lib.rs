mod assists;
mod lint;

pub mod options;
mod registry;
mod suppression_action;
pub mod utils;

pub use crate::registry::visit_registry;
use crate::suppression_action::JsonSuppressionAction;
use biome_analyze::{
    AnalysisFilter, AnalyzerOptions, AnalyzerSignal, ControlFlow, LanguageRoot, MatchQueryParams,
    MetadataRegistry, RuleAction, RuleRegistry, SuppressionDiagnostic, SuppressionKind,
};
use biome_diagnostics::Error;
use biome_json_syntax::{JsonFileSource, JsonLanguage};

pub(crate) type JsonRuleAction = RuleAction<JsonLanguage>;

/// Return the static [MetadataRegistry] for the JSON analyzer rules
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
/// then call `emit_signal` when an analysis rule emits a diagnostic or action
pub fn analyze<'a, F, B>(
    root: &LanguageRoot<JsonLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    file_source: JsonFileSource,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<JsonLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(root, filter, |_| {}, options, file_source, emit_signal)
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action.
/// Additionally, this function takes a `inspect_matcher` function that can be
/// used to inspect the "query matches" emitted by the analyzer before they are
/// processed by the lint rules registry
pub fn analyze_with_inspect_matcher<'a, V, F, B>(
    root: &LanguageRoot<JsonLanguage>,
    filter: AnalysisFilter,
    inspect_matcher: V,
    options: &'a AnalyzerOptions,
    file_source: JsonFileSource,
    mut emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    V: FnMut(&MatchQueryParams<JsonLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<JsonLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    fn parse_linter_suppression_comment(
        _text: &str,
    ) -> Vec<Result<SuppressionKind, SuppressionDiagnostic>> {
        vec![]
    }
    let mut registry = RuleRegistry::builder(&filter, root);
    visit_registry(&mut registry);

    let (registry, mut services, diagnostics, visitors) = registry.build();

    // Bail if we can't parse a rule option
    if !diagnostics.is_empty() {
        return (None, diagnostics);
    }

    let mut analyzer = biome_analyze::Analyzer::new(
        metadata(),
        biome_analyze::InspectMatcher::new(registry, inspect_matcher),
        parse_linter_suppression_comment,
        Box::new(JsonSuppressionAction),
        &mut emit_signal,
    );

    for ((phase, _), visitor) in visitors {
        analyzer.add_visitor(phase, visitor);
    }

    services.insert_service(file_source);

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
    use biome_diagnostics::termcolor::NoColor;
    use biome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic, Severity};
    use biome_json_parser::{parse_json, JsonParserOptions};
    use biome_json_syntax::{JsonFileSource, TextRange};
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

        const SOURCE: &str = r#"	{
		"loreum": "",
		"loreum": "",
		"loreum": ""
	}
"#;

        let parsed = parse_json(SOURCE, JsonParserOptions::default());

        let mut error_ranges: Vec<TextRange> = Vec::new();
        let rule_filter = RuleFilter::Rule("nursery", "noDuplicateJsonKeys");
        let options = AnalyzerOptions::default();
        analyze(
            &parsed.tree(),
            AnalysisFilter {
                enabled_rules: Some(slice::from_ref(&rule_filter)),
                ..AnalysisFilter::default()
            },
            &options,
            JsonFileSource::json(),
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
