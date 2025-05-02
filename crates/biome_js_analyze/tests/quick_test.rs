use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_deserialize::TextRange;
use biome_diagnostics::{Diagnostic, DiagnosticExt, Severity, print_diagnostic_to_string};
use biome_fs::TemporaryFs;
use biome_js_analyze::{JsAnalyzerServices, analyze};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsFileSource;
use biome_package::{Dependencies, PackageJson};
use biome_project_layout::ProjectLayout;
use biome_test_utils::module_graph_for_test_file;
use camino::Utf8PathBuf;
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
    const FILENAME: &str = "dummyFile.ts";
    const SOURCE: &str = r#"async function returnsPromise() {
  return 'value';
}
returnsPromise();
"#;

    let parsed = parse(SOURCE, JsFileSource::tsx(), JsParserOptions::default());

    let mut fs = TemporaryFs::new("quick_test");
    fs.create_file(FILENAME, SOURCE);
    let file_path = Utf8PathBuf::from(format!("{}/{FILENAME}", fs.cli_path()));

    let mut error_ranges: Vec<TextRange> = Vec::new();
    let options = AnalyzerOptions::default().with_file_path(file_path.clone());
    let rule_filter = RuleFilter::Rule("nursery", "noFloatingPromises");

    let mut dependencies = Dependencies::default();
    dependencies.add("buffer", "latest");

    let project_layout = project_layout_with_top_level_dependencies(dependencies);
    let services = crate::JsAnalyzerServices::from((
        module_graph_for_test_file(file_path.as_path(), project_layout.as_ref()),
        project_layout,
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
                    .with_file_path(FILENAME)
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
