use crate::TestArgs as Args;
use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;

const CONFIG: &str = r#"{
    "html": {
        "experimentalFullSupportEnabled": true,
        "formatter": { "enabled": true }
    },
    "linter": { "enabled": false },
    "assist": { "enabled": false }
}"#;

const INPUT: &str = r#"---
import Card from "./Card.astro";
const title="Hello";
const items=[{id:1,label:"One"}];
---
<!doctype html>
<html lang="en"><head><title>{title}</title></head><body><Card title={title}/><><ul class:list={["items",{active:items.length>0}]}>{items.map((item)=><li data-id={item.id}>{item.label}</li>)}</ul></><script>document.documentElement.dataset.ready="true";</script><style>.items{display:grid}</style><pre is:raw>{title}</pre></body></html>
"#;

const DOCTYPE_AFTER_SCRIPT: &str = r#"---
const title="Astro";
---
<script is:inline>console.log("ready");</script>
<!doctype html>
<html lang="en"><head><title>{title}</title></head><body></body></html>
"#;

#[test]
fn astro_full_support_formatting_is_idempotent() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), CONFIG.as_bytes());
    fs.insert("stability.astro".into(), INPUT.as_bytes());
    let mut console = BufferConsole::default();

    let (fs, first_format_result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", "stability.astro"].as_slice()),
    );
    assert!(
        first_format_result.is_ok(),
        "first format returned {first_format_result:?}\n{console:#?}"
    );

    let (fs, check_result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "stability.astro"].as_slice()),
    );
    assert!(
        check_result.is_ok(),
        "check returned {check_result:?}\n{console:#?}"
    );

    let (fs, second_format_result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", "stability.astro"].as_slice()),
    );
    assert!(
        second_format_result.is_ok(),
        "second format returned {second_format_result:?}\n{console:#?}"
    );

    let result = first_format_result
        .followed_by(check_result)
        .followed_by(second_format_result);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "astro_full_support_formatting_is_idempotent",
        fs,
        console,
        result,
    ));
}

#[test]
fn astro_full_support_accepts_doctype_after_script() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), CONFIG.as_bytes());
    fs.insert(
        "doctype-after-script.astro".into(),
        DOCTYPE_AFTER_SCRIPT.as_bytes(),
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "doctype-after-script.astro"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}\n{console:#?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "astro_full_support_accepts_doctype_after_script",
        fs,
        console,
        result,
    ));
}
