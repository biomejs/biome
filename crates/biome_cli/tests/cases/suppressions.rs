use bpaf::Args;
use std::path::Path;

use crate::snap_test::SnapshotPayload;
use crate::{assert_cli_snapshot, run_cli, FORMATTED};
use biome_console::BufferConsole;
use biome_fs::{FileSystemExt, MemoryFileSystem};
use biome_service::DynRef;

const SUPPRESS_BEFORE: &str = "(1 >= -0)";
const SUPPRESS_AFTER: &str =
    "// biome-ignore lint/suspicious/noCompareNegZero: <explanation>\n(1 >= -0)";

#[test]
fn ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--suppress"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
}

#[test]
fn err_when_both_write_and_suppress_are_passed() {
    let mut fs = MemoryFileSystem::new_read_only();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--write"),
                ("--suppress"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "err_when_both_write_and_suppress_are_passed",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--suppress"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_multiple_ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(
        file_path.into(),
        [SUPPRESS_BEFORE, SUPPRESS_BEFORE].join("\n").as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--suppress"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, [SUPPRESS_AFTER, SUPPRESS_AFTER].join("\n"));

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_multiple_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_only_ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--suppress"),
                ("--only=lint/suspicious/noCompareNegZero"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_only_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_skip_ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--suppress"),
                ("--skip=lint/suspicious/noCompareNegZero"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_skip_ok",
        fs,
        console,
        result,
    ));
}
