use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const UNFORMATTED: &str = "#   Heading\n\n\n##   Section";
const FORMATTED: &str = "# Heading\n\n## Section\n";
const UNFORMATTED_PROSE: &str = "This is a long paragraph with enough words to exceed the configured line width and require wrapping.";
const PROSE_WRAP_NEVER: &str = "This is a long paragraph with enough words to exceed the configured line width and require wrapping.\n";
const PROSE_WRAP_ALWAYS: &str = "This is a long paragraph with enough\nwords to exceed the configured line\nwidth and require wrapping.\n";

#[test]
fn format_markdown_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.md");
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
        "format_markdown_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_and_write_markdown_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.md");
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
        "format_and_write_markdown_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_markdown_files_with_nursery_rules() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
    "markdown": {
        "linter": {
            "enabled": true
        }
    },
    "linter": {
        "enabled": false,
        "rules": {
            "nursery": {
                "useTopLevelHeading": "error"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), b"## Second level heading\n");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_markdown_files_with_nursery_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_files_with_prose_wrap_override() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
    "markdown": {
        "formatter": {
            "proseWrap": "never"
        }
    },
    "overrides": [
        {
            "includes": ["special/**"],
            "markdown": {
                "formatter": {
                    "lineWidth": 40,
                    "proseWrap": "always"
                }
            }
        }
    ]
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), UNFORMATTED_PROSE.as_bytes());

    let overridden_file_path = Utf8Path::new("special/file.md");
    fs.insert(overridden_file_path.into(), UNFORMATTED_PROSE.as_bytes());

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
    assert_file_contents(&fs, file_path, PROSE_WRAP_NEVER);
    assert_file_contents(&fs, overridden_file_path, PROSE_WRAP_ALWAYS);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_files_with_prose_wrap_override",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_files_with_prose_wrap_cli_option() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), UNFORMATTED_PROSE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--markdown-formatter-line-width",
                "40",
                "--markdown-formatter-prose-wrap",
                "always",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, PROSE_WRAP_ALWAYS);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_files_with_prose_wrap_cli_option",
        fs,
        console,
        result,
    ));
}
