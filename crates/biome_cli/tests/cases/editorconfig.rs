use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn should_use_editorconfig() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = space
indent_size = 8
"#,
    );

    let test_file = Utf8Path::new("test.js");
    let contents = r#"function setName(name) {
 currentName = name;
}
"#;
    fs.insert(test_file.into(), contents);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--use-editorconfig=true",
                test_file.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_use_editorconfig",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_use_editorconfig() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = space
indent_size = 2
"#,
    );

    let test_file = Utf8Path::new("test.js");
    let contents = r#"function setName(name) {
 currentName = name;
}
"#;
    fs.insert(test_file.into(), contents);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--use-editorconfig=false",
                test_file.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_use_editorconfig",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_use_editorconfig_enabled_from_biome_conf() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = space
indent_size = 8
"#,
    );

    let biomeconfig = Utf8Path::new("biome.json");
    fs.insert(
        biomeconfig.into(),
        r#"{
    "formatter": {
        "useEditorconfig": true
    }
}
"#,
    );

    let test_file = Utf8Path::new("test.js");
    let contents = r#"function setName(name) {
 currentName = name;
}
"#;
    fs.insert(test_file.into(), contents);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_use_editorconfig_enabled_from_biome_conf",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_use_editorconfig_check() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = space
indent_size = 8
"#,
    );

    let test_file = Utf8Path::new("test.js");
    let contents = r#"function setName(name) {
 currentName = name;
}
"#;
    fs.insert(test_file.into(), contents);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--use-editorconfig=true", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_use_editorconfig_check",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_use_editorconfig_check_enabled_from_biome_conf() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = space
indent_size = 8
"#,
    );

    let biomeconfig = Utf8Path::new("biome.json");
    fs.insert(
        biomeconfig.into(),
        r#"{
    "formatter": {
        "useEditorconfig": true
    }
}
"#,
    );

    let test_file = Utf8Path::new("test.js");
    let contents = r#"function setName(name) {
 currentName = name;
}
"#;
    fs.insert(test_file.into(), contents);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_use_editorconfig_check_enabled_from_biome_conf",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_have_biome_override_editorconfig() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = tab
"#,
    );
    let biome_config_path = Utf8Path::new("biome.json");
    fs.insert(
        biome_config_path.into(),
        r#"
{
    "formatter": {
        "lineWidth": 90
    }
}
"#,
    );

    let test_file = Utf8Path::new("test.js");
    let contents = r#"console.log(
	"really long string that should break if the line width is <=90, but not at 100",
);
"#;
    fs.insert(test_file.into(), contents);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--use-editorconfig=true",
                test_file.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_have_biome_override_editorconfig",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_use_higher_editorconfig_when_find_biome_conf() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = space
indent_size = 2
"#,
    );

    let biome_config_path = Utf8Path::new("foo/biome.json");
    fs.insert(
        biome_config_path.into(),
        r#"
{
    "formatter": {
        "lineWidth": 60
    }
}
"#,
    );
    let test_file = Utf8Path::new("foo/test.js");
    fs.insert(
        test_file.into(),
        r#"
    if (foo) {
        console.log("test 123");
    }
    "#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", "--config-path=foo", test_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_use_higher_editorconfig_when_find_biome_conf",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_have_cli_override_editorconfig() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = space
indent_size = 8
"#,
    );

    let test_file = Utf8Path::new("test.js");
    fs.insert(
        test_file.into(),
        r#"function setName(name) {
 currentName = name;
}
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--use-editorconfig=true",
                "--indent-width=4",
                test_file.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_have_cli_override_editorconfig",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_apply_path_overrides() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = tab

[foo/**]
indent_style = space
"#,
    );

    let test_file = Utf8Path::new("tabs.js");
    fs.insert(
        test_file.into(),
        r#"
    if (foo) {
        console.log("this should be indented with tabs");
    }
    "#,
    );
    let test_file2 = Utf8Path::new("foo/spaces.js");
    fs.insert(
        test_file.into(),
        r#"
    if (foo) {
        console.log("this should be indented with spaces");
    }
    "#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--use-editorconfig=true",
                test_file.as_str(),
                test_file2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_apply_path_overrides",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_use_editorconfig_ci() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = space
indent_size = 8
"#,
    );

    let test_file = Utf8Path::new("test.js");
    let contents = r#"function setName(name) {
 currentName = name;
}
"#;
    fs.insert(test_file.into(), contents);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--use-editorconfig=true", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_use_editorconfig_ci",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_use_editorconfig_ci_enabled_from_biome_conf() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = space
indent_size = 8
"#,
    );

    let biomeconfig = Utf8Path::new("biome.json");
    fs.insert(
        biomeconfig.into(),
        r#"{
    "formatter": {
        "useEditorconfig": true
    }
}
"#,
    );

    let test_file = Utf8Path::new("test.js");
    let contents = r#"console.log("really long string that should cause a break if the line width remains at the default 80 characters");
"#;
    fs.insert(test_file.into(), contents);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", test_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_use_editorconfig_ci_enabled_from_biome_conf",
        fs,
        console,
        result,
    ));
}

#[test]
fn non_closed_section() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*
indent_style = space
indent_size = 8
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--use-editorconfig=true"].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "non_closed_section",
        fs,
        console,
        result,
    ));
}

#[test]
fn root_parse_error() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
root = on
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--use-editorconfig=true"].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "root_parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn end_of_line_parse_error() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
end_of_line = lfcr
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--use-editorconfig=true"].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "end_of_line_parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn indent_size_parse_error() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
  [*]
  indent_size = 1000
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--use-editorconfig=true"].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_size_parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn indent_size_can_set_to_tab() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Utf8Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
        [*]
        indent_size = tab
        "#,
    );

    let test_file = Utf8Path::new("test.js");
    let contents = r#"function setName(name) {
 currentName = name;
}
"#;
    fs.insert(test_file.into(), contents);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_size_can_set_to_tab",
        fs,
        console,
        result,
    ));
}
