use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const CONFIG: &str = r#"{ "html": { "formatter": { "enabled": true } } }"#;

/// The value of a `style` attribute is CSS, so it is formatted as a list of
/// declarations. It stays on the tag's line while it fits, and breaks onto its
/// own lines once it does not.
#[test]
fn formats_style_attribute_as_css() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(config.into(), CONFIG.as_bytes());

    let file = Utf8Path::new("index.html");
    fs.insert(
        file.into(),
        r#"<div style="color:#fFf;  background:red"></div>
<div style="all: initial;display:block;contain:content;text-align:center;max-width:500px;margin:0 auto"></div>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formats_style_attribute_as_css",
        fs,
        console,
        result,
    ));
}

/// A value that is not a list of declarations is left exactly as written: a
/// prop that happens to be called `style`, an interpolation, or nothing at all.
#[test]
fn leaves_non_css_style_attributes_alone() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(config.into(), CONFIG.as_bytes());

    let file = Utf8Path::new("index.html");
    fs.insert(
        file.into(),
        r#"<div style="{{ dynamic }}"></div>
<div style="primary"></div>
<div style=""></div>
<div style="   "></div>
<div style></div>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "leaves_non_css_style_attributes_alone",
        fs,
        console,
        result,
    ));
}
