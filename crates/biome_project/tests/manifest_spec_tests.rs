use biome_diagnostics::{print_diagnostic_to_string, DiagnosticExt};
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_project::{NodeJsProject, Project};
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

    let mut project = NodeJsProject::default();
    match extension {
        "json" => {
            let parsed = parse_json(input_code.as_str(), JsonParserOptions::default());
            project.from_root(&parsed.tree());
        }
        _ => {
            panic!("Extension not supported");
        }
    };

    let result = project.analyze();

    assert!(
        project.has_errors() || !result.diagnostics.is_empty(),
        "The file {} should have diagnostics, but it doesn't have any",
        input
    );

    let mut diagnostics_string = project
        .diagnostics
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

    for diagnostic in result.diagnostics {
        diagnostics_string.push_str(&print_diagnostic_to_string(
            &diagnostic
                .with_file_path(file_name)
                .with_file_source_code(input_code.as_str()),
        ));
        diagnostics_string.push_str("\n\n");
    }

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, diagnostics_string, file_name);
    });
}
