use biome_configuration::Configuration;
use biome_deserialize::json::deserialize_from_json_str;
use biome_diagnostics::{print_diagnostic_to_string, DiagnosticExt};
use biome_json_parser::JsonParserOptions;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::Path;

tests_macros::gen_tests! {"tests/invalid/**/*.{json,jsonc}", crate::run_invalid_configurations, "module"}
tests_macros::gen_tests! {"tests/valid/**/*.{json,jsonc}", crate::run_valid_configurations, "module"}

fn run_invalid_configurations(input: &'static str, _: &str, _: &str, _: &str) {
    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let result = match input_file.extension().map(OsStr::as_encoded_bytes) {
        Some(b"json") => deserialize_from_json_str::<Configuration>(
            input_code.as_str(),
            JsonParserOptions::default(),
            "",
        ),
        Some(b"jsonc") => deserialize_from_json_str::<Configuration>(
            input_code.as_str(),
            JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
            "",
        ),
        _ => {
            panic!("Extension not supported");
        }
    };

    assert!(
        result.has_errors() || result.has_warnings(),
        "This test should have diagnostics, but it doesn't have any"
    );

    let result = result
        .into_diagnostics()
        .into_iter()
        .map(|diagnostic| {
            print_diagnostic_to_string(
                &diagnostic
                    .with_file_path(file_name)
                    .with_file_source_code(input_code.as_str()),
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, result, file_name);
    });
}

fn run_valid_configurations(input: &'static str, _: &str, _: &str, _: &str) {
    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let result = match input_file.extension().map(OsStr::as_encoded_bytes) {
        Some(b"json") => deserialize_from_json_str::<Configuration>(
            input_code.as_str(),
            JsonParserOptions::default(),
            "",
        ),
        Some(b"jsonc") => deserialize_from_json_str::<Configuration>(
            input_code.as_str(),
            JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
            "",
        ),
        _ => {
            panic!("Extension not supported");
        }
    };

    let has_errors = result.has_errors();

    if !result.diagnostics().is_empty() {
        let diagnostics = result
            .into_diagnostics()
            .into_iter()
            .map(|diagnostic| {
                print_diagnostic_to_string(
                    &diagnostic
                        .with_file_path(file_name)
                        .with_file_source_code(input_code.as_str()),
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");
        if has_errors {
            panic!(
                "This test should not have diagnostics, but some have been emitted.\n {diagnostics}"
            );
        }
    } else {
        // test passed, no errors
    }
}

#[ignore]
#[test]
fn quick_test() {
    let source = r#"{
        "javascript": {
            "formatter": {
                "overrides": [
                {}]
            }
        }
    }"#;
    let result =
        deserialize_from_json_str::<Configuration>(source, JsonParserOptions::default(), "");

    dbg!(result.diagnostics());
    assert!(!result.has_errors());
}
