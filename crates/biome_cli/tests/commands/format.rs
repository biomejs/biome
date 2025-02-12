use crate::configs::{
    CONFIG_DISABLED_FORMATTER, CONFIG_FILE_SIZE_LIMIT, CONFIG_FORMAT, CONFIG_FORMAT_JSONC,
    CONFIG_ISSUE_3175_1, CONFIG_ISSUE_3175_2,
};
use crate::snap_test::{assert_file_contents, markup_to_string, SnapshotPayload};
use crate::{
    assert_cli_snapshot, run_cli, CUSTOM_FORMAT_BEFORE, FORMATTED, LINT_ERROR, UNFORMATTED,
};
use biome_console::{markup, BufferConsole, MarkupBuf};
use biome_fs::{FileSystemExt, MemoryFileSystem};
use bpaf::Args;
use camino::{Utf8Path, Utf8PathBuf};

// six spaces
const CUSTOM_FORMAT_AFTER: &str = r#"function f() {
      return { something };
}
"#;

const APPLY_JSX_QUOTE_STYLE_BEFORE: &str = r#"
<div
  bar="foo"
  baz={"foo"}
/>"#;

const APPLY_JSX_QUOTE_STYLE_AFTER: &str = r#"<div bar='foo' baz={"foo"} />;
"#;

const APPLY_QUOTE_STYLE_BEFORE: &str = r#"
let a = "something";
let b = {
    "hey": "hello"
};"#;

const APPLY_QUOTE_STYLE_AFTER: &str = "let a = 'something';
let b = {\n\t'hey': 'hello',\n};\n";

const APPLY_CSS_QUOTE_STYLE_BEFORE: &str =
    r#"[class='foo'] { background-image: url("/path/to/file.jpg")}"#;

const APPLY_CSS_QUOTE_STYLE_AFTER: &str =
    "[class='foo'] {\n\tbackground-image: url('/path/to/file.jpg');\n}\n";

const SVELTE_IMPLICIT_JS_FILE_UNFORMATTED: &str = r#"<script>
import {    something } from "file.svelte";
statement ( ) ;
</script>
<div></div>"#;

const SVELTE_IMPLICIT_JS_FILE_FORMATTED: &str = r#"<script>
import { something } from "file.svelte";
statement();
</script>
<div></div>"#;

const SVELTE_EXPLICIT_JS_FILE_UNFORMATTED: &str = r#"<script lang="js">
import {    something } from "file.svelte";
statement ( ) ;
</script>
<div></div>"#;

const SVELTE_EXPLICIT_JS_FILE_FORMATTED: &str = r#"<script lang="js">
import { something } from "file.svelte";
statement();
</script>
<div></div>"#;

const SVELTE_TS_FILE_UNFORMATTED: &str = r#"<script setup lang="ts">
import     { type     something } from "file.svelte";
const hello  :      string      = "world";
</script>
<div></div>"#;

const SVELTE_TS_FILE_FORMATTED: &str = r#"<script setup lang="ts">
import { type something } from "file.svelte";
const hello: string = "world";
</script>
<div></div>"#;

const APPLY_TRAILING_COMMAS_BEFORE: &str = r#"
const a = [
	longlonglonglongItem1longlonglonglongItem1,
	longlonglonglongItem1longlonglonglongItem2,
	longlonglonglongItem1longlonglonglongItem3,
];
"#;

const APPLY_TRAILING_COMMAS_AFTER: &str = r#"const a = [
	longlonglonglongItem1longlonglonglongItem1,
	longlonglonglongItem1longlonglonglongItem2,
	longlonglonglongItem1longlonglonglongItem3
];
"#;

const APPLY_ARROW_PARENTHESES_BEFORE: &str = r#"
action => {}
(action) => {}
({ action }) => {}
([ action ]) => {}
(...action) => {}
(action = 1) => {}
"#;

const APPLY_ARROW_PARENTHESES_AFTER: &str = r#"action => {};
action => {};
({ action }) => {};
([action]) => {};
(...action) => {};
(action = 1) => {};
"#;

const APPLY_BRACKET_SPACING_BEFORE: &str = r#"import { Foo } from "bar";
let foo = { a, b };
const { a, b } = foo;
"#;

const APPLY_BRACKET_SPACING_AFTER: &str = r#"import {Foo} from "bar";
let foo = {a, b};
const {a, b} = foo;
"#;

const APPLY_BRACKET_SPACING_BEFORE_GRAPHQL: &str = r#"{
	field_value(
		object_value: {key: "value"}
	)
}"#;

const APPLY_BRACKET_SPACING_AFTER_GRAPHQL: &str = r#"{
	field_value(object_value: {key: "value"})
}
"#;

const APPLY_BRACKET_SAME_LINE_BEFORE: &str = r#"<Foo
	className={style}
	reallyLongAttributeName1={longComplexValue}
	reallyLongAttributeName2={anotherLongValue}
/>;

<Foo
	className={style}
	reallyLongAttributeName1={longComplexValue}
	reallyLongAttributeName2={anotherLongValue}
>
	Hi
</Foo>;
"#;

