use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const LINTER_CONFIG: &str =
    r#"{ "html": { "linter": { "enabled": true }, "experimentalFullSupportEnabled": true } }"#;

#[test]
fn lints_dom_style_attributes_as_css() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(config.into(), LINTER_CONFIG.as_bytes());

    let html_file = Utf8Path::new("index.html");
    fs.insert(
        html_file.into(),
        r#"<div style="colr: blue"></div>
<my-element style="colr: blue"></my-element>
"#
        .as_bytes(),
    );

    let vue_file = Utf8Path::new("component.vue");
    fs.insert(
        vue_file.into(),
        r#"<template><Button style="colr: blue" /></template>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", html_file.as_str(), vue_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lints_dom_style_attributes_as_css",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_lint_component_style_props_as_css() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(config.into(), LINTER_CONFIG.as_bytes());

    let svelte_file = Utf8Path::new("component.svelte");
    fs.insert(
        svelte_file.into(),
        r#"<Button style="colr: blue" />
"#
        .as_bytes(),
    );

    let astro_file = Utf8Path::new("component.astro");
    fs.insert(
        astro_file.into(),
        r#"<Button style="colr: blue" />
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", svelte_file.as_str(), astro_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_lint_component_style_props_as_css",
        fs,
        console,
        result,
    ));
}
