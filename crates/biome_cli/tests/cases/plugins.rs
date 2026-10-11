use crate::TestArgs as Args;
use crate::run_cli_with_dyn_fs;
use crate::run_cli_with_server_workspace;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::{MemoryFileSystem, TemporaryFs};
#[cfg(unix)]
use std::os::unix::fs::symlink;

#[test]
fn rejects_bare_plugin_package() {
    let mut fs = TemporaryFs::new("rejects_bare_plugin_package");

    fs.create_file("biome.json", r#"{ "plugins": ["@shared/plugin"] }"#);
    fs.create_file(
        "node_modules/@shared/plugin/package.json",
        r#"{ "name": "@shared/plugin" }"#,
    );
    fs.create_file(
        "node_modules/@shared/plugin/biome-manifest.json",
        r#"{
    "version": 1,
    "plugins": { "rules": [{ "noAssign": "rules/noAssign.grit" }] }
}"#,
    );
    fs.create_file("test.js", "Object.assign({}, value);\n");

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/test.js", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "rejects_bare_plugin_package",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn rejects_bare_package_import_in_manifest() {
    let fs = MemoryFileSystem::default();

    fs.insert(
        "biome-manifest.json".into(),
        r#"{
    "version": 1,
    "plugins": { "rules": ["@shared/rules"] }
}"#,
    );

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["lint", "biome-manifest.json"].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "rejects_bare_package_import_in_manifest",
        fs,
        console,
        result,
    ));
}

