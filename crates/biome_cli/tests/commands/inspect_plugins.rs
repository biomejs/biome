use crate::{
    run_cli,
    snap_test::{SnapshotPayload, assert_cli_snapshot},
};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use serde_json::{Map, Value, json};

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
fn plugin_inventory() {
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
        Args::from(["inspect", "plugins"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "plugin_inventory",
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
