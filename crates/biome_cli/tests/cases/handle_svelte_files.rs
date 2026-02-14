use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
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
fn embedded_bindings_are_tracked_correctly() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script>
import { Component } from "./component.svelte";
let hello = "Hello World";
let array = [];
let props = [];
</script>

<html>
    <span>{hello}</span>
    <span>{notDefined}</span>
    {#each array as item}
    {/each}
    <Component />
    <input {...props} />
</html>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUnusedVariables", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "embedded_bindings_are_tracked_correctly",
        fs,
        console,
        result,
    ));
}

#[test]
fn use_const_not_triggered_in_snippet_sources() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script>
let hello = "Hello World";
</script>

<html>
    <span>{hello}</span>
</html>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=useConst", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "use_const_not_triggered_in_snippet_sources",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_unused_imports_is_not_triggered_in_snippet_sources() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script>
import Component from "./Component.vue"
</script>

<Component />
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUnusedImports", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_imports_is_not_triggered_in_snippet_sources",
        fs,
        console,
        result,
    ));
}

const SVELTE_ENUM_IN_TEMPLATE: &str = r#"<script lang="ts">
import { Component, FooEnum } from './types';
</script>
<main>
  <Component />
  {FooEnum.Foo}
</main>"#;

#[test]
fn use_import_type_not_triggered_for_enum_in_template() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file = Utf8Path::new("file.svelte");
    fs.insert(file.into(), SVELTE_ENUM_IN_TEMPLATE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=useImportType", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "use_import_type_not_triggered_for_enum_in_template",
        fs,
        console,
        result,
    ));
}

#[test]
fn use_import_type_not_triggered_for_enum_in_template_v2() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
import { Avatar as AvatarPrimitive } from "bits-ui"; // <-- false positive
import { cn } from "$lib/utils.js";

let {
	ref = $bindable(null),
	class: className,
}: AvatarPrimitive.FallbackProps = $props();
</script>

<!-- used as value here -->
<AvatarPrimitive.Fallback
	bind:ref
	class="something nice"
/>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=useImportType", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "use_import_type_not_triggered_for_enum_in_template_v2",
        fs,
        console,
        result,
    ));
}
#[test]
fn no_useless_lone_block_statements_is_not_triggered() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<CollapsiblePrimitive.Content>
	{#snippet child({ props, open })}
		{#if open}
			<div {...props} transition:slide={{ duration }}>
				{@render children?.()}
			</div>
		{/if}
	{/snippet}
</CollapsiblePrimitive.Content>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUselessLoneBlockStatements", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_useless_lone_block_statements_is_not_triggered",
        fs,
        console,
        result,
    ));
}

#[test]
fn supports_ts_in_embedded_expressions() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
// this makes it typescript
</script>

<div
	onclick={(e) => {
		if ((e.target as HTMLElement).closest("button")) {
			return;
		}
		e.currentTarget.parentElement?.querySelector("input")?.focus();
	}}
>
</div>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUselessLoneBlockStatements", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "supports_ts_in_embedded_expressions",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_unused_variables_in_svelte_directives() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script>
const isActive = false;
const color = "red";
let inputValue = "";
let isChecked = false;
</script>

<main>
  <!-- bind: directive -->
  <input bind:value={inputValue} />
  
  <!-- bind: directive with checkbox -->
  <input type="checkbox" bind:checked={isChecked} />
  
  <!-- class: directive -->
  <div class:active={isActive}>Active</div>
  
  <!-- style: directive -->
  <div style:color={color}>Styled</div>
  
  <!-- Using variables in text expressions -->
  <p>{inputValue}</p>
  <p>{isChecked}</p>
</main>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUnusedVariables", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_variables_in_svelte_directives",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_comma_operator_triggered_in_svelte_template_expression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script>
  let x = 1;
</script>

<!-- Comma operator in template expression - should be flagged -->
<p>{(console.log("side effect"), x)}</p>"#
            .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noCommaOperator", file.as_str()].as_slice()),
    );

    // The comma operator SHOULD be flagged in Svelte (hack only applies to Vue)
    // Result is Ok because it's a warning, but console should contain the diagnostic
    assert!(result.is_ok(), "run_cli returned {result:?}");
    let has_comma_operator = console.out_buffer.iter().any(|m| {
        let content = format!("{:?}", m.content);
        content.contains("noCommaOperator")
    });
    assert!(
        has_comma_operator,
        "Expected noCommaOperator diagnostic in console output"
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_comma_operator_triggered_in_svelte_template_expression",
        fs,
        console,
        result,
    ));
}
