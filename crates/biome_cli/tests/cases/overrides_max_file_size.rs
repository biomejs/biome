use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_cli::CliDiagnostic;
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const FORMATTED: &str = "statement();\n";
const UNFORMATTED: &str = "  statement(  )  ";
const TEST_FILE: &str = "test.js";
const CLI_COMMANDS: [&str; 2] = ["check", "format"];

fn setup_test(
    includes: &str,
    max_size: u64,
    cli_command: &str,
    file_content: Option<&str>,
) -> (MemoryFileSystem, BufferConsole, Result<(), CliDiagnostic>) {
    assert!(
        CLI_COMMANDS.contains(&cli_command),
        "Command must be one of {:?}",
        CLI_COMMANDS.join(", ")
    );

    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    let file_content = file_content.unwrap_or(FORMATTED);

    fs.insert(
        file_path.into(),
        format!(
            r#"{{
                "files": {{
                    "maxSize": 1024
                 }},
                "overrides": [
                    {{
                        "includes": [
                            "{includes}"
                        ],
                        "files": {{ "maxSize": {max_size} }}
                    }}
                ]
            }}"#
        )
        .as_bytes(),
    );

    let test_file = Utf8Path::new(TEST_FILE);
    fs.insert(test_file.into(), file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from([cli_command, test_file.as_str()].as_slice()),
    );

    (fs, console, result)
}

#[test]
fn overrides_files_max_size_option_pass() {
    for cli_command in CLI_COMMANDS {
        let (fs, console, result) = setup_test(TEST_FILE, 1024, cli_command, None);

        assert!(result.is_ok(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "overrides_files_max_size_option_pass",
            fs,
            console,
            result,
        ));
    }
}

#[test]
fn overrides_files_max_size_option_invalid_value() {
    for cli_command in CLI_COMMANDS {
        let (fs, console, result) = setup_test(TEST_FILE, 0, cli_command, None);

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "overrides_files_max_size_option_invalid_value",
            fs,
            console,
            result,
        ));
    }
}

#[test]
fn overrides_files_max_size_too_large_limit() {
    for cli_command in CLI_COMMANDS {
        let (fs, console, result) = setup_test(TEST_FILE, 1, cli_command, None);

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            format!("overrides_files_max_size_too_large_limit_{cli_command}").as_str(),
            fs,
            console,
            result,
        ));
    }
}

#[test]
fn overrides_files_max_size_ignored_includes_does_not_match_filename() {
    for cli_command in CLI_COMMANDS {
        let (fs, console, result) = setup_test("invalidFile.js", 1, cli_command, None);

        assert!(result.is_ok(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "overrides_files_max_size_ignored_includes_does_not_match_filename",
            fs,
            console,
            result,
        ));
    }
}

#[test]
fn overrides_files_max_size_ignored_includes_does_not_match_filename_invalid_format() {
    for cli_command in CLI_COMMANDS {
        let (fs, console, result) =
            setup_test("invalidFile.js", 1, cli_command, Option::Some(UNFORMATTED));

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            format!("overrides_files_max_size_ignored_includes_does_not_match_filename_invalid_format_{cli_command}").as_str(),
            fs,
            console,
            result,
        ));
    }
}
