use crate::snap_test::SnapshotPayload;
use crate::{assert_cli_snapshot, run_cli};
use bpaf::Args;
use rome_console::BufferConsole;
use rome_fs::MemoryFileSystem;
use rome_service::DynRef;

#[test]
fn ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("--version")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "version_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn full() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("version")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "version_full",
        fs,
        console,
        result,
    ));
}
