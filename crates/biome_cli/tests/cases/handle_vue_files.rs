use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::{FileSystemExt, MemoryFileSystem};
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
:slotted(div) {
  color: red;
}
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
fn parse_vue_css_v_bind_function() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        r#"<template>
  <div class="red"></div>
</template>

<style>
.red {
  color: v-bind(color);
}
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
        "parse_vue_css_v_bind_function",
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
        r#"<html lang="en"><head><title>Vue</title></head><body></body></html>

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

#[test]
fn embedded_bindings_are_tracked_correctly() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<script>
import { Component } from "./component.vue";
let hello = "Hello World";
</script>

<script>
let greeting = "Hello World";
function reasonText() {
	return "foo"
}
</script>


<template>
    <span>{hello}</span>
    <span>{notDefined}</span>
    <span>{greeting}</span>
    <Component />
    <span class="dc-reason">{{ reasonText() }}</span>
</template>
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
fn no_unused_variables_in_vue_directives() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<script setup>
const supported = ref(true);
const enabled = ref(false);
const count = ref(0);
const isActive = ref(false);

function toggleCaptions() {
	enabled.value = !enabled.value;
}

function handleClick() {
	count.value++;
}
</script>

<template>
  <!-- v-on shorthand with function call -->
  <button @click="toggleCaptions()">Toggle</button>
  
  <!-- v-on shorthand with function reference -->
  <button @click="handleClick">Click</button>
  
  <!-- v-bind shorthand with expression -->
  <button :disabled="!supported">Disabled</button>
  
  <!-- v-if directive -->
  <div v-if="count > 0">Count: {{ count }}</div>
  
  <!-- v-show directive -->
  <div v-show="isActive">Active</div>
  
  <!-- v-for directive - skip for now as it has special syntax -->
  <!-- <li v-for="item in items" :key="item">{{ item }}</li> -->
</template>
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
        "no_unused_variables_in_vue_directives",
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

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<script>
let hello = "Hello World";
</script>

<template>
    <span>{hello}</span>
</template>
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

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<script>
import Component from "./Component.vue"
</script>

<template>
    <Component />
</template>
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

#[test]
fn use_import_type_not_triggered_for_enum_in_template() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<script setup lang="ts">
import { Component, FooEnum } from './types';
</script>
<template>
  <Component />
  <div>{{ FooEnum.Foo }}</div>
</template>"#
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

    let file = Utf8Path::new("file.vue");
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

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<script>
{
	const x = 1;
}
</script>

<template>
	<div>{{ x }}</div>
</template>
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

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<script setup lang="ts">
        const num = 1
        </script>

        <template>
          <h1>{{ num as any }}</h1>
        </template>

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
fn fails_for_ts_grammar_when_lang_is_not_ts() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<script setup>
        const num = 1
        </script>

        <template>
          <h1>{{ num as any }}</h1>
        </template>

"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUselessLoneBlockStatements", file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fails_for_ts_grammar_when_lang_is_not_ts",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_vue_should_not_add_extra_newlines_in_embedded_snippet() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<script>
import { computed } from "vue";
</script>"#
            .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_vue_should_not_add_extra_newlines_in_embedded_snippet",
        fs,
        console,
        result,
    ));
}

#[test]
fn unused_suppression_has_correct_span_in_vue_file() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<template>
  <div>Hello</div>
</template>

<script lang="ts" setup>
console.log("foo");
// biome-ignore lint/correctness/noUnusedImports: migrating to biome
import { mdiSquareOutline } from "@mdi/js";
</script>"#
            .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file.as_str()].as_slice()),
    );

    // Note: result is Ok because warnings don't cause the CLI to fail
    assert!(
        result.is_ok(),
        "run_cli returned {result:?}, output: {:?}",
        console.out_buffer
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unused_suppression_has_correct_span_in_vue_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_does_not_add_comments_for_imports_used_in_templates() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"<script>
import { mdiSquareOutline } from "@mdi/js";
</script>

<template>
  <v-icon :icon="mdiSquareOutline" />
</template>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--suppress", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file).unwrap().read_to_string(&mut buffer).unwrap();

    // Verify no suppression comment was added - the import should not be flagged
    // as unused since it's used in the template
    assert!(
        !buffer.contains("biome-ignore"),
        "Suppress should not add comments for imports used in templates. File content:\n{}",
        buffer
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_does_not_add_comments_for_imports_used_in_templates",
        fs,
        console,
        result,
    ));
}
