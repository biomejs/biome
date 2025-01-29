use biome_analyze::{AnalysisFilter, AnalyzerAction, ControlFlow, Never, RuleFilter};
use biome_diagnostics::advice::CodeSuggestionAdvice;
use biome_diagnostics::{DiagnosticExt, Severity};
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_json_syntax::JsonLanguage;
use biome_rowan::AstNode;
use biome_test_utils::{
    assert_errors_are_absent, code_fix_to_string, diagnostic_to_string,
    has_bogus_nodes_or_empty_slots, parse_test_path, register_leak_checker,
    write_analyzer_snapshot,
};
use camino::Utf8Path;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::{env, slice};

tests_macros::gen_tests! {"tests/specs/**/*.{json,jsonc}", crate::run_test, "module"}

fn run_test(input: &'static str, _: &str, directory_path: &str, _: &str) {
    register_leak_checker();

    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();

    let (group, rule) = parse_test_path(input_file);
    if rule == "specs" || rule == "suppression" {
        panic!("the test file must be placed in the {rule}/<group-name>/<rule-name>/ directory");
    }
    if group == "specs" || group == "suppression" {
        panic!("the test file must be placed in the {group}/{rule}/<rule-name>/ directory");
    }

    let mut snapshot = String::new();

    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let quantity_diagnostics = analyze_and_snap(
        &mut snapshot,
        &input_code,
        file_name,
        input_file,
        PathBuf::from(directory_path),
    );

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });

    if input_code.contains("/* should not generate diagnostics */") && quantity_diagnostics > 0 {
        panic!("This test should not generate diagnostics");
    }
}

pub(crate) fn analyze_and_snap(
    snapshot: &mut String,
    input_code: &str,
    file_name: &str,
    input_file: &Utf8Path,
    directory_path: PathBuf,
) -> usize {
    let parse_options = if file_name.ends_with(".jsonc") {
        JsonParserOptions::default()
            .with_allow_comments()
            .with_allow_trailing_commas()
    } else {
        JsonParserOptions::default()
    };
    let parsed = parse_json(input_code, parse_options);
    let root = parsed.tree();

    let mut diagnostics = Vec::new();
    let mut code_fixes = Vec::new();
    let rule_name = directory_path.file_name().unwrap().to_str().unwrap();
    let rule_filter = RuleFilter::Rule("migrations", rule_name);
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..Default::default()
    };
    let (_, errors) = biome_migrate::migrate_configuration(&root, filter, input_file, |event| {
        if let Some(mut diag) = event.diagnostic() {
            for action in event.actions() {
                if !action.is_suppression() {
                    check_code_action(input_file, input_code, &action, parse_options);
                    diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                }
            }

            let error = diag.with_severity(Severity::Warning);
            diagnostics.push(diagnostic_to_string(file_name, input_code, error));
            return ControlFlow::Continue(());
        }

        for action in event.actions() {
            if !action.is_suppression() {
                check_code_action(input_file, input_code, &action, parse_options);
                code_fixes.push(code_fix_to_string(input_code, action));
            }
        }

        ControlFlow::<Never>::Continue(())
    });

    for error in errors {
        diagnostics.push(diagnostic_to_string(file_name, input_code, error));
    }
    write_analyzer_snapshot(
        snapshot,
        input_code,
        diagnostics.as_slice(),
        code_fixes.as_slice(),
        "json",
    );

    diagnostics.len()
}

fn check_code_action(
    path: &Utf8Path,
    source: &str,
    action: &AnalyzerAction<JsonLanguage>,
    parse_options: JsonParserOptions,
) {
    let (new_tree, text_edit) = match action
        .mutation
        .clone()
        .commit_with_text_range_and_edit(true)
    {
        (new_tree, Some((_, text_edit))) => (new_tree, text_edit),
        (new_tree, None) => (new_tree, Default::default()),
    };

    let output = text_edit.new_string(source);

    // Checks that applying the text edits returned by the BatchMutation
    // returns the same code as printing the modified syntax tree
    assert_eq!(new_tree.to_string(), output);

    if has_bogus_nodes_or_empty_slots(&new_tree) {
        panic!("modified tree has bogus nodes or empty slots:\n{new_tree:#?} \n\n {new_tree}")
    }

    // Checks the returned tree contains no missing children node
    if format!("{new_tree:?}").contains("missing (required)") {
        panic!("modified tree has missing children:\n{new_tree:#?}")
    }

    // Re-parse the modified code and panic if the resulting tree has syntax errors
    let re_parse = parse_json(&output, parse_options);
    assert_errors_are_absent(re_parse.tree().syntax(), re_parse.diagnostics(), path);
}
