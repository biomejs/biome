use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::{FileSystemExt, MemoryFileSystem};
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

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
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": { "include": ["test.js"] }
}
"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(test2)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED);

    drop(file);

    let mut file = fs
        .open(test)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, FORMATTED);

    drop(file);

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
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": { "include": ["test.js", "test2.js"], "ignore": ["test.js"] }
}
"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(test2)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, FORMATTED);

    drop(file);

    let mut file = fs
        .open(test)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED);

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_handle_included_files_if_overridden_by_ignore",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_handle_if_included_in_formatter() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": { "ignore": ["test.js"] }, "formatter": { "include": ["test.js"] }
}
"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--write"), test.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(test)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, FORMATTED);

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_handle_if_included_in_formatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_handle_included_files_if_overridden_by_ignore_formatter() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "formatter": { "include": ["test.js", "test2.js"], "ignore": ["test.js"] }
}
"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(test2)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, FORMATTED);

    drop(file);

    let mut file = fs
        .open(test)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED);

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_handle_included_files_if_overridden_by_ignore_formatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_lint_included_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": { "ignore": ["test.js"] }, "linter": { "include": ["test.js"] }
}
"#,
    );

    let file_path = Path::new("test.js");
    fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                file_path.as_os_str().to_str().unwrap(),
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

    assert_eq!(buffer, FIX_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_lint_included_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_handle_included_files_if_overridden_by_ignore_linter() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "linter": { "include": ["test.js", "test2.js"], "ignore": ["test.js"] }
}
"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), FIX_BEFORE.as_bytes());

    let test2 = Path::new("test2.js");
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

    let mut file = fs
        .open(test2)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, FIX_AFTER);

    drop(file);

    let mut file = fs
        .open(test)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, FIX_BEFORE);

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_handle_included_files_if_overridden_by_ignore_linter",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_organize_imports_of_included_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "formatter": { "enabled": false },
  "linter": { "enabled": false },
   "files": { "ignore": ["test2.js", "test.js"] }, "organizeImports": { "include": ["test.js"] }
}
"#,
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNORGANIZED.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNORGANIZED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                ("--apply"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(test2)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNORGANIZED);

    drop(file);

    let mut file = fs
        .open(test)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, ORGANIZED);

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_organize_imports_of_included_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_handle_included_files_if_overridden_by_organize_imports() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "formatter": { "enabled": false },
  "linter": { "enabled": false },
  "organizeImports": { "include": ["test.js", "test2.js"], "ignore": ["test.js"] }
}
"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNORGANIZED.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNORGANIZED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                ("--apply"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(test2)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, ORGANIZED);

    drop(file);

    let mut file = fs
        .open(test)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNORGANIZED);

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_handle_included_files_if_overridden_by_organize_imports",
        fs,
        console,
        result,
    ));
}
