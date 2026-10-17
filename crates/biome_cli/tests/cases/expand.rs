use crate::TestArgs as Args;
use crate::snap_test::SnapshotPayload;
use crate::{assert_cli_snapshot, run_cli};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

const JAVASCRIPT: &str =
    "const single = { a: 1 }; const multiple = { a: 1, b: 2 }; const array = [1, 2];";
const JSON: &str = r#"{ "single": { "a": 1 }, "multiple": { "a": 1, "b": 2 }, "array": [1, 2] }"#;

#[test]
fn expand_when_multiple_from_global_configuration() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{ "formatter": { "expand": "whenMultiple" } }"#.as_bytes(),
    );
    fs.insert(Utf8Path::new("file.js").into(), JAVASCRIPT.as_bytes());
    fs.insert(Utf8Path::new("file.json").into(), JSON.as_bytes());
    fs.insert(Utf8Path::new("package.json").into(), JSON.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", "file.js", "file.json", "package.json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "expand_when_multiple_from_global_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn expand_when_multiple_from_cli() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(Utf8Path::new("file.js").into(), JAVASCRIPT.as_bytes());
    fs.insert(Utf8Path::new("file.json").into(), JSON.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--expand=when-multiple",
                "file.js",
                "file.json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "expand_when_multiple_from_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn expand_when_multiple_from_language_cli_overrides() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{ "formatter": { "expand": "never" } }"#.as_bytes(),
    );
    fs.insert(Utf8Path::new("file.js").into(), JAVASCRIPT.as_bytes());
    fs.insert(Utf8Path::new("file.json").into(), JSON.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--javascript-formatter-expand=when-multiple",
                "--json-formatter-expand=when-multiple",
                "file.js",
                "file.json",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "expand_when_multiple_from_language_cli_overrides",
        fs,
        console,
        result,
    ));
}

#[test]
fn expand_when_multiple_from_configuration_overrides() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
            "formatter": { "expand": "never" },
            "overrides": [{
                "includes": ["selected.js", "selected.json"],
                "formatter": { "expand": "whenMultiple" }
            }]
        }"#
        .as_bytes(),
    );
    fs.insert(Utf8Path::new("selected.js").into(), JAVASCRIPT.as_bytes());
    fs.insert(Utf8Path::new("selected.json").into(), JSON.as_bytes());
    fs.insert(Utf8Path::new("other.js").into(), JAVASCRIPT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "selected.js",
                "selected.json",
                "other.js",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "expand_when_multiple_from_configuration_overrides",
        fs,
        console,
        result,
    ));
}
