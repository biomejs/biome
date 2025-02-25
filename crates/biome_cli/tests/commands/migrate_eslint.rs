use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn migrate_eslintrcjson() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let eslintrc = r#"{
        "ignorePatterns": [
            "**/*.test.js", // trailing comma amd comment
        ],
        "globals": {
            "var1": "writable",
            "var2": "readonly"
        },
        "rules": {
            "dot-notation": 0,
            "default-param-last": "off",
            "eqeqeq": "warn",
            "getter-return": [2,
                // support unknown options
                { "allowImplicit": true }
            ],
            "no-eval": 1,
            "no-extra-label": ["error"]
        },
        "overrides": [{
            "files": ["bin/*.js", "lib/*.js"],
            "excludedFiles": "*.test.js",
            "rules": {
                "eqeqeq": ["off"]
            }
        }],
        "unknownField": "ignored"
    }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcjson",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrc() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let eslintrc = r#"{
        "ignorePatterns": [
            "**/*.test.js", // trailing comma amd comment
        ],
        "globals": {
            "var1": "writable",
            "var2": "readonly"
        },
        "rules": {
            "dot-notation": 0,
            "default-param-last": "off",
            "eqeqeq": "warn",
            "getter-return": [2,
                // support unknown options
                { "allowImplicit": true }
            ],
            "no-eval": 1,
            "no-extra-label": ["error"]
        },
        "overrides": [{
            "files": ["bin/*.js", "lib/*.js"],
            "excludedFiles": "*.test.js",
            "rules": {
                "eqeqeq": ["off"]
            }
        }],
        "unknownField": "ignored"
    }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrc",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrcjson_write() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let eslintrc = r#"{
        "ignorePatterns": [
            "**/*.test.js", // trailing comma amd comment
        ],
        "globals": {
            "var1": "writable",
            "var2": "readonly"
        },
        "rules": {
            "dot-notation": 0,
            "default-param-last": "off",
            "eqeqeq": "warn",
            "getter-return": [2,
                // support unknown options
                { "allowImplicit": true }
            ],
            "no-eval": 1,
            "no-extra-label": ["error"]
        },
        "overrides": [{
            "files": ["bin/*.js", "lib/*.js"],
            "excludedFiles": "*.test.js",
            "rules": {
                "eqeqeq": ["off"]
            }
        }],
        "unknownField": "ignored"
    }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcjson_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrcjson_fix() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let eslintrc = r#"{
        "ignorePatterns": [
            "**/*.test.js", // trailing comma amd comment
        ],
        "globals": {
            "var1": "writable",
            "var2": "readonly"
        },
        "rules": {
            "dot-notation": 0,
            "default-param-last": "off",
            "eqeqeq": "warn",
            "getter-return": [2,
                // support unknown options
                { "allowImplicit": true }
            ],
            "no-eval": 1,
            "no-extra-label": ["error"]
        },
        "overrides": [{
            "files": ["bin/*.js", "lib/*.js"],
            "excludedFiles": "*.test.js",
            "rules": {
                "eqeqeq": ["off"]
            }
        }],
        "unknownField": "ignored"
    }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint", "--fix"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcjson_fix",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrcjson_override_existing_config() {
    let biomejson = r#"{ "linter": { "rules": { "recommended": true, "suspicious": { "noDoubleEquals": "error" } } } }"#;
    let eslintrc = r#"{ "rules": { "eqeqeq": "off" } }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcjson_override_existing_config",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrcjson_exclude_inspired() {
    let biomejson = r#"{}"#;
    let eslintrc = r#"{ "rules": { "no-else-return": "error" } }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcjson_exclude_inspired",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrcjson_include_inspired() {
    let biomejson = r#"{}"#;
    let eslintrc = r#"{ "rules": { "no-else-return": "error" } }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint", "--include-inspired"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcjson_include_inspired",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrcjson_rule_options() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let eslintrc = r#"{
        "rules": {
            "no-restricted-globals": ["error", "event", "fdescribe"],
            "jsx-a11y/aria-role": ["error", {
                "allowedInvalidRoles": ["text"],
                "ignoreNonDOM": true
            }],
            "@typescript-eslint/array-type": ["error", { "default": "generic" }],
            "@typescript-eslint/naming-convention": ["error",
                {
                    "selector": "property",
                    "leadingUnderscore": "forbid"
                },
                {
                    "selector": "property",
                    "modifiers": ["private"],
                    "format": ["strictCamelCase"],
                    "leadingUnderscore": "require"
                },
                {
                    "selector": "interface",
                    "prefix": ["I", "IO"]
                },
                {
                    "selector": "enumMember",
                    "format": ["UPPER_CASE"]
                },
                {
                    "selector": "variable",
                    "types": ["boolean"],
                    "format": ["UPPER_CASE"]
                }
            ],
            "unicorn/filename-case": ["error", {
                "cases": {
                    "camelCase": true,
                    "pascalCase": true
                }
            }]
        },
        "overrides": [{
            "files": "default.js",
            "rules": {
                "no-restricted-globals": "error",
                "jsx-a11y/aria-role": "error",
                "@typescript-eslint/array-type": "error",
                "@typescript-eslint/naming-convention": "error",
                "unicorn/filename-case": "error"
            }
        }, {
            "files": ["alternative.js"],
            "rules": {
                "no-restricted-globals": ["error",
                    {
                        "name": "event",
                        "message": "Use local parameter instead."
                    },
                    {
                        "name": "fdescribe",
                        "message": "Do not commit fdescribe. Use describe instead."
                    }
                ],
                "@typescript-eslint/array-type": ["error", { "default": "array" }],
                "@typescript-eslint/naming-convention": ["error",
                    {
                        "selector": "default",
                        "format": ["UPPER_CASE"]
                    }
                ],
                "unicorn/filename-case": ["error", {
                    "case": "kebabCase",
                    "multipleFileExtensions": true
                }]
            }
        }]
    }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint", "--include-inspired"].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcjson_rule_options",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrcjson_empty() {
    let biomejson = r#"{ "linter": { "enabled": true } }
"#;
    let eslintrc = r#"{}"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcjson_empty",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrcjson_missing_biomejson() {
    let eslintrc = r#"{}"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcjson_missing_biomejson",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrcyaml_unsupported() {
    let biomejson = r#"{}"#;
    let eslintrc = "";

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.yaml").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcyaml_unsupported",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslint_config_packagejson() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let packagejson = r#"{
        "name": "foo",
        "version": "0.0.0",
        "eslintConfig": {
            "rules": {
                "eqeqeq": "warn"
            }
        },
        "eslintIgnore": ["/dist", "test", "!test/x/**"]
    }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new("package.json").into(), packagejson.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslint_config_packagejson",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_no_eslint_config_packagejson() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let packagejson = r#"{
        "name": "foo",
        "version": "0.0.0"
    }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new("package.json").into(), packagejson.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_no_eslint_config_packagejson",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintignore() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let eslintrc = r#"{ "rules": { "eqeqeq": "off" } }"#;
    let eslintignore = r#"
