use biome_analyze::AnalyzerOptions;
use biome_analyze::{AnalysisFilter, AnalyzerAction, ControlFlow, Never, RuleFilter};
use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_diagnostics::advice::CodeSuggestionAdvice;
use biome_diagnostics::termcolor::Buffer;
use biome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic};
use biome_rowan::{AstNode, SyntaxNode};
use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::TailwindLanguage;
use biome_test_utils::{
    CheckActionType, assert_diagnostics_expectation_comment, diagnostic_to_string,
    has_bogus_nodes_or_empty_slots, parse_test_path, register_leak_checker, scripts_from_json,
    write_analyzer_snapshot,
};
use camino::Utf8Path;
use similar::TextDiff;
use std::ops::Deref;
use std::{fs::read_to_string, slice};

tests_macros::gen_tests! {"tests/specs/**/*.{tw,json,jsonc}", crate::run_test, "module"}

fn run_test(input: &'static str, _: &str, _: &str, _: &str) {
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
    if biome_tailwind_analyze::METADATA
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
    let extension = input_file.extension().unwrap_or_default();

    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    if let Some(scripts) = scripts_from_json(extension, &input_code) {
        for script in scripts {
            analyze_and_snap(
                &mut snapshot,
                &script,
                filter,
                file_name,
                input_file,
                CheckActionType::Lint,
            );
        }
    } else {
        analyze_and_snap(
            &mut snapshot,
            &input_code,
            filter,
            file_name,
            input_file,
            CheckActionType::Lint,
        );
    };

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });
}

pub(crate) fn analyze_and_snap(
    snapshot: &mut String,
    input_code: &str,
    filter: AnalysisFilter,
    file_name: &str,
    input_file: &Utf8Path,
    check_action_type: CheckActionType,
) {
    let mut diagnostics = Vec::new();
    let parsed = parse_tailwind(input_code);
    let root = parsed.tree();

    let mut code_fixes = Vec::new();
    let options = AnalyzerOptions::default().with_file_path(input_file.to_path_buf());

    let (_, errors) = biome_tailwind_analyze::analyze(&root, filter, &options, |event| {
        if let Some(mut diag) = event.diagnostic() {
            for action in event.actions() {
                if check_action_type.is_suppression() {
                    if action.is_suppression() {
                        check_code_action(input_file, input_code, &action);
                        diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                    }
                } else if !action.is_suppression() {
                    check_code_action(input_file, input_code, &action);
                    diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                }
            }

            diagnostics.push(diagnostic_to_string(file_name, input_code, diag.into()));
            return ControlFlow::Continue(());
        }

        for action in event.actions() {
            if check_action_type.is_suppression() {
                if action.category.matches("quickfix.suppressRule") {
                    check_code_action(input_file, input_code, &action);
                    code_fixes.push(code_fix_to_string(input_code, action));
                }
            } else if !action.category.matches("quickfix.suppressRule") {
                check_code_action(input_file, input_code, &action);
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
        "tw",
        parsed.diagnostics().len(),
    );

    assert_diagnostics_expectation_comment(input_file, root.syntax(), diagnostics);
}

fn check_code_action(path: &Utf8Path, source: &str, action: &AnalyzerAction<TailwindLanguage>) {
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
    let re_parse = parse_tailwind(&output);
    assert_errors_are_absent(re_parse.tree().syntax(), re_parse.diagnostics(), path);
}

fn assert_errors_are_absent(
    program: &SyntaxNode<TailwindLanguage>,
    diagnostics: &[impl Diagnostic + Clone + Send + Sync + 'static],
    path: &Utf8Path,
) {
    let debug_tree = format!("{program:?}");
    let has_missing_children = debug_tree.contains("missing (required)");

    if diagnostics.is_empty() && !has_bogus_nodes_or_empty_slots(program) && !has_missing_children {
        return;
    }

    let mut buffer = Buffer::no_color();
    for diagnostic in diagnostics {
        let error = diagnostic
            .clone()
            .with_file_path(path.as_str())
            .with_file_source_code(program.to_string());
        Formatter::new(&mut Termcolor(&mut buffer))
            .write_markup(markup! {{ PrintDiagnostic::verbose(&error) }})
            .unwrap();
    }

    panic!(
        "unexpected parser errors in {}:\n{}\n{debug_tree}",
        path,
        String::from_utf8_lossy(buffer.as_slice())
    );
}

fn code_fix_to_string(source: &str, action: AnalyzerAction<TailwindLanguage>) -> String {
    let (_, text_edit) = action.mutation.to_text_range_and_edit().unwrap_or_default();
    let output = text_edit.new_string(source);
    let diff = TextDiff::from_lines(source, &output);

    let mut diff = diff.unified_diff();
    diff.context_radius(3);
    diff.to_string()
}
