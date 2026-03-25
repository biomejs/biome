use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn should_apply_tailwind_shorthand_fixes_in_javascript() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file = Utf8Path::new("file.js");
    fs.insert(js_file.into(), r#"clsx("ml-2 mr-2");"#.as_bytes());

    fs.insert(
        Utf8Path::new("biome.json").into(),
        br#"{
    "javascript": {
        "experimentalEmbeddedSnippetsEnabled": true,
        "linter": {
            "enabled": true
        }
    },
    "linter": {
        "rules": {
            "recommended": false,
            "nursery": {
                "useTailwindShorthandClasses": "error"
            }
        }
    }
}"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", js_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, js_file, r#"clsx("mx-2");"#);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_apply_tailwind_shorthand_fixes_in_javascript",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_apply_tailwind_shorthand_fixes_in_html() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let html_file = Utf8Path::new("file.html");
    fs.insert(html_file.into(), br#"<div class="ml-2 mr-2"></div>"#);

    fs.insert(
        Utf8Path::new("biome.json").into(),
        br#"{
    "html": {
        "experimentalFullSupportEnabled": true,
        "linter": {
            "enabled": true
        }
    },
    "linter": {
        "rules": {
            "recommended": false,
            "nursery": {
                "useTailwindShorthandClasses": "error"
            }
        }
    }
}"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", html_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, html_file, r#"<div class="mx-2"></div>"#);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_apply_tailwind_shorthand_fixes_in_html",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_apply_tailwind_shorthand_fixes_in_svelte() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file.into(),
        br#"<div class={clsx("ml-2 mr-2")}></div>"#,
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        br#"{
    "html": {
        "experimentalFullSupportEnabled": true,
        "linter": {
            "enabled": true
        }
    },
    "linter": {
        "rules": {
            "recommended": false,
            "nursery": {
                "useTailwindShorthandClasses": "error"
            }
        }
    }
}"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", svelte_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, svelte_file, r#"<div class={clsx("mx-2")}></div>"#);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_apply_tailwind_shorthand_fixes_in_svelte",
        fs,
        console,
        result,
    ));
}
