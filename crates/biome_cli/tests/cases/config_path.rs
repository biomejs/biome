use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn set_config_path_to_directory() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("src/index.js");
    fs.insert(file_path.into(), "a['b']  =  42;".as_bytes());

    let config_path = Path::new("config/biome.jsonc");
    fs.insert(
        config_path.into(),
        r#"{
  "organizeImports": {
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

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("check"), ("--config-path=config"), ("src")].as_slice()),
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

    let file_path = Path::new("src/index.js");
    fs.insert(file_path.into(), "a['b']  =  42;".as_bytes());

    let config_path = Path::new("config/a.jsonc");
    fs.insert(
        config_path.into(),
        r#"{
  "organizeImports": {
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

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("check"), ("--config-path=config/a.jsonc"), ("src")].as_slice()),
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
