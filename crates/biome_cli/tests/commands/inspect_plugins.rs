use crate::{
    run_cli,
    snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents},
};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;
use serde_json::{Map, Value, json};

#[test]
fn no_configuration() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn empty_configuration() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), "{}");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "empty_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn empty_plugins() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), r#"{ "plugins": [] }"#);
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "empty_plugins",
        fs,
        console,
        result,
    ));
}

#[test]
fn invalid_grit_resolves_human() {
    let fs = MemoryFileSystem::default();
    let configuration = r#"{ "plugins": ["./rules/../rules/invalid.grit"] }"#;
    fs.insert("biome.json".into(), configuration);
    fs.insert("rules/invalid.grit".into(), "`unterminated");
    fs.insert("file.js".into(), "  debugger");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_eq!(fs.files.read().len(), 3);
    assert_file_contents(&fs, Utf8Path::new("biome.json"), configuration);
    assert_file_contents(&fs, Utf8Path::new("rules/invalid.grit"), "`unterminated");
    assert_file_contents(&fs, Utf8Path::new("file.js"), "  debugger");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "invalid_grit_resolves_human",
        fs,
        console,
        result,
    ));
}

#[test]
fn nested_target_keeps_root_configuration() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), r#"{ "plugins": ["./root.grit"] }"#);
    fs.insert(
        "nested/biome.json".into(),
        r#"{ "root": false, "plugins": ["./missing.grit"] }"#,
    );
    fs.insert("root.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--path=nested/file.js"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "nested_target_keeps_root_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn nested_target_keeps_explicit_configuration() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), r#"{ "plugins": ["./root.grit"] }"#);
    fs.insert(
        "config/biome.json".into(),
        r#"{ "plugins": ["./custom.grit"] }"#,
    );
    fs.insert(
        "nested/biome.json".into(),
        r#"{ "root": false, "plugins": ["./missing.grit"] }"#,
    );
    fs.insert("root.grit".into(), "`unterminated");
    fs.insert("config/custom.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "inspect",
                "plugins",
                "--config-path=config/biome.json",
                "--path=nested/file.js",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "nested_target_keeps_explicit_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn multiple_plugins_presets_and_origins() {
    let fs = multiple_plugins_fixture();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_eq!(console.out_buffer.len(), 1);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "multiple_plugins_presets_and_origins",
        fs,
        console,
        result,
    ));
}

#[test]
fn parent_relative_target_matches_normalized_target() {
    let fs = target_normalization_fixture();
    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--path=src/../other/file.ts"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "parent_relative_target_matches_normalized_target",
        fs,
        console,
        result,
    ));
}

#[test]
fn normalized_target_selects_matching_rules() {
    let fs = target_normalization_fixture();
    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--path=other/file.ts"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "normalized_target_selects_matching_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn multiple_plugins_presets_and_origins_for_test_file() {
    let fs = multiple_plugins_fixture();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--path=src/app.test.ts"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_eq!(console.out_buffer.len(), 1);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "multiple_plugins_presets_and_origins_for_test_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn npm_presets_and_transitive_rules_keep_leaf_origins() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "plugins": ["wrapper/presets/recommended", "wrapper/first"] }"#,
    );
    insert_manifest_package(
        &fs,
        "node_modules/wrapper",
        "wrapper",
        r#"{
            "version": 1,
            "plugins": {
                "rules": ["middle/presets/recommended"],
                "presets": { "recommended": ["middle/second", "middle/first"] }
            }
        }"#,
    );
    insert_manifest_package(
        &fs,
        "node_modules/middle",
        "middle",
        r#"{
            "version": 1,
            "plugins": {
                "rules": ["leaf/presets/recommended"],
                "presets": { "recommended": ["leaf/second", "leaf/first"] }
            }
        }"#,
    );
    insert_manifest_package(
        &fs,
        "node_modules/leaf",
        "leaf",
        r#"{
            "version": 1,
            "plugins": {
                "rules": [{ "first": "first.grit", "second": "second.grit", "unused": "missing.grit" }],
                "presets": { "recommended": ["second", "first"] }
            }
        }"#,
    );
    fs.insert("node_modules/leaf/first.grit".into(), "`unterminated");
    fs.insert("node_modules/leaf/second.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "npm_presets_and_transitive_rules_keep_leaf_origins",
        fs,
        console,
        result,
    ));
}

