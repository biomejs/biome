use crate::{
    run_cli,
    snap_test::{SnapshotPayload, assert_cli_snapshot_with_redactor},
};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;

fn assert_inspect_snapshot(
    test_name: &str,
    fs: MemoryFileSystem,
    console: BufferConsole,
    result: Result<(), biome_cli::CliDiagnostic>,
) {
    assert_cli_snapshot_with_redactor(
        SnapshotPayload::new(module_path!(), test_name, fs, console, result),
        normalize_inspect_snapshot_paths,
    );
}

fn normalize_inspect_snapshot_paths(content: String) -> String {
    content.replace("\\\\", "/").replace('\\', "/")
}

#[test]
fn root_value_uses_diagnostic_output() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "linter": {
    "rules": {
      "suspicious": {
        "noDebugger": "error"
      }
    }
  }
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "linter.rules.suspicious.noDebugger"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot("root_value_uses_diagnostic_output", fs, console, result);
}

#[test]
fn jsonc_resolved_configuration() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.jsonc".into(),
        r#"{
  // formatter settings
  "formatter": {
    "lineWidth": 100,
  },
  "overrides": [
    {
      "includes": ["**/*.test.js"],
      "formatter": { "lineWidth": 120 }
    }
  ]
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot("jsonc_resolved_configuration", fs, console, result);
}

#[test]
fn absent_key_is_successful() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "formatter": { "lineWidth": 100 } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter.unknown"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot("absent_key_is_successful", fs, console, result);
}

#[test]
fn extended_value_reports_resolved_path() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "extends": ["base.json"], "formatter": { "indentWidth": 4 } }"#,
    );
    fs.insert(
        "base.json".into(),
        r#"{
  "formatter": {
    "lineWidth": 100
  }
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter.lineWidth"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot("extended_value_reports_resolved_path", fs, console, result);
}

#[test]
fn package_extended_value_reports_specifier() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "extends": ["@shared/config/biome"] }"#,
    );
    fs.insert(
        "node_modules/@shared/config/biome.json".into(),
        r#"{ "formatter": { "lineWidth": 100 } }"#,
    );
    fs.insert(
        "node_modules/@shared/config/package.json".into(),
        r#"{
  "name": "@shared/config",
  "exports": {
    "./biome": "./biome.json"
  }
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter.lineWidth", "--json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "package_extended_value_reports_specifier",
        fs,
        console,
        result,
    );
}

#[test]
fn composite_value_reports_contributors() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "extends": ["base.json"],
  "files": { "includes": ["tests/**"] }
}"#,
    );
    fs.insert(
        "base.json".into(),
        r#"{ "files": { "includes": ["src/**"] } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "files.includes", "--json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot("composite_value_reports_contributors", fs, console, result);
}

#[test]
fn composite_value_diagnostic_identifies_contributors() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "extends": ["base.json"],
  "files": { "includes": ["tests/**"] }
}"#,
    );
    fs.insert(
        "base.json".into(),
        r#"{ "files": { "includes": ["src/**"] } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "files.includes"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "composite_value_diagnostic_identifies_contributors",
        fs,
        console,
        result,
    );
}

#[test]
fn composite_value_diagnostic_shows_three_extended_sources() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "extends": ["first.json", "second.json", "third.json"] }"#,
    );
    fs.insert(
        "first.json".into(),
        r#"{ "files": { "includes": ["first/**"] } }"#,
    );
    fs.insert(
        "second.json".into(),
        r#"{ "files": { "includes": ["second/**"] } }"#,
    );
    fs.insert(
        "third.json".into(),
        r#"{ "files": { "includes": ["third/**"] } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "files.includes"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "composite_value_diagnostic_shows_three_extended_sources",
        fs,
        console,
        result,
    );
}

#[test]
fn identical_composite_declarations_report_the_last_source() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "extends": ["base.json"],
  "formatter": { "lineWidth": 100 }
}"#,
    );
    fs.insert(
        "base.json".into(),
        r#"{ "formatter": { "lineWidth": 100 } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter", "--json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "identical_composite_declarations_report_the_last_source",
        fs,
        console,
        result,
    );
}

