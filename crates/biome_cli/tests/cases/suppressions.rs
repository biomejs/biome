use crate::snap_test::SnapshotPayload;
use crate::{FORMATTED, assert_cli_snapshot, run_cli};
use biome_console::BufferConsole;
use biome_fs::{FileSystemExt, MemoryFileSystem};
use bpaf::Args;
use camino::Utf8Path;

const SUPPRESS_BEFORE: &str = "(1 >= -0)";
const SUPPRESS_AFTER: &str =
    "// biome-ignore lint/suspicious/noCompareNegZero: ignored using `--suppress`\n(1 >= -0)";

const SUPPRESS_WITH_REASON: &str =
    "// biome-ignore lint/suspicious/noCompareNegZero: We love Biome\n(1 >= -0)";

#[test]
fn ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("check.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let (_, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--suppress", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
}

#[test]
fn err_when_both_write_and_suppress_are_passed() {
    let fs = MemoryFileSystem::new_read_only();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("check.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--suppress", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "err_when_both_write_and_suppress_are_passed",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--suppress", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_multiple_ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(
        file_path.into(),
        [SUPPRESS_BEFORE, SUPPRESS_BEFORE].join("\n").as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--suppress", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, [SUPPRESS_AFTER, SUPPRESS_AFTER].join("\n"));

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_multiple_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_only_ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--suppress",
                "--only=lint/suspicious/noCompareNegZero",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_only_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_skip_ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--suppress",
                "--skip=lint/suspicious/noCompareNegZero",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_skip_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn err_when_only_reason() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--reason", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "err_when_only_reason",
        fs,
        console,
        result,
    ));
}

#[test]
fn custom_explanation_with_reason() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--suppress",
                "--reason=We love Biome",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_WITH_REASON);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "custom_explanation_with_reason",
        fs,
        console,
        result,
    ));
}

#[test]
fn unused_suppression_after_top_level() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(
        file_path.into(),
        *b"/**
* biome-ignore-all lint/style/useConst: reason
*/


let foo = 2;
/**
* biome-ignore lint/style/useConst: reason
*/
let bar = 33;",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unused_suppression_after_top_level",
        fs,
        console,
        result,
    ));
}

#[test]
fn misplaced_top_level_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(
        file_path.into(),
        *b"
let foo = 2;
/**
* biome-ignore-all lint/style/useConst: reason
* biome-ignore-all lint/suspicious/noDebugger: reason
*/
debugger
let bar = 33;",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "misplaced_top_level_suppression",
        fs,
        console,
        result,
    ));
}

#[test]
fn unused_range_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore-all lint/suspicious/noDoubleEquals: single rule
a == b;
// biome-ignore-start lint/suspicious/noDoubleEquals: single rule
a == b;
a == b;
// biome-ignore-end lint/suspicious/noDoubleEquals: single rule",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unused_range_suppression",
        fs,
        console,
        result,
    ));
}

#[test]
fn syntax_rule_line_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore syntax/correctness/noTypeOnlyImportAttributes: bug
import type { ChalkInstance } from \"chalk\" with { \"resolution-mode\": \"import\" };

function sommething(chalk: ChalkInstance) {
  console.log(chalk.yellow('we do something here'));
}",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    if let Result::Err(e) = &result {
        println!("{e:#?}");
    }

    assert!(result.is_ok(), "run_cli returned {result:#?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "syntax_rule_line_suppression",
        fs,
        console,
        result,
    ));
}

#[test]
fn syntax_rule_range_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore-start syntax/correctness/noTypeOnlyImportAttributes: bug
import type { ChalkInstance } from \"chalk\" with { \"resolution-mode\": \"import\" };
import type { ChalkInstance2 } from \"chalk2\" with { \"resolution-mode\": \"import\" };
// biome-ignore-end syntax/correctness/noTypeOnlyImportAttributes: bug

function sommething(chalk: ChalkInstance) {
  console.log(chalk.yellow('we do something here'));
}",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "syntax_rule_range_suppression",
        fs,
        console,
        result,
    ));
}

#[test]
fn syntax_rule_top_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore-all syntax/correctness/noTypeOnlyImportAttributes: bug
import type { ChalkInstance } from \"chalk\" with { \"resolution-mode\": \"import\" };
import type { ChalkInstance2 } from \"chalk2\" with { \"resolution-mode\": \"import\" };

function sommething(chalk: ChalkInstance) {
  console.log(chalk.yellow('we do something here'));
}",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "syntax_rule_top_suppression",
        fs,
        console,
        result,
    ));
}

#[test]
fn err_when_missing_range_end() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore-start syntax/correctness/noTypeOnlyImportAttributes: bug
import type { ChalkInstance } from \"chalk\" with { \"resolution-mode\": \"import\" };
import type { ChalkInstance2 } from \"chalk2\" with { \"resolution-mode\": \"import\" };

function sommething(chalk: ChalkInstance) {
  console.log(chalk.yellow('we do something here'));
}",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "err_when_missing_range_end",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_emit_diagnostics_for_incorrect_reason() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"// biome-ignore-all lint/style/useConst:
var foo = 2;
// biome-ignore-all lint/style/useConst: <explanation>
var bar = 33;",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_emit_diagnostics_for_incorrect_reason",
        fs,
        console,
        result,
    ));
}
