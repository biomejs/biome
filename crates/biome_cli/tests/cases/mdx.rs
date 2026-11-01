use crate::TestArgs as Args;
use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

const UNFORMATTED: &str = r#"import {Chart} from './chart.js'
export const meta = {title: 'Hello'}

#   Hello, {props.name}

<Note type='tip'>

Some   *text*   with a <Badge   color='green'>badge</Badge>.

</Note>

<Chart data={[1,2,3]}   title='x' />

{ 1+1 }
"#;

const FORMATTED: &str = r#"import { Chart } from "./chart.js";
export const meta = { title: "Hello" };

# Hello, {props.name}

<Note type="tip">

Some _text_ with a <Badge color="green">badge</Badge>.

</Note>

<Chart data={[1, 2, 3]} title="x" />

{1 + 1}
"#;

#[test]
fn format_mdx_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.mdx");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_mdx_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_mdx_files_with_javascript_syntax_errors() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.mdx");
    let content = "#   Hello\n\n{1 +}\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_mdx_files_with_javascript_syntax_errors",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_mdx_files_with_mdx_syntax_errors() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.mdx");
    let content = "<Note>\n\n<!-- comment -->\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_mdx_files_with_mdx_syntax_errors",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_mdx_files_with_javascript_syntax_errors() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.mdx");
    fs.insert(file_path.into(), "# Hello\n\n{1 +}\n".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_mdx_files_with_javascript_syntax_errors",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_mdx_files_with_embedded_formatting_disabled() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{ "markdown": { "formatter": { "formatEmbeds": false } } }"#.as_bytes(),
    );
    let file_path = Utf8Path::new("file.mdx");
    let content = "#   Hello\n\n{1 +}\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_mdx_files_with_embedded_formatting_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_inline_and_contained_mdx_constructs() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.mdx");
    let content = r#"See <Link href="https://example.com/docs/getting-started" target="_blank" rel="noopener">the docs</Link>.

- An item with {  value  } and <Badge   color='green' />

> A quote with <Badge   color='green' />
"#;
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(
        &fs,
        file_path,
        r#"See <Link href="https://example.com/docs/getting-started" target="_blank" rel="noopener">the docs</Link>.

- An item with {value} and <Badge color="green" />

> A quote with <Badge color="green" />
"#,
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_inline_and_contained_mdx_constructs",
        fs,
        console,
        result,
    ));
}

const LINT_CONFIG: &str = r#"{
    "markdown": {
        "analyzeEmbeds": true
    },
    "linter": {
        "rules": {
            "correctness": {
                "noUndeclaredVariables": "error",
                "noUnusedImports": "error"
            },
            "nursery": {
                "useTopLevelHeading": "error"
            }
        }
    }
}"#;

#[test]
fn lint_mdx_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(Utf8Path::new("biome.json").into(), LINT_CONFIG.as_bytes());
    let file_path = Utf8Path::new("file.mdx");
    let content = r#"import {Chart} from './chart.js'
import {Unused} from './unused.js'

# Hello, {props.name}

<Callout>

<Chart data={values} />

</Callout>
"#;
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_mdx_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_mdx_files_with_suppression_comments() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(Utf8Path::new("biome.json").into(), LINT_CONFIG.as_bytes());
    let file_path = Utf8Path::new("file.mdx");
    let content = r#"{/* biome-ignore lint/correctness/noUnusedImports: the example shows an import */}
import {Unused} from './unused.js'

# Hello
"#;
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_mdx_files_with_suppression_comments",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_mdx_files_bindings_across_constructs() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(Utf8Path::new("biome.json").into(), LINT_CONFIG.as_bytes());
    let file_path = Utf8Path::new("file.mdx");
    let content = r#"import {Chart} from './chart.js'
import {format} from './format.js'

# Hello

```jsx
<Chart />
```

> {format(
>   1
> )}
"#;
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_mdx_files_bindings_across_constructs",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_mdx_files_with_top_level_suppression_comments() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(Utf8Path::new("biome.json").into(), LINT_CONFIG.as_bytes());
    let file_path = Utf8Path::new("file.mdx");
    let content = r#"{/* biome-ignore-all lint/correctness/noUnusedImports: generated imports */}
import {Unused} from './unused.js'

# Hello
"#;
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_mdx_files_with_top_level_suppression_comments",
        fs,
        console,
        result,
    ));
}
