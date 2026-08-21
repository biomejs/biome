use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const UNFORMATTED: &str = "items: [  1 , 2 ]";
const FORMATTED: &str = "items: [1, 2]\n";
const UNFORMATTED_NESTED: &str = "root:\n - item";
const TWO_SPACE_INDENT: &str = "root:\n  - item\n";
const FOUR_SPACE_INDENT: &str = "root:\n    - item\n";

#[test]
fn format_yaml_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.yaml");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_yaml_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_and_write_yaml_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.yaml");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_and_write_yaml_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_yaml_files_with_indent_width_override() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
    "yaml": {
        "formatter": {
            "indentWidth": 2
        }
    },
    "overrides": [
        {
            "includes": ["special/**"],
            "yaml": {
                "formatter": {
                    "indentWidth": 4
                }
            }
        }
    ]
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.yaml");
    fs.insert(file_path.into(), UNFORMATTED_NESTED.as_bytes());

    let overridden_file_path = Utf8Path::new("special/file.yml");
    fs.insert(overridden_file_path.into(), UNFORMATTED_NESTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                file_path.as_str(),
                overridden_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, TWO_SPACE_INDENT);
    assert_file_contents(&fs, overridden_file_path, FOUR_SPACE_INDENT);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_yaml_files_with_indent_width_override",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_yaml_files_with_indent_width_cli_option() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.yaml");
    fs.insert(file_path.into(), UNFORMATTED_NESTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--yaml-formatter-indent-width",
                "4",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, FOUR_SPACE_INDENT);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_yaml_files_with_indent_width_cli_option",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_yaml_files_with_indent_width_cli_option() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.yaml");
    fs.insert(file_path.into(), FOUR_SPACE_INDENT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--yaml-formatter-indent-width",
                "4",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, FOUR_SPACE_INDENT);
}

#[test]
fn ci_yaml_files_with_indent_width_cli_option() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.yaml");
    fs.insert(file_path.into(), FOUR_SPACE_INDENT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "ci",
                "--yaml-formatter-indent-width",
                "4",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, FOUR_SPACE_INDENT);
}
