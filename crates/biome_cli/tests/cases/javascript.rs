use bpaf::Args;
use camino::Utf8Path;

use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;

use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};

#[test]
fn should_pull_diagnostics_from_embedded_languages_when_formatting() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file = Utf8Path::new("file.js");
    fs.insert(
        js_file.into(),
        r#"const Foo = gql`query { people(id: ) { }`;
const Bar = styled.div`background-color: ; align: center; padding: 0;`;
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "javascript": {
        "experimentalEmbeddedSnippetsEnabled": true,
        "formatter": {
            "enabled": true
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", js_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_pull_diagnostics_from_embedded_languages_when_formatting",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_pull_diagnostics_from_embedded_languages_when_linting() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file = Utf8Path::new("file.js");
    fs.insert(
        js_file.into(),
        r#"const Foo = gql`query { people(id: $personId) { name } }`;
const Bar = styled.div`background-color: red !important; align: center; padding: 0;`;
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "javascript": {
        "experimentalEmbeddedSnippetsEnabled": true,
        "formatter": {
            "enabled": true
        },
        "linter": {
            "enabled": true
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", js_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_pull_diagnostics_from_embedded_languages_when_linting",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_apply_fixes_to_embedded_languages() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file = Utf8Path::new("file.js");
    fs.insert(
        js_file.into(),
        r#"const Foo = gql`query { people(id: $personId) { name } }`;
const Baz = styled.div`background-color: red !important; align: center; padding: 0;`;
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "javascript": {
        "experimentalEmbeddedSnippetsEnabled": true,
        "formatter": {
            "enabled": true
        },
        "linter": {
            "enabled": true
        },
        "assist": {
            "enabled": true
        }
    },
    "assist": {
        "actions": {
            "source": {
                "useSortedProperties": "on"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--unsafe", js_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_apply_fixes_to_embedded_languages",
        fs,
        console,
        result,
    ));
}
