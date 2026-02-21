use crate::run_cli;
use crate::snap_test::{CliSnapshot, SnapshotPayload};
use biome_cli::CliDiagnostic;
use biome_console::{BufferConsole, Console};
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::{Utf8Path, Utf8PathBuf};
use serial_test::serial;
use std::{env, fs};

#[test]
#[serial]
fn ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_rage(fs, &mut console, Args::from(["rage"].as_slice()));

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "rage_ok",
        fs,
        console,
        result,
    ));
}

#[test]
#[serial]
fn with_configuration() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Utf8Path::new("biome.json").to_path_buf(),
        r#"{
  "formatter": {
    "enabled": false
  }
}"#,
    );

    let (fs, result) = run_rage(fs, &mut console, Args::from(["rage"].as_slice()));

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
#[serial]
fn with_no_configuration() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_rage(fs, &mut console, Args::from(["rage"].as_slice()));

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_no_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
#[serial]
fn with_jsonc_configuration() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Utf8Path::new("biome.jsonc").to_path_buf(),
        r#"{
  "formatter": {
    // disable formatter
    "enabled": false,
  }
}"#,
    );

    let (fs, result) = run_rage(fs, &mut console, Args::from(["rage"].as_slice()));

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_jsonc_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
#[serial]
fn with_malformed_configuration() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Utf8Path::new("biome.json").to_path_buf(),
        r#"{
  "formatter": {
    "enabled":
  }
}"#,
    );

    let (fs, result) = run_rage(fs, &mut console, Args::from(["rage"].as_slice()));

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_malformed_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
#[serial]
fn with_formatter_configuration() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Utf8Path::new("biome.json").to_path_buf(),
        r#"{
  "formatter": {
    "attributePosition": "multiline",
    "enabled": true,
    "formatWithErrors": true,
    "includes": [
      "**/*.html",
      "**/*.css",
      "**/*.js",
      "**/*.ts",
      "**/*.tsx",
      "**/*.jsx",
      "**/*.json",
      "**/*.md",
      "!configuration-schema.json"
    ],
    "indentStyle": "space",
    "indentWidth": 2,
    "lineEnding": "lf",
    "lineWidth": 120
  },
  "javascript": {
    "formatter": {
        "enabled": true,
        "arrowParentheses": "always",
        "jsxQuoteStyle": "single",
        "indentWidth": 2,
        "indentStyle":"tab",
        "lineEnding": "lf",
        "lineWidth": 100
    }
  },
  "json": {
    "formatter": {
        "enabled": true,
        "indentStyle": "space",
        "indentWidth": 2,
        "lineEnding": "lf",
        "lineWidth": 100
    }
  }
}"#,
    );

    let (fs, result) = run_rage(
        fs,
        &mut console,
        Args::from(["rage", "--formatter"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_formatter_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
#[serial]
fn with_linter_configuration() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Utf8Path::new("biome.json").to_path_buf(),
        r#"{
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": false,
      "a11y": {
        "noAccessKey": "off",
        "noAutofocus": "off"
      },
      "complexity": {
        "recommended": true
      },
      "suspicious": {
        "noCommentText": {
          "level": "warn"
        }
      },
      "style": {
        "noNonNullAssertion": "off"
      }
    }
  }
}"#,
    );

    let (fs, result) = run_rage(
        fs,
        &mut console,
        Args::from(["rage", "--linter"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_linter_configuration",
        fs,
        console,
        result,
    ));
}

/// Runs the `rage` command mocking out the log directory.
fn run_rage(
    fs: MemoryFileSystem,
    console: &mut dyn Console,
    args: Args,
) -> (MemoryFileSystem, Result<(), CliDiagnostic>) {
    let _test_dir = TestLogDir::new("biome-rage-test");
    run_cli(fs, console, args)
}

fn assert_rage_snapshot(payload: SnapshotPayload<'_>) {
    let test_name = payload.test_name;
    let module_path = payload.module_path;

    let mut snapshot = CliSnapshot::from(payload);

    // Replace any platform specific content that may yield unstable results.
    for message in snapshot.messages.iter_mut() {
        *message = message
            .lines()
            .map(|line| match line.trim_start().split_once(':') {
                Some((
                    "CPU Architecture" | "OS" | "NO_COLOR" | "TERM" | "BIOME_LOG_PATH"
                    | "Color support",
                    value,
                )) => line.replace(value.trim_start(), "**PLACEHOLDER**"),
                _ => line.to_string(),
            })
            .collect::<Vec<_>>()
            .join("\n");
    }

    let content = snapshot.emit_content_snapshot();

    let module_path = module_path.replace("::", "_");
    let snapshot_path = Utf8PathBuf::from("../snapshots").join(module_path);

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => snapshot_path
    }, {
        insta::assert_snapshot!(test_name, content);

    });
}

/// Mocks out the directory from which `rage` reads the server logs. Ensures that the test directory
/// gets removed at the end of the test.
struct TestLogDir {
    path: Utf8PathBuf,
}

impl TestLogDir {
    fn new(name: &str) -> Self {
        let path = env::temp_dir().join(name);

        Self {
            path: Utf8PathBuf::from_path_buf(path).unwrap(),
        }
    }
}

impl Drop for TestLogDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).ok();
    }
}
