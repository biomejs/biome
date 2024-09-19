use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn should_use_editorconfig() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
max_line_length = 300
"#,
    );

    let test_file = Path::new("test.js");
    let contents = r#"console.log("really long string that should cause a break if the line width remains at the default 80 characters");
"#;
    fs.insert(test_file.into(), contents);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                ("--use-editorconfig=true"),
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test_file, contents);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_use_editorconfig",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_use_editorconfig_enabled_from_biome_conf() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
max_line_length = 300
"#,
    );

    let biomeconfig = Path::new("biome.json");
    fs.insert(
        biomeconfig.into(),
        r#"{
    "formatter": {
        "useEditorconfig": true
    }
}
"#,
    );

    let test_file = Path::new("test.js");
    let contents = r#"console.log("really long string that should cause a break if the line width remains at the default 80 characters");
"#;
    fs.insert(test_file.into(), contents);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test_file, contents);
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
max_line_length = 300
"#,
    );

    let test_file = Path::new("test.js");
    let contents = r#"console.log("really long string that should cause a break if the line width remains at the default 80 characters");
"#;
    fs.insert(test_file.into(), contents);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                ("--use-editorconfig=true"),
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test_file, contents);
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
max_line_length = 300
"#,
    );

    let biomeconfig = Path::new("biome.json");
    fs.insert(
        biomeconfig.into(),
        r#"{
    "formatter": {
        "useEditorconfig": true
    }
}
"#,
    );

    let test_file = Path::new("test.js");
    let contents = r#"console.log("really long string that should cause a break if the line width remains at the default 80 characters");
"#;
    fs.insert(test_file.into(), contents);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("check"), test_file.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test_file, contents);
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
max_line_length = 100
indent_style = tab
"#,
    );
    let biomeconfig = Path::new("biome.json");
    fs.insert(
        biomeconfig.into(),
        r#"
{
    "formatter": {
        "lineWidth": 90
    }
}
"#,
    );

    let test_file = Path::new("test.js");
    let contents = r#"console.log(
	"really long string that should break if the line width is <=90, but not at 100",
);
"#;
    fs.insert(test_file.into(), contents);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                ("--use-editorconfig=true"),
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test_file, contents);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_have_biome_override_editorconfig",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_have_cli_override_editorconfig() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
max_line_length = 90
"#,
    );

    let test_file = Path::new("test.js");
    fs.insert(test_file.into(), r#"console.log("really long string that should break if the line width is <=90, but not at 100");
"#);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                ("--line-width=100"),
                ("--use-editorconfig=true"),
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
indent_style = tab

[foo/**]
indent_style = space
"#,
    );

    let test_file = Path::new("tabs.js");
    fs.insert(
        test_file.into(),
        r#"
    if (foo) {
        console.log("this should be indented with tabs");
    }
    "#,
    );
    let test_file2 = Path::new("foo/spaces.js");
    fs.insert(
        test_file.into(),
        r#"
    if (foo) {
        console.log("this should be indented with spaces");
    }
    "#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("check"),
                ("--use-editorconfig=true"),
                test_file.as_os_str().to_str().unwrap(),
                test_file2.as_os_str().to_str().unwrap(),
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
max_line_length = 300
"#,
    );

    let test_file = Path::new("test.js");
    let contents = r#"console.log("really long string that should cause a break if the line width remains at the default 80 characters");
"#;
    fs.insert(test_file.into(), contents);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("ci"),
                ("--use-editorconfig=true"),
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test_file, contents);
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
max_line_length = 300
"#,
    );

    let biomeconfig = Path::new("biome.json");
    fs.insert(
        biomeconfig.into(),
        r#"{
    "formatter": {
        "useEditorconfig": true
    }
}
"#,
    );

    let test_file = Path::new("test.js");
    let contents = r#"console.log("really long string that should cause a break if the line width remains at the default 80 characters");
"#;
    fs.insert(test_file.into(), contents);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("ci"), test_file.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test_file, contents);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_use_editorconfig_ci_enabled_from_biome_conf",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_emit_diagnostics() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let editorconfig = Path::new(".editorconfig");
    fs.insert(
        editorconfig.into(),
        r#"
[*]
insert_final_newline = false
"#,
    );

    let test_file = Path::new("test.js");
    let contents = r#"console.log("foo");
"#;
    fs.insert(test_file.into(), contents);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                ("--use-editorconfig=true"),
                test_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test_file, contents);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_emit_diagnostics",
        fs,
        console,
        result,
    ));
}