const APPLY_BRACKET_SAME_LINE_AFTER: &str = r#"<Foo
	className={style}
	reallyLongAttributeName1={longComplexValue}
	reallyLongAttributeName2={anotherLongValue}
/>;

<Foo
	className={style}
	reallyLongAttributeName1={longComplexValue}
	reallyLongAttributeName2={anotherLongValue}>
	Hi
</Foo>;
"#;

const APPLY_ATTRIBUTE_POSITION_BEFORE: &str = r#"<Foo className={style}	reallyLongAttributeName1={longComplexValue}
reallyLongAttributeName2={anotherLongValue} />;

<Foo reallyLongAttributeName1={longComplexValue}reallyLongAttributeName2={anotherLongValue}>Hi</Foo>;"#;

const APPLY_ATTRIBUTE_POSITION_AFTER: &str = r#"<Foo
	className={style}
	reallyLongAttributeName1={longComplexValue}
	reallyLongAttributeName2={anotherLongValue}
/>;

<Foo
	reallyLongAttributeName1={longComplexValue}
	reallyLongAttributeName2={anotherLongValue}
>
	Hi
</Foo>;
"#;

#[cfg(not(windows))]
const DEFAULT_CONFIGURATION_BEFORE: &str = r#"function f() {
    return { a, b }
  }"#;

#[cfg(not(windows))]
const DEFAULT_CONFIGURATION_AFTER: &str = "function f() {
      return { a, b };
}
";

const CUSTOM_CONFIGURATION_BEFORE: &str = r#"function f() {
  return { a, b }
}"#;

const CUSTOM_CONFIGURATION_AFTER: &str = "function f() {
        return {
                a,
                b,
        };
}
";

#[test]
fn format_help() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--help"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_help",
        fs,
        console,
        result,
    ));
}

#[test]
fn print() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatter_print",
        fs,
        console,
        result,
    ));
}

#[test]
fn write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, FORMATTED);

    assert_eq!(console.out_buffer.len(), 1);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatter_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_shows_parse_diagnostics() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), "while ) {}".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_shows_parse_diagnostics",
        fs,
        console,
        result,
    ));
}

#[test]
fn write_only_files_in_correct_base() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_to_format = Utf8Path::new("src/format.js");
    fs.insert(
        file_to_format.into(),
        <&str>::clone(&UNFORMATTED).as_bytes(),
    );

    let file_to_not_format = Utf8Path::new("scripts/format.js");
    fs.insert(file_to_not_format.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(&["format", "--write", "./src"]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_to_format, FORMATTED);

    assert_file_contents(&fs, file_to_not_format, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "write_only_files_in_correct_base",
        fs,
        console,
        result,
    ));
}

// Ensures lint warnings are not printed in format mode
#[test]
fn lint_warning() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, LINT_ERROR);

    // The console buffer is expected to contain the following message:
    // 0: "Formatter would have printed the following content"
    // 1: "Checked 1 files"
    assert_eq!(
        console.out_buffer.len(),
        2,
        "console {:#?}",
        console.out_buffer
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatter_lint_warning",
        fs,
        console,
        result,
    ));
}

#[test]
// FIXME: redact snapshot for custom paths in configuration
#[cfg(not(windows))]
fn custom_config_file_path() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_path = Utf8Path::new("/test/biome.json");
    fs.insert(config_path.into(), CONFIG_FORMAT.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), DEFAULT_CONFIGURATION_BEFORE.as_bytes());

    let mut config_path = Utf8PathBuf::from(config_path);
    config_path.pop();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                format!("--config-path={}", config_path.to_string().as_str()).as_str(),
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, DEFAULT_CONFIGURATION_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "custom_config_file_path",
        fs,
        console,
        result,
    ));
}

// Should throw an error when an invalid configuration path is specified
#[test]
fn invalid_config_file_path() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_path = Utf8Path::new("test");
    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), *b"content");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--config-path",
                config_path.as_str(),
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "invalid_config_file_path",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_configuration() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_CONFIGURATION_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--line-width",
                "10",
                "--indent-style",
                "space",
                "--indent-width",
                "8",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, CUSTOM_CONFIGURATION_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_configuration_over_config_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_FORMAT.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_CONFIGURATION_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--line-width",
                "10",
                "--indent-style",
                "space",
                "--indent-width",
                "8",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, CUSTOM_CONFIGURATION_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_configuration_over_config_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_configuration_over_config_file_issue_3175_v1() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_ISSUE_3175_1.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), "import React from 'react';\n".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--javascript-formatter-quote-style",
                "single",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, "import React from 'react';\n");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_configuration_over_config_file_issue_3175_v1",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_configuration_over_config_file_issue_3175_v2() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let source = r#"function f() {
  return 'hey';
}
"#;

    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_ISSUE_3175_2.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), source.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--indent-style", "space", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, source);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_configuration_over_config_file_issue_3175_v2",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_jsx_quote_style() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.jsx");
    fs.insert(file_path.into(), APPLY_JSX_QUOTE_STYLE_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--jsx-quote-style",
                "single",
                "--quote-properties",
                "preserve",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, APPLY_JSX_QUOTE_STYLE_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_jsx_quote_style",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_quote_style() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), APPLY_QUOTE_STYLE_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--javascript-formatter-quote-style",
                "single",
                "--quote-properties",
                "preserve",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, APPLY_QUOTE_STYLE_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_quote_style",
        fs,
        console,
        result,
    ));
}