#[test]
fn local_manifest_export_name_can_be_suppressed() {
    let mut fs = TemporaryFs::new("local_manifest_export_name_can_be_suppressed");

    fs.create_file("biome.json", r#"{ "plugins": ["./plugin"] }"#);
    fs.create_file(
        "plugin/biome-manifest.json",
        r#"{
    "version": 1,
    "plugins": { "rules": [{ "noAssign": "rules/assign.grit" }] }
}"#,
    );
    fs.create_file(
        "plugin/rules/assign.grit",
        r#"`Object.assign($args)` where {
    register_diagnostic(span = $args, message = "Prefer object spread")
}"#,
    );
    fs.create_file(
        "test.js",
        "// biome-ignore lint/plugin/noAssign: compatibility\nObject.assign({}, value);\n",
    );

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/test.js", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "local_manifest_export_name_can_be_suppressed",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn package_manifest_loads_javascript_plugin() {
    let mut fs = TemporaryFs::new("package_manifest_loads_javascript_plugin");

    fs.create_file("biome.json", r#"{ "plugins": ["@scope/plugin/noAssign"] }"#);
    fs.create_file(
        "node_modules/@scope/plugin/package.json",
        r#"{ "name": "@scope/plugin" }"#,
    );
    fs.create_file(
        "node_modules/@scope/plugin/biome-manifest.json",
        r#"{
    "version": 1,
    "plugins": { "rules": [{ "noAssign": "rules/noAssign.js" }] }
}"#,
    );
    fs.create_file(
        "node_modules/@scope/plugin/rules/noAssign.js",
        r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";

export const noAssign = defineRule({
    query: ast("JS_CALL_EXPRESSION"),
    run(node) {
        registerDiagnostic(node, "error", "Do not use Object.assign");
    },
});"#,
    );
    fs.create_file(
        "test.js",
        "// biome-ignore lint/plugin/@scope/plugin/noAssign: compatibility\nObject.assign({});\nObject.assign({});\n",
    );

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/test.js", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "package_manifest_loads_javascript_plugin",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn package_manifest_preset_is_loaded_once() {
    let mut fs = TemporaryFs::new("package_manifest_preset_is_loaded_once");

    fs.create_file(
        "biome.json",
        r#"{
    "plugins": [
        "@scope/plugin/presets/recommended",
        { "path": "@scope/plugin/presets/recommended" }
    ]
}"#,
    );
    fs.create_file(
        "node_modules/@scope/plugin/package.json",
        r#"{ "name": "@scope/plugin" }"#,
    );
    fs.create_file(
        "node_modules/@scope/plugin/biome-manifest.json",
        r#"{
    "version": 1,
    "plugins": {
        "rules": [{
            "noAssign": "rules/noAssign.grit",
            "noKeys": "rules/noKeys.grit"
        }],
        "presets": { "recommended": ["noAssign", "noKeys"] }
    }
}"#,
    );
    fs.create_file(
        "node_modules/@scope/plugin/rules/noAssign.grit",
        r#"`Object.assign($args)` where {
    register_diagnostic(span = $args, message = "Do not use Object.assign")
}"#,
    );
    fs.create_file(
        "node_modules/@scope/plugin/rules/noKeys.grit",
        r#"`Object.keys($args)` where {
    register_diagnostic(span = $args, message = "Do not use Object.keys")
}"#,
    );
    fs.create_file("test.js", "Object.assign({}); Object.keys({});\n");

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/test.js", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "package_manifest_preset_is_loaded_once",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn package_qualified_suppression_only_suppresses_selected_package() {
    let mut fs = TemporaryFs::new("package_qualified_suppression_only_suppresses_selected_package");

    fs.create_file(
        "biome.json",
        r#"{ "plugins": ["@scope/first-plugin/noAssign", "second-plugin/noAssign"] }"#,
    );
    for package in ["@scope/first-plugin", "second-plugin"] {
        fs.create_file(
            &format!("node_modules/{package}/package.json"),
            &format!(r#"{{ "name": "{package}" }}"#),
        );
        fs.create_file(
            &format!("node_modules/{package}/biome-manifest.json"),
            r#"{
    "version": 1,
    "plugins": { "rules": [{ "noAssign": "rules/noAssign.grit" }] }
}"#,
        );
        fs.create_file(
            &format!("node_modules/{package}/rules/noAssign.grit"),
            &format!(
                r#"`Object.assign($args)` where {{
    register_diagnostic(span = $args, message = "Diagnostic from {package}")
}}"#
            ),
        );
    }
    fs.create_file(
        "test.js",
        "// biome-ignore lint/plugin/@scope/first-plugin/noAssign: compatibility\nObject.assign({});\n",
    );

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/test.js", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "package_qualified_suppression_only_suppresses_selected_package",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn local_plugins_allow_duplicate_rule_names() {
    let mut fs = TemporaryFs::new("local_plugins_allow_duplicate_rule_names");

    fs.create_file(
        "biome.json",
        r#"{ "plugins": ["./first/noAssign.grit", "./second/noAssign.grit"] }"#,
    );
    fs.create_file(
        "first/noAssign.grit",
        r#"`Object.assign($args)` where {
    register_diagnostic(span = $args, message = "Diagnostic from first plugin")
}"#,
    );
    fs.create_file(
        "second/noAssign.grit",
        r#"`Object.assign($args)` where {
    register_diagnostic(span = $args, message = "Diagnostic from second plugin")
}"#,
    );
    fs.create_file("test.js", "Object.assign({});\n");

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/test.js", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "local_plugins_allow_duplicate_rule_names",
        fs.create_mem(),
        console,
        result,
    ));
}

