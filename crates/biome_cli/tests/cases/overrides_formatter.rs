use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

const UNFORMATTED: &str = "  statement(  )  ";
const UNFORMATTED_JSON: &str = r#"{ "asta": ["lorem", "ipsum", "first", "second"] }"#;
const FORMATTED_JSON: &str =
    "{\n    \"asta\": [\n        \"lorem\",\n        \"ipsum\",\n        \"first\",\n        \"second\"\n    ]\n}\n";
const UNFORMATTED_CSS: &str = "html {}";
const FORMATTED_CSS: &str = "html {\n}\n";

const UNFORMATTED_LINE_WIDTH: &str = r#"const a = ["loreum", "ipsum"]"#;
const FORMATTED: &str = "statement();\n";
const FORMATTED_LINE_WIDTH_OVERRIDDEN: &str = "const a = [\n\t\"loreum\",\n\t\"ipsum\",\n];\n";

const FORMATTED_LINE_WITH_SPACES: &str = "const a = [\n  \"loreum\",\n  \"ipsum\",\n];\n";

const FORMATTED_LINE_WIDTH: &str = "const a = [\"loreum\", \"ipsum\"];\n";

const FORMATTED_WITH_SINGLE_QUOTES: &str = "const a = ['loreum', 'ipsum'];\n";
const FORMATTED_WITH_NO_SEMICOLONS: &str = "const a = [\"loreum\", \"ipsum\"]\n";

const CSS_UNFORMATTED_QUOTES: &str =
    r#"[class='foo'] { background-image: url("/path/to/file.jpg")}"#;
const CSS_FORMATTED_SINGLE_QUOTES_AND_SPACES: &str =
    "[class='foo'] {\n  background-image: url('/path/to/file.jpg');\n}\n";

