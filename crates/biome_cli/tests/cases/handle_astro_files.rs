use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, markup_to_string, SnapshotPayload};
use biome_console::{markup, BufferConsole};
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const ASTRO_FILE_UNFORMATTED: &str = r#"---
import {    something } from "file.astro";

statement ( ) ;

---
<div></div>"#;

const ASTRO_FILE_FORMATTED: &str = r#"---
import { something } from "file.astro";

statement();
---
<div></div>"#;

const ASTRO_FILE_DEBUGGER_BEFORE: &str = r#"---
debugger;
---
<div></div>"#;

const ASTRO_FILE_DEBUGGER_AFTER: &str = r#"---
---
<div></div>"#;

const ASTRO_FILE_USELESS_RENAME_BEFORE: &str = r#"---
import {a as a} from 'mod';
---
<div></div>"#;

const ASTRO_FILE_USELESS_RENAME_AFTER: &str = r#"---
import {a} from 'mod';
---
<div></div>"#;

const ASTRO_FILE_IMPORTS_BEFORE: &str = r#"---
import { getLocale } from "astro:i18n";
import { Code } from "astro:components";
---
<div></div>"#;

const ASTRO_RETURN: &str = r#"---
const foo = true;
if (foo) {
    return "Something";
}

---
<div></div>"#;

const ASTRO_CARRIAGE_RETURN_LINE_FEED_FILE_UNFORMATTED: &str =
    "---\r\n  const a    = \"b\";\r\n---\r\n<div></div>";

const ASTRO_FILE_CHECK_BEFORE: &str = r#"---
import {a as a} from 'mod';
import {    something } from "file.astro";
debugger;
statement ( ) ;
var foo: string = "";
---
<div></div>"#;

const ASTRO_FILE_ASTRO_GLOBAL_OBJECT: &str = r#"---
const { some } = Astro.props
---
<div>{some}</div>"#;

#[test]
fn format_astro_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_astro_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_astro_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_astro_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_empty_astro_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), "<div></div>".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_empty_astro_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_astro_carriage_return_line_feed_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_CARRIAGE_RETURN_LINE_FEED_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_astro_carriage_return_line_feed_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_astro_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_FILE_DEBUGGER_BEFORE.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_astro_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_and_fix_astro_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_FILE_DEBUGGER_BEFORE.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_and_fix_astro_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn sorts_imports_check() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_IMPORTS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                astro_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "sorts_imports_check",
        fs,
        console,
        result,
    ));
}

#[test]
fn sorts_imports_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_IMPORTS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                "--write",
                astro_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "sorts_imports_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_throw_parse_error_for_return() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_RETURN.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_throw_parse_error_for_return",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_stdin_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(ASTRO_FILE_UNFORMATTED.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, ASTRO_FILE_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_stdin_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_stdin_write_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(ASTRO_FILE_UNFORMATTED.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, ASTRO_FILE_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_stdin_write_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_stdin_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push(ASTRO_FILE_USELESS_RENAME_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, ASTRO_FILE_USELESS_RENAME_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_stdin_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_stdin_write_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push(ASTRO_FILE_USELESS_RENAME_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, ASTRO_FILE_USELESS_RENAME_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_stdin_write_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_stdin_write_unsafe_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push(ASTRO_FILE_DEBUGGER_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--write",
                "--unsafe",
                "--stdin-file-path",
                "file.astro",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, ASTRO_FILE_DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_stdin_write_unsafe_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_stdin_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(ASTRO_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, ASTRO_FILE_CHECK_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_stdin_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_stdin_write_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(ASTRO_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_stdin_write_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_stdin_write_unsafe_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(ASTRO_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--write",
                "--unsafe",
                "--stdin-file-path",
                "file.astro",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_stdin_write_unsafe_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn astro_global_object() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_FILE_ASTRO_GLOBAL_OBJECT.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "astro_global",
        fs,
        console,
        result,
    ));
}