#[cfg(unix)]
#[test]
fn rejects_package_plugin_manifest_outside_package() {
    let mut fs = TemporaryFs::new("rejects_package_plugin_manifest_outside_package");

    fs.create_file("biome.json", r#"{ "plugins": ["plugin/noAssign"] }"#);
    fs.create_file(
        "node_modules/plugin/package.json",
        r#"{
    "name": "plugin",
    "exports": { "biome": "./biome-manifest.json" }
}"#,
    );
    let outside_manifest = fs.create_file(
        "outside/biome-manifest.json",
        r#"{
    "version": 1,
    "plugins": { "rules": [{ "noAssign": "noAssign.grit" }] }
}"#,
    );
    fs.create_file("outside/noAssign.grit", r#"`Object.assign($args)`"#);
    symlink(
        outside_manifest,
        fs.working_directory
            .join("node_modules/plugin/biome-manifest.json"),
    )
    .unwrap();
    fs.create_file("test.js", "Object.assign({}, value);\n");

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/test.js", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "rejects_package_plugin_manifest_outside_package",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn extends_config_with_object_syntax_plugin_from_npm_package() {
    let mut fs = TemporaryFs::new("extends_config_with_object_syntax_plugin_from_npm_package");

    fs.create_file("biome.json", r#"{ "extends": ["@shared/config/biome"] }"#);

    fs.create_file(
        "node_modules/@shared/config/biome.jsonc",
        r#"{ "root": false, "plugins": [{ "path": "./grit/no-object-assign.grit", "resolutionKind": "config" }], "linter": { "enabled": true } }"#,
    );
    fs.create_file(
        "node_modules/@shared/config/package.json",
        r#"{
    "name": "@shared/config",
    "exports": {
        "./biome": "./biome.jsonc"
    }
}"#,
    );
    fs.create_file(
        "node_modules/@shared/config/grit/no-object-assign.grit",
        r#"`$fn($args)` where {
    $fn <: `Object.assign`,
    register_diagnostic(
        span = $fn,
        message = "Prefer object spread instead of Object.assign()",
        severity = "warn"
    )
}"#,
    );
    fs.create_file("test.js", "const merged = Object.assign({}, a, b);\n");

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/test.js", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extends_config_with_object_syntax_plugin_from_npm_package",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn extended_config_resolves_plugin_package_from_config() {
    let mut fs = TemporaryFs::new("extended_config_resolves_plugin_package_from_config");

    fs.create_file("biome.json", r#"{ "extends": ["@shared/config/biome"] }"#);
    fs.create_file(
        "node_modules/@shared/config/biome.jsonc",
        r#"{
    "root": false,
    "plugins": [{ "path": "@shared/plugin/presets/recommended", "resolutionKind": "config" }]
}"#,
    );
    fs.create_file(
        "node_modules/@shared/config/package.json",
        r#"{
    "name": "@shared/config",
    "exports": { "./biome": "./biome.jsonc" }
}"#,
    );
    fs.create_file(
        "node_modules/@shared/config/node_modules/@shared/plugin/package.json",
        r#"{ "name": "@shared/plugin" }"#,
    );
    fs.create_file(
        "node_modules/@shared/config/node_modules/@shared/plugin/biome-manifest.json",
        r#"{
    "version": 1,
    "plugins": {
        "rules": ["@shared/rules/presets/recommended"],
        "presets": { "recommended": ["@shared/rules/noAssign"] }
    }
}"#,
    );
    fs.create_file(
        "node_modules/@shared/config/node_modules/@shared/rules/package.json",
        r#"{ "name": "@shared/rules" }"#,
    );
    fs.create_file(
        "node_modules/@shared/config/node_modules/@shared/rules/biome-manifest.json",
        r#"{
    "version": 1,
    "plugins": {
        "rules": [{ "noAssign": "rules/noAssign.grit" }],
        "presets": { "recommended": ["noAssign"] }
    }
}"#,
    );
    fs.create_file(
        "node_modules/@shared/config/node_modules/@shared/rules/rules/noAssign.grit",
        r#"`Object.assign($args)` where {
    register_diagnostic(
        span = $args,
        message = "Prefer object spread instead of Object.assign()"
    )
}"#,
    );
    fs.create_file(
        "test.js",
        "// biome-ignore lint/plugin/@shared/plugin/noAssign: compatibility\nObject.assign({}, value);\n",
    );

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/test.js", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "extended_config_resolves_plugin_package_from_config",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn plugin_includes_are_relative_to_configuration_directory() {
    let mut fs = TemporaryFs::new("plugin_includes_are_relative_to_configuration_directory");

    fs.create_file(
        "biome.json",
        r#"{
    "plugins": [
        {
            "path": "./no-assign.grit",
            "includes": ["packages/**/src/**", "!packages/**/src/generated/**"]
        }
    ]
}"#,
    );
    fs.create_file(
        "no-assign.grit",
        r#"`Object.assign($args)` where {
    register_diagnostic(span = $args, message = "Prefer object spread")
}"#,
    );
    fs.create_file(
        "packages/example/src/index.js",
        "Object.assign({}, value);\n",
    );
    fs.create_file(
        "packages/example/src/generated/index.js",
        "Object.assign({}, value);\n",
    );
    fs.create_file(
        "packages/example/scripts/index.js",
        "Object.assign({}, value);\n",
    );

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "plugin_includes_are_relative_to_configuration_directory",
        fs.create_mem(),
        console,
        result,
    ));
}

/// Returns a GritQL plugin that reports every `Object.assign()` call with `message`.
fn report_assign_plugin(message: &str) -> String {
    format!(
        r#"`Object.assign($args)` where {{
    register_diagnostic(span = $args, message = "{message}")
}}"#
    )
}

