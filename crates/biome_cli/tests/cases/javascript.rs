use crate::TestArgs as Args;
use camino::Utf8Path;

use biome_console::BufferConsole;
use biome_fs::{MemoryFileSystem, TemporaryFs};

use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use crate::{run_cli, run_cli_with_dyn_fs};

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
fn javascript_comments_suppress_embedded_css_and_graphql_rules() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let file = Utf8Path::new("file.js");
    fs.insert(
        file.into(),
        r#"// biome-ignore lint/correctness/useGraphqlNamedOperations: expected anonymous query
const first = gql`query { people { name } }`;
const second = gql`query { people { name } }`;
// biome-ignore lint/correctness/noUnknownProperty: expected typo
const ignored = styled.div`colr: red;`;
const reported = styled.div`colr: red;`;"#
            .as_bytes(),
    );
    fs.insert(
        "biome.json".into(),
        r#"{
  "javascript": { "experimentalEmbeddedSnippetsEnabled": true },
  "linter": {
    "rules": {
      "recommended": false,
      "correctness": {
        "useGraphqlNamedOperations": "error",
        "noUnknownProperty": "error"
      }
    }
  }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file.as_str()].as_slice()),
    );
    assert!(result.is_err(), "{result:?}\n{console:#?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "javascript_comments_suppress_embedded_css_and_graphql_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn embedded_css_rules_check_top_level_declarations() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let file = Utf8Path::new("file.js");
    fs.insert(
        file.into(),
        r#"const Button = styled.div`
  --size: 1px;
  --size: 2px;
  color: red;
  color: blue;
  width: --size;
  &:hover {
    height: --size;
  }
`;"#
        .as_bytes(),
    );
    fs.insert(
        "biome.json".into(),
        r#"{
  "javascript": { "experimentalEmbeddedSnippetsEnabled": true },
  "linter": {
    "rules": {
      "preset": "none",
      "correctness": { "noMissingVarFunction": "error" },
      "suspicious": {
        "noDuplicateProperties": "error",
        "noDuplicateCustomProperties": "error"
      }
    }
  }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file.as_str()].as_slice()),
    );
    assert!(result.is_err(), "{result:?}\n{console:#?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "embedded_css_rules_check_top_level_declarations",
        fs,
        console,
        result,
    ));
}

#[test]
fn host_and_css_suppressions_are_both_used() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let file = Utf8Path::new("file.js");
    fs.insert(
        file.into(),
        r#"// biome-ignore-start lint/correctness/noUnknownProperty: temporary migration
const ignored = styled.div`
  /* biome-ignore lint/correctness/noUnknownProperty: generated declaration */
  colr: red;
  backgrond-color: red;
`;
// biome-ignore-end lint/correctness/noUnknownProperty: temporary migration"#
            .as_bytes(),
    );
    fs.insert(
        "biome.json".into(),
        r#"{
  "javascript": { "experimentalEmbeddedSnippetsEnabled": true },
  "linter": {
    "rules": {
      "recommended": false,
      "correctness": { "noUnknownProperty": "error" }
    }
  }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--error-on-warnings", file.as_str()].as_slice()),
    );
    assert!(result.is_ok(), "{result:?}\n{console:#?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "host_and_css_suppressions_are_both_used",
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

#[test]
fn lint_reads_jsx_factory_from_tsconfig() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("lint_reads_jsx_factory_from_tsconfig");
    fs.create_file(
        "biome.json",
        r#"{
    "javascript": {
        "jsxRuntime": "reactClassic"
    }
}"#,
    );
    fs.create_file(
        "tsconfig.json",
        r#"{
    "compilerOptions": {
        "jsx": "react",
        "jsxFactory": "h",
        "jsxFragmentFactory": "Fragment"
    }
}"#,
    );
    // `h` and `Fragment` are only used by the JSX, so they are reported as
    // unused unless the factories are read from `tsconfig.json`.
    fs.create_file(
        "file.jsx",
        r#"import { h, Fragment } from "preact";

export const App = () => (
    <>
        <div>Hello</div>
    </>
);
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=correctness/noUnusedImports", fs.cli_path()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_reads_jsx_factory_from_tsconfig",
        fs.create_mem(),
        console,
        result,
    ));
}