#[test]
#[ignore = "Enable when we are ready to handle CSS files"]
fn applies_custom_css_quote_style() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_path = Utf8Path::new("file.css");
    fs.insert(
        css_file_path.into(),
        APPLY_CSS_QUOTE_STYLE_BEFORE.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--css-formatter-quote-style",
                "single",
                "--write",
                css_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, css_file_path, APPLY_CSS_QUOTE_STYLE_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_css_quote_style",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_trailing_commas() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), APPLY_TRAILING_COMMAS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--trailing-commas",
                "none",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, APPLY_TRAILING_COMMAS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_trailing_commas",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_attribute_position() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), APPLY_ATTRIBUTE_POSITION_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--attribute-position",
                "multiline",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, APPLY_ATTRIBUTE_POSITION_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_attribute_position",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_arrow_parentheses() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), APPLY_ARROW_PARENTHESES_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--arrow-parentheses",
                "as-needed",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, APPLY_ARROW_PARENTHESES_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_arrow_parentheses",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_bracket_spacing() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), APPLY_BRACKET_SPACING_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--bracket-spacing",
                "false",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, APPLY_BRACKET_SPACING_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_bracket_spacing",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_bracket_same_line() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.jsx");
    fs.insert(file_path.into(), APPLY_BRACKET_SAME_LINE_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--bracket-same-line",
                "true",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, APPLY_BRACKET_SAME_LINE_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_bracket_same_line",
        fs,
        console,
        result,
    ));
}

#[test]
fn trailing_commas_parse_errors() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--trailing-commas", "NONE", "file.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "trailing_commas_parse_errors",
        fs,
        console,
        result,
    ));
}

#[test]
fn with_semicolons_options() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--semicolons=as-needed",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, "statement()\n");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_semicolons_options",
        fs,
        console,
        result,
    ));
}

#[test]
fn with_invalid_semicolons_option() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--semicolons", "asneed", "file.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_invalid_semicolons_option",
        fs,
        console,
        result,
    ));
}

#[test]
fn indent_style_parse_errors() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--indent-style", "invalid", "file.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_style_parse_errors",
        fs,
        console,
        result,
    ));
}

#[test]
fn indent_size_parse_errors_negative() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--indent-width=-1", "file.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_size_parse_errors_negative",
        fs,
        console,
        result,
    ));
}

#[test]
fn indent_size_parse_errors_overflow() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--indent-width=257", "file.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_size_parse_errors_overflow",
        fs,
        console,
        result,
    ));
}

#[test]
fn line_width_parse_errors_negative() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--line-width=-1", "file.js"].as_slice()),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "line_width_parse_errors_negative",
        fs,
        console,
        result,
    ));
}

#[test]
fn line_width_parse_errors_overflow() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--line-width", "321", "file.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "line_width_parse_errors_overflow",
        fs,
        console,
        result,
    ));
}

#[test]
fn quote_properties_parse_errors_letter_case() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--quote-properties", "As-needed", "file.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "quote_properties_parse_errors_letter_case",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_with_configuration() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_FORMAT.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "file.js", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, CUSTOM_FORMAT_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_with_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_is_disabled() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_DISABLED_FORMATTER.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "file.js", "--write"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, CUSTOM_FORMAT_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_is_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_stdin_successfully() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push("function f() {return{}}".to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path", "mock.js"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, "function f() {\n\treturn {};\n}\n");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_stdin_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_stdin_with_errors() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path", "mock.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_stdin_with_errors",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_format_if_disabled() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_DISABLED_FORMATTER.as_bytes());

    console
        .in_buffer
        .push("function f() {return{}}".to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path", "mock.js"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, "function f() {return{}}".to_string());

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_format_if_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_format_ignored_files() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{ "formatter": { "includes": ["**/*.js", "!test.js"] } }"#.as_bytes(),
    );

    let file_path = Utf8Path::new("test.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "test.js", "--write"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_format_ignored_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_format_if_files_are_listed_in_ignore_option() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "files": { "includes": ["**", "!test1.js"] },
            "formatter": { "includes": ["**", "!test2.js"] }
        }"#
        .as_bytes(),
    );

    let file_path_test1 = Utf8Path::new("test1.js");
    fs.insert(file_path_test1.into(), UNFORMATTED.as_bytes());

    let file_path_test2 = Utf8Path::new("test2.js");
    fs.insert(file_path_test2.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                file_path_test1.as_str(),
                file_path_test2.as_str(),
                "--write",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path_test1)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, UNFORMATTED);

    let mut buffer = String::new();
    fs.open(file_path_test2)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_format_if_files_are_listed_in_ignore_option",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_format_ignored_directories() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "formatter": {
                "includes": [
                    "**",
                    "!test1.js",
                    "!test2.js",
                    "!test3/**/*",
                    "!/test4/**/*",
                    "!test5/**/*",
                    "!**/test6/*.js",
                    "!**/*.test7.js"
                ]
            }
        }"#
        .as_bytes(),
    );

    const FILES: [(&str, bool); 9] = [
        ("test.js", true),
        ("test1.js", false),
        ("test2.js", false),
        ("test3/test.js", false),
        ("test4/test.js", true),
        ("test5/test.js", false),
        ("test6/test.js", false),
        ("test/test.test7.js", false),
        ("test.test7.js", false),
    ];

    for (file_path, _) in FILES {
        let file_path = Utf8Path::new(file_path);
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());
    }

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "./", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    for (file_path, expect_formatted) in FILES {
        let expected = if expect_formatted {
            FORMATTED
        } else {
            UNFORMATTED
        };
        assert_file_contents(&fs, Utf8Path::new(file_path), expected);
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_format_ignored_directories",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_format_ignored_file_in_included_directory() {
    let config = r#"{
        "formatter": {
          "includes": ["src/**", "!src/file2.js"]
        }
    }"#;
    let files = [("src/file1.js", true), ("src/file2.js", false)];

    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), config);
    for (file_path, _) in files {
        let file_path = Utf8Path::new(file_path);
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());
    }

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", ".", "--write"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    for (file_path, expect_formatted) in files {
        let expected = if expect_formatted {
            FORMATTED
        } else {
            UNFORMATTED
        };
        assert_file_contents(&fs, Utf8Path::new(file_path), expected);
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_format_ignored_file_in_included_directory",
        fs,
        console,
        result,
    ));
}

