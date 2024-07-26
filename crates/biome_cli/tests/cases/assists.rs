use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn json_sort_keys() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Path::new("biome.json");
    fs.insert(
        astro_file_path.into(),
        r#"{ "assists": { "enabled": true }, "formatter": { "enabled": false } }"#.as_bytes(),
    );
    let astro_file_path = Path::new("file.json");
    fs.insert(
        astro_file_path.into(),
        r#"{ "zod": true, "lorem": "ipsum", "foo": "bar" }"#.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("check"), astro_file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    // assert!(result.is_err(), "run_cli returned {result:?}");

    // assert_file_contents(
    //     &fs,
    //     astro_file_path,
    //     crate::cases::handle_astro_files::ASTRO_FILE_UNFORMATTED,
    // );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "json_sort_keys",
        fs,
        console,
        result,
    ));
}
