use biome_diagnostics::{print_diagnostic_to_string, DiagnosticExt};
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_package::{NodeJsPackage, Package};
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::Path;

mod manifest {
    tests_macros::gen_tests! {"tests/manifest/invalid/*.{json}", crate::run_invalid_manifests, "module"}
}

mod tsconfig {
    tests_macros::gen_tests! {"tests/tsconfig/invalid/**/*.{json}", crate::run_invalid_tsconfig, "module"}
    tests_macros::gen_tests! {"tests/tsconfig/valid/**/*.{json}", crate::run_valid_tsconfig, "module"}
}

fn run_invalid_manifests(input: &'static str, _: &str, _: &str, _: &str) {
    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let mut package = NodeJsPackage::default();
    match input_file.extension().map(OsStr::as_encoded_bytes) {
        Some(b"json") => {
            let parsed = parse_json(input_code.as_str(), JsonParserOptions::default());
            package.insert_serialized_manifest(&parsed.tree());
        }
        _ => {
            panic!("Extension not supported");
        }
    };

    let result = package.analyze();

    assert!(
        package.has_errors() || !result.diagnostics.is_empty(),
        "The file {input} should have diagnostics, but it doesn't have any"
    );

    let mut diagnostics_string = package
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

fn run_invalid_tsconfig(input: &'static str, _: &str, _: &str, _: &str) {
    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let mut project = NodeJsPackage::default();
    match input_file.extension().map(OsStr::as_encoded_bytes) {
        Some(b"json") => {
            let parsed = parse_json(
                input_code.as_str(),
                JsonParserOptions::default().with_allow_comments(),
            );
            project.deserialize_tsconfig(&parsed.tree());
        }
        _ => {
            panic!("Extension not supported");
        }
    };

    let result = project.analyze();

    assert!(
        project.has_errors() || !result.diagnostics.is_empty(),
        "The file {input} should have diagnostics, but it doesn't have any"
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

fn run_valid_tsconfig(input: &'static str, _: &str, _: &str, _: &str) {
    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let mut project = NodeJsPackage::default();
    match input_file.extension().map(OsStr::as_encoded_bytes) {
        Some(b"json") => {
            let parsed = parse_json(
                input_code.as_str(),
                JsonParserOptions::default().with_allow_comments(),
            );
            project.deserialize_tsconfig(&parsed.tree());
        }
        _ => {
            panic!("Extension not supported");
        }
    };

    let result = project.analyze();

    assert!(
        !project.has_errors() && result.diagnostics.is_empty(),
        "The file {input} should not have diagnostics, but it has some./"
    );

    let mut snapshot_result = String::new();

    snapshot_result.push_str("## Input\n\n");
    snapshot_result.push_str(&input_code);
    snapshot_result.push_str("\n\n");
    snapshot_result.push_str("## Data structure\n\n");
    snapshot_result.push_str(&format!("{:#?}", project.tsconfig));

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot_result, file_name);
    });
}
