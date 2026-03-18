use bpaf::Args;
use camino::Utf8Path;

use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;

use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};

/// Regression test for https://github.com/biomejs/biome/issues/9484
///
/// The JS formatter's syntax rewriter removes unnecessary parentheses, which
/// shifts text positions. Embedded node ranges must be mapped back through the
/// source map so they match the original-tree ranges stored in `FormatEmbedNode`.
#[test]
fn should_format_embedded_graphql_after_parenthesized_expression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file = Utf8Path::new("file.tsx");
    fs.insert(
        js_file.into(),
        r#"const a = {};
console.log((a));
const query = graphql`query Q { field }`;
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

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_format_embedded_graphql_after_parenthesized_expression",
        fs,
        console,
        result,
    ));
}

/// Regression test for https://github.com/biomejs/biome/issues/9484
///
/// Same root cause as the parenthesized expression test above, but exercises
/// JSX inside a function return where the syntax rewriter may remove
/// parentheses around the JSX tag expression.
#[test]
fn should_format_embedded_graphql_in_relay_component() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file = Utf8Path::new("file.tsx");
    fs.insert(
        js_file.into(),
        r#"import { graphql, useLazyLoadQuery } from 'react-relay';

const Table = () => {
  const query = useLazyLoadQuery(graphql`
    query Q {
      field
    }
  `, {});
  return <div></div>;
};
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

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_format_embedded_graphql_in_relay_component",
        fs,
        console,
        result,
    ));
}

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
