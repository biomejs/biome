use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn migrate_help() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "--help"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_help",
        fs,
        console,
        result,
    ));
}

#[test]
fn migrate_config_up_to_date() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, configuration_path, configuration);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "migrate_config_up_to_date",
        fs,
        console,
        result,
    ));
}

#[test]
fn missing_configuration_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate")].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "missing_configuration_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn emit_diagnostic_for_rome_json() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;

    let configuration_path = Path::new("rome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "emit_diagnostic_for_rome_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_create_biome_json_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;

    let configuration_path = Path::new("rome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_create_biome_json_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"{ "useTabs": false, "semi": true, "singleQuote": true }"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "prettier"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_with_ignore() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"{ "useTabs": false, "semi": true, "singleQuote": true }"#;
    let prettier_ignore = r#"
dist/**

node_modules/**

# I am a comment
generated/*.spec.js
"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let prettier_ignore_path = Path::new(".prettierignore");
    fs.insert(prettier_ignore_path.into(), prettier_ignore.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "prettier"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_with_ignore",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_no_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "prettier"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_no_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_yml_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"useTabs: true"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "prettier"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_yml_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"{ "useTabs": false, "semi": true, "singleQuote": true }"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "prettier", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_write_with_ignore_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"{ "useTabs": false, "semi": true, "singleQuote": true }"#;
    let prettier_ignore = r#"
dist/**

node_modules/**

# I am a comment
generated/*.spec.js
"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let prettier_ignore_path = Path::new(".prettierignore");
    fs.insert(prettier_ignore_path.into(), prettier_ignore.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "prettier", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_write_with_ignore_file",
        fs,
        console,
        result,
    ));
}
