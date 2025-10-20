use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

const SVELTE_FILE_IMPORTS_BEFORE: &str = r#"<script lang="ts">
import Button from "./components/Button.svelte";
import * as svelteUse from "svelte-use";
</script>
<div></div>"#;

const SVELTE_FILE_IMPORTS_AFTER: &str = r#"<script lang="ts">
import * as svelteUse from "svelte-use";
import Button from "./components/Button.svelte";
</script>
<div></div>"#;

const SVELTE_TS_CONTEXT_MODULE_FILE_UNFORMATTED: &str = r#"<script context="module" lang="ts">
import     Button     from "./components/Button.svelte";
const hello  :      string      = "world";
</script>
<div></div>"#;

const SVELTE_CARRIAGE_RETURN_LINE_FEED_FILE_UNFORMATTED: &str =
    "<script>\r\n  const a    = \"b\";\r\n</script>\r\n<div></div>";

const SVELTE_TS_FILE_LINT_BEFORE: &str = r#"<script context="module" lang="ts">
var foo: string = "";
</script>
<div></div>"#;

const SVELTE_TS_FILE_CHECK_BEFORE: &str = r#"<script context="module" lang="ts">
import { Form as   Form }     from './components/Form.svelte' ;
import     Button     from "./components/Button.svelte";
debugger;
statement ( ) ;
var foo: string = "";
</script>
<div></div>"#;

#[test]
fn sorts_imports_check() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_FILE_IMPORTS_BEFORE.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &[
                "check",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                svelte_file_path.as_str(),
            ],
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, SVELTE_FILE_IMPORTS_BEFORE);

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

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_FILE_IMPORTS_BEFORE.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &[
                "check",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                "--write",
                svelte_file_path.as_str(),
            ],
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, SVELTE_FILE_IMPORTS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "sorts_imports_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_svelte_ts_context_module_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_TS_CONTEXT_MODULE_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["format", svelte_file_path.as_str()],
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(
        &fs,
        svelte_file_path,
        SVELTE_TS_CONTEXT_MODULE_FILE_UNFORMATTED,
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_ts_context_module_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_svelte_ts_context_module_files_write() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_TS_CONTEXT_MODULE_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["format", "--write", svelte_file_path.as_str()],
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_ts_context_module_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_svelte_carriage_return_line_feed_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_CARRIAGE_RETURN_LINE_FEED_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["format", svelte_file_path.as_str()],
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(
        &fs,
        svelte_file_path,
        SVELTE_CARRIAGE_RETURN_LINE_FEED_FILE_UNFORMATTED,
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_carriage_return_line_feed_files",
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

    let astro_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        astro_file_path.into(),
        r#"<script>
import z from "zod";
import { sure } from "sure.js";
import s from "src/utils";

let schema = z.object().optional();
schema + sure()
</script>

<html><head><title>Svelte</title></head><body></body></html>

<style>
#id { font-family: comic-sans } .class { background: red}
</style>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["check", "--write", "--unsafe", astro_file_path.as_str()],
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
fn full_support_ts() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        astro_file_path.into(),
        r#"<script lang="ts">
import z from "zod";
import { sure } from "sure.js";
import s from "src/utils";

interface Props {
    title: string;
}

let schema = z.object().optional();
schema + sure();
const props: Props = { title: "Hello" };
</script>

<html><head><title>Svelte</title></head><body></body></html>

<style>
#id { font-family: comic-sans } .class { background: red}
</style>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["check", "--write", "--unsafe", astro_file_path.as_str()],
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "full_support_ts",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_stdin_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push(SVELTE_TS_CONTEXT_MODULE_FILE_UNFORMATTED.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["format", "--stdin-file-path", "file.svelte"],
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

    console
        .in_buffer
        .push(SVELTE_TS_CONTEXT_MODULE_FILE_UNFORMATTED.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["format", "--write", "--stdin-file-path", "file.svelte"],
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
        .push(SVELTE_TS_FILE_LINT_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["lint", "--stdin-file-path", "file.svelte"],
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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
        .push(SVELTE_TS_FILE_LINT_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["lint", "--write", "--stdin-file-path", "file.svelte"],
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
        .push(SVELTE_TS_FILE_LINT_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &[
                "lint",
                "--write",
                "--unsafe",
                "--stdin-file-path",
                "file.svelte",
            ],
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

    console
        .in_buffer
        .push(SVELTE_TS_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["check", "--stdin-file-path", "file.svelte"],
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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

    console
        .in_buffer
        .push(SVELTE_TS_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &["check", "--write", "--stdin-file-path", "file.svelte"],
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

    console
        .in_buffer
        .push(SVELTE_TS_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        &[
                "check",
                "--write",
                "--unsafe",
                "--stdin-file-path",
                "file.svelte",
            ],
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
