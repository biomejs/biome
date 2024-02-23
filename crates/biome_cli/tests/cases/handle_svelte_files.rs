use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

const SVELTE_FILE_IMPORTS_BEFORE: &str = r#"<script setup lang="ts">
import Button from "./components/Button.svelte";
import * as svelteUse from "svelte-use";
</script>
<div></div>"#;

const SVELTE_FILE_IMPORTS_AFTER: &str = r#"<script setup lang="ts">
import * as svelteUse from "svelte-use";
import Button from "./components/Button.svelte";
</script>
<div></div>"#;

#[test]
fn sorts_imports_check() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Path::new("file.svelte");
    fs.insert(
        astro_file_path.into(),
        SVELTE_FILE_IMPORTS_BEFORE.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                "--formatter-enabled=false",
                "--linter-enabled=false",
                astro_file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, astro_file_path, SVELTE_FILE_IMPORTS_BEFORE);

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

    let astro_file_path = Path::new("file.svelte");
    fs.insert(
        astro_file_path.into(),
        SVELTE_FILE_IMPORTS_BEFORE.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                "--formatter-enabled=false",
                "--linter-enabled=false",
                "--apply",
                astro_file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, astro_file_path, SVELTE_FILE_IMPORTS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "sorts_imports_write",
        fs,
        console,
        result,
    ));
}
