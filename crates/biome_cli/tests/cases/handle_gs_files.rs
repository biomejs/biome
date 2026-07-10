use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

// Google Apps Script uses the `.gs` extension and is plain JavaScript running in
// Google's runtime (no module system, extra service globals).

const GS_UNFORMATTED: &str = "  statement(  )  ";
const GS_FORMATTED: &str = "statement();\n";

// `noUndeclaredVariables` is not recommended, so it must be enabled explicitly.
const NO_UNDECLARED_CONFIG: &str = r#"{
    "linter": {
        "rules": {
            "correctness": {
                "noUndeclaredVariables": "error"
            }
        }
    }
}"#;

const GS_GLOBALS_CODE: &str = r#"SpreadsheetApp.getActiveSpreadsheet();
Logger.log("hello");
notAGlobal();
"#;

const GS_REASSIGN_CODE: &str = r#"SpreadsheetApp = null;
Logger = 1;
localVar = 2;
"#;

const GS_MODULE_SYNTAX_CODE: &str = r#"import { x } from "./y";
export const z = 1;
"#;

const GS_CHECK_CODE: &str = "SpreadsheetApp   =   null ;\n";

#[test]
fn format_gs_file() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("Code.gs");
    fs.insert(file_path.into(), GS_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, GS_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_gs_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_gs_globals_are_recognized() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert("biome.json".into(), NO_UNDECLARED_CONFIG.as_bytes());

    let file_path = Utf8Path::new("Code.gs");
    fs.insert(file_path.into(), GS_GLOBALS_CODE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    // Only `notAGlobal` is reported; `SpreadsheetApp`/`Logger` are recognized.
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_gs_globals_are_recognized",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_gs_globals_are_not_leaked_to_cjs() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert("biome.json".into(), NO_UNDECLARED_CONFIG.as_bytes());

    // The same code in a `.cjs` file: the Apps Script globals must be reported,
    // proving they are scoped to `.gs` files only.
    let file_path = Utf8Path::new("code.cjs");
    fs.insert(file_path.into(), GS_GLOBALS_CODE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_gs_globals_are_not_leaked_to_cjs",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_gs_flags_global_reassignment() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // `noGlobalAssign` is recommended: reassigning a service global is reported,
    // while assigning to a non-global identifier is not.
    let file_path = Utf8Path::new("Code.gs");
    fs.insert(file_path.into(), GS_REASSIGN_CODE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_gs_flags_global_reassignment",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_gs_rejects_module_syntax() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Apps Script has no module system, so `import`/`export` are parse errors.
    let file_path = Utf8Path::new("Code.gs");
    fs.insert(file_path.into(), GS_MODULE_SYNTAX_CODE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_gs_rejects_module_syntax",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_gs_file() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // `check` runs the formatter and the linter: the file is both misformatted
    // and reassigns a service global.
    let file_path = Utf8Path::new("Code.gs");
    fs.insert(file_path.into(), GS_CHECK_CODE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_gs_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn search_gs_file() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // `.gs` files are searchable as JavaScript.
    let file_path = Utf8Path::new("Code.gs");
    fs.insert(file_path.into(), GS_GLOBALS_CODE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["search", "`SpreadsheetApp`", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "search_gs_file",
        fs,
        console,
        result,
    ));
}
