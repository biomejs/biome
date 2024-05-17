use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn should_not_format_files_by_default() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_content = r#"html {}"#;
    let css_file = Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), css_file.as_os_str().to_str().unwrap()].as_slice()),
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
    let css_file = Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "format",
                "--css-formatter-enabled=true",
                css_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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
    let css_file = Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--css-formatter-enabled=true",
                css_file.as_os_str().to_str().unwrap(),
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

#[test]
fn should_not_lint_files_by_default() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "linter": { "rules": { "all": true } }
}
"#
        .as_bytes(),
    );

    let css_file_content = r#"html {}"#;
    let css_file = Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(["lint", css_file.as_os_str().to_str().unwrap()].as_slice()),
    );

    // no files processed error
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_lint_files_by_default",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_lint_files_by_when_enabled() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "linter": { "rules": { "all": true } }
}
"#
        .as_bytes(),
    );

    let css_file_content = r#"html {}"#;
    let css_file = Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "lint",
                "--css-linter-enabled=true",
                css_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    // diagnostic
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_lint_files_by_when_enabled",
        fs,
        console,
        result,
    ));
}
