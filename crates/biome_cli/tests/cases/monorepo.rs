use crate::run_cli_with_dyn_fs;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::TemporaryFs;
use bpaf::Args;
use camino::Utf8Path;

const ROOT: &str = r#"
{
    "javascript": {
        "formatter": {
            "quoteStyle": "double"
        }
    }
}
"#;

#[test]
fn should_fail_for_nested_roots() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("should_fail_for_nested_roots");

    let file_path1 = Utf8Path::new("biome.json");
    fs.create_file(file_path1.as_str(), ROOT);

    let file_path2 = Utf8Path::new("packages/lib/biome.json");
    fs.create_file(file_path2.as_str(), ROOT);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_fail_for_nested_roots",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_format_nested_files_differently() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("should_format_nested_files_differently");

    let file_path1 = Utf8Path::new("biome.json");
    fs.create_file(
        file_path1.as_str(),
        r#"
{
    "javascript": {
        "formatter": {
            "quoteStyle": "double"
        }
    }
}
"#,
    );
    fs.create_file(
        "file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    let file_path2 = Utf8Path::new("packages/lib/biome.json");
    fs.create_file(
        file_path2.as_str(),
        r#"
{
    "root": false,
    "formatter": {
        "indentStyle": "space",
        "indentWidth": 8
    },
    "javascript": {
        "formatter": {
            "quoteStyle": "single"
        }
  }
}
"#,
    );
    fs.create_file(
        "packages/lib/file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_format_nested_files_differently",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_extend_from_the_root_config() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("should_extend_from_the_root_config");

    let file_path1 = Utf8Path::new("biome.json");
    fs.create_file(
        file_path1.as_str(),
        r#"
{
    "root": true,
    "formatter": {
        "indentStyle": "space",
        "indentWidth": 2
    },
    "javascript": {
        "formatter": {
            "quoteStyle": "single"
        }
    }
}
"#,
    );
    fs.create_file(
        "file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    let file_path2 = Utf8Path::new("packages/lib/biome.json");
    fs.create_file(
        file_path2.as_str(),
        r#"
{
    "root": false,
    "extends": "//",
    "formatter": {
        "indentStyle": "space",
        "indentWidth": 8
    },
    "javascript": {
        "formatter": {
            "quoteStyle": "double"
        }
    }
}
"#,
    );
    fs.create_file(
        "packages/lib/file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_extend_from_the_root_config",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn when_running_from_a_subdirectory_should_extend_from_the_root_config() {
    let mut console = BufferConsole::default();
    let mut fs =
        TemporaryFs::new("when_running_from_a_subdirectory_should_extend_from_the_root_config");

    let file_path1 = Utf8Path::new("biome.json");
    fs.create_file(
        file_path1.as_str(),
        r#"
{
    "formatter": {
        "indentStyle": "space",
        "indentWidth": 2
    }
}
"#,
    );
    fs.create_file(
        "file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    let file_path2 = Utf8Path::new("packages/lib/biome.json");
    fs.create_file(
        file_path2.as_str(),
        r#"
{
    "root": false,
    "extends": "//",
    "javascript": {
        "formatter": {
            "quoteStyle": "double"
        }
    }
}
"#,
    );
    fs.create_file(
        "packages/lib/file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    // after we create the files, we change the workspace directory so
    // we simulate the run of biome from a nested package
    fs.append_to_working_directory("packages/lib");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "when_running_from_a_subdirectory_should_extend_from_the_root_config",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_error_when_no_root_config_is_found() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("should_error_when_no_root_config_is_found");

    let file_path1 = Utf8Path::new("biome.json");
    fs.create_file(
        file_path1.as_str(),
        r#"
{
    "root": false,
    "formatter": {
        "indentStyle": "space",
        "indentWidth": 2
    }
}
"#,
    );
    fs.create_file(
        "file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    let file_path2 = Utf8Path::new("packages/lib/biome.json");
    fs.create_file(
        file_path2.as_str(),
        r#"
{
    "extends": "//",
    "javascript": {
        "formatter": {
            "quoteStyle": "double"
        }
    }
}
"#,
    );
    fs.create_file(
        "packages/lib/file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    // after we create the files, we change the workspace directory so
    // we simulate the run of biome from a nested package
    fs.append_to_working_directory("packages/lib");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_error_when_no_root_config_is_found",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_ignore_files_in_nested_projects() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("should_ignore_files_in_nested_projects");

    fs.create_file(
        "biome.json",
        r#"
{
    "root": true,
    "vcs": {
        "enabled": true,
        "clientKind": "git",
        "useIgnoreFile": true
    }
}
"#,
    );
    fs.create_file(".gitignore", ".next");
    fs.create_file(".next/file.json", "[\n\n\n\n]");
    fs.create_file(
        "file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    fs.create_file("packages/lib/biome.json", r#"{ "extends": "//" }"#);

    fs.create_file("packages/lib/.gitignore", ".next");
    fs.create_file("packages/lib/.next/file.json", "[\n\n\n\n]");
    fs.create_file(
        "packages/lib/file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["check", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_ignore_files_in_nested_projects",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_not_lint_when_root_is_disabled_but_nested_is_enabled() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("should_not_lint_when_root_is_disabled_but_nested_is_enabled");

    fs.create_file("biome.json", r#"{ "linter": {"enabled": false } }"#);

    fs.create_file(
        "packages/lib/biome.json",
        r#"{ "extends": "//", "linter": {"enabled": true } }"#,
    );

    fs.create_file("file.js", "debugger");

    fs.create_file("packages/lib/file.js", "debugger");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_lint_when_root_is_disabled_but_nested_is_enabled",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_find_settings_when_run_from_nested_dir() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("should_find_settings_when_run_from_nested_dir");

    fs.create_file(
        "biome.jsonc",
        r#"{
    "linter": {
        "rules": {
            "correctness": { "noUnusedVariables": "off" },
            "suspicious": { "noDebugger": "off" }
        }
    }
}"#,
    );

    fs.create_file(
        "packages/lib/biome.jsonc",
        r#"{
    "extends": "//",
    "linter": {
        "rules": {
            "correctness": { "noUnusedVariables": "error" }
        }
    }
}"#,
    );

    fs.create_file("file.js", "let a; debugger");

    fs.create_file("packages/lib/file.js", "let a; debugger");

    fs.append_to_working_directory("packages/lib");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_find_settings_when_run_from_nested_dir",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_find_settings_when_targeting_file_in_nested_dir() {
    let mut fs = TemporaryFs::new("should_find_settings_when_targeting_file_in_nested_dir");

    fs.create_file(
        "biome.jsonc",
        r#"{
    "linter": {
        "includes": ["**/*.js"],
        "rules": {
            "correctness": { "noUnusedVariables": "off" },
            "suspicious": { "noDebugger": "off" }
        }
    }
}"#,
    );

    fs.create_file(
        "packages/lib/biome.jsonc",
        r#"{
    "extends": "//",
    "linter": {
        "rules": {
            "correctness": { "noUnusedVariables": "error" }
        }
    }
}"#,
    );

    fs.create_file("file.js", "let a; debugger");

    fs.create_file("packages/lib/file.js", "let a; debugger");

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/packages/lib/file.js", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_find_settings_when_targeting_file_in_nested_dir",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_find_settings_when_targeting_nested_dir() {
    let mut fs = TemporaryFs::new("should_find_settings_when_targeting_nested_dir");

    fs.create_file(
        "biome.jsonc",
        r#"{
    "linter": {
        "includes": ["**/*.js"],
        "rules": {
            "correctness": { "noUnusedVariables": "off" },
            "suspicious": { "noDebugger": "off" }
        }
    }
}"#,
    );

    fs.create_file(
        "packages/lib/biome.jsonc",
        r#"{
    "extends": "//",
    "linter": {
        "rules": {
            "correctness": { "noUnusedVariables": "error" }
        }
    }
}"#,
    );

    fs.create_file("file.js", "let a; debugger");

    fs.create_file("packages/lib/file.js", "let a; debugger");

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/packages/lib", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_find_settings_when_targeting_nested_dir",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_find_settings_when_targeting_parent_of_nested_dir() {
    let mut fs = TemporaryFs::new("should_find_settings_when_targeting_parent_of_nested_dir");

    fs.create_file(
        "biome.jsonc",
        r#"{
    "linter": {
        "includes": ["**/*.js"],
        "rules": {
            "correctness": { "noUnusedVariables": "off" },
            "suspicious": { "noDebugger": "off" }
        }
    }
}"#,
    );

    fs.create_file(
        "packages/lib/biome.jsonc",
        r#"{
    "extends": "//",
    "linter": {
        "rules": {
            "correctness": { "noUnusedVariables": "error" }
        }
    }
}"#,
    );

    fs.create_file("file.js", "let a; debugger");

    fs.create_file("packages/lib/file.js", "let a; debugger");

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", &format!("{}/packages", fs.cli_path())].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_find_settings_when_targeting_parent_of_nested_dir",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_ignore_nested_configuration_in_ignored_directory() {
    let mut fs = TemporaryFs::new("should_ignore_nested_configuration_in_ignored_directory");

    fs.create_file(
        "biome.jsonc",
        r#"{
    "files": {
        "includes": ["**/*.js", "!vendor/**"],
    },
    "linter": {
        "rules": {
            "correctness": { "noUnusedVariables": "off" },
            "suspicious": { "noDebugger": "off" }
        }
    }
}"#,
    );

    fs.create_file(
        "vendor/biome.jsonc",
        r#"{
    "root": true,
    "linter": {
        "rules": {
            "correctness": { "noUnusedVariables": "error" }
        }
    }
}"#,
    );

    fs.create_file("file.js", "let a; debugger");

    fs.create_file("vendor/foo/file.js", "let a; debugger");

    let mut console = BufferConsole::default();
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint"].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_ignore_nested_configuration_in_ignored_directory",
        fs.create_mem(),
        console,
        result,
    ));
}
