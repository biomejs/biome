use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn migrate_help() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "--help"].as_slice()),
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

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let (fs, result) = run_cli(fs, &mut console, Args::from(["migrate"].as_slice()));

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, configuration_path, configuration);

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
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(fs, &mut console, Args::from(["migrate"].as_slice()));

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
fn should_emit_incompatible_arguments_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "--write", "--fix"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_suggest_error_incompatible_arguments",
        fs,
        console,
        result,
    ));
}
