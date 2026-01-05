use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const VUE_IMPLICIT_JS_FILE_UNFORMATTED: &str = r#"<script>
import {    something } from "file.vue";
statement ( ) ;
</script>
<template></template>"#;

const VUE_EXPLICIT_JS_FILE_UNFORMATTED: &str = r#"<script lang="js">
import {    something } from "file.vue";
statement ( ) ;
</script>
<template></template>"#;

const VUE_TS_FILE_UNFORMATTED: &str = r#"<script setup lang="ts">
import     { type     something } from "file.vue";
const hello  :      string      = "world";
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), "<template></template>".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

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
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_TS_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_TS_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

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
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), "<template></template>".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "sorts_imports_write",
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

    let astro_file_path = Utf8Path::new("file.vue");
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

    let astro_file_path = Utf8Path::new("file.vue");
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
fn full_support_enabled_and_scss_is_skipped() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.vue");
    fs.insert(
        astro_file_path.into(),
        r#"<html><head><title>Svelte</title></head><body></body></html>

<style lang="scss">
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

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "full_support_enabled_and_scss_is_skipped",
        fs,
        console,
        result,
    ));
}

#[test]
fn full_support_tsx() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        r#"<script lang="tsx">
import z from "zod";
import { sure } from "sure.js";
import s from "src/utils";

interface Props {
    title: string;
}

let schema = z.object().optional();
schema + sure();
const props: Props = { title: "Hello" };

function FunctionalComponent() {
    return <div></div>;
}

</script>

<template>
    <div></div>
</template>

<style>
.class { background: red}
</style>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--unsafe", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "full_support_tsx",
        fs,
        console,
        result,
    ));
}

#[test]
fn full_support_jsx() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        r#"<script lang="jsx">
import z from "zod";
import { sure } from "sure.js";
import s from "src/utils";

let schema = z.object().optional();
schema + sure();

function FunctionalComponent() {
    return <div></div>;
}

</script>

<template>
    <div></div>
</template>

<style>
.class { background: red}
</style>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--unsafe", vue_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "full_support_jsx",
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
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_TS_FILE_SETUP_GLOBALS.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", vue_file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "vue_compiler_macros_as_globals",
        fs,
        console,
        result,
    ));
}
