use crate::TestArgs as Args;
use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

#[test]
fn prettier_migrate() {
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
fn prettier_migrate_default_options() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{}"#;
    let prettier = r#"{}"#;

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
        "prettier_migrate_default_options",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_preserves_markdown_frontmatter() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert("biome.json".into(), b"{}");
    fs.insert(".prettierrc".into(), b"{}");
    fs.insert(
        "SKILL.md".into(),
        include_bytes!("../specs/migrate_prettier/SKILL.md"),
    );

    let (fs, migration_result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier", "--write"].as_slice()),
    );
    assert!(
        migration_result.is_ok(),
        "run_cli returned {migration_result:?}"
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", "SKILL.md"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(
        &fs,
        Utf8Path::new("SKILL.md"),
        include_str!("../specs/migrate_prettier/SKILL.md.prettier-snap"),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_preserves_markdown_frontmatter",
        fs,
        console,
        migration_result.followed_by(result),
    ));
}

#[test]
fn prettier_migrate_with_ignore() {
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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

#[test]
fn prettier_migrate_override_with_bad_print_width() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "formatter": { "enabled": true } }"#;
    let prettier = r#"{
        "overrides": [{
            "files": ["**/*.test.js"],
            "options": { "printWidth": 666 }
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

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_override_with_bad_print_width",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_with_bad_top_level_print_width() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "formatter": { "enabled": true } }"#;
    let prettier = r#"{ "printWidth": 666 }"#;

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
        "prettier_migrate_with_bad_top_level_print_width",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_embedded_language_formatting() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "formatter": { "enabled": true } }"#;
    let prettier = r#"{
        "semi": true,
        "singleQuote": false,
        "embeddedLanguageFormatting": "off",
        "overrides": [{
            "files": ["docs/**/*.md"],
            "options": { "embeddedLanguageFormatting": "auto" }
        }]
    }"#;

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
        "prettier_migrate_embedded_language_formatting",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_override_bracket_spacing() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{}"#;
    let prettier = r#"{
        "overrides": [{ "files": ["*.js"], "options": { "bracketSpacing": false } }]
    }"#;

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
        "prettier_migrate_override_bracket_spacing",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_bracket_spacing() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{}"#;
    let prettier = r#"{ "bracketSpacing": false }"#;

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
        "prettier_migrate_bracket_spacing",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_write_formats_with_migrated_options() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "formatter": { "indentStyle": "tab" } }"#;
    let prettier = r#"{}"#;

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, migrate_result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier", "--write"].as_slice()),
    );

    assert!(
        migrate_result.is_ok(),
        "run_cli returned {migrate_result:?}"
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", configuration_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli rerun returned {result:?}");
    let result = migrate_result.followed_by(result);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_write_formats_with_migrated_options",
        fs,
        console,
        result,
    ));
}

#[test]
fn prettier_migrate_write_keeps_extended_formatter_options() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let base = r#"{ "formatter": { "indentStyle": "tab", "trailingNewline": false } }"#;
    let configuration = r#"{ "extends": ["./base.json"] }"#;
    let prettier = r#"{}"#;

    fs.insert(Utf8Path::new("base.json").into(), base.as_bytes());

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let prettier_path = Utf8Path::new(".prettierrc");
    fs.insert(prettier_path.into(), prettier.as_bytes());

    let (fs, migrate_result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "prettier", "--write"].as_slice()),
    );

    assert!(
        migrate_result.is_ok(),
        "run_cli returned {migrate_result:?}"
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", configuration_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli rerun returned {result:?}");
    let result = migrate_result.followed_by(result);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "prettier_migrate_write_keeps_extended_formatter_options",
        fs,
        console,
        result,
    ));
}
