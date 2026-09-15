use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use crate::{UNFORMATTED, run_cli, run_cli_with_server_workspace};
use biome_console::{BufferConsole, LogLevel};
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::{Utf8Path, Utf8PathBuf};

const TEST_CONTENTS: &str = "debugger;";

#[test]
fn logs_the_appropriate_messages_according_to_set_diagnostics_level() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
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

    assert!(
        messages
            .iter()
            .filter(|m| m.level == LogLevel::Log)
            .any(|m| {
                let content = format!("{:?}", m.content);

                !content.contains("noDebugger")
            })
    );

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
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..10 {
        let file_path = Utf8PathBuf::from(format!("src/folder_{i}/package-lock.json"));
        fs.insert(file_path, "{}".as_bytes());
    }
    let file_path = Utf8PathBuf::from("src/file.js".to_string());
    fs.insert(file_path, UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
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
fn reads_pnpm_workspace_catalog() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("pnpm-workspace.yaml").into(),
        r#"
packages:
  - "src"
catalogs:
  react19:
    react: 19.0.0
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("package.json").into(),
        r#"
{
  "name": "app",
  "dependencies": {
    "react": "catalog:react19"
  }
}
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"
{
  "javascript": {
    "resolver": {
      "experimentalPnpmCatalogs": true
    }
  },
    "linter": {
    "rules": {
      "suspicious": {
        "noReactForwardRef": "error"
      }
    }
  }
}
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("src/input.jsx").into(),
        r#"
import { forwardRef } from "react";

export const Component = forwardRef((props, ref) => {
  return <div ref={ref} {...props} />;
});
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "src/input.jsx"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "reads_pnpm_workspace_catalog",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_fail_when_max_diagnostics_is_zero() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..1 {
        let file_path = Utf8PathBuf::from(format!("src/folder_{i}/package-lock.json"));
        fs.insert(file_path, "{}".as_bytes());
    }
    let file_path = Utf8PathBuf::from("src/file.js".to_string());
    fs.insert(file_path, UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--max-diagnostics", "0", "src"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    for i in 0..1 {
        let file_path = Utf8PathBuf::from(format!("src/folder_{i}/package-lock.json"));
        fs.remove(Utf8Path::new(&file_path));
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_fail_when_max_diagnostics_is_zero",
        fs,
        console,
        result,
    ));
}

#[test]
fn max_diagnostics_verbose() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..8 {
        let file_path = Utf8PathBuf::from(format!("src/folder_{i}/package-lock.json"));
        fs.insert(file_path, "{}".as_bytes());
    }
    let file_path = Utf8PathBuf::from("src/file.js".to_string());
    fs.insert(file_path, UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
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
    let fs = MemoryFileSystem::default();
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

    assert!(
        messages
            .iter()
            .filter(|m| m.level == LogLevel::Error)
            .any(|m| {
                let content = format!("{:?}", m.content);
                content.contains("assist")
            })
    );

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
    let fs = MemoryFileSystem::default();
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

    let (fs, result) = run_cli(
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

    assert!(
        messages
            .iter()
            .filter(|m| m.level == LogLevel::Log)
            .any(|m| {
                let content = format!("{:?}", m.content);

                content.contains(&errors)
            })
    );
}

#[test]
fn reads_bun_workspace_catalogs_for_react_compiler() {
    const RULE: &str = "nursery/useReactCompiler";
    const SOURCE: &str = r#"import { useState } from "react";
export function Component(props) {
    if (props.enabled) { useState(0); }
    return <div />;
}"#;
    for root_manifest in [
        r#"{"workspaces":{"packages":["packages/*"],"catalog":{"react":"19.0.0"},"catalogs":{"react19":{"react":"19.0.0"}}}}"#,
        r#"{"workspaces":["packages/*"],"catalog":{"react":"19.0.0"},"catalogs":{"react19":{"react":"19.0.0"}}}"#,
        r#"{"workspaces":["packages/*"],"catalogs":{"default":{"react":"19.0.0"},"react19":{"react":"19.0.0"}}}"#,
    ] {
        for (version, enabled, expected) in [
            ("catalog:", true, true),
            ("catalog:default", true, true),
            ("catalog: ", true, true),
            ("catalog: react19 ", true, true),
            ("catalog:react19", true, true),
            ("catalog:", false, false),
            ("catalog:missing", true, false),
        ] {
            let fs = MemoryFileSystem::default();
            let mut console = BufferConsole::default();
            fs.insert("package.json".into(), root_manifest.as_bytes());
            fs.insert(
                "packages/app/package.json".into(),
                serde_json::json!({
                    "dependencies": {"react": version}
                })
                .to_string()
                .as_bytes(),
            );
            fs.insert("biome.json".into(), serde_json::json!({
                "linter": {"domains": {"project": "all"}, "rules": {"nursery": {"useReactCompiler": "error"}}},
                "javascript": {"resolver": {"experimentalBunCatalogs": enabled}}
            }).to_string().as_bytes());
            fs.insert("packages/app/input.jsx".into(), SOURCE.as_bytes());
            let only = format!("--only={RULE}");
            let (_, result) = run_cli_with_server_workspace(
                fs,
                &mut console,
                Args::from(
                    [
                        "lint",
                        "--error-on-warnings",
                        only.as_str(),
                        "packages/app/input.jsx",
                    ]
                    .as_slice(),
                ),
            );
            let output = console
                .out_buffer
                .iter()
                .map(|message| format!("{:?}", message.content))
                .collect::<String>();
            assert_eq!(
                output.contains(RULE),
                expected,
                "{version}, enabled={enabled}: {output}"
            );
            assert_eq!(result.is_err(), expected, "{result:?}: {output}");
        }
    }
}
