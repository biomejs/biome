use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
#[ignore = "enable once we have the configuration to turn it on"]
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
