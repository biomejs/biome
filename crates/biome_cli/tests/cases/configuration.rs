use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::{FileSystem, MemoryFileSystem};
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn can_read_hidden_biome_json_file() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("src/index.js");
    fs.insert(file_path.into(), "a['b']  =  42;".as_bytes());

    let config_path = Utf8Path::new(".biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "javascript": {
    "formatter": {
      "quoteStyle": "single"
    }
  }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(fs, &mut console, Args::from(["check"].as_slice()));

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "can_read_hidden_biome_json_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn can_read_hidden_biome_jsonc_file() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("src/index.js");
    fs.insert(file_path.into(), "a['b']  =  42;".as_bytes());

    let config_path = Utf8Path::new(".biome.jsonc");
    fs.insert(
        config_path.into(),
        r#"{
  "javascript": {
    "formatter": {
      // comment
      "quoteStyle": "single"
    }
  }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(fs, &mut console, Args::from(["check"].as_slice()));

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "can_read_hidden_biome_jsonc_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn can_read_configuration_from_user_home() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("src/index.js");
    fs.insert(file_path.into(), "a['b']  =  42;".as_bytes());

    let config_dir = fs.user_config_dir().expect("config dir to exist");
    let config_file = config_dir.join("biome.json");
    fs.insert(
        config_file,
        r#"{
  "javascript": {
    "formatter": {
      "quoteStyle": "single"
    }
  }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(fs, &mut console, Args::from(["check"].as_slice()));

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "can_read_configuration_from_user_home",
        fs,
        console,
        result,
    ));
}

#[test]
fn uses_project_config_before_user_config() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("src/index.js");
    fs.insert(file_path.into(), "a['b']  =  42;".as_bytes());

    let config_path = Utf8Path::new(".biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "javascript": {
    "formatter": {
      "quoteStyle": "single"
    }
  }
}"#
        .as_bytes(),
    );

    let config_dir = fs.user_config_dir().expect("config dir to exist");
    let config_file = config_dir.join("biome.json");
    fs.insert(
        config_file,
        r#"{
  "javascript": {
    "formatter": {
      "enabled": false
    }
  }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(fs, &mut console, Args::from(["check"].as_slice()));

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "uses_project_config_before_user_config",
        fs,
        console,
        result,
    ));
}
