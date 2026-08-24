use crate::{
    run_cli,
    snap_test::{SnapshotPayload, assert_cli_snapshot},
};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;

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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "root_value_uses_diagnostic_output",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "jsonc_resolved_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn resolved_configuration_lists_source_paths_in_merge_order() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "extends": ["first.json", "package-a", "second.json", "package-b"] }"#,
    );
    fs.insert(
        "first.json".into(),
        r#"{ "formatter": { "lineWidth": 100 } }"#,
    );
    insert_configuration_package(
        &fs,
        "node_modules/package-a",
        "package-a",
        "1.0.0",
        r#"{ "formatter": { "indentWidth": 4 } }"#,
    );
    fs.insert(
        "second.json".into(),
        r#"{ "formatter": { "indentStyle": "space" } }"#,
    );
    insert_configuration_package(
        &fs,
        "node_modules/package-b",
        "package-b",
        "1.0.0",
        r#"{ "formatter": { "lineEnding": "lf" } }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "resolved_configuration_lists_source_paths_in_merge_order",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "absent_key_is_successful",
        fs,
        console,
        result,
    ));
}

#[test]
fn absent_child_of_override_scalar_has_no_source() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "overrides": [
    {
      "includes": ["**/*.js"],
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
                "formatter.lineWidth.extra",
                "--path=file.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "absent_child_of_override_scalar_has_no_source",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extended_value_reports_resolved_path",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "package_extended_value_reports_specifier",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "composite_value_reports_contributors",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "composite_value_diagnostic_identifies_contributors",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "composite_value_diagnostic_shows_three_extended_sources",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "identical_composite_declarations_report_the_last_source",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "identical_override_composite_reports_the_override_source",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "last_matching_override_wins",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "later_matching_override_uses_runtime_root_fallback",
        fs,
        console,
        result,
    ));
}

#[test]
fn equal_runtime_root_fallback_reports_root_source() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "formatter": { "formatWithErrors": false },
  "overrides": [
    {
      "includes": ["**/*.test.js"],
      "formatter": { "formatWithErrors": false }
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "equal_runtime_root_fallback_reports_root_source",
        fs,
        console,
        result,
    ));
}

#[test]
fn later_matching_override_resets_unsafe_parameter_decorators() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "overrides": [
    {
      "includes": ["**/*.test.js"],
      "javascript": {
        "parser": { "unsafeParameterDecoratorsEnabled": true }
      }
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
                "javascript.parser.unsafeParameterDecoratorsEnabled",
                "--path=file.test.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "later_matching_override_resets_unsafe_parameter_decorators",
        fs,
        console,
        result,
    ));
}

#[test]
fn javascript_assist_override_matches_runtime() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "javascript": { "assist": { "enabled": true } },
  "overrides": [
    {
      "includes": ["**/*.js"],
      "javascript": { "assist": { "enabled": false } }
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
                "javascript.assist.enabled",
                "--path=file.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "javascript_assist_override_matches_runtime",
        fs,
        console,
        result,
    ));
}

#[test]
fn later_global_override_replaces_language_value() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "javascript": { "formatter": { "lineWidth": 120 } },
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
        Args::from(
            [
                "inspect",
                "config",
                "javascript.formatter.lineWidth",
                "--path=file.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "later_global_override_replaces_language_value",
        fs,
        console,
        result,
    ));
}

#[test]
fn runtime_ignored_override_keeps_base_source() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "javascript": {
    "resolver": { "experimentalPnpmCatalogs": false }
  },
  "overrides": [
    {
      "includes": ["**/*.js"],
      "javascript": {
        "resolver": { "experimentalPnpmCatalogs": true }
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
                "javascript.resolver.experimentalPnpmCatalogs",
                "--path=file.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "runtime_ignored_override_keeps_base_source",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "non_matching_override_keeps_base_value",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "path_requires_key",
        fs,
        console,
        result,
    ));
}

#[test]
fn malformed_key_is_an_invalid_argument() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "config", "formatter..lineWidth"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "malformed_key_is_an_invalid_argument",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "malformed_configuration_is_an_error",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "malformed_extended_configuration_uses_extended_path",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "missing_explicit_configuration_is_an_error",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_configuration_json_is_empty",
        fs,
        console,
        result,
    ));
}

#[test]
fn json_output_includes_source_range() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.jsonc".into(),
        r#"{
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "json_output_includes_source_range",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "matching_extended_override_keeps_source_axes",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "matching_override_appends_plugins",
        fs,
        console,
        result,
    ));
}

#[test]
fn indexed_base_plugin_keeps_base_source() {
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
                "plugins.0",
                "--path=file.test.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indexed_base_plugin_keeps_base_source",
        fs,
        console,
        result,
    ));
}

#[test]
fn indexed_override_plugin_uses_local_source_index() {
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
                "plugins.1",
                "--path=file.test.js",
                "--json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indexed_override_plugin_uses_local_source_index",
        fs,
        console,
        result,
    ));
}

#[test]
fn indexed_extended_array_value_uses_local_source_index() {
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
        Args::from(["inspect", "config", "files.includes.1", "--json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indexed_extended_array_value_uses_local_source_index",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "typed_merge_preserves_extended_shorthand_origin",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "matching_override_preserves_group_shorthand",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "null_value_does_not_replace_extended_provenance",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "replaced_composite_omits_superseded_source",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "conflicting_nested_package_versions_block_inspection",
        fs,
        console,
        result,
    ));
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