#[test]
fn plugin_inventory_json() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
            "plugins": [
                { "path": "./unfiltered.grit", "includes": ["src/**/*.ts"] },
                "./unfiltered.grit",
            { "path": "./filtered.grit", "includes": ["src/**/*.ts", "!**/*.test.ts"] },
            { "path": "./throw.ts", "includes": ["src/**/*.ts"] },
                "./broken"
            ],
            "overrides": [
                {
                    "includes": ["tests/**"],
                    "plugins": [
                { "path": "./filtered.grit", "includes": ["**/*.test.ts"] },
                { "path": "./throw.ts", "includes": ["**/*.test.ts"] },
                        { "path": "./unfiltered.grit", "includes": ["**/*.test.ts"] }
                    ]
                },
                { "includes": ["scripts/**"], "plugins": ["./filtered.grit"] }
            ]
        }"#,
    );
    fs.insert("unfiltered.grit".into(), "`console.log($message)`");
    fs.insert("filtered.grit".into(), "`console.log($message)`");
    fs.insert("broken/biome-manifest.json".into(), r#"{ "version": 2 }"#);
    fs.insert("throw.ts".into(), "throw new Error('must not execute');");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--json"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "plugin_inventory_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn repeated_preset_keeps_alternative_include_globs() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
            "plugins": [
                { "path": "plugin/presets/recommended", "includes": ["src/**/*.ts", "!**/*.test.ts"] }
            ],
            "overrides": [{
                "includes": ["tests/**"],
                "plugins": [
                    { "path": "plugin/presets/recommended", "includes": ["**/*.test.ts"] }
                ]
            }]
        }"#,
    );
    insert_manifest_package(
        &fs,
        "node_modules/plugin",
        "plugin",
        r#"{
            "version": 1,
            "plugins": {
                "rules": [{ "noConsole": "noConsole.grit" }],
                "presets": { "recommended": ["noConsole"] }
            }
        }"#,
    );
    fs.insert(
        "node_modules/plugin/noConsole.grit".into(),
        "`console.log($message)`",
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "repeated_preset_keeps_alternative_include_globs",
        fs,
        console,
        result,
    ));
}

#[test]
fn local_extended_config_keeps_package_resolution_bases() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "extends": ["shared/base.json"] }"#,
    );
    fs.insert(
        "shared/base.json".into(),
        r#"{
            "plugins": [
                { "path": "plugin/selected", "resolutionKind": "config" },
                "plugin/selected"
            ]
        }"#,
    );
    for package in ["shared/node_modules/plugin", "node_modules/plugin"] {
        insert_manifest_package(
            &fs,
            package,
            "plugin",
            r#"{ "version": 1, "plugins": { "rules": [{ "selected": "selected.grit", "unused": "missing.grit" }] } }"#,
        );
        fs.insert(format!("{package}/selected.grit").into(), "`unterminated");
    }
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "local_extended_config_keeps_package_resolution_bases",
        fs,
        console,
        result,
    ));
}

#[test]
fn manifest_extended_config_keeps_package_resolution_bases() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "extends": ["@shared/config/configs/recommended"] }"#,
    );
    insert_manifest_package(
        &fs,
        "node_modules/@shared/config",
        "@shared/config",
        r#"{ "version": 1, "configs": [{ "recommended": "./recommended.jsonc" }] }"#,
    );
    fs.insert(
        "node_modules/@shared/config/recommended.jsonc".into(),
        r#"{
            "plugins": [
                { "path": "plugin/selected", "resolutionKind": "config" },
                "plugin/selected"
            ]
        }"#,
    );
    for package in [
        "node_modules/@shared/config/node_modules/plugin",
        "node_modules/plugin",
    ] {
        insert_manifest_package(
            &fs,
            package,
            "plugin",
            r#"{ "version": 1, "plugins": { "rules": [{ "selected": "selected.grit", "unused": "missing.grit" }] } }"#,
        );
        fs.insert(format!("{package}/selected.grit").into(), "`unterminated");
    }
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "manifest_extended_config_keeps_package_resolution_bases",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_target_leaves_includes_unevaluated() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
            "plugins": [
                { "path": "./rule.grit", "includes": [] },
                { "path": "./rule.grit", "includes": ["**/*.ts"] }
            ],
            "overrides": [
                { "includes": ["**/*.js"], "plugins": [{ "path": "./rule.grit", "includes": [] }] },
                { "includes": [], "plugins": ["./rule.grit"] },
                { "plugins": ["./rule.grit"] }
            ]
        }"#,
    );
    fs.insert("rule.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_target_leaves_includes_unevaluated",
        fs,
        console,
        result,
    ));
}

