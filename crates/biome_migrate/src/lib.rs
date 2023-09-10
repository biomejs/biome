mod analyzers;
mod macros;
mod registry;

use crate::registry::visit_migration_registry;
pub use biome_analyze::ControlFlow;
use biome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerSignal, InspectMatcher,
    LanguageRoot, MatchQueryParams, MetadataRegistry, RuleRegistry,
};
use biome_diagnostics::Error;
use biome_json_syntax::JsonLanguage;
use std::convert::Infallible;
use std::path::{Path, PathBuf};

/// Return the static [MetadataRegistry] for the JS analyzer rules
pub fn metadata() -> &'static MetadataRegistry {
    lazy_static::lazy_static! {
        static ref METADATA: MetadataRegistry = {
            let mut metadata = MetadataRegistry::default();
            visit_migration_registry(&mut metadata);
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
    root: &LanguageRoot<JsonLanguage>,
    configuration_file_path: &'a Path,
    inspect_matcher: V,
    mut emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    V: FnMut(&MatchQueryParams<JsonLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<JsonLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    let filter = AnalysisFilter::default();
    let options = AnalyzerOptions {
        file_path: PathBuf::from(configuration_file_path),
        ..AnalyzerOptions::default()
    };
    let mut registry = RuleRegistry::builder(&filter, root);
    visit_migration_registry(&mut registry);

    let (migration_registry, services, diagnostics, visitors) = registry.build();

    // Bail if we can't parse a rule option
    if !diagnostics.is_empty() {
        return (None, diagnostics);
    }

    let mut analyzer = Analyzer::new(
        metadata(),
        InspectMatcher::new(migration_registry, inspect_matcher),
        |_| -> Vec<Result<_, Infallible>> { unreachable!() },
        |_| {},
        &mut emit_signal,
    );

    for ((phase, _), visitor) in visitors {
        analyzer.add_visitor(phase, visitor);
    }

    (
        analyzer.run(AnalyzerContext {
            root: root.clone(),
            range: filter.range,
            services,
            options: &options,
        }),
        diagnostics,
    )
}

pub fn migrate_configuration<'a, F, B>(
    root: &LanguageRoot<JsonLanguage>,
    configuration_file_path: &'a Path,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<JsonLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(root, configuration_file_path, |_| {}, emit_signal)
}

#[cfg(test)]
mod test {
    use crate::migrate_configuration;
    use biome_analyze::{ControlFlow, Never};
    use rome_json_parser::{parse_json, JsonParserOptions};
    use std::path::Path;

    #[test]
    #[ignore]
    fn smoke() {
        let source = r#"{ "something": "else" }"#;

        let parsed = parse_json(source, JsonParserOptions::default());

        migrate_configuration(&parsed.tree().value().unwrap(), Path::new(""), |signal| {
            for action in signal.actions() {
                let new_code = action.mutation.commit();
                eprintln!("{new_code}");
            }

            ControlFlow::<Never>::Continue(())
        });
    }
}
