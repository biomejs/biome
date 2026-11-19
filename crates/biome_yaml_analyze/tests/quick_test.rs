use biome_analyze::{
    ActionFilter, AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter,
};
use biome_diagnostics::{Diagnostic, DiagnosticExt, Severity, print_diagnostic_to_string};
use biome_yaml_parser::parse_yaml;
use std::slice;

use biome_rowan::TextRange;
use biome_yaml_analyze::analyze;

// use this test check if your snippet produces the diagnostics you wish, without using a snapshot
#[ignore]
#[test]
fn quick_test() {
    const FILENAME: &str = "dummyFile.yaml";
    const SOURCE: &str = r#"
person:
  name: John Doe
  age: 50
"#;

    let parsed = parse_yaml(SOURCE);

    let mut error_ranges: Vec<TextRange> = Vec::new();
    let options = AnalyzerOptions::default();
    let rule_filter = RuleFilter::Rule("suspicious", "noDuplicateObjectKeys");
    analyze(
        &parsed.tree(),
        AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        },
        &options,
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

            for action in signal.actions(ActionFilter::all()) {
                let new_code = action.mutation.commit();
                eprintln!("{new_code}");
            }

            ControlFlow::<Never>::Continue(())
        },
    );

    // assert_eq!(error_ranges.as_slice(), &[]);
}
