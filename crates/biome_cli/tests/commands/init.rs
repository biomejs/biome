use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn init_help() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("init"), "--help"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "init_help",
        fs,
        console,
        result,
    ));
}

#[test]
fn creates_config_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("init")].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "creates_config_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn creates_config_jsonc_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("init"), "--jsonc"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "creates_config_jsonc_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn creates_config_file_when_biome_installed_via_package_manager() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("./node_modules/@biomejs/biome/configuration_schema.json");
    fs.insert(file_path.into(), *b"{}");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("init")].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "creates_config_file_when_biome_installed_via_package_manager",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_create_config_file_if_json_exists() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), *b"{}");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("init")].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_create_config_file_if_json_exists",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_create_config_file_if_jsonc_exists() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.jsonc");
    fs.insert(file_path.into(), *b"{}");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("init")].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_create_config_file_if_jsonc_exists",
        fs,
        console,
        result,
    ));
}