/// Lints the whole project in `fs` and snapshots the result as `test_name`.
fn lint_project(fs: TemporaryFs, test_name: &str) {
    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        test_name,
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn plugin_includes_in_overrides_are_anchored_to_project_directory() {
    let mut fs = TemporaryFs::new("plugin_includes_in_overrides_are_anchored_to_project_directory");

    fs.create_file(
        "biome.json",
        r#"{
    "overrides": [
        {
            "includes": ["src/**", "!src/vendor/**"],
            "plugins": [
                { "path": "./no-assign.grit", "includes": ["src/**", "!src/generated/**"] }
            ]
        }
    ]
}"#,
    );
    fs.create_file(
        "no-assign.grit",
        &report_assign_plugin("Prefer object spread"),
    );
    for path in [
        "src/index.js",
        "src/generated/index.js",
        "src/vendor/index.js",
        "lib/index.js",
        "packages/app/src/index.js",
    ] {
        fs.create_file(path, "Object.assign({}, value);\n");
    }

    lint_project(
        fs,
        "plugin_includes_in_overrides_are_anchored_to_project_directory",
    );
}

#[test]
fn javascript_plugin_includes_are_relative_to_project_directory() {
    let mut fs = TemporaryFs::new("javascript_plugin_includes_are_relative_to_project_directory");

    fs.create_file(
        "biome.json",
        r#"{
    "plugins": [
        { "path": "./no-assign.js", "includes": ["src/**", "!src/generated/**"] }
    ]
}"#,
    );
    fs.create_file(
        "no-assign.js",
        r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";

export const noAssign = defineRule({
    query: ast("JS_CALL_EXPRESSION"),
    run(node) {
        registerDiagnostic(node, "error", "Do not use Object.assign");
    },
});"#,
    );
    for path in [
        "src/index.js",
        "src/generated/index.js",
        "packages/app/src/index.js",
    ] {
        fs.create_file(path, "Object.assign({}, value);\n");
    }

    lint_project(
        fs,
        "javascript_plugin_includes_are_relative_to_project_directory",
    );
}

#[test]
fn package_preset_includes_apply_to_every_rule() {
    let mut fs = TemporaryFs::new("package_preset_includes_apply_to_every_rule");

    fs.create_file(
        "biome.json",
        r#"{
    "plugins": [
        {
            "path": "@scope/plugin/presets/recommended",
            "includes": ["src/**", "!src/generated/**"]
        }
    ]
}"#,
    );
    fs.create_file(
        "node_modules/@scope/plugin/package.json",
        r#"{ "name": "@scope/plugin" }"#,
    );
    fs.create_file(
        "node_modules/@scope/plugin/biome-manifest.json",
        r#"{
    "version": 1,
    "plugins": {
        "rules": [{
            "noAssign": "rules/noAssign.grit",
            "noKeys": "rules/noKeys.js"
        }],
        "presets": { "recommended": ["noAssign", "noKeys"] }
    }
}"#,
    );
    fs.create_file(
        "node_modules/@scope/plugin/rules/noAssign.grit",
        &report_assign_plugin("Do not use Object.assign"),
    );
    fs.create_file(
        "node_modules/@scope/plugin/rules/noKeys.js",
        r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";

export const noKeys = defineRule({
    query: ast("JS_CALL_EXPRESSION"),
    run(node) {
        if (node.callee.text.startsWith("Object.keys")) {
            registerDiagnostic(node, "error", "Do not use Object.keys");
        }
    },
});"#,
    );
    for path in [
        "src/index.js",
        "src/generated/index.js",
        "packages/app/src/index.js",
    ] {
        fs.create_file(path, "Object.assign({}, value);\nObject.keys(value);\n");
    }

    lint_project(fs, "package_preset_includes_apply_to_every_rule");
}