#[test]
fn disabled_only_declarations() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
            "plugins": [{ "path": "./base.grit", "includes": [] }],
            "overrides": [{ "includes": [], "plugins": ["./override.grit"] }]
        }"#,
    );
    fs.insert("base.grit".into(), "`unterminated");
    fs.insert("override.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_eq!(console.out_buffer.len(), 1);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "disabled_only_declarations",
        fs,
        console,
        result,
    ));
}

#[test]
fn selection_keeps_configuration_and_plugin_includes_separate() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
            "plugins": ["./rule.grit", { "path": "./rule.grit" }, { "path": "./rule.grit", "includes": [] }],
            "overrides": [
                { "includes": ["**/*.ts"], "plugins": ["./rule.grit", { "path": "./rule.grit", "includes": [] }] },
                { "includes": ["**/*.js"], "plugins": ["./rule.grit"] },
                { "plugins": ["./rule.grit"] },
                { "includes": [], "plugins": ["./rule.grit"] }
            ]
        }"#,
    );
    fs.insert("rule.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--path=src/file.ts"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "selection_keeps_configuration_and_plugin_includes_separate",
        fs,
        console,
        result,
    ));
}

#[test]
fn absolute_target_matches_plugin_globs() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), r#"{ "plugins": ["./wrong.grit"] }"#);
    fs.insert(
        "/project/biome.json".into(),
        r#"{
            "plugins": [
                { "path": "./rule.grit", "includes": ["src/**/*.ts"] },
                { "path": "./rule.grit", "includes": ["/project/src/**/*.ts"] },
                { "path": "./rule.grit", "includes": ["**/*.ts", "!**/*.test.ts"] },
                { "path": "./rule.grit", "includes": ["**/*.ts", "!**/*.test.ts", "**/keep.test.ts"] }
            ],
            "overrides": [{
                "includes": ["src/**/*.ts"],
                "plugins": [{ "path": "./rule.grit", "includes": ["src/**/*.ts"] }]
            }]
        }"#,
    );
    fs.insert(
        "/project/src/biome.json".into(),
        r#"{ "root": false, "plugins": ["./nested.grit"] }"#,
    );
    fs.insert("/project/rule.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "inspect",
                "plugins",
                "--config-path=/project/biome.json",
                "--path=/project/src/file.ts",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "absolute_target_matches_plugin_globs",
        fs,
        console,
        result,
    ));
}

#[test]
fn plugin_negation_excludes_test_files() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "plugins": [
            { "path": "./rule.grit", "includes": ["**/*.ts", "!**/*.test.ts"] },
            { "path": "./rule.grit", "includes": ["**/*.ts", "!**/*.test.ts", "**/keep.test.ts"] }
        ] }"#,
    );
    fs.insert("rule.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--path=src/skip.test.ts"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_eq!(console.out_buffer.len(), 1);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "plugin_negation_excludes_test_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn plugin_reinclude_selects_test_files() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "plugins": [
            { "path": "./rule.grit", "includes": ["**/*.ts", "!**/*.test.ts"] },
            { "path": "./rule.grit", "includes": ["**/*.ts", "!**/*.test.ts", "**/keep.test.ts"] }
        ] }"#,
    );
    fs.insert("rule.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--path=src/keep.test.ts"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "plugin_reinclude_selects_test_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn duplicates_keep_base_merge_order_before_override_declaration_order() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "extends": ["first.json", "second.json"], "plugins": ["./root.grit", "./shared.grit", "./shared.grit"], "overrides": [{ "includes": ["**/*.js"], "plugins": ["./shared.grit", "./shared.grit"] }] }"#,
    );
    fs.insert(
        "first.json".into(),
        r#"{ "plugins": ["./first.grit", "./shared.grit"], "overrides": [{ "includes": ["**/*.js"], "plugins": ["./shared.grit"] }] }"#,
    );
    fs.insert(
        "second.json".into(),
        r#"{ "plugins": ["./second.grit", "./shared.grit"], "overrides": [{ "includes": ["**/*.js"], "plugins": ["./second.grit"] }] }"#,
    );
    for name in ["first", "second", "root", "shared"] {
        fs.insert(format!("{name}.grit").into(), "`unterminated");
    }
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "duplicates_keep_base_merge_order_before_override_declaration_order",
        fs,
        console,
        result,
    ));
}

