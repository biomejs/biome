//! Checks that walking only the node kinds the visitors read gives the same
//! signals as walking every node, over the analyzer and parser fixtures.

use crate::{AnalysisFilter, ControlFlow, CssAnalyzerServices, analyze};
use biome_analyze::{ActionFilter, AnalyzerOptions, Never, RuleFilter};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_diagnostics::{Diagnostic, DiagnosticExt, print_diagnostic_to_string};
use biome_languages::CssFileSource;
use camino::{Utf8Path, Utf8PathBuf};

fn collect_sources(dir: &Utf8Path, sources: &mut Vec<Utf8PathBuf>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| Utf8PathBuf::try_from(entry.unwrap().path()).unwrap())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_sources(&path, sources);
        } else if matches!(path.extension(), Some("css" | "scss")) {
            sources.push(path);
        }
    }
}

/// Returns what the analyzer emits for `filter`, in emission order.
fn signals(
    text: &str,
    source_type: CssFileSource,
    path: &Utf8Path,
    filter: AnalysisFilter,
    walk_every_node: bool,
) -> Vec<String> {
    let parsed = parse_css(text, source_type, CssParserOptions::default());
    let mut options = AnalyzerOptions::default().with_file_path(path);
    if walk_every_node {
        options = options.with_walk_every_node();
    }
    let services = CssAnalyzerServices::default().with_file_source(source_type);
    let mut signals = Vec::new();
    let (_, errors) = analyze(&parsed.tree(), filter, &options, services, &[], |signal| {
        if let Some(diagnostic) = signal.diagnostic() {
            let span = diagnostic.location().span;
            let printed = print_diagnostic_to_string(&diagnostic.with_file_path(path.as_str()));
            signals.push(format!("{span:?} {printed}"));
        }
        for action in signal.actions(ActionFilter::all()) {
            if action.is_suppression() {
                continue;
            }
            let edit = action.mutation.to_text_range_and_edit();
            signals.push(format!(
                "action {:?} {:?} {edit:?}",
                action.rule_name, action.category
            ));
        }
        ControlFlow::<Never>::Continue(())
    });
    assert!(errors.is_empty(), "{path}: {errors:?}");
    signals
}

#[test]
fn kind_filtered_walk_emits_the_signals_of_a_full_walk() {
    let manifest = Utf8Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut sources = Vec::new();
    collect_sources(&manifest.join("tests/specs"), &mut sources);
    collect_sources(
        &manifest.join("../biome_css_parser/tests/css_test_suite"),
        &mut sources,
    );
    assert!(sources.len() > 500);

    let shorthand = [RuleFilter::Rule(
        "suspicious",
        "noShorthandPropertyOverrides",
    )];
    let duplicate_properties = [RuleFilter::Rule("suspicious", "noDuplicateProperties")];
    let rule_sets: [Option<&[RuleFilter]>; 3] =
        [Some(&shorthand), Some(&duplicate_properties), None];

    let mut compared = 0;
    for path in &sources {
        let Ok(source_type) = CssFileSource::try_from(path.as_path()) else {
            continue;
        };
        let text = std::fs::read_to_string(path).unwrap();
        for enabled_rules in rule_sets {
            let filter = AnalysisFilter {
                enabled_rules,
                ..AnalysisFilter::default()
            };
            let filtered = signals(&text, source_type, path, filter, false);
            let full = signals(&text, source_type, path, filter, true);
            assert_eq!(filtered, full, "{path} with rules {enabled_rules:?}");
            compared += full.len();
        }
    }
    assert!(compared > 500);
}
