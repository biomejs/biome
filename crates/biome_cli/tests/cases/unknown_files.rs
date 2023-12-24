use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use crate::{run_cli, UNFORMATTED};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn should_print_a_diagnostic_unknown_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Path::new("format.yml");
    fs.insert(file_path1.into(), "".as_bytes());

    let file_path2 = Path::new("format.js");
    fs.insert(file_path2.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                file_path1.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_print_a_diagnostic_unknown_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_print_a_diagnostic_unknown_file_because_ignored() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Path::new("biome.json");
    fs.insert(
        file_path1.into(),
        r#"{ "files": { "ignoreUnknown": true } }"#.as_bytes(),
    );

    let file_path1 = Path::new("format.yml");
    fs.insert(file_path1.into(), "".as_bytes());

    let file_path2 = Path::new("format.js");
    fs.insert(file_path2.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                file_path1.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_print_a_diagnostic_unknown_file_because_ignored",
        fs,
        console,
        result,
    ));
}