#[test]
fn local_rules_with_the_same_name_remain_distinct() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "plugins": ["./first/shared.grit", "./second/shared.grit", "./first/shared.grit"] }"#,
    );
    fs.insert("first/shared.grit".into(), "`unterminated");
    fs.insert("second/shared.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_eq!(console.out_buffer.len(), 1);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "local_rules_with_the_same_name_remain_distinct",
        fs,
        console,
        result,
    ));
}

#[test]
fn local_manifest_and_direct_import_resolve_once() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "plugins": ["./rules", "./rules/foo.grit"] }"#,
    );
    fs.insert(
        "rules/biome-manifest.json".into(),
        r#"{ "version": 1, "plugins": { "rules": [{ "foo": "foo.grit" }] } }"#,
    );
    fs.insert("rules/foo.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_eq!(console.out_buffer.len(), 1);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "local_manifest_and_direct_import_resolve_once",
        fs,
        console,
        result,
    ));
}

#[test]
fn local_manifest_aliases_keep_export_names() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "plugins": [{ "path": "./rules", "includes": ["**/*.ts"] }, "./rules/foo.grit"] }"#,
    );
    fs.insert(
        "rules/biome-manifest.json".into(),
        r#"{ "version": 1, "plugins": { "rules": [{ "foo": "foo.grit", "alias": "foo.grit" }] } }"#,
    );
    fs.insert("rules/foo.grit".into(), "`unterminated");
    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "local_manifest_aliases_keep_export_names",
        fs,
        console,
        result,
    ));
}

#[test]
fn excluded_plugin_errors_human() {
    let fs = MemoryFileSystem::default();
    insert_excluded_plugin_errors(&fs);
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--path=file.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert!(console.out_buffer.is_empty());
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "excluded_plugin_errors_human",
        fs,
        console,
        result,
    ));
}

#[test]
fn excluded_plugin_errors_verbose() {
    let fs = MemoryFileSystem::default();
    insert_excluded_plugin_errors(&fs);
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--path=file.js", "--verbose"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "excluded_plugin_errors_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn local_config_only_manifest_resolves_to_empty_rules() {
    let fs = MemoryFileSystem::default();
    fs.insert("biome.json".into(), r#"{ "plugins": ["./config-only"] }"#);
    fs.insert(
        "config-only/biome-manifest.jsonc".into(),
        r#"{ "version": 1, "configs": [{ "recommended": "./missing.json" }] }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "local_config_only_manifest_resolves_to_empty_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn config_relative_resolution_errors_stop_configuration_loading() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "plugins": [{ "path": "missing-package/rule", "resolutionKind": "config" }] }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "config_relative_resolution_errors_stop_configuration_loading",
        fs,
        console,
        result,
    ));
}

#[test]
fn local_manifest_exports_human() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "plugins": ["./rules", "./config-only"] }"#,
    );
    fs.insert(
        "rules/biome-manifest.json".into(),
        r#"{ "version": 1, "plugins": { "rules": [{ "second": "second.grit", "first": "first.grit" }] } }"#,
    );
    fs.insert("rules/first.grit".into(), "`unterminated");
    fs.insert("rules/second.grit".into(), "`unterminated");
    fs.insert(
        "config-only/biome-manifest.json".into(),
        r#"{ "version": 1, "configs": [{ "recommended": "config.json" }] }"#,
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--path=src/file.js"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "local_manifest_exports_human",
        fs,
        console,
        result,
    ));
}

