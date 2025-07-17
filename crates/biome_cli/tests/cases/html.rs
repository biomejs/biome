use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn should_error_when_interpolation_is_disabled() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let html_file = Utf8Path::new("file.html");
    fs.insert(
        html_file.into(),
        r#"<div>{{ $interpolation }}</div>
"#
        .as_bytes(),
    );
    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
        "formatter": {
            "enabled": true
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", html_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_error_when_interpolation_is_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_error_when_interpolation_is_enabled() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let html_file = Utf8Path::new("file.html");
    fs.insert(
        html_file.into(),
        r#"<div>{{ $interpolation }}</div>
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
        "parser": {
            "interpolation": true
        },
        "formatter": {
            "enabled": true
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", html_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_error_when_interpolation_is_enabled",
        fs,
        console,
        result,
    ));
}
