mod declare_transformation;
mod registry;
mod transformers;

use crate::registry::visit_transformation_registry;
use biome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerSignal, ApplySuppression,
    ControlFlow, InspectMatcher, LanguageRoot, MatchQueryParams, MetadataRegistry, RuleRegistry,
    SuppressionAction,
};
use biome_diagnostics::Error;
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_rowan::{BatchMutation, SyntaxToken};
use std::convert::Infallible;
use std::ops::Deref;
use std::sync::LazyLock;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_transformation_registry(&mut metadata);
    metadata
});

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

    struct TestAction;
    impl SuppressionAction for TestAction {
        type Language = JsLanguage;

        fn find_token_for_inline_suppression(
            &self,
            _: SyntaxToken<Self::Language>,
        ) -> Option<ApplySuppression<Self::Language>> {
            None
        }

        fn apply_top_level_suppression(
            &self,
            _: &mut BatchMutation<Self::Language>,
            _: SyntaxToken<Self::Language>,
            _: &str,
        ) {
            unreachable!("")
        }

        fn apply_inline_suppression(
            &self,
            _: &mut BatchMutation<Self::Language>,
            _: ApplySuppression<Self::Language>,
            _: &str,
            _: &str,
        ) {
            unreachable!("")
        }
    }
    let mut analyzer = Analyzer::new(
        METADATA.deref(),
        InspectMatcher::new(registry, inspect_matcher),
        |_, _| -> Vec<Result<_, Infallible>> { unreachable!() },
        Box::new(TestAction),
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
    use biome_analyze::{AnalyzerOptions, Never, RuleCategoriesBuilder, RuleFilter};
    use biome_js_parser::{parse, JsParserOptions};
    use biome_js_syntax::JsFileSource;
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
                categories: RuleCategoriesBuilder::default()
                    .with_transformation()
                    .build(),
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
    }
}
