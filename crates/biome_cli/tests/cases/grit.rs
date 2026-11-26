use crate::TestArgs as Args;
use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

const GRIT_PATTERN: &str = r#"language js

`foo($x)` where { $x <: `1`, register_diagnostic(span=$x, message="mm") }
"#;

#[test]
fn grit_line_width_takes_precedence_over_global_line_width() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
    "formatter": { "lineWidth": 120 },
    "grit": { "formatter": { "lineWidth": 40 } }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.grit");
    fs.insert(file_path.into(), GRIT_PATTERN.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "grit_line_width_takes_precedence_over_global_line_width",
        fs,
        console,
        result,
    ));
}
