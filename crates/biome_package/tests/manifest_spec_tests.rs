use biome_diagnostics::{DiagnosticExt, print_diagnostic_to_string};
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_package::{NodeJsPackage, Package};
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use std::fs::read_to_string;

mod manifest {
    tests_macros::gen_tests! {"tests/manifest/invalid/*.{json}", crate::run_invalid_manifests, "module"}
}

mod tsconfig {
    tests_macros::gen_tests! {"tests/tsconfig/invalid/**/*.{json}", crate::run_invalid_tsconfig, "module"}
    tests_macros::gen_tests! {"tests/tsconfig/valid/**/*.{json}", crate::run_valid_tsconfig, "module"}
}

fn run_invalid_manifests(input: &'static str, _: &str, _: &str, _: &str) {
    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let mut package = NodeJsPackage::default();
    match input_file.extension().map(str::as_bytes) {
        Some(b"json") => {
            let parsed = parse_json(input_code.as_str(), JsonParserOptions::default());
            package.insert_serialized_manifest(&parsed.tree(), input_file);
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
    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let mut project = NodeJsPackage::default();
    match input_file.extension().map(str::as_bytes) {
        Some(b"json") => {
            let parsed = parse_json(
                input_code.as_str(),
                JsonParserOptions::default().with_allow_comments(),
            );
            project.insert_serialized_tsconfig(&parsed.tree(), input_file);
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
    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let mut project = NodeJsPackage::default();
    match input_file.extension().map(str::as_bytes) {
        Some(b"json") => {
            let parsed = parse_json(
                input_code.as_str(),
                JsonParserOptions::default().with_allow_comments(),
            );
            project.insert_serialized_tsconfig(&parsed.tree(), input_file);
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

    let strip_prefix = |path: &mut Utf8PathBuf| {
        if path.to_string().is_empty() {
            return;
        }

        assert!(path.is_absolute());
        let mut stripped_path = Utf8PathBuf::from("<PREFIX>");
        let mut past_prefix = false;
        for component in path.components() {
            if past_prefix {
                stripped_path.push(component);
            } else if component == Utf8Component::Normal("tests") {
                past_prefix = true;
            }
        }
        *path = stripped_path;
    };

    let mut tsconfig = project.tsconfig.unwrap();
    strip_prefix(&mut tsconfig.path);
    strip_prefix(&mut tsconfig.compiler_options.paths_base);
    tsconfig
        .compiler_options
        .base_url
        .as_mut()
        .map(strip_prefix);

    snapshot_result.push_str("## Input\n\n");
    snapshot_result.push_str(&input_code);
    snapshot_result.push_str("\n\n");
    snapshot_result.push_str("## Data structure\n\n");
    snapshot_result.push_str(&format!("{tsconfig:#?}").replace("\\\\", "/"));

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot_result, file_name);
    });
}
