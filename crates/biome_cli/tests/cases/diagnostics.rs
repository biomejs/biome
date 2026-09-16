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

#[test]
fn bun_catalogs_use_workspace_root() {
    for (root_catalog, expected) in [
        (serde_json::json!({"react": "19.0.0"}), true),
        (serde_json::json!({"react": "18.3.1"}), false),
        (serde_json::json!({}), false),
    ] {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "package.json".into(),
            serde_json::json!({
                "workspaces": ["packages/*"], "catalog": root_catalog
            })
            .to_string()
            .as_bytes(),
        );
        fs.insert(
            "packages/app/package.json".into(),
            serde_json::json!({
                "workspaces": ["nested/*"],
                "catalog": {"react": "19.0.0"},
                "dependencies": {"react": "catalog:"}
            })
            .to_string()
            .as_bytes(),
        );
        assert_catalog_diagnostic(fs, (false, true), true, expected);
    }
}

#[test]
fn bun_catalogs_resolve_above_biome_root() {
    use biome_cli::{CliSession, biome_command};
    use biome_fs::BiomePath;
    use biome_service::workspace::{FileContent, OpenFileParams, OpenProjectParams};
    use biome_service::{App, WorkspaceRef, workspace};
    use std::sync::Arc;

    let mut fs = biome_fs::TemporaryFs::new("bun_catalogs_resolve_above_biome_root");
    let manifest_path = fs.create_file("package.json", "{}");
    fs.create_file(
        "packages/app/package.json",
        r#"{"dependencies":{"react":"catalog:"}}"#,
    );
    fs.create_file(
        "packages/app/biome.json",
        r#"{
            "javascript": {"resolver": {"experimentalBunCatalogs": true}},
            "linter": {"rules": {"suspicious": {"noReactForwardRef": "error"}}}
        }"#,
    );
    fs.create_file(
        "packages/app/input.jsx",
        r#"import { forwardRef } from "react";
export const Component = forwardRef((props, ref) => <div ref={ref} {...props} />);
"#,
    );
    fs.append_to_working_directory("packages/app");
    let workspace = workspace::server(Arc::new(fs.create_os()), None);
    for (version, unsaved_version, expected) in [
        (Some("18.3.1"), None, false),
        (Some("19.0.0"), None, true),
        (Some("18.3.1"), None, false),
        (None, None, false),
        (Some("18.3.1"), None, false),
        (Some("18.3.1"), Some("19.0.0"), true),
    ] {
        if let Some(version) = version {
            std::fs::write(
                &manifest_path,
                serde_json::json!({
                    "workspaces": ["packages/*"], "catalog": {"react": version}
                })
                .to_string(),
            )
            .unwrap();
        } else {
            std::fs::remove_file(&manifest_path).unwrap();
        }
        if let Some(version) = unsaved_version {
            let project_key = workspace
                .open_project(OpenProjectParams {
                    path: BiomePath::new(manifest_path.parent().unwrap()),
                    open_uninitialized: true,
                })
                .unwrap()
                .project_key;
            workspace
                .open_file(OpenFileParams {
                    project_key,
                    path: BiomePath::new(&manifest_path),
                    content: FileContent::from_client(
                        serde_json::json!({
                            "workspaces": ["packages/*"], "catalog": {"react": version}
                        })
                        .to_string(),
                    ),
                    document_file_source: None,
                    persist_node_cache: false,
                    inline_config: None,
                    editor_features: None,
                })
                .unwrap();
        }
        let mut console = BufferConsole::default();
        let session = CliSession {
            app: App::new(&mut console, WorkspaceRef::Borrowed(workspace.as_ref())),
            watcher_factory: None,
        };
        let command = biome_command()
            .run_inner(Args::from(
                ["lint", "--only=suspicious/noReactForwardRef", fs.cli_path()].as_slice(),
            ))
            .unwrap();
        let result = session.run(command);
        let output = console
            .out_buffer
            .iter()
            .map(|message| format!("{:?}", message.content))
            .collect::<String>();
        assert_eq!(
            output.contains("noReactForwardRef"),
            expected,
            "{version:?}: {output}"
        );
        assert_eq!(result.is_err(), expected, "{result:?}: {output}");
    }
}

