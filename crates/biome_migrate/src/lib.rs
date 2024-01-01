mod analyzers;
mod macros;
mod registry;
mod version_services;

use crate::registry::visit_migration_registry;
use crate::version_services::TheVersion;
pub use biome_analyze::ControlFlow;
use biome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerSignal, InspectMatcher,
    LanguageRoot, MatchQueryParams, MetadataRegistry, RuleAction, RuleRegistry,
};
use biome_diagnostics::Error;
use biome_json_syntax::JsonLanguage;
use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::Arc;

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
    version: String,
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

    let (migration_registry, mut services, diagnostics, visitors) = registry.build();

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

    services.insert_service(Arc::new(TheVersion(version)));

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
    version: String,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<JsonLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(root, configuration_file_path, version, |_| {}, emit_signal)
}

pub(crate) type MigrationAction = RuleAction<JsonLanguage>;

#[cfg(test)]
mod test {
    use crate::migrate_configuration;
    use biome_analyze::{ControlFlow, Never};
    use biome_console::fmt::{Formatter, Termcolor};
    use biome_console::{markup, Markup};
    use biome_diagnostics::termcolor::NoColor;
    use biome_diagnostics::{DiagnosticExt, PrintDiagnostic, Severity};
    use biome_json_parser::{parse_json, JsonParserOptions};
    use std::path::Path;

    fn markup_to_string(markup: Markup) -> String {
        let mut buffer = Vec::new();
        let mut write = Termcolor(NoColor::new(&mut buffer));
        let mut fmt = Formatter::new(&mut write);
        fmt.write_markup(markup).unwrap();

        String::from_utf8(buffer).unwrap()
    }

    #[test]
    #[ignore]
    fn quick_test() {
        let source = r#"{
	"$schema": "https://biomejs.dev/schemas/1.0.0/schema.json"
}
"#;

        let parsed = parse_json(source, JsonParserOptions::default());

        migrate_configuration(
            &parsed.tree(),
            Path::new(""),
            "1.5.0".to_string(),
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let error = diag
                        .with_severity(Severity::Warning)
                        .with_file_path("dummyFile")
                        .with_file_source_code(source);
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
    }
}
