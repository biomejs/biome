use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::{CssFileSource, TextRange};
use biome_diagnostics::{Diagnostic, DiagnosticExt, Severity, print_diagnostic_to_string};
use std::slice;

use biome_css_analyze::{CssAnalyzerServices, analyze};

// use this test check if your snippet produces the diagnostics you wish, without using a snapshot
#[ignore]
#[test]
fn quick_test() {
    const FILENAME: &str = "dummyFile.css";
    const SOURCE: &str = r#"
d { font: 1em SF Mono, Liberation Mono, sans-serif; }
"#;

    let parsed = parse_css(SOURCE, CssFileSource::css(), CssParserOptions::default());

    let mut error_ranges: Vec<TextRange> = Vec::new();
    let options = AnalyzerOptions::default();
    let rule_filter = RuleFilter::Rule("suspicious", "noDuplicateFontNames");
    let semantic_model = biome_css_semantic::semantic_model(&parsed.tree());
    let services = CssAnalyzerServices::default()
        .with_file_source(CssFileSource::default())
        .with_semantic_model(&semantic_model);

    analyze(
        &parsed.tree(),
        AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        },
        &options,
        services,
        &[],
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
                eprintln!("{new_code}");
            }

            ControlFlow::<Never>::Continue(())
        },
    );

    // assert_eq!(error_ranges.as_slice(), &[]);
}
