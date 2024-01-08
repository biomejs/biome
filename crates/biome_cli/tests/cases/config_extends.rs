use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_formatter::LineWidth;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn extends_config_ok_formatter_no_linter() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = Path::new("biome.json");
    fs.insert(
        rome_json.into(),
        r#"{ "extends": ["format.json", "linter.json"] }"#,
    );
    let format = Path::new("format.json");
    fs.insert(
        format.into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    let lint = Path::new("linter.json");
    fs.insert(lint.into(), r#"{ "linter": { "enabled": false } }"#);

    let test_file = Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("check"), test_file.as_os_str().to_str().unwrap()].as_slice()),
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
fn extends_config_ok_linter_not_formatter() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = Path::new("biome.json");
    fs.insert(
        rome_json.into(),
        r#"{ "extends": ["format.json", "linter.json"] }"#,
    );
    let format = Path::new("format.json");
    fs.insert(format.into(), r#"{ "formatter": { "enabled": true } }"#);
    let lint = Path::new("linter.json");
    fs.insert(
        lint.into(),
        r#"{
  "linter": {
    "rules": {
      "all": false,
      "suspicious": {
        "noDebugger": "warn"
      }
    }
  }
}
        "#,
    );

    let test_file = Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("check"), test_file.as_os_str().to_str().unwrap()].as_slice()),
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = Path::new("biome.json");
    fs.insert(
        rome_json.into(),
        r#"{ "extends": ["formatTYPO.json", "linter.json"] }"#,
    );
    let format = Path::new("format.json");
    fs.insert(
        format.into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    let lint = Path::new("linter.json");
    fs.insert(lint.into(), r#"{ "linter": { "enabled": false } }"#);

    let test_file = Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("check"), test_file.as_os_str().to_str().unwrap()].as_slice()),
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = Path::new("biome.json");
    fs.insert(
        rome_json.into(),
        r#"{ "extends": ["formatTYPO.json", "linter.json"] }"#,
    );
    let format = Path::new("format.json");
    fs.insert(
        format.into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    let lint = Path::new("linter.json");
    fs.insert(lint.into(), r#"{ "linter": { "enabled": false } }"#);

    let test_file = Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                "--verbose",
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = Path::new("config/biome.json");
    fs.insert(
        rome_json.into(),
        r#"{ "extends": ["format.json", "linter.json"] }"#,
    );
    let format = Path::new("config/format.json");
    fs.insert(
        format.into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );
    let lint = Path::new("config/linter.json");
    fs.insert(lint.into(), r#"{ "linter": { "enabled": true } }"#);

    let test_file = Path::new("test.js");
    fs.insert(test_file.into(), r#"debugger; console.log("string"); "#);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                "--config-path=config/",
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let format = Path::new("format.json");
    fs.insert(
        format.into(),
        r#"{ "javascript": { "formatter": { "quoteStyle": "single" } } }"#,
    );

    let rome_json = Path::new("biome.json");
    fs.insert(
        rome_json.into(),
        r#"{ "extends": ["format.json"], "formatter": { "lineWidth": 20 } }"#,
    );

    let test_file = Path::new("test.js");
    fs.insert(
        test_file.into(),
        r#"debugger; const a = ["lorem", "ipsum"]; "#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                "--write",
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let format = Path::new("format.json");
    fs.insert(format.into(), r#"{ "formatter": { "lineWidth": 20 } }"#);

    let rome_json = Path::new("biome.json");
    fs.insert(
        rome_json.into(),
        r#"{ "extends": ["format.json"], "formatter": { "indentStyle": "space", "indentWidth": 2 } }"#,
    );

    let test_file = Path::new("test.js");
    fs.insert(
        test_file.into(),
        r#"debugger; const a = ["lorem", "ipsum"]; "#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                "--write",
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let format = Path::new("format.json");
    fs.insert(format.into(), r#"{ "formatter": { "lineWidth": 20 } }"#);

    let rome_json = Path::new("biome.json");
    fs.insert(
        rome_json.into(),
        format!(
            r#"{{ "extends": ["format.json"], "formatter": {{ "lineWidth": {} }} }}"#,
            LineWidth::default().get()
        ),
    );

    let test_file = Path::new("test.js");
    fs.insert(
        test_file.into(),
        r#"debugger; const a = ["lorem", "ipsum"]; "#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                "--write",
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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