#[test]
fn does_handle_included_file_and_disable_formatter() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "include": ["test.js", "special/**"]
  },
  "overrides": [{ "include": ["special/**"], "formatter": { "enabled": false } }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, UNFORMATTED);
    assert_file_contents(&fs, test, FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_handle_included_file_and_disable_formatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_formatting() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [{ "include": ["special/**"], "formatter": { "lineWidth": 20 } }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FORMATTED_LINE_WIDTH_OVERRIDDEN);
    assert_file_contents(&fs, test, FORMATTED_LINE_WIDTH);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_formatting",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_formatting_and_all_of_them() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "include": ["special/**"], "formatter": { "lineWidth": 130 } },
    { "include": ["special/**"], "formatter": { "lineWidth": 20 } }
   ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FORMATTED_LINE_WIDTH_OVERRIDDEN);
    assert_file_contents(&fs, test, FORMATTED_LINE_WIDTH);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_formatting_and_all_of_them",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_overrides() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "include": ["test.js"], "formatter": { "lineWidth": 20 } },
    { "include": ["test2.js"], "formatter": { "lineWidth": 20, "indentStyle": "space" } }
   ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, FORMATTED_LINE_WIDTH_OVERRIDDEN);
    assert_file_contents(&fs, test2, FORMATTED_LINE_WITH_SPACES);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_overrides",
        fs,
        console,
        result,
    ));
}

/// Issue: https://github.com/biomejs/biome/issues/2924
#[test]
fn complex_enable_disable_overrides() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "formatter": {
    "lineWidth": 20
  },
  "javascript": {
    "formatter": {
      "enabled": false
    }
  },
  "overrides": [
    { "include": ["formatted.js"], "formatter": { "enabled": true } },
    {
      "include": ["dirty.js"],
      "linter": {
        "rules": {
          "performance": {
            "noBarrelFile": "off"
          }
        }
      }
    }
  ]
}

"#
        .as_bytes(),
    );

    let formatted = Path::new("formatted.js");
    fs.insert(formatted.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let unformatted = Path::new("dirty.js");
    fs.insert(unformatted.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                formatted.as_os_str().to_str().unwrap(),
                unformatted.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert_file_contents(&fs, formatted, FORMATTED_LINE_WIDTH_OVERRIDDEN);
    assert_file_contents(&fs, unformatted, UNFORMATTED_LINE_WIDTH);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "complex_enable_disable_overrides",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_languages() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "include": ["test.js"], "formatter": { "lineWidth": 120 }, "javascript": { "formatter": { "quoteStyle": "single" } } },
    { "include": ["test2.js"], "formatter": { "lineWidth": 120, "indentStyle": "space" }, "javascript": { "formatter": { "semicolons": "asNeeded" } } },
    { "include": ["test.css"], "formatter": { "lineWidth": 120, "indentStyle": "space" }, "css": { "formatter": { "quoteStyle": "single" } } }
   ]
}
"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());
    let test_css = Path::new("test.css");
    fs.insert(test_css.into(), CSS_UNFORMATTED_QUOTES.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
                test_css.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, FORMATTED_WITH_SINGLE_QUOTES);
    assert_file_contents(&fs, test2, FORMATTED_WITH_NO_SEMICOLONS);
    assert_file_contents(&fs, test_css, CSS_FORMATTED_SINGLE_QUOTES_AND_SPACES);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_languages",
        fs,
        console,
        result,
    ));
}

#[test]
#[ignore = "Enable when we are ready to handle CSS files"]
fn does_include_file_with_different_languages_and_files() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "include": ["test.js"], "formatter": { "lineWidth": 120 }, "javascript": { "formatter": { "quoteStyle": "single" } } },
    {
        "include": ["test2.js"],
        "formatter": { "lineWidth": 120, "indentStyle": "space" },
        "javascript": { "formatter": { "semicolons": "asNeeded" } },
        "json": { "formatter": { "indentStyle": "space", "lineWidth": 20, "indentWidth": 4 } }
    },
    {
        "include": ["test3.json"],
        "formatter": { "lineWidth": 120, "indentStyle": "space" },
        "json": { "formatter": { "indentStyle": "space", "lineWidth": 20, "indentWidth": 4 } }
    }
  ]
}

"#
            .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let json_file = Path::new("test3.json");
    fs.insert(json_file.into(), UNFORMATTED_JSON.as_bytes());

    let css_file = Path::new("test4.css");
    fs.insert(css_file.into(), UNFORMATTED_CSS.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
                json_file.as_os_str().to_str().unwrap(),
                css_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, FORMATTED_WITH_SINGLE_QUOTES);
    assert_file_contents(&fs, test2, FORMATTED_WITH_NO_SEMICOLONS);
    assert_file_contents(&fs, json_file, FORMATTED_JSON);
    assert_file_contents(&fs, css_file, FORMATTED_CSS);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_languages_and_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_change_formatting_settings() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
        "formatter": { "lineWidth": 20, "indentStyle": "space" },
  "overrides": [
    { "include": ["test.js"], "linter": { "enabled": false } }
  ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, FORMATTED_LINE_WITH_SPACES);
    assert_file_contents(&fs, test2, FORMATTED_LINE_WITH_SPACES);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_change_formatting_settings",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_change_formatting_language_settings() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
        "javascript": { "formatter": { "quoteStyle": "single" } },
  "overrides": [
    { "include": ["test.js"], "linter": { "enabled": false } }
  ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, FORMATTED_WITH_SINGLE_QUOTES);
    assert_file_contents(&fs, test2, FORMATTED_WITH_SINGLE_QUOTES);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_change_formatting_language_settings",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_change_formatting_language_settings_2() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
    "javascript": { "formatter": { "lineWidth": 20 } },
  "overrides": [
    { "include": ["test.js"], "linter": { "enabled": false } }
  ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, FORMATTED_LINE_WIDTH_OVERRIDDEN);
    assert_file_contents(&fs, test2, FORMATTED_LINE_WIDTH_OVERRIDDEN);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_change_formatting_language_settings_2",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_conceal_previous_overrides() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "javascript": { "formatter": { "quoteStyle": "single" } },
  "overrides": [
    { "include": ["*.js"], "javascript": { "formatter": { "quoteStyle": "double" } } },
    { "include": ["test.js"], "javascript": { "formatter": { "indentWidth": 4 } } }
  ]
}"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "format",
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_conceal_previous_overrides",
        fs,
        console,
        result,
    ));
}

#[test]
fn takes_last_formatter_enabled_into_account() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "overrides": [
                {
                    "include": ["*.js"],
                    "formatter": { "enabled": false }
                }, {
                    "include": ["*.js"],
                    "formatter": { "enabled": true }
                }
            ]
        }"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(["format", test.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "takes_last_formatter_enabled_into_account",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_override_well_known_special_files_when_config_override_is_present() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "overrides": [
                {
                    "include": [
                        "**/*.json"
                    ],
                    "formatter": { "enabled": false }
                }
            ]
        }"#
        .as_bytes(),
    );

    let tsconfig = Path::new("tsconfig.json");
    fs.insert(
        tsconfig.into(),
        r#"{
    // This is a comment
    "compilerOptions": {},
}"#,
    );

    let other_json = Path::new("other.json");
    fs.insert(
        other_json.into(),
        r#"{
    "asta": ["lorem", "ipsum", "first", "second"]
}"#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "check",
                other_json.as_os_str().to_str().unwrap(),
                tsconfig.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_override_well_known_special_files_when_config_override_is_present",
        fs,
        console,
        result,
    ));
}

#[test]
fn allow_trailing_commas_on_well_known_files() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "formatter": {
                "indentStyle": "space",
                "indentWidth": 4
            },
            "overrides": [
                {
                    "include": [
                        "**/*.json"
                    ],
                    "json": { "parser": { "allowTrailingCommas": true } }
                }
            ]
        }"#
        .as_bytes(),
    );

    let tsconfig = Path::new("tsconfig.json");
    fs.insert(
        tsconfig.into(),
        r#"{
    // This is a comment
    "compilerOptions": {},
}"#,
    );

    let vscode_settings = Path::new(".vscode/settings.json");
    fs.insert(
        vscode_settings.into(),
        r#"{
    // This is a comment
    "editor.rulers": [80, 100],
}"#,
    );

    let other_json = Path::new("other.json");
    fs.insert(
        other_json.into(),
        r#"{
    "asta": ["lorem", "ipsum", "first", "second"],
}"#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                "check",
                other_json.as_os_str().to_str().unwrap(),
                tsconfig.as_os_str().to_str().unwrap(),
                vscode_settings.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "allow_trailing_commas_on_well_known_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn disallow_comments_on_well_known_files() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "formatter": {
                "indentStyle": "space",
                "indentWidth": 4
            },
            "overrides": [
                {
                    "include": [
                        "**/*.json"
                    ],
                    "json": { "parser": { "allowComments": false } }
                }
            ]
        }"#
        .as_bytes(),
    );

    let tsconfig = Path::new("tsconfig.json");
    fs.insert(
        tsconfig.into(),
        r#"{
    // This is a comment
    "compilerOptions": {}
}"#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(["check", tsconfig.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "disallow_comments_on_well_known_files",
        fs,
        console,
        result,
    ));
}
