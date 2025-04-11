use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_deserialize::TextRange;
use biome_diagnostics::{Diagnostic, DiagnosticExt, Severity, print_diagnostic_to_string};
use biome_js_analyze::{JsAnalyzerServices, analyze};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsFileSource;
use biome_package::{Dependencies, PackageJson};
use biome_project_layout::ProjectLayout;
use std::slice;
use std::sync::Arc;

fn project_layout_with_top_level_dependencies(dependencies: Dependencies) -> Arc<ProjectLayout> {
    let manifest = PackageJson::default().with_dependencies(dependencies);

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest("/".into(), manifest);

    Arc::new(project_layout)
}

// use this test check if your snippet produces the diagnostics you wish, without using a snapshot
#[ignore]
#[test]
fn quick_test() {
    const SOURCE: &str = r#"f({ prop: () => {} })"#;

    let parsed = parse(SOURCE, JsFileSource::tsx(), JsParserOptions::default());

    let mut error_ranges: Vec<TextRange> = Vec::new();
    let options = AnalyzerOptions::default();
    let rule_filter = RuleFilter::Rule("nursery", "useExplicitType");

    let mut dependencies = Dependencies::default();
    dependencies.add("buffer", "latest");

    let services = crate::JsAnalyzerServices::from((
        Default::default(),
        project_layout_with_top_level_dependencies(dependencies),
        JsFileSource::tsx(),
    ));

    analyze(
        &parsed.tree(),
        AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        },
        &options,
        &[],
        services,
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