#[test]
fn include_ignore_cascade() {
    // Only `file1.js` will be formatted:
    // - `file2.js` is ignored at top-level
    // - `file3.js` is ignored at formatter-level
    // - `file4.js` is not included at top-level
    let config = r#"{
        "files": {
            "includes": ["file1.js", "file2.js", "file3.js", "!file2.js"]
        },
        "formatter": {
            "includes": ["file1.js", "file2.js", "!file3.js"]
        }
    }"#;
    let files = [
        ("file1.js", true),
        ("file2.js", false),
        ("file3.js", false),
        ("file4.js", false),
    ];

    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), config);
    for (file_path, _) in files {
        let file_path = Utf8Path::new(file_path);
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());
    }

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", ".", "--write"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    for (file_path, expect_formatted) in files {
        let expected = if expect_formatted {
            FORMATTED
        } else {
            UNFORMATTED
        };
        assert_file_contents(&fs, Utf8Path::new(file_path), expected);
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "include_ignore_cascade",
        fs,
        console,
        result,
    ));
}

#[test]
fn fs_error_read_only() {
    let mut fs = MemoryFileSystem::new_read_only();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("test.js");
    fs.insert(file_path.into(), *b"content");

    let (mut fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    // Do not store the content of the file in the snapshot
    fs.remove(file_path);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_read_only",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), "statement();\n".repeat(80660).as_bytes());

    let (mut fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str(), "--write"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    // Do not store the content of the file in the snapshot
    fs.remove(file_path);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large_config_limit() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(Utf8PathBuf::from("biome.json"), CONFIG_FILE_SIZE_LIMIT);

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large_config_limit",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large_cli_limit() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--files-max-size=16", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large_cli_limit",
        fs,
        console,
        result,
    ));
}

#[test]
fn files_max_size_parse_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--files-max-size=-1", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "files_max_size_parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn max_diagnostics_default() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..60 {
        let file_path = Utf8PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, UNFORMATTED.as_bytes());
    }

    let (mut fs, result) = run_cli(fs, &mut console, Args::from(["format", "src"].as_slice()));

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut diagnostic_count = 0;
    let mut filtered_messages = Vec::new();

    for msg in console.out_buffer {
        let MarkupBuf(nodes) = &msg.content;
        let is_diagnostic = nodes.iter().any(|node| {
            node.content
                .contains("Formatter would have printed the following content")
        });

        if is_diagnostic {
            diagnostic_count += 1;
        } else {
            filtered_messages.push(msg);
        }
    }

    console.out_buffer = filtered_messages;

    for i in 0..60 {
        let file_path = format!("src/file_{i}.js");
        fs.remove(Utf8Path::new(&file_path));
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics_default",
        fs,
        console,
        result,
    ));

    assert_eq!(diagnostic_count, 20);
}

#[test]
fn max_diagnostics() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..60 {
        let file_path = Utf8PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, UNFORMATTED.as_bytes());
    }

    let (mut fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--max-diagnostics", "10", "src"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut diagnostic_count = 0;
    let mut filtered_messages = Vec::new();

    for msg in console.out_buffer {
        let MarkupBuf(nodes) = &msg.content;
        let is_diagnostic = nodes.iter().any(|node| {
            node.content
                .contains("Formatter would have printed the following content")
        });

        if is_diagnostic {
            diagnostic_count += 1;
        } else {
            filtered_messages.push(msg);
        }
    }

    console.out_buffer = filtered_messages;

    for i in 0..60 {
        let file_path = format!("src/file_{i}.js");
        fs.remove(Utf8Path::new(&file_path));
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics",
        fs,
        console,
        result,
    ));

    assert_eq!(diagnostic_count, 10);
}

