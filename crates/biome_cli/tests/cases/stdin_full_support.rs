use crate::run_cli;
use crate::snap_test::markup_to_string;
use biome_console::{BufferConsole, markup};
use biome_fs::MemoryFileSystem;
use bpaf::Args;

const CONFIG: &str = r#"{
    "html": {
        "experimentalFullSupportEnabled": true,
        "formatter": { "enabled": true }
    },
    "linter": {
        "rules": { "style": { "useImportType": "on" } }
    }
}"#;

const CASES: &[(&str, &str)] = &[
    (
        "+page.svelte",
        r#"<script lang="ts">
import Modal from "./Modal.svelte"
let myModal: Modal
</script>
<Modal bind:this={myModal} />"#,
    ),
    (
        "Component.vue",
        r#"<script setup lang="ts">
import Modal from "./Modal.vue"
let myModal: Modal
</script>
<template><Modal ref="myModal" /></template>"#,
    ),
    (
        "Component.astro",
        r#"---
import Modal from "./Modal.astro"
let myModal: Modal
---
<Modal />"#,
    ),
];

#[test]
fn stdin_full_support_uses_framework_markup_references() {
    for &(path, input) in CASES {
        let fs = MemoryFileSystem::default();
        fs.insert("biome.json".into(), CONFIG.as_bytes());
        let mut console = BufferConsole::default();
        console.in_buffer.push(input.to_string());

        let (_, result) = run_cli(
            fs,
            &mut console,
            Args::from(
                [
                    "check",
                    "--write",
                    "--formatter-enabled=false",
                    "--stdin-file-path",
                    path,
                ]
                .as_slice(),
            ),
        );

        assert!(result.is_ok(), "run_cli returned {result:?} for {path}");
        let message = console
            .out_buffer
            .first()
            .unwrap_or_else(|| panic!("Console should have written a message for {path}"));
        let content = markup_to_string(markup! {
            {message.content}
        });
        assert!(content.contains("import Modal from"), "{path}: {content}");
        assert!(
            !content.contains("import type Modal from"),
            "{path}: {content}"
        );
    }
}

#[test]
fn stdin_full_support_uses_vue_custom_directive_references() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), CONFIG.as_bytes());
    let mut console = BufferConsole::default();
    console.in_buffer.push(
        r#"<script setup>
import vClickOutside from "./v-click-outside.js"
</script>
<template><div v-click-outside /></template>"#
            .to_string(),
    );

    let (_, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--write",
                "--unsafe",
                "--only=correctness/noUnusedImports",
                "--stdin-file-path",
                "Component.vue",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");
    let content = markup_to_string(markup! {
        {message.content}
    });
    assert!(content.contains("import vClickOutside from"), "{content}");
}

#[test]
fn stdin_full_support_formats_vue_markup() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), CONFIG.as_bytes());
    let mut console = BufferConsole::default();
    console.in_buffer.push(
        r#"<script setup>const value=1;</script>
<template><div class="foo">{{ value }}</div></template>"#
            .to_string(),
    );

    let (_, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path", "Component.vue"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");
    let content = markup_to_string(markup! {
        {message.content}
    });
    assert!(content.contains("<template>\n"), "{content}");
}
