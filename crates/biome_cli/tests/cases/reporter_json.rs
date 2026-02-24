use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const MAIN_1: &str = r#"import { z} from "z"
import { z, b , a} from "lodash"

a ==b

debugger

let f;
		let f;"#;

const MAIN_2: &str = r#"import { z} from "z"
import { z, b , a} from "lodash"

a ==b

debugger

let f;
		let f;"#;

#[test]
fn reports_diagnostics_json_check_command() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Utf8Path::new("main.ts");
    fs.insert(file_path1.into(), MAIN_1.as_bytes());

    let file_path2 = Utf8Path::new("index.ts");
    fs.insert(file_path2.into(), MAIN_2.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--reporter=json-pretty",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "reports_diagnostics_json_check_command",
        fs,
        console,
        result,
    ));
}

#[test]
fn reports_diagnostics_json_ci_command() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Utf8Path::new("main.ts");
    fs.insert(file_path1.into(), MAIN_1.as_bytes());

    let file_path2 = Utf8Path::new("index.ts");
    fs.insert(file_path2.into(), MAIN_2.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "ci",
                "--reporter=json-pretty",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "reports_diagnostics_json_ci_command",
        fs,
        console,
        result,
    ));
}

#[test]
fn reports_diagnostics_json_lint_command() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Utf8Path::new("main.ts");
    fs.insert(file_path1.into(), MAIN_1.as_bytes());

    let file_path2 = Utf8Path::new("index.ts");
    fs.insert(file_path2.into(), MAIN_2.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--reporter=json-pretty",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "reports_diagnostics_json_lint_command",
        fs,
        console,
        result,
    ));
}

#[test]
fn reports_diagnostics_json_format_command() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Utf8Path::new("main.ts");
    fs.insert(file_path1.into(), MAIN_1.as_bytes());

    let file_path2 = Utf8Path::new("index.ts");
    fs.insert(file_path2.into(), MAIN_2.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--reporter=json-pretty",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "reports_diagnostics_json_format_command",
        fs,
        console,
        result,
    ));
}

#[test]
fn reports_diagnostics_json_check_command_file() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Utf8Path::new("main.ts");
    fs.insert(file_path1.into(), MAIN_1.as_bytes());

    let file_path2 = Utf8Path::new("index.ts");
    fs.insert(file_path2.into(), MAIN_2.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--reporter=json-pretty",
                "--reporter-file=file.json",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "reports_diagnostics_json_check_command_file",
        fs,
        console,
        result,
    ));
}
