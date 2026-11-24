use crate::TestArgs as Args;
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

const SVELTE_TS_GENERICS_FILE_UNFORMATTED: &str = r#"<script lang="ts" generics="T extends Record<string, unknown>, U extends FormPath<T>">
import     type     { FormPath }     from "sveltekit-superforms";
const hello  :      string      = "world";
</script>
<div></div>"#;

const SVELTE_TS_GENERICS_FILE_FORMATTED: &str = r#"<script lang="ts" generics="T extends Record<string, unknown>, U extends FormPath<T>">
import type { FormPath } from "sveltekit-superforms";
const hello: string = "world";
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
        Args::from(
            [
                "check",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                svelte_file_path.as_str(),
            ]
            .as_slice(),
        ),
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
        Args::from(
            [
                "check",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                "--write",
                svelte_file_path.as_str(),
            ]
            .as_slice(),
        ),
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
        Args::from(["format", svelte_file_path.as_str()].as_slice()),
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
        Args::from(["format", "--write", svelte_file_path.as_str()].as_slice()),
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
        Args::from(["format", svelte_file_path.as_str()].as_slice()),
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
:global(div) {
  color: red;
}
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
        Args::from(["check", "--write", "--unsafe", astro_file_path.as_str()].as_slice()),
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
fn svelte_generics_are_parsed_and_formatted() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": { "enabled": true }, "linter": { "enabled": true }, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let path = Utf8Path::new("file.svelte");
    fs.insert(
        path.into(),
        r#"<script lang="ts" generics="TData   extends   { selected?: boolean; name?: string },TRow">
let { items, selected }: { items: TData[]; selected: TRow } = $props();
</script>

<p>{items.length}{selected}</p>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "svelte_generics_are_parsed_and_formatted",
        fs,
        console,
        result,
    ));
}

#[test]
fn full_support_preserves_const_tag_assignments() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        br#"{
            "html": {
                "experimentalFullSupportEnabled": true,
                "formatter": { "enabled": true }
            },
            "linter": { "rules": { "recommended": false } },
            "assist": { "enabled": false }
        }"#,
    );
    let path = Utf8Path::new("file.svelte");
    fs.insert(
        path.into(),
        br#"{#if true}
    {@const value=source}
    {@const {property}=source}
    {@const [item]=source}
    {@const nested=(source=1)+2}
    {@const callback=()=>source=1}
{/if}
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", path.as_str()].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "full_support_preserves_const_tag_assignments",
        fs,
        console,
        result,
    ));
}

#[test]
fn formats_typed_const_tags() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        br#"{ "html": { "experimentalFullSupportEnabled": true, "formatter": { "enabled": true } } }"#,
    );
    let path = Utf8Path::new("file.svelte");
    fs.insert(
        path.into(),
        br#"<script lang="ts">
const items = [1, 2, 3];
</script>

{#each items as item}
    {@const doubled  :  number=item*2}
    {@const cells:Record<string,typeof item>={value:item}}
    {@const {value}: {value:number}=cells}
    {@const [first]:number[]=[item]}
    <span>{doubled}{cells.value}{value}{first}</span>
{/each}
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", path.as_str()].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");
    let (fs, lint_result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--only=correctness/noUndeclaredVariables",
                path.as_str(),
            ]
            .as_slice(),
        ),
    );
    assert!(lint_result.is_ok(), "run_cli returned {lint_result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formats_typed_const_tags",
        fs,
        console,
        result.followed_by(lint_result),
    ));
}

#[test]
fn formats_declaration_tags() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{
            "html": {
                "formatter": { "enabled": true },
                "experimentalFullSupportEnabled": true
            }
        }"#
        .as_bytes(),
    );

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        r#"<script>
const source={value:1};
</script>

