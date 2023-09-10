mod declare_transformation;
mod registry;
mod transformers;

use crate::registry::visit_transformation_registry;
use biome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerSignal, ControlFlow,
    InspectMatcher, LanguageRoot, MatchQueryParams, MetadataRegistry, RuleRegistry,
};
use biome_diagnostics::Error;
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_rowan::BatchMutation;
use std::convert::Infallible;

/// Return the static [MetadataRegistry] for the JS analyzer rules
pub fn metadata() -> &'static MetadataRegistry {
    lazy_static::lazy_static! {
        static ref METADATA: MetadataRegistry = {
            let mut metadata = MetadataRegistry::default();
            visit_transformation_registry(&mut metadata);
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
    mut emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    V: FnMut(&MatchQueryParams<JsLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    let mut registry = RuleRegistry::builder(&filter, root);
    visit_transformation_registry(&mut registry);

    let (registry, mut services, diagnostics, visitors) = registry.build();

    // Bail if we can't parse a rule option
    if !diagnostics.is_empty() {
        return (None, diagnostics);
    }

    let mut analyzer = Analyzer::new(
        metadata(),
        InspectMatcher::new(registry, inspect_matcher),
        |_| -> Vec<Result<_, Infallible>> { unreachable!() },
        |_| {},
        &mut emit_signal,
    );

    for ((phase, _), visitor) in visitors {
        analyzer.add_visitor(phase, visitor);
    }

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
pub fn transform<'a, F, B>(
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    source_type: JsFileSource,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(root, filter, |_| {}, options, source_type, emit_signal)
}

pub(crate) type JsBatchMutation = BatchMutation<JsLanguage>;

#[cfg(test)]
mod tests {
    use biome_analyze::{AnalyzerOptions, Never, RuleCategories, RuleFilter};
    use biome_js_syntax::JsFileSource;
    use rome_js_parser::{parse, JsParserOptions};
    use std::slice;

    use crate::{transform, AnalysisFilter, ControlFlow};

    #[ignore]
    #[test]
    fn quick_test() {
        const SOURCE: &str = r#"enum Foo { Lorem, Ipsum }"#;

        let parsed = parse(SOURCE, JsFileSource::tsx(), JsParserOptions::default());

        let options = AnalyzerOptions::default();
        let rule_filter = RuleFilter::Rule("transformations", "transformEnum");

        transform(
            &parsed.tree(),
            AnalysisFilter {
                categories: RuleCategories::TRANSFORMATION,
                enabled_rules: Some(slice::from_ref(&rule_filter)),
                ..AnalysisFilter::default()
            },
            &options,
            JsFileSource::tsx(),
            |signal| {
                for transformation in signal.transformations() {
                    let new_code = transformation.mutation.commit();
                    eprintln!("{new_code}");
                }

                ControlFlow::<Never>::Continue(())
            },
        );

        // assert_eq!(error_ranges.as_slice(), &[]);
    }
}