#[test]
fn workspace_catalog_resolvers_are_independent() {
    for (pnpm, bun, expected) in [
        (false, false, false),
        (false, true, true),
        (true, false, false),
        (true, true, false),
    ] {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "package.json".into(),
            br#"{
            "workspaces": ["packages/*"], "catalog": {"react": "19.0.0"}
        }"#,
        );
        fs.insert("pnpm-workspace.yaml".into(), b"catalog:\n  react: 18.3.1\n");
        fs.insert(
            "packages/app/package.json".into(),
            br#"{"dependencies":{"react":"catalog:"}}"#,
        );
        assert_catalog_diagnostic(fs, (pnpm, bun), true, expected);
    }
}

#[test]
fn bun_catalogs_resolve_root_dependencies() {
    for enabled in [false, true] {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "package.json".into(),
            br#"{
            "workspaces": [], "catalog": {"react": "19.0.0"},
            "dependencies": {"react": "catalog:"}
        }"#,
        );
        assert_catalog_diagnostic(fs, (false, enabled), false, enabled);
    }
}

fn assert_catalog_diagnostic(
    fs: MemoryFileSystem,
    (pnpm, bun): (bool, bool),
    nested: bool,
    expected: bool,
) {
    let mut console = BufferConsole::default();
    let path = if nested {
        "packages/app/input.jsx"
    } else {
        "input.jsx"
    };
    fs.insert("biome.json".into(), serde_json::json!({
        "javascript": {"resolver": {"experimentalPnpmCatalogs": pnpm, "experimentalBunCatalogs": bun}},
        "linter": {"rules": {"suspicious": {"noReactForwardRef": "error"}}}
    }).to_string().as_bytes());
    fs.insert(
        path.into(),
        br#"import { forwardRef } from "react";
export const Component = forwardRef((props, ref) => <div ref={ref} {...props} />);
"#,
    );
    let (_, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["lint", "--only=suspicious/noReactForwardRef", path].as_slice()),
    );
    let output = console
        .out_buffer
        .iter()
        .map(|message| format!("{:?}", message.content))
        .collect::<String>();
    assert_eq!(output.contains("noReactForwardRef"), expected, "{output}");
    assert_eq!(result.is_err(), expected, "{result:?}: {output}");
}

#[test]
fn pnpm_catalogs_refresh_for_stdin_without_project_scanner() {
    use biome_cli::{CliSession, biome_command};
    use biome_service::{App, WorkspaceRef, workspace};
    use std::sync::Arc;

    let fs = Arc::new(MemoryFileSystem::default());
    fs.insert(
        "package.json".into(),
        br#"{"dependencies":{"react":"catalog:"}}"#,
    );
    fs.insert(
        "biome.json".into(),
        br#"{
        "javascript": {"resolver": {"experimentalPnpmCatalogs": true}},
        "linter": {"rules": {"suspicious": {"noReactForwardRef": "error"}}}
    }"#,
    );
    let workspace = workspace::server(fs.clone(), None);
    for (version, expected) in [
        (Some("18.3.1"), false),
        (Some("19.0.0"), true),
        (None, false),
    ] {
        if let Some(version) = version {
            fs.insert(
                "pnpm-workspace.yaml".into(),
                format!("catalog:\n  react: {version}\n").as_bytes(),
            );
        } else {
            fs.remove(Utf8Path::new("pnpm-workspace.yaml"));
        }
        let mut console = BufferConsole::default();
        console.in_buffer.push(
            r#"import { forwardRef } from "react";
export const Component = forwardRef((props, ref) => <div ref={ref} {...props} />);"#
                .to_string(),
        );
        let session = CliSession {
            app: App::new(&mut console, WorkspaceRef::Borrowed(workspace.as_ref())),
            watcher_factory: None,
        };
        let command = biome_command()
            .run_inner(Args::from(
                [
                    "lint",
                    "--write",
                    "--unsafe",
                    "--only=suspicious/noReactForwardRef",
                    "--stdin-file-path=input.jsx",
                ]
                .as_slice(),
            ))
            .unwrap();
        let result = session.run(command);
        let output = console
            .out_buffer
            .iter()
            .map(|message| format!("{:?}", message.content))
            .collect::<String>();
        assert_eq!(
            !output.contains("forwardRef("),
            expected,
            "{version:?}: {output}"
        );
        assert!(result.is_ok(), "{result:?}: {output}");
    }
}

