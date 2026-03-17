use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const BIOME_CONFIG_INDENT: &str = r#"
{
    "html": {
        "formatter": {
            "indentScriptAndStyle": true
        }
    }
}
"#;

const VUE_FILE_UNFORMATTED: &str = r#"<script>
import {    something } from "file.vue";
statement ( ) ;
</script>
<template></template>"#;

#[test]
fn unindent_vue_by_default() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unindent_vue_by_default",
        fs,
        console,
        result,
    ));
}

#[test]
fn indent_vue_by_cli() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--html-formatter-indent-script-and-style=true",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_vue_by_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn indent_vue_by_config() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_FILE_UNFORMATTED.as_bytes());
    let biome_config = Utf8Path::new("biome.json");
    fs.insert(biome_config.into(), BIOME_CONFIG_INDENT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_vue_by_config",
        fs,
        console,
        result,
    ));
}

// A properly formatted biome.json with indentScriptAndStyle enabled.
// Uses tab indentation (Biome's default) so it won't be reformatted.
const BIOME_CONFIG_INDENT_FORMATTED: &str =
    "{\n\t\"html\": {\n\t\t\"formatter\": {\n\t\t\t\"indentScriptAndStyle\": true\n\t\t}\n\t}\n}\n";

// An already-formatted Svelte file with indentScriptAndStyle indentation applied.
// Running `check --write` on this should NOT report any changes.
const SVELTE_FILE_ALREADY_FORMATTED: &str =
    "<script lang=\"ts\">\n\tconst line1 = \"fail\";\n\tconsole.log(line1);\n</script>\n";

#[test]
fn check_write_svelte_indent_is_idempotent() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_FILE_ALREADY_FORMATTED.as_bytes(),
    );
    let biome_config = Utf8Path::new("biome.json");
    fs.insert(
        biome_config.into(),
        BIOME_CONFIG_INDENT_FORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_write_svelte_indent_is_idempotent",
        fs,
        console,
        result,
    ));
}

// Same test for Vue
const VUE_FILE_ALREADY_FORMATTED: &str = "<script>\n\tconst line1 = \"fail\";\n\tconsole.log(line1);\n</script>\n<template></template>\n";

#[test]
fn check_write_vue_indent_is_idempotent() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let vue_file_path = Utf8Path::new("file.vue");
    fs.insert(vue_file_path.into(), VUE_FILE_ALREADY_FORMATTED.as_bytes());
    let biome_config = Utf8Path::new("biome.json");
    fs.insert(
        biome_config.into(),
        BIOME_CONFIG_INDENT_FORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_write_vue_indent_is_idempotent",
        fs,
        console,
        result,
    ));
}

const SVELTE_FILE_UNFORMATTED: &str = r#"<script>
import {    something } from "file.svelte";
statement ( ) ;
</script>
<div></div>"#;

#[test]
fn unindent_svelte_by_default() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(svelte_file_path.into(), SVELTE_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_svelte_by_default",
        fs,
        console,
        result,
    ));
}

#[test]
fn indent_svelte_by_cli() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(svelte_file_path.into(), SVELTE_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--html-formatter-indent-script-and-style=true",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_svelte_by_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn indent_svelte_by_config() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(svelte_file_path.into(), SVELTE_FILE_UNFORMATTED.as_bytes());
    let biome_config = Utf8Path::new("biome.json");
    fs.insert(biome_config.into(), BIOME_CONFIG_INDENT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_svelte_by_config",
        fs,
        console,
        result,
    ));
}