#[test]
fn javascript_and_typescript_resolve_without_evaluation() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{ "plugins": [
            "./known.grit",
            "./throw.js",
            { "path": "./throw.ts", "includes": ["src/**/*.ts", "!**/*.test.ts"] }
        ] }"#,
    );
    fs.insert("known.grit".into(), "`unterminated");
    fs.insert("throw.js".into(), "throw new Error('must not execute');");
    fs.insert(
        "throw.ts".into(),
        "const error: Error = new Error('must not execute'); throw error;",
    );
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "javascript_and_typescript_resolve_without_evaluation",
        fs,
        console,
        result,
    ));
}

#[test]
fn help() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["inspect", "plugins", "--help"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "help",
        fs,
        console,
        result,
    ));
}

fn target_normalization_fixture() -> MemoryFileSystem {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
  "plugins": [
    "./base.grit",
    { "path": "./src-only.grit", "includes": ["src/**"] },
    { "path": "./other-only.grit", "includes": ["other/**"] }
  ],
  "overrides": [
    { "includes": ["src/**"], "plugins": ["./src-override.grit"] }
  ]
}"#,
    );
    for name in ["base", "src-only", "other-only", "src-override"] {
        fs.insert(format!("{name}.grit").into(), "`unterminated");
    }
    fs
}

