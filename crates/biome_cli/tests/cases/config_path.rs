use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn set_config_path_to_directory() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("src/index.js");
    fs.insert(file_path.into(), "a['b']  =  42;".as_bytes());

    let config_path = Utf8Path::new("config/biome.jsonc");
    fs.insert(
        config_path.into(),
        r#"{
  "assist": {
    "enabled": true
  },
  "linter": {
    "enabled": false
  },
  "formatter": {
    "enabled": true,
  },
  "javascript": {
    "formatter": {
      "quoteStyle": "single", // comment
    }
  }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--config-path=config", "src"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "set_config_path_to_directory",
        fs,
        console,
        result,
    ));
}

#[test]
fn set_config_path_to_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("src/index.js");
    fs.insert(file_path.into(), "a['b']  =  42;".as_bytes());

    let config_path = Utf8Path::new("config/a.jsonc");
    fs.insert(
        config_path.into(),
        r#"{
  "assist": {
    "enabled": true
  },
  "linter": {
    "enabled": false
  },
  "formatter": {
    "enabled": true,
  },
  "javascript": {
    "formatter": {
      "quoteStyle": "single", // comment
    }
  }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--config-path=config/a.jsonc", "src"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "set_config_path_to_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn raises_an_error_when_the_config_file_is_not_json() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_path = Utf8Path::new("biome.yml");
    fs.insert(config_path.into(), r#"blah: foo"#.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--config-path=biome.yml", "src"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "raises_an_error_when_the_config_file_is_not_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn raises_an_error_for_no_configuration_file_found() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file = Utf8Path::new("file.js");
    fs.insert(
        file.into(),
        r#"function name() { return "lorem" }"#.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--config-path=config", file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "raises_an_error_for_no_configuration_file_found",
        fs,
        console,
        result,
    ));
}
