use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use crate::{run_cli, run_cli_with_dyn_fs};
use biome_console::BufferConsole;
use biome_fs::{MemoryFileSystem, TemporaryFs};
use bpaf::Args;
use camino::Utf8Path;

const UNFORMATTED: &str = "  statement(  )  ";
const FORMATTED: &str = "statement();\n";

const FIX_BEFORE: &str = "(1 >= -0)";
const FIX_AFTER: &str = "(1 >= 0)";

const UNORGANIZED: &str = r#"import * as something from "../something";
import { lorem, foom, bar } from "foo";"#;
const ORGANIZED: &str = r#"import { bar, foom, lorem } from "foo";
import * as something from "../something";"#;

#[test]
fn does_handle_only_included_files() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": { "includes": ["test.js"] }
}
"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, UNFORMATTED);

    assert_file_contents(&fs, test, FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_handle_only_included_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_handle_included_files_if_overridden_by_ignore() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{ "files": { "includes": ["test.js", "test2.js", "!test.js"] } }"#.as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FORMATTED);

    assert_file_contents(&fs, test, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_handle_included_files_if_overridden_by_ignore",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_handle_included_files_if_overridden_by_ignore_formatter() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{ "formatter": { "includes": ["test.js", "test2.js", "!test.js"] } }"#.as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FORMATTED);

    assert_file_contents(&fs, test, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_handle_included_files_if_overridden_by_ignore_formatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_handle_included_files_if_overridden_by_ignore_linter() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{ "linter": { "includes": ["test.js", "test2.js", "!test.js"] } }
"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), FIX_BEFORE.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), FIX_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", test.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FIX_AFTER);

    assert_file_contents(&fs, test, FIX_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_handle_included_files_if_overridden_by_ignore_linter",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_handle_included_files_if_overridden_by_organize_imports() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "formatter": { "enabled": false },
            "linter": { "enabled": false },
            "assist": { "includes": ["test.js", "test2.js", "!test.js"] }
        }"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNORGANIZED.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNORGANIZED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", test.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, ORGANIZED);

    assert_file_contents(&fs, test, UNORGANIZED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_handle_included_files_if_overridden_by_organize_imports",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_handle_files_inside_ignored_folder() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("does_not_handle_files_inside_ignored_folder");
    let file_path = Utf8Path::new("biome.json");
    fs.create_file(
        file_path.as_str(),
        r#"{
            "formatter": { "enabled": true, "includes": ["**", "!**/ignored"] },
            "linter": { "enabled": false }
        }"#,
    );

    let test = Utf8Path::new("test.js");
    fs.create_file(test.as_str(), UNFORMATTED);

    let test2 = Utf8Path::new("ignored/test2.js");
    fs.create_file(test2.as_str(), UNFORMATTED);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["check", "--write", fs.cli_path()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let working_dir = fs.working_directory;

    let test_content = std::fs::read_to_string(working_dir.join(test)).unwrap();
    assert_eq!(test_content, FORMATTED);

    let test2_content = std::fs::read_to_string(working_dir.join(test2)).unwrap();
    assert_eq!(test2_content, UNFORMATTED);
}
