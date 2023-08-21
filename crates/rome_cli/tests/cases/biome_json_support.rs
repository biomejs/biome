use crate::configs::CONFIG_FORMAT;
use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use bpaf::Args;
use rome_console::BufferConsole;
use rome_fs::{FileSystemExt, MemoryFileSystem};
use rome_service::DynRef;
use std::path::{Path, PathBuf};

const CUSTOM_CONFIGURATION_BEFORE: &str = r#"function f() {
  return { a, b }
}"#;

const CUSTOM_CONFIGURATION_AFTER: &str = "function f() {
        return {
                a,
                b,
        };
}
";

#[test]
fn formatter_biome_json() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_FORMAT.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_CONFIGURATION_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--line-width"),
                ("10"),
                ("--indent-style"),
                ("space"),
                ("--indent-size"),
                ("8"),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, CUSTOM_CONFIGURATION_AFTER);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatter_biome_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn linter_biome_json() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), "debugger;\n".as_bytes());

    let config_path = Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "suspicious": {
            "noDebugger": "off"
        }
    }
  }
}"#
        .as_bytes(),
    );

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

    assert_eq!(buffer, "debugger;\n");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "linter_biome_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_biome_json() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), "debugger".as_bytes());

    let config_path = Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "suspicious": {
            "noDebugger": "off"
        }
    }
  }
}"#
        .as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
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

    assert_eq!(buffer, "debugger;\n");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_biome_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_biome_json() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        PathBuf::from("biome.json"),
        r#"{
  "formatter": {
    "enabled": false
  }
}
"#
        .as_bytes(),
    );

    let input_file = Path::new("file.js");

    fs.insert(input_file.into(), "  statement(  )  ".as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("ci"), input_file.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(input_file)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, "  statement(  )  ");

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_biome_json",
        fs,
        console,
        result,
    ));
}
