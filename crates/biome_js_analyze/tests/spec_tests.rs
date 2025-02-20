use biome_analyze::{
    AnalysisFilter, AnalyzerAction, AnalyzerPluginSlice, ControlFlow, Never, RuleFilter,
};
use biome_diagnostics::advice::CodeSuggestionAdvice;
use biome_fs::OsFileSystem;
use biome_js_analyze::JsAnalyzerServices;
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{JsFileSource, JsLanguage, ModuleKind};
use biome_package::PackageType;
use biome_plugin_loader::AnalyzerGritPlugin;
use biome_rowan::AstNode;
use biome_test_utils::{
    assert_errors_are_absent, code_fix_to_string, create_analyzer_options,
    dependency_graph_for_test_file, diagnostic_to_string, has_bogus_nodes_or_empty_slots,
    parse_test_path, project_layout_with_node_manifest, register_leak_checker, scripts_from_json,
    write_analyzer_snapshot, CheckActionType,
};
use camino::{Utf8Component, Utf8Path};
use std::ops::Deref;
use std::sync::Arc;
use std::{fs::read_to_string, slice};

tests_macros::gen_tests! {"tests/specs/**/*.{cjs,cts,js,jsx,tsx,ts,json,jsonc,svelte}", crate::run_test, "module"}
tests_macros::gen_tests! {"tests/suppression/**/*.{cjs,cts,js,jsx,tsx,ts,json,jsonc,svelte}", crate::run_suppression_test, "module"}
tests_macros::gen_tests! {"tests/plugin/*.grit", crate::run_plugin_test, "module"}

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
    if biome_js_analyze::METADATA
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
    let quantity_diagnostics = if let Some(scripts) = scripts_from_json(extension, &input_code) {
        for script in scripts {
            analyze_and_snap(
                &mut snapshot,
                &script,
                JsFileSource::js_script(),
                filter,
                file_name,
                input_file,
                CheckActionType::Lint,
                JsParserOptions::default(),
                &[],
            );
        }

        0
    } else {
        let Ok(source_type) = input_file.try_into() else {
            return;
        };
        analyze_and_snap(
            &mut snapshot,
            &input_code,
            source_type,
            filter,
            file_name,
            input_file,
            CheckActionType::Lint,
            JsParserOptions::default(),
            &[],
        )
    };

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