fn multiple_plugins_fixture() -> MemoryFileSystem {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "biome.json".into(),
        r#"{
            "extends": ["shared/base.json", "@acme/biome-config/configs/recommended"],
            "plugins": [
                "@acme/javascript/presets/strict",
                "@acme/javascript/noDebugger",
                "@acme/javascript/noRestrictedGlobals",
                {
                    "path": "@acme/javascript/noConsole",
                    "includes": ["src/**/*.ts", "!**/*.test.ts"]
                }
            ],
            "overrides": [{
                "includes": ["**/*.test.ts"],
                "plugins": [
                    "@acme/testing/presets/strict",
                    "@acme/testing/noFakeTimers",
                    { "path": "@acme/testing/expectExpect", "includes": ["src/**/*.test.ts"] }
                ]
            }]
        }"#,
    );
    fs.insert(
        "shared/base.json".into(),
        r#"{
            "plugins": [
                "@acme/javascript/presets/recommended",
                { "path": "@acme/accessibility/presets/recommended", "resolutionKind": "config" },
                {
                    "path": "@acme/accessibility/presets/strict",
                    "resolutionKind": "config",
                    "includes": ["src/**/*.tsx"]
                },
                { "path": "@acme/accessibility/noAutofocus", "resolutionKind": "config" },
                { "path": "@acme/accessibility/useValidLang", "resolutionKind": "config" }
            ],
            "overrides": [{
                "includes": ["**/*.test.ts", "**/*.spec.ts"],
                "plugins": ["@acme/testing/presets/recommended"]
            }]
        }"#,
    );
    insert_manifest_package(
        &fs,
        "node_modules/@acme/biome-config",
        "@acme/biome-config",
        r#"{ "version": 1, "configs": [{ "recommended": "./recommended.jsonc" }] }"#,
    );
    fs.insert(
        "node_modules/@acme/biome-config/recommended.jsonc".into(),
        r#"{
            "plugins": [
                { "path": "@acme/security/presets/recommended", "resolutionKind": "config" },
                {
                    "path": "@acme/security/presets/strict",
                    "resolutionKind": "config",
                    "includes": ["src/**/*.ts", "!**/*.test.ts"]
                },
                { "path": "@acme/security/noSecrets", "resolutionKind": "config" },
                { "path": "@acme/security/useSecureCookies", "resolutionKind": "config" },
            ],
            "overrides": [{
                "includes": ["e2e/**/*.ts"],
                "plugins": ["@acme/testing/presets/strict"]
            }],
        }"#,
    );

    let javascript_rules = [
        "noDebugger",
        "noAlert",
        "noEval",
        "noVar",
        "useConst",
        "useStrictEquality",
        "useObjectShorthand",
        "useTemplate",
        "noConsole",
        "noImplicitCoercion",
        "noNestedTernary",
    ];
    insert_manifest_package(
        &fs,
        "node_modules/@acme/javascript",
        "@acme/javascript",
        r#"{
            "version": 1,
            "plugins": {
                "rules": [
                    "@acme/standards/presets/all",
                    { "noRestrictedGlobals": "rules/noRestrictedGlobals.grit" }
                ],
                "presets": {
                    "recommended": [
                        "@acme/standards/noDebugger", "@acme/standards/noAlert",
                        "@acme/standards/noEval", "@acme/standards/noVar",
                        "@acme/standards/useConst", "@acme/standards/useStrictEquality",
                        "@acme/standards/useObjectShorthand", "@acme/standards/useTemplate"
                    ],
                    "strict": [
                        "@acme/standards/noDebugger", "@acme/standards/noAlert",
                        "@acme/standards/noEval", "@acme/standards/noVar",
                        "@acme/standards/useConst", "@acme/standards/useStrictEquality",
                        "@acme/standards/useObjectShorthand", "@acme/standards/useTemplate",
                        "@acme/standards/noConsole", "@acme/standards/noImplicitCoercion",
                        "@acme/standards/noNestedTernary"
                    ]
                }
            }
        }"#,
    );
    fs.insert(
        "node_modules/@acme/javascript/rules/noRestrictedGlobals.grit".into(),
        "`console.log($message)`",
    );
    insert_manifest_package(
        &fs,
        "node_modules/@acme/javascript/node_modules/@acme/standards",
        "@acme/standards",
        &json!({
            "version": 1,
            "plugins": {
                "rules": ["@acme/platform/presets/all"],
                "presets": {
                    "all": javascript_rules.iter().map(|name| format!("@acme/platform/{name}")).collect::<Vec<_>>()
                }
            }
        })
        .to_string(),
    );
    insert_manifest_package(
        &fs,
        "node_modules/@acme/platform",
        "@acme/platform",
        &json!({
            "version": 1,
            "plugins": {
                "rules": ["@acme/language-rules/presets/all"],
                "presets": {
                    "all": javascript_rules.iter().map(|name| format!("@acme/language-rules/{name}")).collect::<Vec<_>>()
                }
            }
        })
        .to_string(),
    );
    let language_rules_directory = "node_modules/@acme/platform/node_modules/@acme/language-rules";
    let rules = insert_grit_rules(&fs, language_rules_directory, &javascript_rules);
    insert_manifest_package(
        &fs,
        language_rules_directory,
        "@acme/language-rules",
        &json!({
            "version": 1,
            "plugins": {
                "rules": [rules],
                "presets": { "all": javascript_rules }
            }
        })
        .to_string(),
    );

    let accessibility_directory = "shared/node_modules/@acme/accessibility";
    let rules = insert_grit_rules(
        &fs,
        &format!("{accessibility_directory}/dist"),
        &[
            "noAutofocus",
            "noPositiveTabindex",
            "noRedundantRoles",
            "useAltText",
            "useAnchorContent",
            "useAriaPropsForRole",
            "useButtonType",
            "useHeadingContent",
            "useHtmlLang",
            "useKeyWithClickEvents",
            "useMediaCaption",
            "useValidLang",
        ],
    );
    fs.insert(
        format!("{accessibility_directory}/package.json").into(),
        r#"{
            "name": "@acme/accessibility",
            "exports": {
                ".": {
                    "biome": "./dist/biome-manifest.jsonc",
                    "default": "./index.js"
                }
            }
        }"#,
    );
    let plugins = json!({
        "rules": [rules],
        "presets": {
            "recommended": [
                "noAutofocus", "noPositiveTabindex", "noRedundantRoles", "useAltText",
                "useAnchorContent", "useAriaPropsForRole", "useButtonType",
                "useHeadingContent", "useHtmlLang", "useKeyWithClickEvents"
            ],
            "strict": [
                "noAutofocus", "noPositiveTabindex", "noRedundantRoles", "useAltText",
                "useAnchorContent", "useAriaPropsForRole", "useButtonType",
                "useHeadingContent", "useHtmlLang", "useKeyWithClickEvents", "useMediaCaption"
            ]
        }
    });
    fs.insert(
        format!("{accessibility_directory}/dist/biome-manifest.jsonc").into(),
        format!("{{\n\"version\": 1,\n\"plugins\": {plugins},\n}}"),
    );

    let security_directory = "node_modules/@acme/biome-config/node_modules/@acme/security";
    let rules = insert_grit_rules(
        &fs,
        security_directory,
        &[
            "noSecrets",
            "noUnsafeInnerHtml",
            "noDocumentWrite",
            "noDynamicRequire",
            "noInsecureRandom",
            "noSqlInterpolation",
            "noShellInterpolation",
            "noPathTraversal",
            "noWeakCrypto",
            "useHttps",
            "useTrustedTypes",
            "useSecureCookies",
        ],
    );
    insert_manifest_package(
        &fs,
        security_directory,
        "@acme/security",
        &json!({
            "version": 1,
            "plugins": {
                "rules": [rules],
                "presets": {
                    "recommended": [
                        "noSecrets", "noUnsafeInnerHtml", "noDocumentWrite", "noDynamicRequire",
                        "noInsecureRandom", "noSqlInterpolation", "noShellInterpolation",
                        "noPathTraversal", "noWeakCrypto", "useHttps"
                    ],
                    "strict": [
                        "noSecrets", "noUnsafeInnerHtml", "noDocumentWrite", "noDynamicRequire",
                        "noInsecureRandom", "noSqlInterpolation", "noShellInterpolation",
                        "noPathTraversal", "noWeakCrypto", "useHttps", "useTrustedTypes"
                    ]
                }
            }
        })
        .to_string(),
    );

    let rules = insert_grit_rules(
        &fs,
        "node_modules/@acme/testing",
        &[
            "noFocusedTests",
            "noDisabledTests",
            "noDuplicateHooks",
            "noIdenticalTitle",
            "noConditionalExpect",
            "noStandaloneExpect",
            "expectExpect",
            "validDescribeCallback",
            "useToStrictEqual",
            "useToHaveLength",
            "useAwaitAsyncMatchers",
            "noFakeTimers",
        ],
    );
    insert_manifest_package(
        &fs,
        "node_modules/@acme/testing",
        "@acme/testing",
        &json!({
            "version": 1,
            "plugins": {
                "rules": [rules],
                "presets": {
                    "recommended": [
                        "noFocusedTests", "noDisabledTests", "noDuplicateHooks", "noIdenticalTitle",
                        "noConditionalExpect", "noStandaloneExpect", "expectExpect", "validDescribeCallback"
                    ],
                    "strict": [
                        "noFocusedTests", "noDisabledTests", "noDuplicateHooks", "noIdenticalTitle",
                        "noConditionalExpect", "noStandaloneExpect", "expectExpect", "validDescribeCallback",
                        "useToStrictEqual", "useToHaveLength", "useAwaitAsyncMatchers"
                    ]
                }
            }
        })
        .to_string(),
    );
    fs
}

