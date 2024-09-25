use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn should_apply_different_formatting() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"
{
    "members": ["app/"]
}
"#,
    );

    let file_path = Path::new("app/biome.json");
    fs.insert(
        file_path.into(),
        r#"
{
    "members": ["nested/app/"]
}
"#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("check"), "./"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_error_for_nested_config_with_members",
        fs,
        console,
        result,
    ));
}
