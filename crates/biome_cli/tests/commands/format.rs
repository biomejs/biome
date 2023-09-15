use crate::configs::{
    CONFIG_DISABLED_FORMATTER, CONFIG_FILE_SIZE_LIMIT, CONFIG_FORMAT,
    CONFIG_FORMATTER_AND_FILES_IGNORE, CONFIG_FORMATTER_IGNORED_DIRECTORIES,
    CONFIG_FORMATTER_IGNORED_FILES, CONFIG_ISSUE_3175_1, CONFIG_ISSUE_3175_2,
};
use crate::snap_test::{markup_to_string, SnapshotPayload};
use crate::{
    assert_cli_snapshot, run_cli, CUSTOM_FORMAT_BEFORE, FORMATTED, LINT_ERROR, UNFORMATTED,
};
use biome_console::{markup, BufferConsole, MarkupBuf};
use biome_fs::{FileSystemExt, MemoryFileSystem};
use biome_service::DynRef;
use bpaf::Args;
use std::path::{Path, PathBuf};

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

const APPLY_TRAILING_COMMA_BEFORE: &str = r#"
const a = [
	longlonglonglongItem1longlonglonglongItem1,
	longlonglonglongItem1longlonglonglongItem2,
	longlonglonglongItem1longlonglonglongItem3,
];
"#;

const APPLY_TRAILING_COMMA_AFTER: &str = r#"const a = [
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

// Without this, Test (windows-latest) fails with: `warning: constant `DEFAULT_CONFIGURATION_BEFORE` is never used`
#[allow(dead_code)]
const DEFAULT_CONFIGURATION_BEFORE: &str = r#"function f() {
    return { a, b }
  }"#;

// Without this, Test (windows-latest) fails with: `warning: constant `DEFAULT_CONFIGURATION_AFTER` is never used`
#[allow(dead_code)]
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), "--help"].as_slice()),
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

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED);

    drop(file);
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

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, FORMATTED);

    assert_eq!(console.out_buffer.len(), 1);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatter_write",
        fs,
        console,
        result,
    ));
}

#[test]
fn write_only_files_in_correct_base() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_to_format = Path::new("src/format.js");
    fs.insert(
        file_to_format.into(),
        <&str>::clone(&UNFORMATTED).as_bytes(),
    );

    let file_to_not_format = Path::new("scripts/format.js");
    fs.insert(file_to_not_format.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--write"), ("./src")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_to_format)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, FORMATTED, "we test the file is formatted");
    drop(file);
    let mut file = fs
        .open(file_to_not_format)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED, "we test the file is not formatted");
    drop(file);
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

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, LINT_ERROR);

    // The console buffer is expected to contain the following message:
    // 0: "Formatter would have printed the following content"
    // 1: "Compared 1 files"
    assert_eq!(
        console.out_buffer.len(),
        2,
        "console {:#?}",
        console.out_buffer
    );

    drop(file);
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

    let config_path = Path::new("/test/rome.json");
    fs.insert(config_path.into(), CONFIG_FORMAT.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), DEFAULT_CONFIGURATION_BEFORE.as_bytes());

    let mut config_path = PathBuf::from(config_path);
    config_path.pop();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                format!(
                    "--config-path={}",
                    config_path.display().to_string().as_str()
                )
                .as_str(),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, DEFAULT_CONFIGURATION_AFTER);

    drop(file);
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
// FIXME: redact snapshot for custom paths in configuration
#[cfg(not(windows))]
fn invalid_config_file_path() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_path = Path::new("test");
    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), *b"content");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--config-path"),
                (config_path.display().to_string().as_str()),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
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

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_CONFIGURATION_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--line-width"),
                ("10"),
                ("--indent-style"),
                ("space"),
                ("--indent-size"),
                ("8"),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, CUSTOM_CONFIGURATION_AFTER);

    drop(file);
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

    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_FORMAT.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_CONFIGURATION_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--line-width"),
                ("10"),
                ("--indent-style"),
                ("space"),
                ("--indent-size"),
                ("8"),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, CUSTOM_CONFIGURATION_AFTER);

    drop(file);
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

    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_ISSUE_3175_1.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), "import React from 'react';\n".as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--quote-style"),
                ("single"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, "import React from 'react';\n");

    drop(file);
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

    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_ISSUE_3175_2.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), source.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--indent-style"),
                ("space"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, source);

    drop(file);
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

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), APPLY_JSX_QUOTE_STYLE_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--jsx-quote-style"),
                ("single"),
                ("--quote-properties"),
                ("preserve"),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, APPLY_JSX_QUOTE_STYLE_AFTER);

    drop(file);
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

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), APPLY_QUOTE_STYLE_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--quote-style"),
                ("single"),
                ("--quote-properties"),
                ("preserve"),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, APPLY_QUOTE_STYLE_AFTER);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_quote_style",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_trailing_comma() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), APPLY_TRAILING_COMMA_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--trailing-comma"),
                ("none"),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, APPLY_TRAILING_COMMA_AFTER);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_trailing_comma",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_arrow_parentheses() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), APPLY_ARROW_PARENTHESES_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--arrow-parentheses"),
                ("as-needed"),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, APPLY_ARROW_PARENTHESES_AFTER);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_arrow_parentheses",
        fs,
        console,
        result,
    ));
}

