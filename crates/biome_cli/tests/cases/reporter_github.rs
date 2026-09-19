use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, message_to_string};
use crate::{run_cli, run_cli_with_dyn_fs};
use biome_console::BufferConsole;
use biome_fs::{MemoryFileSystem, TemporaryFs};
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
fn reports_diagnostics_github_check_command() {
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
                "--reporter=github",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "reports_diagnostics_github_check_command",
        fs,
        console,
        result,
    ));
}

#[test]
fn reports_diagnostics_github_ci_command() {
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
                "--reporter=github",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "reports_diagnostics_github_ci_command",
        fs,
        console,
        result,
    ));
}

#[test]
fn reports_diagnostics_github_lint_command() {
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
                "--reporter=github",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "reports_diagnostics_github_lint_command",
        fs,
        console,
        result,
    ));
}

#[test]
fn reports_diagnostics_github_format_command() {
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
                "--reporter=github",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "reports_diagnostics_github_format_command",
        fs,
        console,
        result,
    ));
}

// Don't snapshot, fails on Windows due to path.
#[test]
fn reports_diagnostics_github_from_nested_directory() {
    let mut fs = TemporaryFs::new("reports_diagnostics_github_from_nested_directory");
    let mut console = BufferConsole::default();

    fs.create_file("packages/ui/main.ts", "debugger;\n");
    fs.append_to_working_directory("packages/ui");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--reporter=github", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let output = console
        .out_buffer
        .iter()
        .map(message_to_string)
        .collect::<String>()
        .replace('\\', "/");
    assert!(
        output.contains("/packages/ui/main.ts"),
        "unexpected output: {output}"
    );
}
