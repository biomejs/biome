use crate::configs::CONFIG_FORMAT;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use crate::{run_cli, UNFORMATTED};
use biome_console::BufferConsole;
use biome_fs::{FileSystemExt, MemoryFileSystem};
use bpaf::Args;
use camino::{Utf8Path, Utf8PathBuf};

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

    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_FORMAT.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_CONFIGURATION_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--line-width",
                "10",
                "--indent-style",
                "space",
                "--indent-width",
                "8",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, CUSTOM_CONFIGURATION_AFTER);
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

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), "debugger;\n".as_bytes());

    let config_path = Utf8Path::new("biome.json");
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

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", file_path.as_str()].as_slice()),
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

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), "debugger".as_bytes());

    let config_path = Utf8Path::new("biome.json");
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

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", file_path.as_str()].as_slice()),
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
        Utf8PathBuf::from("biome.json"),
        r#"{
  "formatter": {
    "enabled": false
  }
}
"#
        .as_bytes(),
    );

    let input_file = Utf8Path::new("file.js");

    fs.insert(input_file.into(), "  statement(  )  ".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", input_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, input_file, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_biome_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn biome_json_is_not_ignored() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8PathBuf::from("biome.json"),
        r#"{
            "files": { "includes": ["**", "!*.json"] },
            "formatter": {
                "enabled": false
            }
        }
        "#
        .as_bytes(),
    );

    let input_file = Utf8Path::new("file.js");

    fs.insert(input_file.into(), "  statement(  )  ".as_bytes());

    let input_file = Utf8Path::new("file.json");

    fs.insert(input_file.into(), "  statement(  )  ".as_bytes());

    let (fs, result) = run_cli(fs, &mut console, Args::from(["ci", "./"].as_slice()));

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "biome_json_is_not_ignored",
        fs,
        console,
        result,
    ));
}

#[test]
fn always_disable_trailing_commas_biome_json() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("biome.json");
    let config = r#"{
    "formatter": {
        "indentStyle": "space",
        "indentWidth": 4
    },
    "json": {
        "formatter": {
            "trailingCommas": "all"
        }
    }
}
"#;
    fs.insert(file_path.into(), config);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "."].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, config);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "always_disable_trailing_commas_biome_json",
        fs,
        console,
        result,
    ));
}