#[test]
fn no_supported_file_found() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(fs, &mut console, Args::from(["format", "."].as_slice()));

    eprintln!("{:?}", console.out_buffer);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_supported_file_found",
        fs,
        console,
        result,
    ));
}

#[test]
fn print_verbose() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--verbose", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "print_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignore_vcs_ignored_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "vcs": {
            "enabled": true,
            "clientKind": "git",
            "useIgnoreFile": true
        }
    }"#;

    let git_ignore = r#"
file2.js
"#;

    let code2 = r#"foo.call();


	bar.call();"#;
    let code1 = r#"array.map(sentence =>


	sentence.split(' ')).flat();"#;

    // ignored files
    let file_path1 = Utf8Path::new("file1.js");
    fs.insert(file_path1.into(), code1.as_bytes());
    let file_path2 = Utf8Path::new("file2.js");
    fs.insert(file_path2.into(), code2.as_bytes());

    // configuration
    let config_path = Utf8Path::new("biome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    // git ignore file
    let ignore_file = Utf8Path::new(".gitignore");
    fs.insert(ignore_file.into(), git_ignore.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_vcs_ignored_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignore_vcs_ignored_file_via_cli() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let git_ignore = r#"
file2.js
"#;

    let code2 = r#"foo.call();


	bar.call();"#;
    let code1 = r#"array.map(sentence =>


	sentence.split(' ')).flat();"#;

    // ignored files
    let file_path1 = Utf8Path::new("file1.js");
    fs.insert(file_path1.into(), code1.as_bytes());
    let file_path2 = Utf8Path::new("file2.js");
    fs.insert(file_path2.into(), code2.as_bytes());

    // git folder
    let git_folder = Utf8Path::new("./.git");
    fs.insert(git_folder.into(), "".as_bytes());

    // git ignore file
    let ignore_file = Utf8Path::new("./.gitignore");
    fs.insert(ignore_file.into(), git_ignore.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--vcs-enabled=true",
                "--vcs-client-kind=git",
                "--vcs-use-ignore-file=true",
                "--vcs-root=.",
                "--write",
                file_path1.as_str(),
                file_path2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_vcs_ignored_file_via_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn include_vcs_ignore_cascade() {
    // Only `file1.js` will be formatted:
    // - `file2.js` is ignored at top-level
    // - `file3.js` is ignored at formatter-level
    // - `file4.js` is ignored in `.gitignore`
    let git_ignore = r#"file4.js"#;
    let config = r#"{
        "vcs": {
            "enabled": true,
            "clientKind": "git",
            "useIgnoreFile": true
        },
        "files": {
            "includes": ["**", "!file2.js"]
        },
        "formatter": {
          "includes": ["file1.js", "file2.js", "file4.js", "!file3.js"]
        }
    }"#;
    let files = [
        ("file1.js", true),
        ("file2.js", false),
        ("file3.js", false),
        ("file4.js", false),
    ];

    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let gitignore_file = Utf8Path::new(".gitignore");
    fs.insert(gitignore_file.into(), git_ignore.as_bytes());
    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), config);
    for (file_path, _) in files {
        let file_path = Utf8Path::new(file_path);
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());
    }

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", ".", "--write"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    for (file_path, expect_formatted) in files {
        let expected = if expect_formatted {
            FORMATTED
        } else {
            UNFORMATTED
        };
        assert_file_contents(&fs, Utf8Path::new(file_path), expected);
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "include_vcs_ignore_cascade",
        fs,
        console,
        result,
    ));
}

#[test]
fn vcs_absolute_path() {
    let git_ignore = r#"file.js"#;
    let config = r#"{
        "vcs": {
            "enabled": true,
            "clientKind": "git",
            "useIgnoreFile": true
        }
    }"#;
    let files = [("/symbolic/link/to/path.js", true)];

    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let gitignore_file = Utf8Path::new(".gitignore");
    fs.insert(gitignore_file.into(), git_ignore.as_bytes());
    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), config);
    for (file_path, _) in files {
        let file_path = Utf8Path::new(file_path);
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());
    }

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", ".", "--write"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    for (file_path, expect_formatted) in files {
        let expected = if expect_formatted {
            FORMATTED
        } else {
            UNFORMATTED
        };
        assert_file_contents(&fs, Utf8Path::new(file_path), expected);
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "vcs_absolute_path",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignores_unknown_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Utf8Path::new("test.txt");
    fs.insert(file_path1.into(), *b"content");

    let file_path2 = Utf8Path::new("test.js");
    fs.insert(file_path2.into(), *b"console.log('bar');\n");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                file_path1.as_str(),
                file_path2.as_str(),
                "--files-ignore-unknown=true",
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignores_unknown_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn doesnt_error_if_no_files_were_processed() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--no-errors-on-unmatched", "file.js"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "doesnt_error_if_no_files_were_processed",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignore_comments_error_when_allow_comments() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_json = r#"{
  "json": {
    "parser": { "allowComments": true }
  }
}

	"#;
    let biome_config = "biome.json";
    let code = r#"
