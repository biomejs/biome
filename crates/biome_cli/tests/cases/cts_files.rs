use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn should_allow_using_export_statements() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("a.cts");
    fs.insert(
        file_path.into(),
        r#"export default { cjs: true };"#.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_allow_using_export_statements",
        fs,
        console,
        result,
    ));
}
