use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, markup_to_string};
use biome_console::{BufferConsole, markup};
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const ASTRO_FILE_UNFORMATTED: &str = r#"---
import {    something } from "file.astro";

statement ( ) ;

---
<div></div>"#;

const ASTRO_FILE_DEBUGGER_BEFORE: &str = r#"---
debugger;
---
<div></div>"#;

const ASTRO_FILE_USELESS_RENAME_BEFORE: &str = r#"---
import {a as a} from 'mod';
export { a };
---
<div></div>"#;

const ASTRO_FILE_IMPORTS_BEFORE: &str = r#"---
import { getLocale } from "astro:i18n";
import { Code } from "astro:components";
---
<div></div>"#;

const ASTRO_RETURN: &str = r#"---
const foo = true;
if (foo) {
    return "Something";
}

---
<div></div>"#;

const ASTRO_CARRIAGE_RETURN_LINE_FEED_FILE_UNFORMATTED: &str =
    "---\r\n  const a    = \"b\";\r\n---\r\n<div></div>";

const ASTRO_RETURN_IN_TEMPLATE: &str = r#"---
const x = 5;
---
<div>{ return x }</div>"#;

const ASTRO_FILE_CHECK_BEFORE: &str = r#"---
import {a as a} from 'mod';
import {    something } from "file.astro";
debugger;
something ( ) ;
var foo: string = "";
---
<div></div>"#;

const ASTRO_FILE_ASTRO_GLOBAL_OBJECT: &str = r#"---
const { some } = Astro.props
---
<div>{some}</div>"#;

const ASTRO_FILE_WITH_TS_SCRIPT_TAG: &str = r#"---
const title = "My Page";
---
<html>
<body>
    <script>
        const message:     string = "Hello TypeScript";
        function greet(name:   string): void {
            console.log(  message + ", " + name );
        }
    </script>
</body>
</html>"#;

#[test]
fn format_astro_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_astro_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_astro_files_write() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_astro_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_empty_astro_files_write() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), "<div></div>".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_empty_astro_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_astro_carriage_return_line_feed_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_CARRIAGE_RETURN_LINE_FEED_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_astro_carriage_return_line_feed_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_astro_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_FILE_DEBUGGER_BEFORE.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_astro_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn full_support() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        r#"---
import z from "zod";
import { sure } from "sure.js";
import s from "src/utils";

// Always considered as used
interface Props {
  name: string;
}

type Props = {
  name: string;
};

// Still reported as unused
interface Foo {
  name: string;
}

type Bar = {
  name: string;
};

function doSomething() {
  // Still reported as unused, Props interface must be at top-level
  interface Props {
    name: string;
  }

  type Props = {
    name: string;
  };
}

const { name } = Astro.props;

let schema = z.object().optional();
schema + sure()
---




<html><head><title>Astro</title></head><body></body></html>

<style>
#id { font-family: comic-sans } .class { background: red}
:global(div) {
  color: red;
}
</style>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--unsafe", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "full_support",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_and_fix_astro_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_FILE_DEBUGGER_BEFORE.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_and_fix_astro_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn sorts_imports_check() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_IMPORTS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                astro_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "sorts_imports_check",
        fs,
        console,
        result,
    ));
}

