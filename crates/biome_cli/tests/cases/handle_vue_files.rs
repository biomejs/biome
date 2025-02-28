use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const VUE_IMPLICIT_JS_FILE_UNFORMATTED: &str = r#"<script>
import {    something } from "file.vue";
statement ( ) ;
</script>
<template></template>"#;

const VUE_IMPLICIT_JS_FILE_FORMATTED: &str = r#"<script>
import { something } from "file.vue";
statement();
</script>
<template></template>"#;

const VUE_EXPLICIT_JS_FILE_UNFORMATTED: &str = r#"<script lang="js">
import {    something } from "file.vue";
statement ( ) ;
</script>
<template></template>"#;

const VUE_EXPLICIT_JS_FILE_FORMATTED: &str = r#"<script lang="js">
import { something } from "file.vue";
statement();
</script>
<template></template>"#;

const VUE_TS_FILE_UNFORMATTED: &str = r#"<script setup lang="ts">
import     { type     something } from "file.vue";
const hello  :      string      = "world";
</script>
<template></template>"#;

const VUE_TS_FILE_FORMATTED: &str = r#"<script setup lang="ts">
import { type something } from "file.vue";
const hello: string = "world";
</script>
<template></template>"#;

const VUE_JS_FILE_NOT_LINTED: &str = r#"<script setup lang="js">
a == b;
delete a.c;

var foo = "";
</script>
<template></template>"#;

const VUE_TS_FILE_NOT_LINTED: &str = r#"<script setup lang="ts">
a == b;
delete a.c;

var foo: string = "";
</script>
<template></template>"#;

const VUE_FILE_IMPORTS_BEFORE: &str = r#"<script setup lang="ts">
import Button from "./components/Button.vue";
import * as vueUse from "vue-use";
</script>
<template></template>"#;

const VUE_FILE_IMPORTS_AFTER: &str = r#"<script setup lang="ts">
import * as vueUse from "vue-use";
import Button from "./components/Button.vue";
</script>
<template></template>"#;

const VUE_CARRIAGE_RETURN_LINE_FEED_FILE_UNFORMATTED: &str =
    "<script>\r\n  const a    = \"b\";\r\n</script>\r\n<template></template>";

const VUE_GENERIC_COMPONENT_FILE_UNFORMATTED: &str = r#"<script generic="T extends Record<string, any>" lang="ts" setup>
const a     =     "a";
</script>"#;

const VUE_TS_FILE_CHECK_BEFORE: &str = r#"<script setup lang="ts">
import {      Button  as Button  }   from  "./components/Button.vue"   ;
import *     as         vueUse  from  "vue-use"   ;

delete a.c;
</script>
<template></template>"#;

const VUE_TS_FILE_SETUP_GLOBALS: &str = r#"<script setup lang="ts">
// These are magic vue macros, and should be treated as globals.
defineProps(['foo'])
defineEmits(['change', 'delete'])
defineModel()

const a = 1
defineExpose({
		a,
})

defineOptions({
		inheritAttrs: false,
})

const slots = defineSlots<{
		default(props: { msg: string }): any
}>()

</script>
<template></template>"#;

#[test]
fn format_vue_implicit_js_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        VUE_IMPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, VUE_IMPLICIT_JS_FILE_UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_vue_implicit_js_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_vue_implicit_js_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        VUE_IMPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, VUE_IMPLICIT_JS_FILE_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_vue_implicit_js_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_vue_explicit_js_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        VUE_EXPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, VUE_EXPLICIT_JS_FILE_UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_vue_explicit_js_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_vue_explicit_js_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        VUE_EXPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, VUE_EXPLICIT_JS_FILE_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_vue_explicit_js_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_empty_vue_js_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), "<template></template>".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, "<template></template>");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_empty_vue_js_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_vue_ts_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_TS_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, VUE_TS_FILE_UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_vue_ts_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_vue_ts_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_TS_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, VUE_TS_FILE_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_vue_ts_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_empty_vue_ts_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), "<template></template>".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, "<template></template>");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_empty_vue_ts_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_vue_carriage_return_line_feed_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        VUE_CARRIAGE_RETURN_LINE_FEED_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(
        &fs,
        vue_file_path,
        VUE_CARRIAGE_RETURN_LINE_FEED_FILE_UNFORMATTED,
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_vue_carriage_return_line_feed_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_vue_generic_component_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        VUE_GENERIC_COMPONENT_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, VUE_GENERIC_COMPONENT_FILE_UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_vue_generic_component_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_vue_js_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_JS_FILE_NOT_LINTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_vue_js_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_vue_ts_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_TS_FILE_NOT_LINTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_vue_ts_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn sorts_imports_check() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_FILE_IMPORTS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                vue_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, VUE_FILE_IMPORTS_BEFORE);

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

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_FILE_IMPORTS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                "--write",
                vue_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, VUE_FILE_IMPORTS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "sorts_imports_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_stdin_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(VUE_TS_FILE_UNFORMATTED.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path", "file.vue"].as_slice()),
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

    console.in_buffer.push(VUE_TS_FILE_UNFORMATTED.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", "--stdin-file-path", "file.vue"].as_slice()),
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

    console.in_buffer.push(VUE_TS_FILE_NOT_LINTED.to_string());

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

    console.in_buffer.push(VUE_TS_FILE_NOT_LINTED.to_string());

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

    console.in_buffer.push(VUE_TS_FILE_NOT_LINTED.to_string());

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

    console.in_buffer.push(VUE_TS_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--stdin-file-path", "file.vue"].as_slice()),
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

    console.in_buffer.push(VUE_TS_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--stdin-file-path", "file.vue"].as_slice()),
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

    console.in_buffer.push(VUE_TS_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--write",
                "--unsafe",
                "--stdin-file-path",
                "file.vue",
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
fn vue_compiler_macros_as_globals() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_TS_FILE_SETUP_GLOBALS.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, vue_file_path, VUE_TS_FILE_SETUP_GLOBALS);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "vue_compiler_macros_as_globals",
        fs,
        console,
        result,
    ));
}
