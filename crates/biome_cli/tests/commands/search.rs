use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

use crate::{
    run_cli,
    snap_test::{SnapshotPayload, assert_cli_snapshot},
};

// Feel free to add content at the end of this dummy file. It shouldn't affect
// existing tests.
const CSS_FILE_CONTENT: &str = r#"div {
    color: green;
}"#;

// Feel free to add content at the end of this dummy file. It shouldn't affect
// existing tests.
const JS_FILE_CONTENT: &str = r#"const a = 'foo';"#;

#[test]
fn search_css_pattern() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.css");
    fs.insert(file_path.into(), CSS_FILE_CONTENT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "search",
                "--language=css",
                "`color: green`",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "search_css_pattern",
        fs,
        console,
        result,
    ));
}

#[test]
fn search_css_pattern_shorthand() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.css");
    fs.insert(file_path.into(), CSS_FILE_CONTENT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["search", "-lcss", "`color: green`", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "search_css_pattern_shorthand",
        fs,
        console,
        result,
    ));
}

#[test]
fn search_js_pattern() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), JS_FILE_CONTENT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["search", "`\"foo\"`", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "search_js_pattern",
        fs,
        console,
        result,
    ));
}

#[test]
fn search_js_pattern_skips_css_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file_path = Utf8Path::new("file.js");
    let css_file_path = Utf8Path::new("file.css");
    fs.insert(js_file_path.into(), JS_FILE_CONTENT.as_bytes());
    fs.insert(css_file_path.into(), CSS_FILE_CONTENT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "search",
                "`\"foo\"`",
                js_file_path.as_str(),
                css_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "search_js_pattern_skips_css_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn search_css_pattern_skips_js_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file_path = Utf8Path::new("file.js");
    let css_file_path = Utf8Path::new("file.css");
    fs.insert(js_file_path.into(), JS_FILE_CONTENT.as_bytes());
    fs.insert(css_file_path.into(), CSS_FILE_CONTENT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "search",
                "--language=css",
                "`color: green`",
                js_file_path.as_str(),
                css_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "search_css_pattern_skips_js_files",
        fs,
        console,
        result,
    ));
}