fn insert_grit_rules(fs: &MemoryFileSystem, directory: &str, names: &[&str]) -> Map<String, Value> {
    names
        .iter()
        .map(|&name| {
            let path = format!("rules/{name}.grit");
            fs.insert(
                format!("{directory}/{path}").into(),
                "`console.log($message)`",
            );
            (name.to_string(), json!(path))
        })
        .collect()
}

fn insert_excluded_plugin_errors(fs: &MemoryFileSystem) {
    fs.insert(
        "biome.json".into(),
        r#"{
            "plugins": ["./before.grit"],
            "overrides": [{
                "includes": ["**/*.test.js"],
                "plugins": [
                    "./missing.grit",
                    "./broken",
                    { "path": "missing-package/presets/recommended", "resolutionKind": "project" },
                    "./after.grit"
                ]
            }]
        }"#,
    );
    fs.insert("before.grit".into(), "`unterminated");
    fs.insert("after.grit".into(), "`unterminated");
    fs.insert("broken/biome-manifest.json".into(), r#"{ "version": 2 }"#);
}

fn insert_manifest_package(fs: &MemoryFileSystem, directory: &str, name: &str, manifest: &str) {
    fs.insert(
        format!("{directory}/package.json").into(),
        json!({ "name": name, "exports": { "biome": "./biome-manifest.json", "default": "./index.js" } }).to_string(),
    );
    fs.insert(format!("{directory}/biome-manifest.json").into(), manifest);
}
