use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_formatter::LineWidth;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn extends_config_ok_formatter_no_linter() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{ "extends": ["format.json", "linter.json"] }"#,
    );
    let format = Utf8Path::new("format.json");
    fs.insert(
        format.into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    let lint = Utf8Path::new("linter.json");
    fs.insert(lint.into(), r#"{ "linter": { "enabled": false } }"#);

    let test_file = Utf8Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extends_config_ok_formatter_no_linter",
        fs,
        console,
        result,
    ));
}

#[test]
fn extends_config_ok_from_npm_package() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{ "extends": ["@shared/format/biome", "@shared/linter/biome"] }"#,
    );

    fs.insert(
        "node_modules/@shared/format/biome.json".into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    fs.insert(
        "node_modules/@shared/format/package.json".into(),
        r#"{
    "name": "@shared/format",
    "exports": {
        "./biome": "./biome.json"
    }
}"#,
    );

    fs.insert(
        "node_modules/@shared/linter/biome.jsonc".into(),
        r#"{ "linter": { "enabled": false, } }"#,
    );
    fs.insert(
        "node_modules/@shared/linter/package.json".into(),
        r#"{
    "name": "@shared/linter",
    "exports": {
        "./biome": "./biome.jsonc"
    }
}"#,
    );

    let test_file = Utf8Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extends_config_ok_from_npm_package",
        fs,
        console,
        result,
    ));
}

// See: https://github.com/biomejs/biome/issues/6217
#[test]
fn extends_config_ok_from_npm_package_with_author_field() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{ "extends": ["@shared/format/biome", "@shared/linter/biome"] }"#,
    );

    fs.insert(
        "node_modules/@shared/format/biome.json".into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    fs.insert(
        "node_modules/@shared/format/package.json".into(),
        r#"{
    "name": "@shared/format",
    "author": "this breaks extending biome",
    "exports": {
        "./biome": "./biome.json"
    }
}"#,
    );

    fs.insert(
        "node_modules/@shared/linter/biome.jsonc".into(),
        r#"{ "linter": { "enabled": false, } }"#,
    );
    fs.insert(
        "node_modules/@shared/linter/package.json".into(),
        r#"{
    "name": "@shared/linter",
    "exports": {
        "./biome": "./biome.jsonc"
    }
}"#,
    );

    let test_file = Utf8Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extends_config_ok_from_npm_package_with_author_field",
        fs,
        console,
        result,
    ));
}

#[test]
fn extends_config_ok_from_npm_package_with_condition_names() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{ "extends": ["@shared/format", "@shared/linter/biome"] }"#,
    );

    fs.insert(
        "node_modules/@shared/format/biome.json".into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    fs.insert(
        "node_modules/@shared/format/package.json".into(),
        r#"{
    "name": "@shared/format",
    "exports": {
        ".": {
            "biome": "./biome.json"
        }
    }
}"#,
    );

    fs.insert(
        "node_modules/@shared/linter/biome.jsonc".into(),
        r#"{ "linter": { "enabled": false, } }"#,
    );
    fs.insert(
        "node_modules/@shared/linter/package.json".into(),
        r#"{
    "name": "@shared/linter",
    "exports": {
        "./biome": {
            "default": "./biome.jsonc"
        }
    }
}"#,
    );

    let test_file = Utf8Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extends_config_ok_from_npm_package_with_condition_names",
        fs,
        console,
        result,
    ));
}