# Comment
/src
*.test.js
**/*.spec.js
test/main.js
"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());
    fs.insert(
        Utf8Path::new(".eslintignore").into(),
        eslintignore.as_bytes(),
    );

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintignore",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintignore_and_ignore_patterns() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let eslintrc = r#"{
        "ignorePatterns": ["**/*.spec.js", "!x.spec.js", "/dist"],
        "rules": { "eqeqeq": "off" }
    }"#;
    let eslintignore = r#"*.test.js"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());
    fs.insert(
        Utf8Path::new(".eslintignore").into(),
        eslintignore.as_bytes(),
    );

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintignore_and_ignore_patterns",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintignore_negated_patterns() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let eslintrc = r#"{ "rules": { "eqeqeq": "off" } }"#;
    let eslintignore = r#"
a/**
!a/b
"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());
    fs.insert(
        Utf8Path::new(".eslintignore").into(),
        eslintignore.as_bytes(),
    );

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintignore_negated_patterns",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_eslintrcjson_extended_rules() {
    let biomejson = r#"{ "linter": { "enabled": true } }"#;
    let eslintrc = r#"{
        "rules": {
            "dot-notation": 0,
            "@typescript-eslint/dot-notation": 2,
            "@typescript-eslint/no-dupe-class-members": 2,
            "no-dupe-class-members": 0
        }
    }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_eslintrcjson_extended_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_merge_with_overrides() {
    let biomejson = r#"{
        "overrides": [{
            "includes": ["*.js"],
            "linter": { "enabled": false }
        }]
    }"#;
    let eslintrc = r#"{
        "overrides": [{
            "files": ["bin/*.js", "lib/*.js", null],
            "excludedFiles": "*.test.js",
            "rules": {
                "eqeqeq": ["off"]
            }
        }]
    }"#;

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), biomejson.as_bytes());
    fs.insert(Utf8Path::new(".eslintrc.json").into(), eslintrc.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "eslint"].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_merge_with_overrides",
        fs,
        console,
        result,
    ));
}