#[test]
fn plugin_includes_globstar_patterns_match_nested_directories() {
    let mut fs = TemporaryFs::new("plugin_includes_globstar_patterns_match_nested_directories");

    fs.create_file(
        "biome.json",
        r#"{
    "plugins": [
        { "path": "./no-assign.grit", "includes": ["**/src/**", "!**/generated/**"] }
    ]
}"#,
    );
    fs.create_file(
        "no-assign.grit",
        &report_assign_plugin("Prefer object spread"),
    );
    for path in [
        "src/index.js",
        "packages/app/src/index.js",
        "packages/app/src/generated/index.js",
        "lib/index.js",
    ] {
        fs.create_file(path, "Object.assign({}, value);\n");
    }

    lint_project(
        fs,
        "plugin_includes_globstar_patterns_match_nested_directories",
    );
}

#[test]
fn root_plugin_includes_are_relative_to_child_config_that_extends_root() {
    let mut fs =
        TemporaryFs::new("root_plugin_includes_are_relative_to_child_config_that_extends_root");

    fs.create_file(
        "biome.json",
        r#"{
    "root": true,
    "plugins": [
        { "path": "./biome/root-relative.grit", "includes": ["packages/mobile/src/**"] },
        { "path": "./biome/child-relative.grit", "includes": ["src/**"] },
        {
            "path": "./biome/root-relative-config.grit",
            "resolutionKind": "config",
            "includes": ["packages/mobile/src/**"]
        },
        {
            "path": "./biome/child-relative-config.grit",
            "resolutionKind": "config",
            "includes": ["src/**"]
        }
    ]
}"#,
    );
    for (name, message) in [
        ("root-relative", "Pattern relative to the root directory"),
        ("child-relative", "Pattern relative to the child directory"),
        (
            "root-relative-config",
            "Pattern relative to the root directory, resolutionKind config",
        ),
        (
            "child-relative-config",
            "Pattern relative to the child directory, resolutionKind config",
        ),
    ] {
        fs.create_file(
            &format!("biome/{name}.grit"),
            &report_assign_plugin(message),
        );
    }
    fs.create_file("packages/mobile/biome.json", r#"{ "extends": "//" }"#);
    fs.create_file("src/file.js", "Object.assign({}, value);\n");
    fs.create_file("packages/mobile/src/file.js", "Object.assign({}, value);\n");

    lint_project(
        fs,
        "root_plugin_includes_are_relative_to_child_config_that_extends_root",
    );
}

#[test]
fn child_plugin_includes_are_relative_to_child_config() {
    let mut fs = TemporaryFs::new("child_plugin_includes_are_relative_to_child_config");

    fs.create_file("biome.json", r#"{ "root": true }"#);
    fs.create_file(
        "packages/mobile/biome.json",
        r#"{
    "root": false,
    "plugins": [
        { "path": "./biome/root-relative.grit", "includes": ["packages/mobile/src/**"] },
        { "path": "./biome/child-relative.grit", "includes": ["src/**"] }
    ]
}"#,
    );
    fs.create_file(
        "packages/mobile/biome/root-relative.grit",
        &report_assign_plugin("Pattern relative to the root directory"),
    );
    fs.create_file(
        "packages/mobile/biome/child-relative.grit",
        &report_assign_plugin("Pattern relative to the child directory"),
    );
    fs.create_file("src/file.js", "Object.assign({}, value);\n");
    fs.create_file("packages/mobile/src/file.js", "Object.assign({}, value);\n");
    fs.create_file("packages/mobile/lib/file.js", "Object.assign({}, value);\n");

    lint_project(fs, "child_plugin_includes_are_relative_to_child_config");
}

#[test]
fn extended_config_plugin_includes_ignore_resolution_kind() {
    let mut fs = TemporaryFs::new("extended_config_plugin_includes_ignore_resolution_kind");

    fs.create_file("biome.json", r#"{ "extends": ["./shared/base.json"] }"#);
    fs.create_file(
        "shared/base.json",
        r#"{
    "plugins": [
        { "path": "./plugins/default.grit", "includes": ["src/**"] },
        { "path": "./plugins/project.grit", "resolutionKind": "project", "includes": ["src/**"] },
        { "path": "./plugins/config.grit", "resolutionKind": "config", "includes": ["src/**"] }
    ]
}"#,
    );
    fs.create_file(
        "plugins/default.grit",
        &report_assign_plugin("Default resolutionKind"),
    );
    fs.create_file(
        "plugins/project.grit",
        &report_assign_plugin("resolutionKind project"),
    );
    fs.create_file(
        "shared/plugins/config.grit",
        &report_assign_plugin("resolutionKind config"),
    );
    fs.create_file("src/file.js", "Object.assign({}, value);\n");
    fs.create_file("shared/src/file.js", "Object.assign({}, value);\n");

    lint_project(fs, "extended_config_plugin_includes_ignore_resolution_kind");
}