#[test]
fn identical_override_composite_reports_the_override_source() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "formatter": { "lineWidth": 100 },
  "overrides": [
    {
      "includes": ["**/*.js"],
      "formatter": { "lineWidth": 100 }
    }
  ]
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter", "--path=file.js", "--json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "identical_override_composite_reports_the_override_source",
        fs,
        console,
        result,
    );
}

#[test]
fn last_matching_override_wins() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "linter": {
    "rules": {
      "suspicious": { "noDebugger": "warn" }
    }
  },
  "overrides": [
    {
      "includes": ["**/*.test.js"],
      "linter": {
        "rules": {
          "suspicious": { "noDebugger": "error" }
        }
      }
    },
    {
      "includes": ["**/*.test.js"],
      "linter": {
        "rules": {
          "suspicious": { "noDebugger": "off" }
        }
      }
    }
  ]
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "inspect",
                "config",
                "linter.rules.suspicious.noDebugger",
                "--path=file.test.js",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot("last_matching_override_wins", fs, console, result);
}

#[test]
fn later_matching_override_uses_runtime_root_fallback() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "formatter": { "formatWithErrors": false },
  "overrides": [
    {
      "includes": ["**/*.test.js"],
      "formatter": { "formatWithErrors": true }
    },
    {
      "includes": ["**/*.test.js"],
      "formatter": { "lineWidth": 120 }
    }
  ]
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "inspect",
                "config",
                "formatter.formatWithErrors",
                "--path=file.test.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "later_matching_override_uses_runtime_root_fallback",
        fs,
        console,
        result,
    );
}

#[test]
fn non_matching_override_keeps_base_value() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "formatter": { "lineWidth": 100 },
  "overrides": [
    {
      "includes": ["**/*.test.js"],
      "formatter": { "lineWidth": 120 }
    }
  ]
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "inspect",
                "config",
                "formatter.lineWidth",
                "--path=file.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "non_matching_override_keeps_base_value",
        fs,
        console,
        result,
    );
}

#[test]
fn path_requires_key() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "--path=file.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_inspect_snapshot("path_requires_key", fs, console, result);
}

#[test]
fn malformed_configuration_is_an_error() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "formatter": { "lineWidth": "wide" } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter.lineWidth"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_inspect_snapshot("malformed_configuration_is_an_error", fs, console, result);
}

#[test]
fn malformed_extended_configuration_uses_extended_path() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), r#"{ "extends": ["base.json"] }"#);
    fs.insert(
        "base.json".into(),
        r#"{ "formatter": { "lineWidth": "wide" } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter.lineWidth"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "malformed_extended_configuration_uses_extended_path",
        fs,
        console,
        result,
    );
}

#[test]
fn missing_explicit_configuration_is_an_error() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "--config-path=missing.json", "--json"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "missing_explicit_configuration_is_an_error",
        fs,
        console,
        result,
    );
}

#[test]
fn no_configuration_json_is_empty() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "--json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot("no_configuration_json_is_empty", fs, console, result);
}

#[test]
fn json_range_uses_utf8_byte_offsets() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.jsonc".into(),
        r#"{
  // café
  "formatter": { "lineWidth": 100 }
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter.lineWidth", "--json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot("json_range_uses_utf8_byte_offsets", fs, console, result);
}

#[test]
fn matching_extended_override_keeps_source_axes() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), r#"{ "extends": ["base.json"] }"#);
    fs.insert(
        "base.json".into(),
        r#"{
  "overrides": [
    {
      "includes": ["**/*.test.js"],
      "formatter": { "lineWidth": 120 }
    }
  ]
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "inspect",
                "config",
                "formatter.lineWidth",
                "--path=file.test.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "matching_extended_override_keeps_source_axes",
        fs,
        console,
        result,
    );
}

#[test]
fn matching_override_appends_plugins() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "plugins": ["base.grit"],
  "overrides": [
    {
      "includes": ["**/*.test.js"],
      "plugins": ["override.grit"]
    }
  ]
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "inspect",
                "config",
                "plugins",
                "--path=file.test.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot("matching_override_appends_plugins", fs, console, result);
}

#[test]
fn typed_merge_preserves_extended_shorthand_origin() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "extends": ["base.json"],
  "linter": {
    "rules": {
      "suspicious": { "noDebugger": "off" }
    }
  }
}"#,
    );
    fs.insert(
        "base.json".into(),
        r#"{
  "linter": {
    "rules": { "suspicious": "error" }
  }
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "inspect",
                "config",
                "linter.rules.suspicious.noConsole",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "typed_merge_preserves_extended_shorthand_origin",
        fs,
        console,
        result,
    );
}

