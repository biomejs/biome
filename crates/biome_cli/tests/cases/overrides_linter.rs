use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

const FIX_BEFORE: &str = "(1 >= -0)";
const FIX_AFTER: &str = "(1 >= 0)";

const DEBUGGER_BEFORE: &str = "debugger";
const DEBUGGER_AFTER: &str = "";

const SIMPLE_NUMBERS_BEFORE: &str = "({ 0x1: 1 });";
const SIMPLE_NUMBERS_AFTER: &str = "({ 1: 1 });";
#[test]
fn does_not_handle_ignored_file() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "include": ["test.js", "special/**"]
  },
  "overrides": [{ "ignore": ["special/**"] }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), FIX_BEFORE.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), FIX_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FIX_BEFORE);
    assert_file_contents(&fs, test, FIX_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_handle_ignored_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_handle_included_file() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "ignore": ["test.js", "special/**"]
  },
  "overrides": [{ "include": ["special/**"] }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), FIX_BEFORE.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), FIX_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FIX_AFTER);
    assert_file_contents(&fs, test, FIX_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_handle_included_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_rules() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [{ "include": ["special/**"], "linter": { "rules": {
    "suspicious": { "noDebugger": "off" }
  } } }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_linting_and_applies_the_first_one() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
    "overrides": [
        {
            "include": [
                "special/**"
            ],
            "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "off"
                    }
                }
            }
        },
        {
            "include": [
                "special/**"
            ],
            "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "error"
                    }
                }
            }
        }
    ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_formatting_and_applies_the_first_one",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_overrides() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
    "overrides": [
        {
            "include": [
                "test.js"
            ],
            "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "off"
                    }
                }
            }
        },
        {
            "include": [
                "test2.js"
            ],
            "linter": {
                "rules": {
                    "complexity": {
                        "useSimpleNumberKeys": "error"
                    }
                }
            }
        }
    ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), SIMPLE_NUMBERS_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test2, SIMPLE_NUMBERS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_overrides",
        fs,
        console,
        result,
    ));
}