#[test]
fn sorts_imports_write() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_FILE_IMPORTS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                "--write",
                astro_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "sorts_imports_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_throw_parse_error_for_return() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_RETURN.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_throw_parse_error_for_return",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_stdin_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(ASTRO_FILE_UNFORMATTED.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_stdin_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_stdin_write_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(ASTRO_FILE_UNFORMATTED.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_stdin_write_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_stdin_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push(ASTRO_FILE_USELESS_RENAME_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, ASTRO_FILE_USELESS_RENAME_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_stdin_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_stdin_write_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push(ASTRO_FILE_USELESS_RENAME_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_stdin_write_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_stdin_write_unsafe_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push(ASTRO_FILE_DEBUGGER_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--write",
                "--unsafe",
                "--stdin-file-path",
                "file.astro",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_stdin_write_unsafe_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_stdin_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(ASTRO_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, ASTRO_FILE_CHECK_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_stdin_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_stdin_write_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(ASTRO_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--stdin-file-path", "file.astro"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_stdin_write_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_stdin_write_unsafe_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(ASTRO_FILE_CHECK_BEFORE.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--write",
                "--unsafe",
                "--stdin-file-path",
                "file.astro",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_stdin_write_unsafe_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn astro_global_object() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_FILE_ASTRO_GLOBAL_OBJECT.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "astro_global",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_astro_with_typescript_script_tag() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        ASTRO_FILE_WITH_TS_SCRIPT_TAG.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_astro_with_typescript_script_tag",
        fs,
        console,
        result,
    ));
}

#[test]
fn dont_indent_frontmatter() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "formatter": {"enabled": true, "indentScriptAndStyle": true}, "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        r#"---
import Foo from "./Foo.astro"
const bar = 123
if (bar>1) {console.log(bar+1)}
---
<Foo>{bar}</Foo>

<style>
#id { font-family: comic-sans } .class { background: red}
</style>

<script>
function foo(){console.log("Hello")}
</script>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "dont_indent_frontmatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_throw_parse_error_for_return_full_support() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "experimentalFullSupportEnabled": true } }"#.as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_RETURN.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", astro_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_throw_parse_error_for_return_full_support",
        fs,
        console,
        result,
    ));
}

#[test]
fn embedded_bindings_are_tracked_correctly() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.astro");
    fs.insert(
        file.into(),
        r#"---
import { Component } from "./component.svelte";
let hello = "Hello World";
let array = [];
let props = [];
---

<html>
    <span>{hello}</span>
    <span>{notDefined}</span>
    { array.map(item => (<span>{item}</span>)) }
    <Component />
    <input {...props}>
</html>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUnusedVariables", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "embedded_bindings_are_tracked_correctly",
        fs,
        console,
        result,
    ));
}

#[test]
fn use_const_not_triggered_in_snippet_sources() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.astro");
    fs.insert(
        file.into(),
        r#"---
let hello = "Hello World";
---

<html>
    <span>{hello}</span>
</html>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=useConst", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "use_const_not_triggered_in_snippet_sources",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_unused_imports_is_not_triggered_in_snippet_sources() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.astro");
    fs.insert(
        file.into(),
        r#"---
import Component from "./Component.vue"
---

<Component />
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUnusedImports", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_imports_is_not_triggered_in_snippet_sources",
        fs,
        console,
        result,
    ));
}

#[test]
fn issue_7912() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "experimentalFullSupportEnabled": true, "formatter": { "enabled": true } } }"#.as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(
        astro_file_path.into(),
        r#"---
            const title = "Hello World";
---

<html>
  <head>
            <title>{title}</title>
  </head>
  <body>
            <h1>{title}</h1>
  </body>
</html>"#
            .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--write",
                "--only=suspicious/noDebugger",
                astro_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_7912",
        fs,
        console,
        result,
    ));
}

const ASTRO_ENUM_IN_TEMPLATE: &str = r#"---
import { Component, FooEnum } from './types';
---
<main>
  <Component />
  {FooEnum.Foo}
</main>"#;

#[test]
fn use_import_type_not_triggered_for_enum_in_template() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file = Utf8Path::new("file.astro");
    fs.insert(file.into(), ASTRO_ENUM_IN_TEMPLATE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=useImportType", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "use_import_type_not_triggered_for_enum_in_template",
        fs,
        console,
        result,
    ));
}

#[test]
fn use_import_type_not_triggered_for_enum_in_template_v2() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.astro");
    fs.insert(
        file.into(),
        r#"---
import { Avatar as AvatarPrimitive } from "bits-ui";
import { cn } from "$lib/utils.js";

let {
	ref = $bindable(null),
	class: className,
}: AvatarPrimitive.FallbackProps = $props();
---

<!-- used as value here -->
<AvatarPrimitive.Fallback
	class="something nice"
/>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=useImportType", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "use_import_type_not_triggered_for_enum_in_template_v2",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_useless_lone_block_statements_is_not_triggered() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let file = Utf8Path::new("file.astro");
    fs.insert(
        file.into(),
        r#"---
{
	const x = 1;
}
---

<div>{x}</div>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUselessLoneBlockStatements", file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_useless_lone_block_statements_is_not_triggered",
        fs,
        console,
        result,
    ));
}

#[test]
fn return_in_template_expression_should_error() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#
            .as_bytes(),
    );

    let astro_file_path = Utf8Path::new("file.astro");
    fs.insert(astro_file_path.into(), ASTRO_RETURN_IN_TEMPLATE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", astro_file_path.as_str()].as_slice()),
    );

    // The result should have errors because return is not allowed in template expressions
    // We expect the check to fail with errors
    assert!(
        result.is_err(),
        "Expected errors but run_cli returned {result:?}"
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "return_in_template_expression_should_error",
        fs,
        console,
        result,
    ));
}
