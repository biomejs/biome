mod analyzers;
mod macros;
mod registry;
mod rule_mover;

use crate::registry::visit_migration_registry;
pub use biome_analyze::ControlFlow;
use biome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerSignal, ApplySuppression,
    InspectMatcher, LanguageRoot, MatchQueryParams, MetadataRegistry, RuleAction, RuleRegistry,
    SuppressionAction,
};
use biome_diagnostics::Error;
use biome_json_syntax::JsonLanguage;
use biome_rowan::{BatchMutation, SyntaxToken};
use camino::Utf8Path;
use std::convert::Infallible;
use std::ops::Deref;
use std::sync::LazyLock;

/// Return the static [MetadataRegistry] for the JS analyzer rules
static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_migration_registry(&mut metadata);
    metadata
});

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action.
/// Additionally, this function takes a `inspect_matcher` function that can be
/// used to inspect the "query matches" emitted by the analyzer before they are
/// processed by the lint rules registry
pub fn analyze_with_inspect_matcher<'a, V, F, B>(
    root: &LanguageRoot<JsonLanguage>,
    filter: AnalysisFilter,
    configuration_file_path: &'a Utf8Path,
    inspect_matcher: V,
    mut emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    V: FnMut(&MatchQueryParams<JsonLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<JsonLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    let options = AnalyzerOptions::default().with_file_path(configuration_file_path);
    let mut registry = RuleRegistry::builder(&filter, root);
    visit_migration_registry(&mut registry);

    let (migration_registry, services, diagnostics, visitors) = registry.build();

    // Bail if we can't parse a rule option
    if !diagnostics.is_empty() {
        return (None, diagnostics);
    }
    struct TestAction;
    impl SuppressionAction for TestAction {
        type Language = JsonLanguage;

        fn find_token_for_inline_suppression(
            &self,
            _: SyntaxToken<Self::Language>,
        ) -> Option<ApplySuppression<Self::Language>> {
            None
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

        fn apply_top_level_suppression(
            &self,
            _: &mut BatchMutation<Self::Language>,
            _: SyntaxToken<Self::Language>,
            _: &str,
        ) {
        }
    }
    let mut analyzer = Analyzer::new(
        METADATA.deref(),
        InspectMatcher::new(migration_registry, inspect_matcher),
        |_, _| -> Vec<Result<_, Infallible>> { Default::default() },
        Box::new(TestAction),
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
    filter: AnalysisFilter,
    configuration_file_path: &'a Utf8Path,
    emit_signal: F,
) -> (Option<B>, Vec<Error>)
where
    F: FnMut(&dyn AnalyzerSignal<JsonLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(root, filter, configuration_file_path, |_| {}, emit_signal)
}

pub(crate) type MigrationAction = RuleAction<JsonLanguage>;

#[cfg(test)]
mod test {
    use crate::migrate_configuration;
    use biome_analyze::{AnalysisFilter, ControlFlow, Never};
    use biome_console::fmt::{Formatter, Termcolor};
    use biome_console::{markup, Markup};
    use biome_diagnostics::termcolor::NoColor;
    use biome_diagnostics::{DiagnosticExt, PrintDiagnostic, Severity};
    use biome_json_parser::{parse_json, JsonParserOptions};
    use camino::Utf8Path;

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
	"$schema": "https://biomejs.dev/schemas/1.0.0/schema.json",
	"javascript": {
		"formatter": {
			"trailingCommas": "all",
			"indentSize": 2
		}
	},
	"overrides": [
		{
			"include": ["scripts/**"],
			"javascript": {
				"formatter": {
					"trailingCommas": "es5",
					"indentSize": 4
				}
			}
		}
	]
}
"#;

        let parsed = parse_json(source, JsonParserOptions::default());

        migrate_configuration(
            &parsed.tree(),
            AnalysisFilter::default(),
            Utf8Path::new(""),
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