/*test*/ [1, 2, 3]
	"#;
    let file_path = Utf8Path::new("somefile.json");
    fs.insert(file_path.into(), code.as_bytes());
    fs.insert(biome_config.into(), config_json);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_comments_error_when_allow_comments",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_jsonc_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let code = r#"
/*test*/ [

/* some other comment*/1, 2, 3]
	"#;
    let file_path = Utf8Path::new("file.jsonc");
    fs.insert(file_path.into(), code.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_jsonc_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_json_when_allow_trailing_commas() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_json = r#"{
    "json": {
        "parser": { "allowTrailingCommas": true }
    }
}"#;
    let biome_config = "biome.json";
    let code = r#"{
    "array": [
        1,
    ],
}"#;
    let file_path = Utf8Path::new("file.json");
    fs.insert(file_path.into(), code.as_bytes());
    fs.insert(biome_config.into(), config_json);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_json_when_allow_trailing_commas",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_json_when_allow_trailing_commas_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_json = r#"{
    "json": {
        "parser": { "allowTrailingCommas": true }
    }
}"#;
    let biome_config = "biome.json";
    let code = r#"{   "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar",
}"#;
    let file_path = Utf8Path::new("file.json");
    fs.insert(file_path.into(), code.as_bytes());
    fs.insert(biome_config.into(), config_json);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, Utf8Path::new(file_path), "{\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\"\n}\n");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_json_when_allow_trailing_commas_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_json_trailing_commas_none() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_json = r#"{
    "json": {
        "parser": { "allowTrailingCommas": true },
        "formatter": { "trailingCommas": "none" }
    }
}"#;
    let biome_config = "biome.json";
    let code = r#"{   "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar",
}"#;
    let file_path = Utf8Path::new("file.json");
    fs.insert(file_path.into(), code.as_bytes());
    fs.insert(biome_config.into(), config_json);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, Utf8Path::new(file_path), "{\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\"\n}\n");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_json_trailing_commas_none",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_json_trailing_commas_all() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_json = r#"{
    "json": {
        "parser": { "allowTrailingCommas": true },
        "formatter": { "trailingCommas": "all" }
    }
}"#;
    let biome_config = "biome.json";
    let code = r#"{   "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar",
}"#;
    let file_path = Utf8Path::new("file.json");
    fs.insert(file_path.into(), code.as_bytes());
    fs.insert(biome_config.into(), config_json);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, Utf8Path::new(file_path), "{\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\"\n}\n");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_json_trailing_commas_all",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_json_trailing_commas_overrides_all() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_json = r#"{
    "json": {
        "parser": { "allowTrailingCommas": true },
        "formatter": { "trailingCommas": "none" }
    },
    "overrides": [{
        "includes": ["file.json"],
        "json": {
            "formatter": { "trailingCommas": "all" }
        }
    }]
}"#;
    let biome_config = "biome.json";
    let code = r#"{   "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar",
}"#;
    let file_path = Utf8Path::new("file.json");
    fs.insert(file_path.into(), code.as_bytes());
    fs.insert(biome_config.into(), config_json);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, Utf8Path::new(file_path), "{\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\"\n}\n");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_json_trailing_commas_overrides_all",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_json_trailing_commas_overrides_none() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_json = r#"{
    "json": {
        "parser": { "allowTrailingCommas": true },
        "formatter": { "trailingCommas": "all" }
    },
    "overrides": [{
        "includes": ["file.json"],
        "json": {
            "formatter": { "trailingCommas": "none" }
        }
    }]
}"#;
    let biome_config = "biome.json";
    let code = r#"{   "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar", "loreum_ipsum_lorem_ipsum":   "bar",
}"#;
    let file_path = Utf8Path::new("file.json");
    fs.insert(file_path.into(), code.as_bytes());
    fs.insert(biome_config.into(), config_json);

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, Utf8Path::new(file_path), "{\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\",\n\t\"loreum_ipsum_lorem_ipsum\": \"bar\"\n}\n");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_json_trailing_commas_overrides_none",
        fs,
        console,
        result,
    ));
}

