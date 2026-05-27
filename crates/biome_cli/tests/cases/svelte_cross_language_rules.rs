use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const BIOME_CONFIG_HTML_FULL_SUPPORT: &str =
    r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#;

/// Imports and variables referenced only in the template must not be flagged as
/// unused. Covers component tags, `{expr}`, member access, `{#each ... as ...}`,
/// and `on:` event-handler bindings.
#[test]
fn no_unused_imports_or_variables_for_template_references() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
import Button from "./Button.svelte";
import type { User } from "./types";
import { formatName } from "./utils";

let count: number = 0;
let user: User = { name: "alice" };
const items = ["a", "b"];
function handleClick() { count++; }
</script>

<Button on:click={handleClick}>
  Count: {count}, name: {formatName(user.name)}
</Button>

{#each items as item}
  <span>{item}</span>
{/each}
"#
        .as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--only=noUnusedImports",
                "--only=noUnusedVariables",
                "--only=useImportType",
                file.as_str(),
            ]
            .as_slice(),
        ),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_imports_or_variables_for_template_references",
        fs,
        console,
        result,
    ));
}

/// Truly-unused imports and `let`/`const` bindings inside the `<script>` block
/// must still be flagged — they used to be silently suppressed because the
/// script's own bindings were registered into the embedded-bindings set.
#[test]
fn no_unused_imports_flags_truly_unused_script_bindings() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
import { unusedImport } from "./other";
let unusedLet = 42;

import Button from "./Button.svelte";
let name = "alice";
</script>

<Button>{name}</Button>
"#
        .as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--only=noUnusedImports",
                "--only=noUnusedVariables",
                file.as_str(),
            ]
            .as_slice(),
        ),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_imports_flags_truly_unused_script_bindings",
        fs,
        console,
        result,
    ));
}

/// `useImportType` fires when a value import is used only as a TypeScript type
/// annotation, even when other imports in the same script ARE referenced in
/// the template.
#[test]
fn use_import_type_fires_on_type_only_value_import() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
import { User } from "./types";
import Button from "./Button.svelte";

let user: User = { name: "alice" };
</script>

<Button>{user.name}</Button>
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
        "use_import_type_fires_on_type_only_value_import",
        fs,
        console,
        result,
    ));
}

/// `useImportType` must NOT fire when the value import is referenced in the
/// template — `<Button />` and `{capitalize(name)}` both count as template
/// value references through `EmbeddedValueReferences`.
#[test]
fn use_import_type_quiet_for_template_referenced_imports() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
import Button from "./Button.svelte";
import { capitalize } from "./utils";

let name = "alice";
</script>

<Button>{capitalize(name)}</Button>
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
        "use_import_type_quiet_for_template_referenced_imports",
        fs,
        console,
        result,
    ));
}

/// `{...spread}` attribute usage references the binding even though the JS
/// semantic model can't see it. Without this support `let props = {...}` would
/// be falsely flagged as unused.
#[test]
fn spread_attribute_counts_as_template_reference() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
const props = { id: "x", "data-test": "y" };
</script>

<input {...props} />
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
        "spread_attribute_counts_as_template_reference",
        fs,
        console,
        result,
    ));
}

/// `{expr}` interpolations embedded inside quoted attribute values
/// (`style="top: {top}px"`, `class="a {cls} b"`) reference their bindings. The
/// parser stores these as opaque string tokens, so they must be scanned out.
#[test]
fn attribute_string_interpolation_counts_as_reference() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
let top = 0;
let cls = "active";
</script>

<div style="top: {top}px" class="card {cls}"></div>
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
        "attribute_string_interpolation_counts_as_reference",
        fs,
        console,
        result,
    ));
}

/// Directive names that resolve to imported values — `use:action`,
/// `transition:fn`, `in:fn`, `out:fn`, `animate:fn` — count as references even
/// when the directive has no `={...}` initializer.
#[test]
fn directive_names_count_as_references() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
import { inView } from "./actions";
import { fade } from "./transitions";
</script>

<div use:inView transition:fade>hi</div>
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
        "directive_names_count_as_references",
        fs,
        console,
        result,
    ));
}

/// Shorthand `bind:open` (no `={...}`) reads the local variable named by the
/// property — it counts as a reference.
#[test]
fn bind_shorthand_counts_as_reference() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
import Dialog from "./Dialog.svelte";
let open = true;
</script>

<Dialog bind:open />
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
        "bind_shorthand_counts_as_reference",
        fs,
        console,
        result,
    ));
}

/// A `{#snippet foo(param)}` parameter used only inside the snippet body must
/// not be flagged by `noUnusedFunctionParameters`.
#[test]
fn snippet_parameter_used_in_body_is_not_unused() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"<script lang="ts">
let show = true;
</script>

{#if show}
  {#snippet card(label: string)}
    <span>{label}</span>
  {/snippet}
{/if}
"#
        .as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUnusedFunctionParameters", file.as_str()].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "snippet_parameter_used_in_body_is_not_unused",
        fs,
        console,
        result,
    ));
}