#[test]
fn bun_catalogs_refresh_through_cli_and_manifest_edits() {
    use biome_cli::{CliSession, biome_command};
    use biome_fs::BiomePath;
    use biome_service::workspace::{
        FileContent, OpenFileParams, OpenProjectParams, PullDiagnosticsParams, UpdateSettingsParams,
    };
    use biome_service::{App, WorkspaceRef, workspace};
    use std::sync::Arc;

    let fs = Arc::new(MemoryFileSystem::default());
    fs.insert(
        "package.json".into(),
        br#"{
        "workspaces": ["packages/*"], "catalog": {"react": "19.0.0"}
    }"#,
    );
    fs.insert(
        "packages/app/package.json".into(),
        br#"{"dependencies":{"react":"catalog:"}}"#,
    );
    fs.insert(
        "packages/app/input.jsx".into(),
        br#"import { forwardRef } from "react";
export const Component = forwardRef((props, ref) => <div ref={ref} {...props} />);
"#,
    );
    let workspace = workspace::server(fs.clone(), None);
    let run = |label: &str, enabled: Option<bool>, expected: bool| {
        let mut configuration = serde_json::json!({
            "linter": {"rules": {"suspicious": {"noReactForwardRef": "error"}}}
        });
        if let Some(enabled) = enabled {
            configuration["javascript"] = serde_json::json!({
                "resolver": {"experimentalBunCatalogs": enabled}
            });
        }
        fs.insert("biome.json".into(), configuration.to_string().as_bytes());
        let mut console = BufferConsole::default();
        let session = CliSession {
            app: App::new(&mut console, WorkspaceRef::Borrowed(workspace.as_ref())),
            watcher_factory: None,
        };
        let command = biome_command()
            .run_inner(Args::from(
                [
                    "lint",
                    "--only=suspicious/noReactForwardRef",
                    "packages/app/input.jsx",
                ]
                .as_slice(),
            ))
            .unwrap();
        let result = session.run(command);
        let output = console
            .out_buffer
            .iter()
            .map(|message| format!("{:?}", message.content))
            .collect::<String>();
        assert_eq!(
            output.contains("noReactForwardRef"),
            expected,
            "{label}: {output}"
        );
        assert_eq!(result.is_err(), expected, "{result:?}: {output}");
    };
    run("enabled", Some(true), true);
    run("disabled", Some(false), false);
    run("enabled", Some(true), true);

    run("javascript config removed", None, false);
    run("enabled", Some(true), true);

    let project_key = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new(""),
            open_uninitialized: true,
        })
        .unwrap()
        .project_key;
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("packages/app/input.jsx"),
            content: FileContent::from_client(
                r#"import { forwardRef } from "react";
export const Component = forwardRef((props, ref) => <div ref={ref} {...props} />);"#,
            ),
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();
    for expected in [true, false] {
        if !expected {
            workspace
                .update_settings(UpdateSettingsParams {
                    project_key,
                    configuration: Default::default(),
                    workspace_directory: Some(BiomePath::new("")),
                    extended_configurations: Default::default(),
                    module_graph_resolution_kind: Default::default(),
                })
                .unwrap();
        }
        let result = workspace
            .pull_diagnostics(PullDiagnosticsParams {
                project_key,
                path: BiomePath::new("packages/app/input.jsx"),
                categories: biome_analyze::RuleCategoriesBuilder::default()
                    .with_lint()
                    .build(),
                only: vec!["lint/suspicious/noReactForwardRef".parse().unwrap()],
                skip: vec![],
                enabled_rules: vec![],
                include_code_fix: false,
                inline_config: None,
                max_diagnostics: None,
                diagnostic_level: biome_diagnostics::Severity::Hint,
                enforce_assist: false,
            })
            .unwrap();
        assert_eq!(!result.diagnostics.is_empty(), expected);
    }
    for (content, expected) in [
        (r#"{"workspaces":[],"catalog":{"react":"18.3.1"}}"#, false),
        (r#"{"workspaces":[],"catalog":{"react":"19.0.0"}}"#, true),
        (r#"{"workspaces":[]}"#, false),
    ] {
        fs.insert("package.json".into(), content.as_bytes());
        workspace
            .open_file(OpenFileParams {
                project_key,
                path: BiomePath::new("package.json"),
                content: FileContent::from_client(content),
                document_file_source: None,
                persist_node_cache: false,
                inline_config: None,
                editor_features: None,
            })
            .unwrap();
        run(content, Some(true), expected);
    }
}
