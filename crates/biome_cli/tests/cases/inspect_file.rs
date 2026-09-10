use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn throws_an_error_for_missing_file() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(fs, &mut console, Args::from(["inspect", "file"].as_slice()));

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "throws_an_error_for_missing_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn inspect_file_help() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "file", "--help"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "inspect_file_help",
        fs,
        console,
        result,
    ));
}

#[test]
fn prints_the_cst() {
    let fs = MemoryFileSystem::default();
    let path = Utf8Path::new("file.js");
    fs.insert(
        path.to_path_buf(),
        r#"function name(value) {
    return value;
}
"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "file", path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prints_the_cst",
        fs,
        console,
        result,
    ));
}

#[test]
fn prints_the_ast() {
    let fs = MemoryFileSystem::default();
    let path = Utf8Path::new("file.js");
    fs.insert(
        path.to_path_buf(),
        r#"function name(value) {
    return value;
}
"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "file", "--ast", path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prints_the_ast",
        fs,
        console,
        result,
    ));
}

#[test]
fn prints_the_ir() {
    let fs = MemoryFileSystem::default();
    let path = Utf8Path::new("file.js");
    fs.insert(
        path.to_path_buf(),
        r#"function name(value) {
    return value;
}
"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "file", "--ir", path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prints_the_ir",
        fs,
        console,
        result,
    ));
}

#[test]
fn prints_the_semantic_for_js() {
    let fs = MemoryFileSystem::default();
    let path = Utf8Path::new("file.ts");
    fs.insert(
        path.to_path_buf(),
        r#"function name(value) {
    return value;
}
export { name }
"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "file", "--semantic", path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prints_the_semantic_for_js",
        fs,
        console,
        result,
    ));
}

#[test]
fn prints_the_semantic_for_css() {
    let fs = MemoryFileSystem::default();
    let path = Utf8Path::new("file.css");
    fs.insert(
        path.to_path_buf(),
        r#"#id {
    font-size: 12px;
}
.class {
    background-color: red;
}
"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "file", "--semantic", path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prints_the_semantic_for_css",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_throw_for_missing_semantic() {
    let fs = MemoryFileSystem::default();
    let path = Utf8Path::new("file.md");
    fs.insert(
        path.to_path_buf(),
        r#"# Header

Paragraph
"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "file", "--semantic", path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_throw_for_missing_semantic",
        fs,
        console,
        result,
    ));
}
