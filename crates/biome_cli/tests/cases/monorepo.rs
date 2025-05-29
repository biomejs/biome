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
