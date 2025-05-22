use crate::run_cli_with_dyn_fs;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::TemporaryFs;
use bpaf::Args;
use camino::Utf8Path;

const ROOT: &str = r#"
{
    "root": true
}
"#;

const ROOT_TO_INHERIT: &str = r#"
{
    "root": true,
    "javascript": {
        "formatter": {
            "quoteStyle": "double"
        }
    }
}
"#;

const NESTED: &str = r#"
{
    "formatter": {
        "indentStyle": "space",
        "indentWidth": 8
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
    fs.create_file(file_path1.as_str(), ROOT);
    fs.create_file(
        "file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    let file_path2 = Utf8Path::new("packages/lib/biome.json");
    fs.create_file(file_path2.as_str(), NESTED);
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
fn when_run_from_child_biome_json_it_should_inherit_root_config() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("when_run_from_child_biome_json_it_should_inherit_root_config");

    let file_path1 = Utf8Path::new("biome.json");
    fs.create_file(file_path1.as_str(), ROOT_TO_INHERIT);
    fs.create_file(
        "file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    let file_path2 = Utf8Path::new("packages/lib/biome.json");
    fs.create_file(file_path2.as_str(), NESTED);
    fs.create_file(
        "packages/lib/file.js",
        "function f() { const lorem_and_ipsum = 'lorem ipsum'; }",
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(
            [
                "format",
                Utf8Path::new(fs.cli_path()).join("packages/lib").as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "when_run_from_child_biome_json_it_should_inherit_root_config",
        fs.create_mem(),
        console,
        result,
    ));
}