#[test]
fn trailing_comma_parse_errors() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--trailing-comma"), ("NONE"), ("file.js")].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "trailing_comma_parse_errors",
        fs,
        console,
        result,
    ));
}

#[test]
fn with_semicolons_options() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--semicolons=as-needed"),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, "statement()\n");

    drop(file);
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
    let mut fs = MemoryFileSystem::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--semicolons"), ("asneed"), ("file.js")].as_slice()),
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
    let mut fs = MemoryFileSystem::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--indent-style"), ("invalid"), ("file.js")].as_slice()),
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
    let mut fs = MemoryFileSystem::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--indent-size=-1"), ("file.js")].as_slice()),
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
    let mut fs = MemoryFileSystem::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--indent-size=257"), ("file.js")].as_slice()),
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
    let mut fs = MemoryFileSystem::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
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
    let mut fs = MemoryFileSystem::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--line-width"), ("321"), ("file.js")].as_slice()),
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
    let mut fs = MemoryFileSystem::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--quote-properties"),
                ("As-needed"),
                ("file.js"),
            ]
            .as_slice(),
        ),
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
    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_FORMAT.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("file.js"), ("--write")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, CUSTOM_FORMAT_AFTER);

    drop(file);
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
    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_DISABLED_FORMATTER.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("file.js"), ("--write")].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, CUSTOM_FORMAT_BEFORE);

    drop(file);
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push("function f() {return{}}".to_string());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--stdin-file-path"), ("mock.js")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .get(0)
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--stdin-file-path"), ("mock.js")].as_slice()),
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

    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_DISABLED_FORMATTER.as_bytes());

    console
        .in_buffer
        .push("function f() {return{}}".to_string());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--stdin-file-path"), ("mock.js")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .get(0)
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
    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_FORMATTER_IGNORED_FILES.as_bytes());

    let file_path = Path::new("test.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("test.js"), ("--write")].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED);

    drop(file);
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

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        CONFIG_FORMATTER_AND_FILES_IGNORE.as_bytes(),
    );

    let file_path_test1 = Path::new("test1.js");
    fs.insert(file_path_test1.into(), UNFORMATTED.as_bytes());

    let file_path_test2 = Path::new("test2.js");
    fs.insert(file_path_test2.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                file_path_test1.as_os_str().to_str().unwrap(),
                file_path_test2.as_os_str().to_str().unwrap(),
                ("--write"),
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

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        CONFIG_FORMATTER_IGNORED_DIRECTORIES.as_bytes(),
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
        let file_path = Path::new(file_path);
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());
    }

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("./"), ("--write")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    for (file_path, expect_formatted) in FILES {
        let mut file = fs
            .open(Path::new(file_path))
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        let expected = if expect_formatted {
            FORMATTED
        } else {
            UNFORMATTED
        };

        assert_eq!(
            content, expected,
            "content of {file_path} doesn't match the expected content"
        );
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
fn fs_error_read_only() {
    let mut fs = MemoryFileSystem::new_read_only();
    let mut console = BufferConsole::default();

    let file_path = Path::new("test.js");
    fs.insert(file_path.into(), *b"content");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), "statement();\n".repeat(80660).as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                file_path.as_os_str().to_str().unwrap(),
                ("--write"),
            ]
            .as_slice(),
        ),
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

    fs.insert(PathBuf::from("biome.json"), CONFIG_FILE_SIZE_LIMIT);

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), file_path.as_os_str().to_str().unwrap()].as_slice()),
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

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--files-max-size=16"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--files-max-size=-1"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
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
        let file_path = PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, UNFORMATTED.as_bytes());
    }

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("src")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

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
        fs.remove(Path::new(&file_path));
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
        let file_path = PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, UNFORMATTED.as_bytes());
    }

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), ("--max-diagnostics"), ("10"), ("src")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

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
        fs.remove(Path::new(&file_path));
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
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), "."].as_slice()),
    );

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

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--verbose"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

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
    let file_path1 = Path::new("file1.js");
    fs.insert(file_path1.into(), code1.as_bytes());
    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), code2.as_bytes());

    // configuration
    let config_path = Path::new("biome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    // git folder
    let git_folder = Path::new(".git");
    fs.insert(git_folder.into(), "".as_bytes());

    // git ignore file
    let ignore_file = Path::new(".gitignore");
    fs.insert(ignore_file.into(), git_ignore.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                file_path1.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
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
    let file_path1 = Path::new("file1.js");
    fs.insert(file_path1.into(), code1.as_bytes());
    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), code2.as_bytes());

    // git folder
    let git_folder = Path::new("./.git");
    fs.insert(git_folder.into(), "".as_bytes());

    // git ignore file
    let ignore_file = Path::new("./.gitignore");
    fs.insert(ignore_file.into(), git_ignore.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--vcs-enabled=true"),
                ("--vcs-client-kind=git"),
                ("--vcs-use-ignore-file=true"),
                ("--vcs-root=."),
                ("--write"),
                file_path1.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
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
fn ignores_unknown_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Path::new("test.txt");
    fs.insert(file_path1.into(), *b"content");

    let file_path2 = Path::new("test.js");
    fs.insert(file_path2.into(), *b"console.log('bar');\n");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                file_path1.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
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
    let mut fs = MemoryFileSystem::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), "--no-errors-on-unmatched", ("file.js")].as_slice()),
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
    let rome_config = "biome.json";
    let code = r#"
