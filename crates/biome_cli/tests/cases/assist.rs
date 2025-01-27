use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn assist_emit_diagnostic() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
            "assist": {
                "enabled": true,
                "actions": {
                  "source": {
                    "useSortedKeys": "on"
                  }
                }
            },
            "formatter": { "enabled": false }
        }"#
        .as_bytes(),
    );
    let file = Utf8Path::new("file.json");
    fs.insert(
        file.into(),
        r#"{ "zod": true, "lorem": "ipsum", "foo": "bar" }"#.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "assist_emit_diagnostic",
        fs,
        console,
        result,
    ));
}

#[test]
fn assist_writes() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
            "assist": {
                "enabled": true,
                "actions": {
                  "source": {
                    "useSortedKeys": "on"
                  }
                }
            },
            "formatter": { "enabled": false }
        }"#
        .as_bytes(),
    );
    let file = Utf8Path::new("file.json");
    fs.insert(
        file.into(),
        r#"{ "zod": true, "lorem": "ipsum", "foo": "bar" }"#.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(
        &fs,
        file,
        r#"{ "foo": "bar" ,"lorem": "ipsum","zod": true }"#,
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "assist_writes",
        fs,
        console,
        result,
    ));
}
