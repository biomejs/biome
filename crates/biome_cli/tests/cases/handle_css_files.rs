use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn should_not_format_files_by_default() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_content = r#"html {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", css_file.as_str()].as_slice()),
    );

    // no files processed error
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_format_files_by_default",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_format_files_by_when_opt_in() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_content = r#"html {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--css-formatter-enabled=true", css_file.as_str()].as_slice()),
    );

    // not formatted error
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_format_files_by_when_opt_in",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_format_write_files_by_when_opt_in() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_content = r#"html {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--css-formatter-enabled=true",
                css_file.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_format_write_files_by_when_opt_in",
        fs,
        console,
        result,
    ));
}
