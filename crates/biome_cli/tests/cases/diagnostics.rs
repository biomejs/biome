use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use crate::{run_cli, UNFORMATTED};
use biome_console::{BufferConsole, LogLevel};
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::{Utf8Path, Utf8PathBuf};

const TEST_CONTENTS: &str = "debugger;";

#[test]
fn logs_the_appropriate_messages_according_to_set_diagnostics_level() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "includes": ["test.js"]
  },
  "linter": {
    "rules": {
        "suspicious": {
            "noDebugger": "warn"
        }
    }
  }
}

"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), TEST_CONTENTS.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--diagnostic-level=error", test.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let messages = &console.out_buffer;

    assert!(messages
        .iter()
        .filter(|m| m.level == LogLevel::Log)
        .any(|m| {
            let content = format!("{:?}", m.content);

            !content.contains("noDebugger")
        }));

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "logs_the_appropriate_messages_according_to_set_diagnostics_level",
        fs,
        console,
        result,
    ));
}

#[test]
fn max_diagnostics_no_verbose() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..10 {
        let file_path = Utf8PathBuf::from(format!("src/folder_{i}/package-lock.json"));
        fs.insert(file_path, "{}".as_bytes());
    }
    let file_path = Utf8PathBuf::from("src/file.js".to_string());
    fs.insert(file_path, UNFORMATTED.as_bytes());

    let (mut fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--max-diagnostics", "10", "src"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    for i in 0..10 {
        let file_path = Utf8PathBuf::from(format!("src/folder_{i}/package-lock.json"));
        fs.remove(Utf8Path::new(&file_path));
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics_no_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn max_diagnostics_verbose() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..8 {
        let file_path = Utf8PathBuf::from(format!("src/folder_{i}/package-lock.json"));
        fs.insert(file_path, "{}".as_bytes());
    }
    let file_path = Utf8PathBuf::from("src/file.js".to_string());
    fs.insert(file_path, UNFORMATTED.as_bytes());

    let (mut fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--max-diagnostics=10", "--verbose", "src"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    for i in 0..8 {
        let file_path = Utf8PathBuf::from(format!("src/folder_{i}/package-lock.json"));
        fs.remove(Utf8Path::new(&file_path));
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn diagnostic_level() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
    "formatter": {
        "enabled": true
    },
    "assist": {
        "enabled": true
    },
    "linter": {
        "enabled": false
    }
}
"#,
    );

    let file_path = Utf8PathBuf::from("src/index.js".to_string());
    fs.insert(
        file_path,
        r#"import { graphql, useFragment, useMutation } from "react-relay";
import { FC, memo, useCallback } from "react";
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--diagnostic-level=error", "src"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let messages = &console.out_buffer;

    assert!(messages
        .iter()
        .filter(|m| m.level == LogLevel::Error)
        .any(|m| {
            let content = format!("{:?}", m.content);
            content.contains("assist")
        }));

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "diagnostic_level",
        fs,
        console,
        result,
    ));
}

#[test]
fn max_diagnostics_are_lifted() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..u8::MAX {
        let file_path = Utf8PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, UNFORMATTED.as_bytes());
    }

    let file_path = Utf8PathBuf::from("file.js".to_string());
    fs.insert(
        file_path.clone(),
        "debugger;".repeat(u8::MAX as usize * 2).as_bytes(),
    );

    let (mut fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--max-diagnostics", "none", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    for i in 0..u8::MAX {
        let file_path = Utf8PathBuf::from(format!("src/file_{i}.js"));
        fs.remove(&file_path);
    }

    let messages = &console.out_buffer;

    let errors = format!("{}", u8::MAX as usize * 2 + 1);

    assert!(messages
        .iter()
        .filter(|m| m.level == LogLevel::Log)
        .any(|m| {
            let content = format!("{:?}", m.content);

            content.contains(&errors)
        }));
}
