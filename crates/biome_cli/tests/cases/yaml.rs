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
const BRACKET_SPACING: &str = include_str!("../specs/yaml/bracket_spacing.yaml");

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

#[test]
fn yaml_bracket_spacing_overrides() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let config = r#"{
        "yaml": { "formatter": { "bracketSpacing": false } },
        "overrides": [
            {
                "includes": ["special/**"],
                "formatter": { "bracketSpacing": true }
            },
            {
                "includes": ["special/compact.yml"],
                "formatter": { "bracketSpacing": true },
                "yaml": { "formatter": { "bracketSpacing": false } }
            }
        ]
    }"#;
    fs.insert("biome.json".into(), config.as_bytes());
    for file in ["file.yaml", "special/file.yml", "special/compact.yml"] {
        fs.insert(file.into(), BRACKET_SPACING.as_bytes());
    }

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "file.yaml",
                "special/file.yml",
                "special/compact.yml",
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "yaml_bracket_spacing_overrides",
        fs,
        console,
        result,
    ));
}

#[test]
fn yaml_bracket_spacing_cli_option() {
    for (scope, flag) in [
        ("global", "--bracket-spacing"),
        ("yaml", "--yaml-formatter-bracket-spacing"),
    ] {
        for value in ["true", "false"] {
            for command in ["format", "check", "ci"] {
                let fs = MemoryFileSystem::default();
                let mut console = BufferConsole::default();
                let path = Utf8Path::new("file.yaml");
                fs.insert(path.into(), BRACKET_SPACING.as_bytes());
                let mut args = vec![command, flag, value, path.as_str()];
                if command != "ci" {
                    args.push("--write");
                }

                let (fs, result) = run_cli(fs, &mut console, Args::from(args.as_slice()));

                assert_cli_snapshot(SnapshotPayload::new(
                    module_path!(),
                    &format!("{command}_yaml_bracket_spacing_{scope}_{value}"),
                    fs,
                    console,
                    result,
                ));
            }
        }
    }
}
