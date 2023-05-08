use rome_deserialize::json::deserialize_from_json_str;
use rome_diagnostics::{print_diagnostic_to_string, DiagnosticExt};
use rome_project::Manifest;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::Path;
tests_macros::gen_tests! {"tests/invalid/*.{json}", crate::run_invalid_configurations, "module"}

fn run_invalid_configurations(input: &'static str, _: &str, _: &str, _: &str) {
    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let extension = input_file.extension().and_then(OsStr::to_str).unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

    let result = match extension {
        "json" => deserialize_from_json_str::<Manifest>(input_code.as_str()),
        _ => {
            panic!("Extension not supported");
        }
    };

    assert!(
        result.has_errors(),
        "This test should have diagnostics, but it doesn't have any"
    );

    let result = result
        .into_error()
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