#[test]
fn treat_known_json_files_as_jsonc_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let code = r#"
/*test*/ [

/* some other comment*/1, 2, 3]
	"#;
    let eslint = Utf8Path::new("files/.eslintrc.json");
    fs.insert(eslint.into(), code.as_bytes());
    let jshint = Utf8Path::new("files/.jshintrc");
    fs.insert(jshint.into(), code.as_bytes());
    let babel = Utf8Path::new("files/.babelrc");
    fs.insert(babel.into(), code.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", eslint.as_str(), jshint.as_str(), babel.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "treat_known_json_files_as_jsonc_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_apply_different_formatting() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
        "formatter": {
            "indentStyle": "space"
        },
        "javascript": {
            "formatter": {
                "lineWidth": 320,
                "indentWidth": 8
            }
        },
        "json": {
            "formatter": {
                "lineWidth": 80,
                "indentWidth": 2
            }
        },
        "css": {
            "formatter": {
                "lineWidth": 40,
                "indentWidth": 6
            }
        }
    }"#,
    );

    let code = r#"
{
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let json_file = Utf8Path::new("input.json");
    fs.insert(json_file.into(), code.as_bytes());
    let code = r#"html {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), code.as_bytes());

    let code = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Utf8Path::new("input.js");
    fs.insert(js_file.into(), code.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                json_file.as_str(),
                js_file.as_str(),
                css_file.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_apply_different_formatting",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_apply_different_formatting_with_cli() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
        "formatter": {
            "indentStyle": "space"
        }
    }"#,
    );

    let json_file_content = r#"
{
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let json_file = Utf8Path::new("input.json");
    fs.insert(json_file.into(), json_file_content.as_bytes());

    let css_file_content = r#"html {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let js_file_content = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Utf8Path::new("input.js");
    fs.insert(js_file.into(), js_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--javascript-formatter-line-width=320",
                "--javascript-formatter-indent-width=8",
                "--json-formatter-line-width=20",
                "--json-formatter-indent-width=2",
                "--css-formatter-line-width=40",
                "--css-formatter-indent-width=6",
                "--css-formatter-enabled=true",
                json_file.as_str(),
                js_file.as_str(),
                css_file.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_apply_different_formatting_with_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_format_json_files_if_disabled() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
        "formatter": {
            "indentStyle": "space"
        },
        "javascript": {
            "formatter": {
                "lineWidth": 80,
                "indentWidth": 4
            }
        },
        "json": {
            "formatter": {
                "enabled": false
            }
        }
    }"#,
    );

    let json_file_content = r#"
{
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let json_file = Utf8Path::new("input.json");
    fs.insert(json_file.into(), json_file_content.as_bytes());

    let js_file_content = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Utf8Path::new("input.js");
    fs.insert(js_file.into(), js_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", json_file.as_str(), js_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, json_file, json_file_content);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_format_json_files_if_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_format_js_files_if_disabled() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
        "formatter": {
            "indentStyle": "space"
        },
        "javascript": {
            "formatter": {
                "enabled": false
            }
        },
        "json": {
            "formatter": {
                "lineWidth": 80,
                "indentWidth": 2
            }
        }
    }"#,
    );

    let json_file_content = r#"
{
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let json_file = Utf8Path::new("input.json");
    fs.insert(json_file.into(), json_file_content.as_bytes());

    let js_file_content = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Utf8Path::new("input.js");
    fs.insert(js_file.into(), js_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", json_file.as_str(), js_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, js_file, js_file_content);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_format_js_files_if_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_format_css_files_if_disabled() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
        "formatter": {
            "indentStyle": "space"
        },
        "javascript": {
            "formatter": {
                "lineWidth": 80,
                "indentWidth": 4
            }
        },
        "css": {
            "formatter": {
                "enabled": false
            }
        }
    }"#,
    );

    let css_file_content = r#"html {

    }
    "#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let js_file_content = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Utf8Path::new("input.js");
    fs.insert(js_file.into(), js_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", css_file.as_str(), js_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, css_file, css_file_content);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_format_css_files_if_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_apply_different_indent_style() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
        "formatter": {
            "indentStyle": "space"
        },
        "javascript": {
            "formatter": {
                "lineWidth": 320,
                "indentWidth": 8,
                "indentStyle": "tab"
            }
        },
        "json": {
            "formatter": {
                "lineWidth": 80,
                "indentWidth": 2,
                "indentStyle": "tab"
            }
        }
    }"#,
    );

    let json_file_content = r#"
{
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let json_file = Utf8Path::new("input.json");
    fs.insert(json_file.into(), json_file_content.as_bytes());

    let js_file_content = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Utf8Path::new("input.js");
    fs.insert(js_file.into(), js_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", json_file.as_str(), js_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(js_file)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert!(content.contains('\t'), "should contain tabs");

    drop(file);

    let mut file = fs
        .open(json_file)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert!(content.contains('\t'), "should contain tabs");

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_apply_different_indent_style",
        fs,
        console,
        result,
    ));
}

#[test]
fn override_don_t_affect_ignored_files() {
    let config = r#"{
        "overrides": [{
            "includes": ["**", "!file2.js"]
        }]
    }"#;
    let files = [("file1.js", true), ("file2.js", true)];

    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), config);
    for (file_path, _) in files {
        let file_path = Utf8Path::new(file_path);
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());
    }

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", ".", "--write"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    for (file_path, expect_formatted) in files {
        let expected = if expect_formatted {
            FORMATTED
        } else {
            UNFORMATTED
        };
        assert_file_contents(&fs, Utf8Path::new(file_path), expected);
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "override_don_t_affect_ignored_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_with_configured_line_ending() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    let config = r#"{
        "formatter": {
            "lineEnding": "crlf",
            "lineWidth": 20
        }
    }"#;
    let code_json = r#"{ "name": "mike", "surname": "ross" }"#;
    let code_js = r#"const b = { "name": "mike", "surname": "ross" }"#;
    let json_file = Utf8Path::new("input.json");
    fs.insert(json_file.into(), code_json.as_bytes());

    let js_file = Utf8Path::new("input.js");
    fs.insert(js_file.into(), code_js.as_bytes());

    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), config);

    let (fs, result) = run_cli(fs, &mut console, Args::from(&["format", ".", "--write"]));
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(
        &fs,
        json_file,
        "{\r\n\t\"name\": \"mike\",\r\n\t\"surname\": \"ross\"\r\n}\r\n",
    );
    assert_file_contents(
        &fs,
        js_file,
        "const b = {\r\n\tname: \"mike\",\r\n\tsurname: \"ross\",\r\n};\r\n",
    );
}

