use biome_analyze::{AnalysisFilter, AnalyzerTransformation, ControlFlow, Never, RuleFilter};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsLanguage;
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_test_utils::{
    assert_diagnostics_expectation_comment, assert_errors_are_absent, create_analyzer_options,
    diagnostic_to_string, has_bogus_nodes_or_empty_slots, register_leak_checker, scripts_from_json,
    write_transformation_snapshot,
};

use camino::Utf8Path;
use std::ops::Deref;
use std::{fs::read_to_string, slice};

tests_macros::gen_tests! {"tests/specs/**/*.{cjs,js,jsx,tsx,ts,json,jsonc}", crate::run_test, "module"}

fn run_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();

    let rule_folder = input_file.parent().unwrap();
    let rule = rule_folder.file_name().unwrap();

    if rule == "specs" {
        panic!("the test file must be placed in the {rule}/<group-name>/<rule-name>/ directory");
    }
    if biome_js_transform::METADATA
        .deref()
        .find_rule("transformations", rule)
        .is_none()
    {
        panic!("could not find rule transformations/{rule}");
    }

    let rule_filter = RuleFilter::Rule("transformations", rule);
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
                JsFileSource::js_script(),
                filter,
                file_name,
                input_file,
                JsParserOptions::default(),
            );
        }
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
            JsParserOptions::default(),
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
    source_type: JsFileSource,
    filter: AnalysisFilter,
    file_name: &str,
    input_file: &Utf8Path,
    parser_options: JsParserOptions,
) {
    let parsed = parse(input_code, source_type, parser_options);
    let root = parsed.tree();

    let mut diagnostics = Vec::new();
    // Use the parent directory as a working directory for relative paths in diagnostics
    let working_directory = input_file.parent().unwrap_or(Utf8Path::new("."));
    let options =
        create_analyzer_options::<JsLanguage>(input_file, working_directory, &mut diagnostics);

    let mut transformations = vec![];
    let (_, errors) =
        biome_js_transform::transform(&root, filter, &options, source_type, |event| {
            if let Some(diagnostic) = event.diagnostic() {
                diagnostics.push(diagnostic_to_string(
                    file_name,
                    input_code,
                    diagnostic.into(),
                ));
            }
            for transformation in event.transformations() {
                check_transformation(
                    input_file,
                    input_code,
                    source_type,
                    &transformation,
                    parser_options,
                );

                // Snapshot the raw output of the transformation without formatting it, so
                // text-level properties (like the whitespace replacement of `stripTypes`
                // preserving positions) stay visible.
                transformations.push(transformation.mutation.commit().to_string());
            }
            ControlFlow::<Never>::Continue(())
        });

    for error in errors {
        diagnostics.push(diagnostic_to_string(file_name, input_code, error));
    }

    // When every transformation preserves the text length (like the whitespace replacement of
    // `stripTypes`), their outputs can be merged bytewise into the final result.
    let final_output = (!transformations.is_empty()
        && transformations
            .iter()
            .all(|transformation| transformation.len() == input_code.len()))
    .then(|| {
        let mut merged = input_code.as_bytes().to_vec();
        for transformation in &transformations {
            for (index, byte) in transformation.bytes().enumerate() {
                if byte != input_code.as_bytes()[index] {
                    merged[index] = byte;
                }
            }
        }
        String::from_utf8(merged).expect("merging transformations must keep the text valid UTF-8")
    });

    if let Some(final_output) = &final_output {
        let re_parse = parse(final_output, source_type, parser_options);
        assert_errors_are_absent(re_parse.tree().syntax(), re_parse.diagnostics(), input_file);
    }

    write_transformation_snapshot(
        snapshot,
        input_code,
        transformations.as_slice(),
        final_output.as_deref(),
        diagnostics.as_slice(),
        source_type.file_extension(),
    );

    assert_diagnostics_expectation_comment(input_file, root.syntax(), diagnostics);
}

fn check_transformation(
    path: &Utf8Path,
    source: &str,
    source_type: JsFileSource,
    transformation: &AnalyzerTransformation<JsLanguage>,
    options: JsParserOptions,
) {
    let (new_tree, text_edit) = match transformation
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
    let re_parse = parse(&output, source_type, options);
    assert_errors_are_absent(re_parse.tree().syntax(), re_parse.diagnostics(), path);
}
