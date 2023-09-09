use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use bpaf::Args;
use rome_console::BufferConsole;
use rome_fs::MemoryFileSystem;
use rome_service::DynRef;

#[test]
fn lsp_proxy_help() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lsp-proxy"), "--help"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lsp_proxy_help",
        fs,
        console,
        result,
    ));
}
