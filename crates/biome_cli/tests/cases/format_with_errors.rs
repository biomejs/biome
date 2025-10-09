use crate::snap_test::SnapshotPayload;
use crate::{assert_cli_snapshot, run_cli};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn check_format_with_errors_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    // File with syntax error
    fs.insert(file_path.into(), "let a = {".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--format-with-errors=true", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_format_with_errors_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_format_with_errors_false() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    // File with syntax error
    fs.insert(file_path.into(), "let a = {".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--format-with-errors=false", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_format_with_errors_false",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_format_with_errors_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    // File with syntax error
    fs.insert(file_path.into(), "let a = {".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--format-with-errors=true", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_format_with_errors_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_format_with_errors_false() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    // File with syntax error
    fs.insert(file_path.into(), "let a = {".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--format-with-errors=false", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_format_with_errors_false",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_format_with_errors_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    // File with syntax error
    fs.insert(file_path.into(), "let a = {".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--format-with-errors=true", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_format_with_errors_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_format_with_errors_false() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    // File with syntax error
    fs.insert(file_path.into(), "let a = {".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--format-with-errors=false", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_format_with_errors_false",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_format_with_errors_overrides_config() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Config with format_with_errors: false
    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "formatter": {
    "formatWithErrors": false
  }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), "let a = {".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--format-with-errors=true", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_format_with_errors_overrides_config",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_format_with_errors_respects_config_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Config with format_with_errors: true
    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "formatter": {
    "formatWithErrors": true
  }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), "let a = {".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_format_with_errors_respects_config_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_format_with_errors_respects_config_false() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Config with format_with_errors: false
    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "formatter": {
    "formatWithErrors": false
  }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), "let a = {".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_format_with_errors_respects_config_false",
        fs,
        console,
        result,
    ));
}