#[test]
fn don_t_format_ignored_known_jsonc_files() {
    let config = r#"{
        "files": {
            "ignoreUnknown": true,
            "includes": ["**", "!.eslintrc"]
        }
    }"#;
    let files = [(".eslintrc", false)];

    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), config);
    for (file_path, _) in files {
        let file_path = Utf8Path::new(file_path);
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());
    }

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", ".", "--write"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    for (file_path, expect_formatted) in files {
        let expected = if expect_formatted {
            FORMATTED
        } else {
            UNFORMATTED
        };
        assert_file_contents(&fs, Utf8Path::new(file_path), expected);
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "don_t_format_ignored_known_jsonc_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_configuration_from_biome_jsonc() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("biome.jsonc");
    fs.insert(file_path.into(), CONFIG_FORMAT_JSONC.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_CONFIGURATION_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, CUSTOM_CONFIGURATION_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_configuration_from_biome_jsonc",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_package_json() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("package.json");
    fs.insert(
        file_path.into(),
        r#"{

    "name":       "@foo/package",
    "dependencies": { "foo": "latest" }

     }"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--indent-style=space",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_eq!(console.out_buffer.len(), 1);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_package_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_svelte_implicit_js_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_IMPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, SVELTE_IMPLICIT_JS_FILE_UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_implicit_js_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_svelte_implicit_js_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_IMPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, SVELTE_IMPLICIT_JS_FILE_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_implicit_js_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_svelte_explicit_js_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_EXPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, SVELTE_EXPLICIT_JS_FILE_UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_explicit_js_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_svelte_explicit_js_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_EXPLICIT_JS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, SVELTE_EXPLICIT_JS_FILE_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_explicit_js_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_empty_svelte_js_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(svelte_file_path.into(), "<div></div>".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, "<div></div>");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_empty_svelte_js_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_svelte_ts_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_TS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, SVELTE_TS_FILE_UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_ts_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_svelte_ts_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(
        svelte_file_path.into(),
        SVELTE_TS_FILE_UNFORMATTED.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, SVELTE_TS_FILE_FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_svelte_ts_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_empty_svelte_ts_files_write() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file_path = Utf8Path::new("file.svelte");
    fs.insert(svelte_file_path.into(), "<div></div>".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", svelte_file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, svelte_file_path, "<div></div>");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_empty_svelte_ts_files_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_format_files_in_folders_ignored_by_linter() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("build/file.js");
    fs.insert(
        file_path.into(),
        r#"
	value['optimizelyService'] = optimizelyService;
		"#,
    );

    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
            "$schema": "https://biomejs.dev/schemas/1.6.1/schema.json",
            "assist": {
                "enabled": true
            },
            "linter": {
                "includes": ["**", "!**/build"],
                "enabled": true,
                "rules": {
                    "recommended": true
                }
            }
        }"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_format_files_in_folders_ignored_by_linter",
        fs,
        console,
        result,
    ));
}

#[test]
fn print_json() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--reporter=json", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "print_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn print_json_pretty() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--reporter=json-pretty", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "print_json_pretty",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_without_file_paths() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(fs, &mut console, Args::from(["format", ""].as_slice()));

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_without_file_paths",
        fs,
        console,
        result,
    ));
}

#[test]
fn fix() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let file_path = Utf8Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--fix", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, FORMATTED);
    assert_eq!(console.out_buffer.len(), 1);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatter_fix",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_error_if_unstaged_files_only_with_staged_flag() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    // Unstaged
    fs.insert(
        Utf8Path::new("file1.js").into(),
        r#"console.log('file1');"#.as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--staged"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_error_if_unstaged_files_only_with_staged_flag",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_error_if_unchanged_files_only_with_changed_flag() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    // Unchanged
    fs.insert(
        Utf8Path::new("file1.js").into(),
        r#"console.log('file1');"#.as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--changed", "--since=main"].as_slice()),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_error_if_unchanged_files_only_with_changed_flag",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_bracket_spacing_for_graphql() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.graphql");
    fs.insert(
        file_path.into(),
        APPLY_BRACKET_SPACING_BEFORE_GRAPHQL.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--bracket-spacing",
                "false",
                "--write",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, APPLY_BRACKET_SPACING_AFTER_GRAPHQL);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_bracket_spacing_graphql",
        fs,
        console,
        result,
    ));
}