/// Returns a GritQL plugin that reports every `Object.keys()` call with `message`.
fn report_keys_plugin(message: &str) -> String {
    format!(
        r#"`Object.keys($args)` where {{
    register_diagnostic(span = $args, message = "{message}")
}}"#
    )
}

/// Creates a plugin package named `@shared/plugin` in `directory`. Its `recommended` preset
/// contains two rules, whose diagnostics start with `origin`.
fn create_preset_package(fs: &mut TemporaryFs, directory: &str, origin: &str) {
    fs.create_file(
        &format!("{directory}/package.json"),
        r#"{ "name": "@shared/plugin" }"#,
    );
    fs.create_file(
        &format!("{directory}/biome-manifest.json"),
        r#"{
    "version": 1,
    "plugins": {
        "rules": [{
            "noAssign": "rules/noAssign.grit",
            "noKeys": "rules/noKeys.grit"
        }],
        "presets": { "recommended": ["noAssign", "noKeys"] }
    }
}"#,
    );
    fs.create_file(
        &format!("{directory}/rules/noAssign.grit"),
        &report_assign_plugin(&format!("{origin}: do not use Object.assign")),
    );
    fs.create_file(
        &format!("{directory}/rules/noKeys.grit"),
        &report_keys_plugin(&format!("{origin}: do not use Object.keys")),
    );
}

#[test]
fn config_relative_package_preset_includes_are_relative_to_consuming_config() {
    let mut fs = TemporaryFs::new(
        "config_relative_package_preset_includes_are_relative_to_consuming_config",
    );

    fs.create_file("biome.json", r#"{ "extends": ["@shared/config/biome"] }"#);
    fs.create_file(
        "node_modules/@shared/config/biome.jsonc",
        r#"{
    "root": false,
    "plugins": [
        {
            "path": "@shared/plugin/presets/recommended",
            "resolutionKind": "config",
            "includes": ["src/**", "!src/generated/**"]
        }
    ]
}"#,
    );
    fs.create_file(
        "node_modules/@shared/config/package.json",
        r#"{
    "name": "@shared/config",
    "exports": { "./biome": "./biome.jsonc" }
}"#,
    );
    create_preset_package(
        &mut fs,
        "node_modules/@shared/config/node_modules/@shared/plugin",
        "Package of the extended configuration",
    );
    for path in [
        "src/index.js",
        "src/generated/index.js",
        "lib/index.js",
        "packages/app/src/index.js",
    ] {
        fs.create_file(path, "Object.assign({}, value);\nObject.keys(value);\n");
    }

    lint_project(
        fs,
        "config_relative_package_preset_includes_are_relative_to_consuming_config",
    );
}

#[test]
fn package_preset_includes_in_local_extended_config_are_relative_to_consuming_config() {
    let mut fs = TemporaryFs::new(
        "package_preset_includes_in_local_extended_config_are_relative_to_consuming_config",
    );

    fs.create_file("biome.json", r#"{ "extends": ["./shared/base.json"] }"#);
    fs.create_file(
        "shared/base.json",
        r#"{
    "plugins": [
        {
            "path": "@shared/plugin/presets/recommended",
            "resolutionKind": "config",
            "includes": ["src/**", "!src/generated/**"]
        },
        {
            "path": "@shared/plugin/presets/recommended",
            "includes": ["src/**", "!src/generated/**"]
        }
    ]
}"#,
    );
    create_preset_package(
        &mut fs,
        "shared/node_modules/@shared/plugin",
        "Package of the extended configuration",
    );
    create_preset_package(
        &mut fs,
        "node_modules/@shared/plugin",
        "Package of the consuming project",
    );
    for path in [
        "src/index.js",
        "src/generated/index.js",
        "shared/src/index.js",
        "packages/app/src/index.js",
    ] {
        fs.create_file(path, "Object.assign({}, value);\nObject.keys(value);\n");
    }

    lint_project(
        fs,
        "package_preset_includes_in_local_extended_config_are_relative_to_consuming_config",
    );
}
