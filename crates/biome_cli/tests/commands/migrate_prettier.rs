use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn prettier_migrate() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"{ "useTabs": false, "semi": true, "singleQuote": true }"#;

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier"].as_slice()),
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
fn prettier_migrate_end_of_line() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{}"#;
    let prettier = r#"{ "endOfLine": "auto" }"#;

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_end_of_line",
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

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let prettier_ignore_path = Utf8Path::new(".prettierignore");
    fs.insert(prettier_ignore_path.into(), prettier_ignore.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier"].as_slice()),
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
fn prettier_migrate_jsonc() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"{ "useTabs": false, "semi": true, "singleQuote": true }"#;

    let configuration_path = Utf8Path::new("biome.jsonc");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_jsonc",
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

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier"].as_slice()),
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

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier"].as_slice()),
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

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier", "--write"].as_slice()),
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
fn prettier_migrate_fix() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"{ "useTabs": false, "semi": true, "singleQuote": true }"#;

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier", "--fix"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_fix",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettierjson_migrate_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"{ "useTabs": false, "semi": true, "singleQuote": true }"#;

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc.json");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettierjson_migrate_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_write_packagejson() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"{
        "name": "Foo",
        "version": "0.0.0",
        "prettier": { "useTabs": false, "semi": true, "singleQuote": true }
    }"#;

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new("package.json");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_write_packagejson",
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

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let prettier_ignore_path = Utf8Path::new(".prettierignore");
    fs.insert(prettier_ignore_path.into(), prettier_ignore.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier", "--write"].as_slice()),
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

#[test]
fn prettier_migrate_write_biome_jsonc() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let prettier = r#"{ "useTabs": false, "semi": true, "singleQuote": true }"#;

    let configuration_path = Utf8Path::new("biome.jsonc");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_write_biome_jsonc",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_overrides() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "formatter": { "enabled": true } }"#;
    let prettier = r#"{
        "overrides": [{
            "files": ["**/*.test.js"],
            "options": { "useTabs": false }
        }, {
            "files": ["**/*.spec.js"],
            "options": { "semi": true, "singleQuote": true }
        }, {
            "files": ["**/*.ts"],
            "options": { "useTabs": false, "semi": true, "singleQuote": true }
        }]
    }"#;

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_overrides",
        fs,
        console,
        result,
    ));
}
