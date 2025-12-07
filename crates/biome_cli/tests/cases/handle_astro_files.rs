use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, markup_to_string};
use biome_console::{BufferConsole, markup};
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const ASTRO_FILE_UNFORMATTED: &str = r#"---
import {    something } from "file.astro";

statement ( ) ;

---
<div></div>"#;

const ASTRO_FILE_DEBUGGER_BEFORE: &str = r#"---
debugger;
---
<div></div>"#;

const ASTRO_FILE_USELESS_RENAME_BEFORE: &str = r#"---
import {a as a} from 'mod';
export { a };
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
something ( ) ;
var foo: string = "";
---
<div></div>"#;

const ASTRO_FILE_ASTRO_GLOBAL_OBJECT: &str = r#"---
const { some } = Astro.props
---
<div>{some}</div>"#;

const ASTRO_FILE_WITH_TS_SCRIPT_TAG: &str = r#"---
const title = "My Page";
---
<html>
<body>
    <script>
        const message:     string = "Hello TypeScript";
        function greet(name:   string): void {
            console.log(  message + ", " + name );
        }
    </script>
</body>
</html>"#;

#[test]
fn format_astro_files() {
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
fn full_support() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        r#"---
import z from "zod";
import { sure } from "sure.js";
import s from "src/utils";

let schema = z.object().optional();
schema + sure()
---




<html><head><title>Astro</title></head><body></body></html>

<style>
#id { font-family: comic-sans } .class { background: red}
</style>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--unsafe", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "full_support",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_and_fix_astro_files() {
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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

#[test]
fn format_astro_with_typescript_script_tag() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_FILE_WITH_TS_SCRIPT_TAG.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_astro_with_typescript_script_tag",
        fs,
        console,
        result,
    ));
}

#[test]
fn dont_indent_frontmatter() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true, "indentScriptAndStyle": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        r#"---
import Foo from "./Foo.astro"
const bar = 123
if (bar>1) {console.log(bar+1)}
---
<Foo>{bar}</Foo>

<style>
#id { font-family: comic-sans } .class { background: red}
</style>

<script>
function foo(){console.log("Hello")}
</script>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "dont_indent_frontmatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_throw_parse_error_for_return_full_support() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

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
        "does_not_throw_parse_error_for_return_full_support",
        fs,
        console,
        result,
    ));
}