#[test]
fn extends_config_ok_linter_not_formatter() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{ "extends": ["format.json", "linter.json"] }"#,
    );
    let format = Utf8Path::new("format.json");
    fs.insert(format.into(), r#"{ "formatter": { "enabled": true } }"#);
    let lint = Utf8Path::new("linter.json");
    fs.insert(
        lint.into(),
        r#"{
  "linter": {
    "rules": {
      "suspicious": {
        "noDebugger": "warn"
      }
    }
  }
}
        "#,
    );

    let test_file = Utf8Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extends_config_ok_linter_not_formatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn extends_should_raise_an_error_for_unresolved_configuration() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{ "extends": ["formatTYPO.json", "linter.json"] }"#,
    );
    let format = Utf8Path::new("format.json");
    fs.insert(
        format.into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    let lint = Utf8Path::new("linter.json");
    fs.insert(lint.into(), r#"{ "linter": { "enabled": false } }"#);

    let test_file = Utf8Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extends_should_raise_an_error_for_unresolved_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn extends_should_raise_an_error_for_unresolved_configuration_and_show_verbose() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{ "extends": ["formatTYPO.json", "linter.json"] }"#,
    );
    let format = Utf8Path::new("format.json");
    fs.insert(
        format.into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    let lint = Utf8Path::new("linter.json");
    fs.insert(lint.into(), r#"{ "linter": { "enabled": false } }"#);

    let test_file = Utf8Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--verbose", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extends_should_raise_an_error_for_unresolved_configuration_and_show_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn extends_resolves_when_using_config_path() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("config/biome.json");
    fs.insert(
        biome_json.into(),
        r#"{ "extends": ["format.json", "linter.json"] }"#,
    );
    let format = Utf8Path::new("config/format.json");
    fs.insert(
        format.into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    let lint = Utf8Path::new("config/linter.json");
    fs.insert(lint.into(), r#"{ "linter": { "enabled": true } }"#);

    let test_file = Utf8Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--config-path=config/", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extends_resolves_when_using_config_path",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_extended_values_in_current_config() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let format = Utf8Path::new("format.json");
    fs.insert(
        format.into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{ "extends": ["format.json"], "formatter": { "lineWidth": 20 } }"#,
    );

    let test_file = Utf8Path::new("test.js");
    fs.insert(
        test_file.into(),
        r#"debugger; const a = ["lorem", "ipsum"]; "#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_extended_values_in_current_config",
        fs,
        console,
        result,
    ));
}

#[test]
fn respects_unaffected_values_from_extended_config() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let format = Utf8Path::new("format.json");
    fs.insert(format.into(), r#"{ "formatter": { "lineWidth": 20 } }"#);

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{ "extends": ["format.json"], "formatter": { "indentStyle": "space", "indentWidth": 2 } }"#,
    );

    let test_file = Utf8Path::new("test.js");
    fs.insert(
        test_file.into(),
        r#"debugger; const a = ["lorem", "ipsum"]; "#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "respects_unaffected_values_from_extended_config",
        fs,
        console,
        result,
    ));
}

#[test]
fn allows_reverting_fields_in_extended_config_to_default() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let format = Utf8Path::new("format.json");
    fs.insert(format.into(), r#"{ "formatter": { "lineWidth": 20 } }"#);

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        format!(
            r#"{{ "extends": ["format.json"], "formatter": {{ "lineWidth": {} }} }}"#,
            LineWidth::default().value()
        ),
    );

    let test_file = Utf8Path::new("test.js");
    fs.insert(
        test_file.into(),
        r#"debugger; const a = ["lorem", "ipsum"]; "#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "allows_reverting_fields_in_extended_config_to_default",
        fs,
        console,
        result,
    ));
}

#[test]
fn extends_config_merge_overrides() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let shared = Utf8Path::new("shared.json");
    fs.insert(
        shared.into(),
        r#"{
            "overrides": [{
                "includes": ["**/*.js"],
                "linter": { "rules": { "suspicious": { "noDebugger": "off" } } }
            }]
        }"#,
    );

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
            "extends": ["shared.json"],
            "overrides": [{
                "includes": ["**/*.js"],
                "linter": { "rules": { "correctness": { "noUnusedVariables": "error" } } }
            }]
        }"#,
    );

    let test_file = Utf8Path::new("test.js");
    fs.insert(test_file.into(), "debugger; const a = 0;");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test_file.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extends_config_merge_overrides",
        fs,
        console,
        result,
    ));
}
