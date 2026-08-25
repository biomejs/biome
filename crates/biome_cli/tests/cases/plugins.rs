use crate::run_cli_with_dyn_fs;
use crate::run_cli_with_server_workspace;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::{MemoryFileSystem, TemporaryFs};
use bpaf::Args;

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
