use biome_analyze::{AnalysisFilter, AnalyzerAction, ControlFlow, Never, RuleFilter};
use biome_diagnostics::advice::CodeSuggestionAdvice;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_syntax::{JsonFileSource, JsonLanguage};
use biome_rowan::AstNode;
use biome_test_utils::{
    CheckActionType, assert_diagnostics_expectation_comment, assert_errors_are_absent,
    code_fix_to_string, create_analyzer_options, diagnostic_to_string,
    has_bogus_nodes_or_empty_slots, parse_test_path, register_leak_checker,
    write_analyzer_snapshot,
};
use camino::Utf8Path;
use std::ops::Deref;
use std::{fs::read_to_string, slice};

tests_macros::gen_tests! {"tests/specs/**/*.{json,jsonc}", crate::run_test, "module"}
tests_macros::gen_tests! {"tests/suppression/**/*.{json,jsonc}", crate::run_suppression_test, "module"}

fn run_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();

    let parser_options = match input_file.extension() {
        Some("json") => JsonParserOptions::default(),
        Some("jsonc") => JsonParserOptions::default()
            .with_allow_comments()
            .with_allow_trailing_commas(),

        _ => {
            panic!("Unknown file extension: {:?}", input_file.extension());
        }
    };

    let (group, rule) = parse_test_path(input_file);
    if rule == "specs" || rule == "suppression" {
        panic!("the test file must be placed in the {rule}/<group-name>/<rule-name>/ directory");
    }
    if group == "specs" || group == "suppression" {
        panic!("the test file must be placed in the {group}/{rule}/<rule-name>/ directory");
    }

    if biome_json_analyze::METADATA
        .deref()
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

    let mut snapshot = String::new();

    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let Ok(file_source) = input_file.try_into() else {
        return;
    };

    analyze_and_snap(
        &mut snapshot,
        &input_code,
        file_source,
        filter,
        file_name,
        input_file,
        CheckActionType::Lint,
        parser_options,
    );

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });
}

fn run_suppression_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();
    let (source_type, parser_options) = match input_file.extension() {
        Some("json") => (JsonFileSource::json(), JsonParserOptions::default()),
        Some("jsonc") => (
            JsonFileSource::json_allow_comments_and_trailing_commas("jsonc"),
            JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
        ),
        _ => {
            panic!("Unknown file extension: {:?}", input_file.extension());
        }
    };
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let (group, rule) = parse_test_path(input_file);

    let rule_filter = RuleFilter::Rule(group, rule);
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let mut snapshot = String::new();
    analyze_and_snap(
        &mut snapshot,
        &input_code,
        source_type,
        filter,
        file_name,
        input_file,
        CheckActionType::Suppression,
        parser_options,
    );

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });
}

#[expect(clippy::too_many_arguments)]
pub(crate) fn analyze_and_snap(
    snapshot: &mut String,
    input_code: &str,
    file_source: JsonFileSource,
    filter: AnalysisFilter,
    file_name: &str,
    input_file: &Utf8Path,
    action_type: CheckActionType,
    parser_options: JsonParserOptions,
) {
    let parsed = parse_json(input_code, parser_options);
    let root = parsed.tree();

    let mut diagnostics = Vec::new();
    let mut code_fixes = Vec::new();
    let options = create_analyzer_options(input_file, &mut diagnostics);

    let (_, errors) = biome_json_analyze::analyze(&root, filter, &options, file_source, |event| {
        if let Some(mut diag) = event.diagnostic() {
            for action in event.actions() {
                if action.is_suppression() {
                    if action_type.is_suppression() {
                        check_code_action(input_file, input_code, &action, parser_options);
                        diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                    }
                } else if !action.is_suppression() {
                    check_code_action(input_file, input_code, &action, parser_options);
                    diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                }
            }

            diagnostics.push(diagnostic_to_string(file_name, input_code, diag.into()));
            return ControlFlow::Continue(());
        }

        for action in event.actions() {
            if !action.is_suppression() {
                check_code_action(input_file, input_code, &action, parser_options);
                code_fixes.push(code_fix_to_string(input_code, action));
            }
        }

        ControlFlow::<Never>::Continue(())
    });

    for error in errors {
        diagnostics.push(diagnostic_to_string(file_name, input_code, error));
    }
    let langauge = format!("{}", file_source.variant());
    write_analyzer_snapshot(
        snapshot,
        input_code,
        diagnostics.as_slice(),
        code_fixes.as_slice(),
        langauge.as_str(),
    );

    assert_diagnostics_expectation_comment(input_file, root.syntax(), diagnostics);
}

fn check_code_action(
    path: &Utf8Path,
    source: &str,
    action: &AnalyzerAction<JsonLanguage>,
    parser_options: JsonParserOptions,
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
    let re_parse = parse_json(&output, parser_options);
    assert_errors_are_absent(re_parse.tree().syntax(), re_parse.diagnostics(), path);
}
