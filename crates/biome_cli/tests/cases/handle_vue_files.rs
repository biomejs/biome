use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

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

#[test]
fn format_vue_implicit_js_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        VUE_IMPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), vue_file_path.as_os_str().to_str().unwrap()].as_slice()),
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

    let vue_file_path = Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        VUE_IMPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                vue_file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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

    let vue_file_path = Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        VUE_EXPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), vue_file_path.as_os_str().to_str().unwrap()].as_slice()),
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

    let vue_file_path = Path::new("file.vue");
    fs.insert(
        vue_file_path.into(),
        VUE_EXPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                vue_file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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

    let vue_file_path = Path::new("file.vue");
    fs.insert(vue_file_path.into(), "<template></template>".as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                vue_file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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

    let vue_file_path = Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_TS_FILE_UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), vue_file_path.as_os_str().to_str().unwrap()].as_slice()),
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

    let vue_file_path = Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_TS_FILE_UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                vue_file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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

    let vue_file_path = Path::new("file.vue");
    fs.insert(vue_file_path.into(), "<template></template>".as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                vue_file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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
fn lint_vue_js_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_JS_FILE_NOT_LINTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), vue_file_path.as_os_str().to_str().unwrap()].as_slice()),
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

    let vue_file_path = Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_TS_FILE_NOT_LINTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), vue_file_path.as_os_str().to_str().unwrap()].as_slice()),
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

    let vue_file_path = Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_FILE_IMPORTS_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                "--formatter-enabled=false",
                "--linter-enabled=false",
                vue_file_path.as_os_str().to_str().unwrap(),
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

    let vue_file_path = Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_FILE_IMPORTS_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                "--formatter-enabled=false",
                "--linter-enabled=false",
                "--apply",
                vue_file_path.as_os_str().to_str().unwrap(),
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
