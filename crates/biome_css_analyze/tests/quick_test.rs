use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::TextRange;
use biome_diagnostics::{Diagnostic, DiagnosticExt, Severity, print_diagnostic_to_string};
use std::slice;

use biome_css_analyze::analyze;

// use this test check if your snippet produces the diagnostics you wish, without using a snapshot
#[ignore]
#[test]
fn quick_test() {
    const FILENAME: &str = "dummyFile.css";
    const SOURCE: &str = r#"
.foo {
  position-anchor: --position-anchor;
}

.bar {
  position-anchor: --position-anchor;
  position: relative;
  box-sizing: border-box;
}

.baz {
  position: relative;
  position-anchor: --position-anchor;
  box-sizing: border-box;
}

.qux {
  position: relative;
  box-sizing: border-box;
  position-anchor: --position-anchor;
}
"#;

    let parsed = parse_css(SOURCE, CssParserOptions::default());

    let mut error_ranges: Vec<TextRange> = Vec::new();
    let options = AnalyzerOptions::default();
    let rule_filter = RuleFilter::Rule("source", "useSortedProperties");

    analyze(
        &parsed.tree(),
        AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        },
        &options,
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
