use biome_analyze::{AnalysisFilter, ControlFlow, Never, RuleFilter};
use biome_diagnostics::advice::CodeSuggestionAdvice;
use biome_diagnostics::{DiagnosticExt, Severity};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::JsFileSource;
use biome_test_utils::{
    code_fix_to_string, create_analyzer_options, diagnostic_to_string, load_manifest,
    parse_test_path, scripts_from_json,
};
use std::{ffi::OsStr, fs::read_to_string, path::Path, slice};

// use this test check if your snippet produces the diagnostics you wish, without using a snapshot
#[ignore]
#[test]
fn run_test() {
    let input_file =
        Path::new("tests/specs/correctness/useExhaustiveDependencies/ignoredDependencies.js");
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();

    let (group, rule) = parse_test_path(input_file);
    if rule == "specs" || rule == "suppression" {
        panic!("the test file must be placed in the {rule}/<group-name>/<rule-name>/ directory");
    }
    if group == "specs" || group == "suppression" {
        panic!("the test file must be placed in the {group}/{rule}/<rule-name>/ directory");
    }
    if biome_js_analyze::metadata()
        .find_rule(group, rule)
        .is_none()
    {
        panic!("could not find rule {group}/{rule}");
    }

    let rule_filter = RuleFilter::Rule(group, rule);
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let extension = input_file.extension().unwrap_or_default();

    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));
    if let Some(scripts) = scripts_from_json(extension, &input_code) {
        for script in scripts {
            analyze(
                &script,
                JsFileSource::js_script(),
                filter,
                file_name,
                input_file,
            );
        }
    } else if let Ok(source_type) = input_file.try_into() {
        analyze(&input_code, source_type, filter, file_name, input_file);
    }
}

fn analyze(
    input_code: &str,
    source_type: JsFileSource,
    filter: AnalysisFilter,
    file_name: &str,
    input_file: &Path,
) {
    let parsed = parse(input_code, source_type, JsParserOptions::default());
    let root = parsed.tree();

    let mut diagnostics = Vec::new();
    let mut code_fixes = Vec::new();
    let options = create_analyzer_options(input_file, &mut diagnostics);
    let manifest = load_manifest(input_file, &mut diagnostics);

    let (_, errors) =
        biome_js_analyze::analyze(&root, filter, &options, source_type, manifest, |event| {
            if let Some(mut diag) = event.diagnostic() {
                for action in event.actions() {
                    diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                }

                let error = diag.with_severity(Severity::Warning);
                diagnostics.push(diagnostic_to_string(file_name, input_code, error));
                return ControlFlow::Continue(());
            }

            for action in event.actions() {
                code_fixes.push(code_fix_to_string(input_code, action));
            }

            ControlFlow::<Never>::Continue(())
        });

    for error in errors {
        diagnostics.push(diagnostic_to_string(file_name, input_code, error));
    }

    println!("Diagnostics:\n{}", diagnostics.join("\n"));
}