/*test*/ [1, 2, 3]
	"#;
    let file_path = Path::new("tsconfig.json");
    fs.insert(file_path.into(), code.as_bytes());
    fs.insert(rome_config.into(), config_json);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

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
    let file_path = Path::new("file.jsonc");
    fs.insert(file_path.into(), code.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("format"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_jsonc_files",
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
    let ts = Path::new("files/typescript.json");
    fs.insert(ts.into(), code.as_bytes());
    let eslint = Path::new("files/.eslintrc.json");
    fs.insert(eslint.into(), code.as_bytes());
    let jshint = Path::new("files/.jshintrc");
    fs.insert(jshint.into(), code.as_bytes());
    let babel = Path::new("files/.babelrc");
    fs.insert(babel.into(), code.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ts.as_os_str().to_str().unwrap(),
                eslint.as_os_str().to_str().unwrap(),
                jshint.as_os_str().to_str().unwrap(),
                babel.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

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

    let biome_json = Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
        "formatter": {
            "indentStyle": "space"
        },
        "javascript": {
            "formatter": {
                "lineWidth": 320,
                "indentSize": 8
            }
        },
        "json": {
            "formatter": {
                "lineWidth": 80,
                "indentSize": 2
            }
        }
    }"#,
    );

    let code = r#"
{
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let json_file = Path::new("input.json");
    fs.insert(json_file.into(), code.as_bytes());

    let code = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Path::new("input.js");
    fs.insert(js_file.into(), code.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                "--write",
                json_file.as_os_str().to_str().unwrap(),
                js_file.as_os_str().to_str().unwrap(),
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

    let biome_json = Path::new("biome.json");
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
    let json_file = Path::new("input.json");
    fs.insert(json_file.into(), json_file_content.as_bytes());

    let js_file_content = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Path::new("input.js");
    fs.insert(js_file.into(), js_file_content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                "--write",
                "--javascript-formatter-line-width=320",
                "--javascript-formatter-indent-size=8",
                "--json-formatter-line-width=20",
                "--json-formatter-indent-size=2",
                json_file.as_os_str().to_str().unwrap(),
                js_file.as_os_str().to_str().unwrap(),
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

    let biome_json = Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
        "formatter": {
            "indentStyle": "space"
        },
        "javascript": {
            "formatter": {
                "lineWidth": 80,
                "indentSize": 4
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
    let json_file = Path::new("input.json");
    fs.insert(json_file.into(), json_file_content.as_bytes());

    let js_file_content = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Path::new("input.js");
    fs.insert(js_file.into(), js_file_content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                "--write",
                json_file.as_os_str().to_str().unwrap(),
                js_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(json_file)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, json_file_content);

    drop(file);

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

    let biome_json = Path::new("biome.json");
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
                "indentSize": 2
            }
        }
    }"#,
    );

    let json_file_content = r#"
{
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let json_file = Path::new("input.json");
    fs.insert(json_file.into(), json_file_content.as_bytes());

    let js_file_content = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Path::new("input.js");
    fs.insert(js_file.into(), js_file_content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                "--write",
                json_file.as_os_str().to_str().unwrap(),
                js_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(js_file)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, js_file_content);

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_format_js_files_if_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_apply_different_indent_style() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let biome_json = Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
        "formatter": {
            "indentStyle": "space"
        },
        "javascript": {
            "formatter": {
                "lineWidth": 320,
                "indentSize": 8,
                "indentStyle": "tab"
            }
        },
        "json": {
            "formatter": {
                "lineWidth": 80,
                "indentSize": 2,
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
    let json_file = Path::new("input.json");
    fs.insert(json_file.into(), json_file_content.as_bytes());

    let js_file_content = r#"
const a = {
    "array": ["lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum", "lorem ipsum"]
}
	"#;
    let js_file = Path::new("input.js");
    fs.insert(js_file.into(), js_file_content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                "--write",
                json_file.as_os_str().to_str().unwrap(),
                js_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(js_file)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert!(content.contains('\t'), "should not contain tabs");

    drop(file);

    let mut file = fs
        .open(json_file)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert!(content.contains('\t'), "should not contain tabs");

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_apply_different_indent_style",
        fs,
        console,
        result,
    ));
}
