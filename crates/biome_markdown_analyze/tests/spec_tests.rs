use biome_analyze::{
    ActionFilter, AnalysisFilter, AnalyzerAction, ControlFlow, Never, Queryable, RegistryVisitor,
    Rule, RuleDomain, RuleFilter, RuleGroup,
};
use biome_diagnostics::advice::CodeSuggestionAdvice;
use biome_markdown_parser::parse_markdown;
use biome_markdown_syntax::MarkdownLanguage;
use biome_rowan::AstNode;
use biome_test_utils::{
    CheckActionType, assert_diagnostics_expectation_comment, assert_errors_are_absent,
    code_fix_to_string, create_analyzer_options, diagnostic_to_string,
    has_bogus_nodes_or_empty_slots, parse_test_path, register_leak_checker, scripts_from_json,
    write_analyzer_snapshot,
};
use camino::Utf8Path;
use std::ops::Deref;
use std::{fs::read_to_string, slice};

tests_macros::gen_tests! {"tests/specs/**/*.{md,json,jsonc}", crate::run_test, "module"}
tests_macros::gen_tests! {"tests/suppression/**/*.{md,json,jsonc}", crate::run_suppression_test, "module"}

/// Checks if any of the enabled rules is in the project domain and requires the module graph.
struct NeedsModuleGraph<'a> {
    enabled_rules: Option<&'a [RuleFilter<'a>]>,
    needs_module_graph: bool,
}

impl<'a> NeedsModuleGraph<'a> {
    fn new(enabled_rules: Option<&'a [RuleFilter<'a>]>) -> Self {
        Self {
            enabled_rules,
            needs_module_graph: false,
        }
    }

    fn compute(mut self) -> bool {
        biome_markdown_analyze::visit_registry(&mut self);
        self.needs_module_graph
    }
}

impl RegistryVisitor<MarkdownLanguage> for NeedsModuleGraph<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = MarkdownLanguage, Output: Clone>>
            + 'static,
    {
        let filter = RuleFilter::Rule(<R::Group as RuleGroup>::NAME, R::METADATA.name);

        if self
            .enabled_rules
            .is_some_and(|enabled_rules| enabled_rules.contains(&filter))
            && R::METADATA.domains.contains(&RuleDomain::Project)
        {
            self.needs_module_graph = true;
        }
    }
}

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
    if biome_markdown_analyze::METADATA
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
    let working_directory = input_file.parent().unwrap_or(input_file);
    let options = create_analyzer_options::<MarkdownLanguage>(
        input_file,
        working_directory,
        &mut diagnostics,
    );

    let parsed = parse_markdown(input_code);
    let root = parsed.tree();

    let mut code_fixes = Vec::new();

    let needs_module_graph = NeedsModuleGraph::new(filter.enabled_rules).compute();

    let (_, errors) = biome_markdown_analyze::analyze(&root, filter, &options, |event| {
        if let Some(mut diag) = event.diagnostic() {
            for action in event.actions(ActionFilter::all()) {
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

        for action in event.actions(ActionFilter::all()) {
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
        "md",
        parsed.diagnostics().len(),
    );

    if needs_module_graph {
        // Normalize Windows paths.
        *snapshot = snapshot.replace('\\', "/");
    }

    assert_diagnostics_expectation_comment(input_file, root.syntax(), diagnostics);
}

fn check_code_action(path: &Utf8Path, source: &str, action: &AnalyzerAction<MarkdownLanguage>) {
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
    let re_parse = parse_markdown(&output);
    assert_errors_are_absent(re_parse.tree().syntax(), re_parse.diagnostics(), path);
}

#[expect(unused)]
pub(crate) fn run_suppression_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();
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
        filter,
        file_name,
        input_file,
        CheckActionType::Suppression,
    );

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });
}
