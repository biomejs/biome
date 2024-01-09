use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn not_process_file_from_stdin_format() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(r#"{ "name": "test" }"#.to_string());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--stdin-file-path=package.json")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_stdin_format",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_file_from_stdin_lint() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(r#"{ "name": "test" }"#.to_string());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), ("--stdin-file-path=package.json")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_stdin_lint",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_file_from_stdin_verbose_format() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(r#"{ "name": "test" }"#.to_string());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), "--verbose", ("--stdin-file-path=package.json")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_stdin_verbose_format",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_file_from_stdin_verbose_lint() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(r#"{ "name": "test" }"#.to_string());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), "--verbose", ("--stdin-file-path=package.json")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_stdin_verbose_lint",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_file_from_cli() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("package.json");
    fs.insert(file_path.into(), r#"{ "name": "test" }"#.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_cli",
        fs,
        console,
        result,
    ));
}
#[test]
fn not_process_file_from_cli_verbose() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("package.json");
    fs.insert(file_path.into(), r#"{ "name": "test" }"#.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                "--verbose",
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_cli_verbose",
        fs,
        console,
        result,
    ));
}