#[expect(clippy::too_many_arguments)]
pub(crate) fn analyze_and_snap(
    snapshot: &mut String,
    input_code: &str,
    mut source_type: JsFileSource,
    filter: AnalysisFilter,
    file_name: &str,
    input_file: &Utf8Path,
    check_action_type: CheckActionType,
    parser_options: JsParserOptions,
    plugins: AnalyzerPluginSlice,
) -> usize {
    let mut diagnostics = Vec::new();
    let mut code_fixes = Vec::new();
    let project_layout = project_layout_with_node_manifest(input_file, &mut diagnostics);

    if let Some((_, manifest)) = project_layout.get_node_manifest_for_path(input_file) {
        if manifest.r#type == Some(PackageType::CommonJs) &&
            // At the moment we treat JS and JSX at the same way
            (source_type.file_extension() == "js" || source_type.file_extension() == "jsx" )
        {
            source_type.set_module_kind(ModuleKind::Script)
        }
    }

    let parsed = parse(input_code, source_type, parser_options.clone());
    let root = parsed.tree();

    let options = create_analyzer_options(input_file, &mut diagnostics);

    // FIXME: We probably want to enable it for all rules? Right now it seems to
    //        trigger a leak panic...
    let dependency_graph = if input_file
        .components()
        .any(|component| component == Utf8Component::Normal("noImportCycles"))
    {
        dependency_graph_for_test_file(input_file, &project_layout)
    } else {
        Default::default()
    };

    let services = JsAnalyzerServices::from((dependency_graph, project_layout, source_type));

    let (_, errors) =
        biome_js_analyze::analyze(&root, filter, &options, plugins, services, |event| {
            if let Some(mut diag) = event.diagnostic() {
                for action in event.actions() {
                    if check_action_type.is_suppression() {
                        if action.is_suppression() {
                            check_code_action(
                                input_file,
                                input_code,
                                source_type,
                                &action,
                                parser_options.clone(),
                            );
                            diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                        }
                    } else if !action.is_suppression() {
                        check_code_action(
                            input_file,
                            input_code,
                            source_type,
                            &action,
                            parser_options.clone(),
                        );
                        diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                    }
                }

                diagnostics.push(diagnostic_to_string(file_name, input_code, diag.into()));
                return ControlFlow::Continue(());
            }

            for action in event.actions() {
                if check_action_type.is_suppression() {
                    if action.category.matches("quickfix.suppressRule") {
                        check_code_action(
                            input_file,
                            input_code,
                            source_type,
                            &action,
                            parser_options.clone(),
                        );
                        code_fixes.push(code_fix_to_string(input_code, action));
                    }
                } else if !action.category.matches("quickfix.suppressRule") {
                    check_code_action(
                        input_file,
                        input_code,
                        source_type,
                        &action,
                        parser_options.clone(),
                    );
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
        source_type.file_extension(),
    );

    // FIXME: I wish we could do this more generically, but we cannot do this
    //        for all tests, since it would cause many incorrect replacements.
    //        Maybe there's a regular expression that could work, but it feels
    //        flimsy too...
    if input_file
        .components()
        .any(|component| component == Utf8Component::Normal("noImportCycles"))
    {
        // Normalize Windows paths.
        *snapshot = snapshot.replace('\\', "/");
    }

    diagnostics.len()
}

fn check_code_action(
    path: &Utf8Path,
    source: &str,
    source_type: JsFileSource,
    action: &AnalyzerAction<JsLanguage>,
    options: JsParserOptions,
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
    assert_eq!(
        new_tree.to_string(),
        output,
        "Code action and syntax tree differ"
    );

    if has_bogus_nodes_or_empty_slots(&new_tree) {
        panic!("modified tree has bogus nodes or empty slots:\n{new_tree:#?} \n\n {new_tree}")
    }

    // Checks the returned tree contains no missing children node
    if format!("{new_tree:?}").contains("missing (required)") {
        panic!("modified tree has missing children:\n{new_tree:#?}")
    }

    // Re-parse the modified code and panic if the resulting tree has syntax errors
    let re_parse = parse(&output, source_type, options);
    assert_errors_are_absent(re_parse.tree().syntax(), re_parse.diagnostics(), path);
}

pub(crate) fn run_suppression_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();
    let source_type = match input_file.extension() {
        Some("js" | "mjs" | "jsx") => JsFileSource::jsx(),
        Some("cjs") => JsFileSource::js_script(),
        Some("ts") => JsFileSource::ts(),
        Some("mts" | "cts") => JsFileSource::ts_restricted(),
        Some("tsx") => JsFileSource::tsx(),
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
        JsParserOptions::default(),
        &[],
    );

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });
}

fn run_plugin_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let plugin_path = Utf8Path::new(input);
    let file_name = plugin_path.file_name().unwrap();
    let input_path = plugin_path.with_extension("js");

    let plugin = match AnalyzerGritPlugin::load(
        &OsFileSystem::new(plugin_path.to_owned()),
        Utf8Path::new(plugin_path),
    ) {
        Ok(plugin) => plugin,
        Err(err) => panic!("Cannot load plugin: {err:?}"),
    };

    let filter = AnalysisFilter {
        enabled_rules: Some(&[]),
        ..AnalysisFilter::default()
    };

    let mut snapshot = String::new();

    let input_code = read_to_string(&input_path)
        .unwrap_or_else(|err| panic!("failed to read {input_path:?}: {err:?}"));
    let Ok(source_type) = input_path.as_path().try_into() else {
        return;
    };
    analyze_and_snap(
        &mut snapshot,
        &input_code,
        source_type,
        filter,
        file_name,
        &input_path,
        CheckActionType::Lint,
        JsParserOptions::default(),
        &[Arc::new(Box::new(plugin))],
    );

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => plugin_path.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });
}
