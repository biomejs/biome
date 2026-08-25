use crate::run_cli_with_dyn_fs;
use crate::run_cli_with_server_workspace;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::{MemoryFileSystem, TemporaryFs};
use bpaf::Args;
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