{let    value=source.value;}
{const first=1,second={value:"}"};}
{const regex=/[}]/;}
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(
        &fs,
        svelte_file_path,
        r#"<script>
const source = { value: 1 };
</script>

{let value = source.value}
{const first = 1,
	second = { value: "}" }}
{const regex = /[}]/}
"#,
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formats_declaration_tags",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_preserves_template_literal_and_inline_spacing() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        r#"{
            "javascript": { "formatter": { "semicolons": "asNeeded" } },
            "html": { "experimentalFullSupportEnabled": true, "formatter": { "enabled": true } }
        }"#,
    );
    console.in_buffer.push(
        r#"<script>
let label=$state(`first
  second`)
</script>

<p class = 'label'>{label}<i>!</i> <b>next</b></p>
"#
        .to_string(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path", "Label.svelte"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_preserves_template_literal_and_inline_spacing",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_preserves_unsupported_style_and_malformed_expression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        r#"{
            "formatter": { "formatWithErrors": true },
            "html": { "experimentalFullSupportEnabled": true, "formatter": { "enabled": true } }
        }"#,
    );
    console.in_buffer.push(
        r#"<script>let count=$state(0)</script>
<style lang="less">  @ink: blue;
    p { color: @ink }  </style>
<p>𝄞{  count +  } <b class = "after">after</b></p>
"#
        .to_string(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path", "Recovery.svelte"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_preserves_unsupported_style_and_malformed_expression",
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
        Args::from(["format", "--stdin-file-path", "file.svelte"].as_slice()),
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
        Args::from(["format", "--write", "--stdin-file-path", "file.svelte"].as_slice()),
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
        Args::from(["lint", "--stdin-file-path", "file.svelte"].as_slice()),
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
        Args::from(["lint", "--write", "--stdin-file-path", "file.svelte"].as_slice()),
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
        Args::from(
            [
                "lint",
                "--write",
                "--unsafe",
                "--stdin-file-path",
                "file.svelte",
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

    console
        .in_buffer
        .push(SVELTE_TS_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--stdin-file-path", "file.svelte"].as_slice()),
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
        Args::from(["check", "--write", "--stdin-file-path", "file.svelte"].as_slice()),
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
        Args::from(
            [
                "check",
                "--write",
                "--unsafe",
                "--stdin-file-path",
                "file.svelte",
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
fn format_svelte_ts_generics_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_TS_GENERICS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, SVELTE_TS_GENERICS_FILE_UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_ts_generics_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_svelte_ts_generics_files_write() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_TS_GENERICS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, SVELTE_TS_GENERICS_FILE_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_ts_generics_files_write",
        fs,
        console,
        result,
    ));
}

// Regression test for https://github.com/biomejs/biome/issues/10247
#[test]
fn full_support_writes_attribute_expression_fix() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{
            "html": { "experimentalFullSupportEnabled": true },
            "linter": {
                "rules": { "recommended": false, "style": { "useBlockStatements": "warn" } }
            }
        }"#
        .as_bytes(),
    );

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        r#"<script lang="ts">
	let open = true;

	function close() {
		open = false;
	}
</script>

<button
	type="button"
	onclick={() => {
		if (open) close();
	}}
>
	Close
</button>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--write",
                "--unsafe",
                "--only=style/useBlockStatements",
                svelte_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(
        &fs,
        svelte_file_path,
        r#"<script lang="ts">
	let open = true;

	function close() {
		open = false;
	}
</script>

<button
	type="button"
	onclick={() => {
		if (open) { close(); }
	}}
>
	Close
</button>
"#,
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "full_support_writes_attribute_expression_fix",
        fs,
        console,
        result,
    ));
}

#[test]
fn full_support_lint_parity_for_components_and_modules() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        br#"{
            "formatter": { "enabled": false },
            "html": {
                "experimentalFullSupportEnabled": true,
                "linter": { "enabled": true }
            },
            "linter": {
                "rules": {
                    "preset": "none",
                    "complexity": { "noUselessStringConcat": "warn" }
                }
            },
            "assist": { "enabled": false }
        }"#,
    );

    let component_path = Utf8Path::new("Component.svelte");
    fs.insert(
        component_path.into(),
        br#"<!-- biome-ignore-start lint/complexity/noUselessStringConcat: intentional concatenation -->
<script context="module">
export const preserved = "a" + "b";
</script>
<!-- biome-ignore-end lint/complexity/noUselessStringConcat: intentional concatenation -->

<script>
const fixed = "c" + "d";
</script>

<p>{preserved}{fixed}</p>
"#,
    );
    let module_path = Utf8Path::new("state.svelte.ts");
    fs.insert(
        module_path.into(),
        br#"// biome-ignore lint/complexity/noUselessStringConcat: intentional concatenation
export const preserved = "a" + "b";
export const fixed = "c" + "d";
"#,
    );

    let (fs, lint_result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", component_path.as_str(), module_path.as_str()].as_slice()),
    );
    assert!(lint_result.is_ok(), "run_cli returned {lint_result:?}");

    let (fs, write_result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--write",
                component_path.as_str(),
                module_path.as_str(),
            ]
            .as_slice(),
        ),
    );
    assert!(write_result.is_ok(), "run_cli returned {write_result:?}");
    assert_file_contents(
        &fs,
        component_path,
        r#"<!-- biome-ignore-start lint/complexity/noUselessStringConcat: intentional concatenation -->
<script context="module">
export const preserved = "a" + "b";
</script>
<!-- biome-ignore-end lint/complexity/noUselessStringConcat: intentional concatenation -->

<script>
const fixed = "cd";
</script>

<p>{preserved}{fixed}</p>
"#,
    );
    assert_file_contents(
        &fs,
        module_path,
        r#"// biome-ignore lint/complexity/noUselessStringConcat: intentional concatenation
export const preserved = "a" + "b";
export const fixed = "cd";
"#,
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "full_support_lint_parity_for_components_and_modules",
        fs,
        console,
        lint_result.followed_by(write_result),
    ));
}

#[test]
fn full_support_reports_cross_snippet_reassignments() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        br#"{
            "formatter": { "enabled": false },
            "html": {
                "experimentalFullSupportEnabled": true,
                "linter": { "enabled": true }
            },
            "linter": {
                "rules": {
                    "preset": "none",
                    "suspicious": {
                        "noClassAssign": "error",
                        "noFunctionAssign": "error",
                        "noImportAssign": "error"
                    }
                }
            },
            "assist": { "enabled": false }
        }"#,
    );

    let component_path = Utf8Path::new("Component.svelte");
    fs.insert(
        component_path.into(),
        br#"<script>
import value from "pkg";
function update() {}
class Widget {}
</script>

<button
    onclick={() => {
        value = 1;
        update = () => {};
        Widget = class {};
    }}
>
    Update
</button>
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", component_path.as_str()].as_slice()),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "full_support_reports_cross_snippet_reassignments",
        fs,
        console,
        result,
    ));
}
