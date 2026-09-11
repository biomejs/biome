use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn migrate_stylelintrcjson() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let stylelintrc = r##"{
        "rules": {
            "block-no-empty": true,
            "color-no-hex": true,
            "no-descending-specificity": null,
            "unit-no-unknown": [true, { "severity": "warning" }],
            "indentation": 2,
            "unknown-stylelint-rule": true
        },
        "ignoreFiles": ["dist/**"]
    }"##;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(
        Utf8Path::new(".stylelintrc.json").into(),
        stylelintrc.as_bytes(),
    );

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "stylelint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_stylelintrcjson",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_stylelintrc() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let stylelintrc = r##"{
        "rules": {
            "block-no-empty": true,
            "font-family-no-duplicate-names": [true],
            "color-no-hex": [true, { "severity": "warning" }]
        }
    }"##;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".stylelintrc").into(), stylelintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "stylelint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_stylelintrc",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_stylelintrcjson_write() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let stylelintrc = r#"{
        "rules": {
            "block-no-empty": true,
            "color-no-hex": true
        }
    }"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(
        Utf8Path::new(".stylelintrc.json").into(),
        stylelintrc.as_bytes(),
    );

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "stylelint", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_stylelintrcjson_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_stylelintrcjson_overrides() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let stylelintrc = r#"{
        "rules": {
            "block-no-empty": true
        },
        "overrides": [
            {
                "files": ["*.scss"],
                "rules": {
                    "color-no-hex": true
                }
            }
        ]
    }"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(
        Utf8Path::new(".stylelintrc.json").into(),
        stylelintrc.as_bytes(),
    );

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "stylelint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_stylelintrcjson_overrides",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_stylelintrcjson_not_found() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "stylelint"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_stylelintrcjson_not_found",
        fs,
        console,
        result,
    ));
}