#[test]
fn matching_override_preserves_group_shorthand() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "linter": {
    "rules": { "suspicious": "error" }
  },
  "overrides": [
    {
      "includes": ["**/*.test.js"],
      "linter": {
        "rules": {
          "suspicious": { "noDebugger": "off" }
        }
      }
    }
  ]
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "inspect",
                "config",
                "linter.rules.suspicious.noConsole",
                "--path=file.test.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "matching_override_preserves_group_shorthand",
        fs,
        console,
        result,
    );
}

#[test]
fn null_value_does_not_replace_extended_provenance() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "extends": ["base.json"],
  "formatter": { "lineWidth": null }
}"#,
    );
    fs.insert(
        "base.json".into(),
        r#"{ "formatter": { "lineWidth": 100 } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter.lineWidth", "--json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "null_value_does_not_replace_extended_provenance",
        fs,
        console,
        result,
    );
}

#[test]
fn replaced_composite_omits_superseded_source() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "extends": ["base.json"],
  "linter": {
    "rules": {
      "suspicious": {
        "noConsole": {
          "level": "warn",
          "options": { "allow": ["error"] }
        }
      }
    }
  }
}"#,
    );
    fs.insert(
        "base.json".into(),
        r#"{
  "linter": {
    "rules": {
      "suspicious": { "noConsole": "error" }
    }
  }
}"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "inspect",
                "config",
                "linter.rules.suspicious.noConsole",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "replaced_composite_omits_superseded_source",
        fs,
        console,
        result,
    );
}

#[test]
fn repeated_nested_configuration_emits_information() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "extends": ["first.json", "second.json"] }"#,
    );
    fs.insert("first.json".into(), r#"{ "extends": ["shared.json"] }"#);
    fs.insert("second.json".into(), r#"{ "extends": ["shared.json"] }"#);
    fs.insert(
        "shared.json".into(),
        r#"{ "formatter": { "lineWidth": 100 } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter.lineWidth"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "repeated_nested_configuration_emits_information",
        fs,
        console,
        result,
    );
}

#[test]
fn conflicting_nested_package_versions_block_inspection() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "extends": ["package-b", "package-c"] }"#,
    );
    insert_configuration_package(
        &fs,
        "node_modules/package-b",
        "package-b",
        "1.0.0",
        r#"{ "extends": ["biome-config-vodoo"] }"#,
    );
    insert_configuration_package(
        &fs,
        "node_modules/package-c",
        "package-c",
        "1.0.0",
        r#"{ "extends": ["biome-config-vodoo"] }"#,
    );
    insert_configuration_package(
        &fs,
        "node_modules/package-b/node_modules/biome-config-vodoo",
        "biome-config-vodoo",
        "0.2.0",
        r#"{ "formatter": { "lineWidth": 90 } }"#,
    );
    insert_configuration_package(
        &fs,
        "node_modules/package-c/node_modules/biome-config-vodoo",
        "biome-config-vodoo",
        "0.5.0",
        r#"{ "formatter": { "lineWidth": 100 } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter.lineWidth"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_inspect_snapshot(
        "conflicting_nested_package_versions_block_inspection",
        fs,
        console,
        result,
    );
}

fn insert_configuration_package(
    fs: &MemoryFileSystem,
    directory: &str,
    name: &str,
    version: &str,
    configuration: &str,
) {
    fs.insert(
        format!("{directory}/package.json").into(),
        format!(r#"{{ "name": "{name}", "version": "{version}", "main": "biome.json" }}"#),
    );
    fs.insert(format!("{directory}/biome.json").into(), configuration);
}

#[test]
fn normalizes_windows_paths_in_inspect_snapshots() {
    let content = r#"{
  "path": "node_modules\\@shared\\config\\biome.json"
}
node_modules\package-b\biome.json configuration"#;

    assert_eq!(
        normalize_inspect_snapshot_paths(content.to_string()),
        r#"{
  "path": "node_modules/@shared/config/biome.json"
}
node_modules/package-b/biome.json configuration"#
    );
}
