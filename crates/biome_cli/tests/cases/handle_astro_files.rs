use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

const ASTRO_FILE_UNFORMATTED: &str = r#"---
import {    something } from "file.astro";

statement ( ) ;

---
<div></div>"#;

const ASTRO_FILE_FORMATTED: &str = r#"---
import { something } from "file.astro";

statement();
---
<div></div>"#;

const ASTRO_FILE_DEBUGGER_BEFORE: &str = r#"---
debugger;
---
<div></div>"#;

const ASTRO_FILE_DEBUGGER_AFTER: &str = r#"---
---
<div></div>"#;

const ASTRO_FILE_IMPORTS_BEFORE: &str = r#"---
import { getLocale } from "astro:i18n";
import { Code } from "astro:components";
---
<div></div>"#;

const ASTRO_FILE_IMPORTS_AFTER: &str = r#"---
import { Code } from "astro:components";
import { getLocale } from "astro:i18n";
---
<div></div>"#;

#[test]
fn format_astro_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), astro_file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, astro_file_path, ASTRO_FILE_UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_astro_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_astro_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                astro_file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, astro_file_path, ASTRO_FILE_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_astro_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_empty_astro_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Path::new("file.astro");
    fs.insert(astro_file_path.into(), "<div></div>".as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                astro_file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, astro_file_path, "<div></div>");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_empty_astro_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_astro_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_FILE_DEBUGGER_BEFORE.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), astro_file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_astro_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_and_fix_astro_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_FILE_DEBUGGER_BEFORE.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                "--apply-unsafe",
                astro_file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, astro_file_path, ASTRO_FILE_DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_and_fix_astro_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn sorts_imports_check() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_IMPORTS_BEFORE.as_bytes());

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

    assert_file_contents(&fs, astro_file_path, ASTRO_FILE_IMPORTS_BEFORE);

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

    let astro_file_path = Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_IMPORTS_BEFORE.as_bytes());

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

    assert_file_contents(&fs, astro_file_path, ASTRO_FILE_IMPORTS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "sorts_imports_write",
        fs,
        console,
        result,
    ));
}
