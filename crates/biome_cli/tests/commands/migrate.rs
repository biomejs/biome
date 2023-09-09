use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use bpaf::Args;
use rome_fs::{FileSystemExt, MemoryFileSystem};
use rome_service::DynRef;
use std::path::Path;

#[test]
fn migrate_help() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "--help"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_help",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_config_up_to_date() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs.open(configuration_path).expect("file to open");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, configuration);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_config_up_to_date",
        fs,
        console,
        result,
    ));
}

#[test]
fn missing_configuration_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate")].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "missing_configuration_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn emit_diagnostic_for_rome_json() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;

    let configuration_path = Path::new("rome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "emit_diagnostic_for_rome_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_create_biome_json_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;

    let configuration_path = Path::new("rome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_create_biome_json_file",
        fs,
        console,
        result,
    ));
}
