use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "includes": ["test.js", "special/**"]
  },
  "overrides": [{ "includes": ["special/**"], "formatter": { "enabled": false } }]
}

"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Utf8Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test.as_str(), test2.as_str()].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [{ "includes": ["special/**"], "formatter": { "lineWidth": 20 } }]
}

"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Utf8Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test.as_str(), test2.as_str()].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "includes": ["special/**"], "formatter": { "lineWidth": 130 } },
    { "includes": ["special/**"], "formatter": { "lineWidth": 20 } }
   ]
}

"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Utf8Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test.as_str(), test2.as_str()].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "includes": ["test.js"], "formatter": { "lineWidth": 20 } },
    { "includes": ["test2.js"], "formatter": { "lineWidth": 20, "indentStyle": "space" } }
   ]
}

"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test.as_str(), test2.as_str()].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
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
    { "includes": ["formatted.js"], "formatter": { "enabled": true } },
    {
      "includes": ["dirty.js"],
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

    let formatted = Utf8Path::new("formatted.js");
    fs.insert(formatted.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let unformatted = Utf8Path::new("dirty.js");
    fs.insert(unformatted.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                formatted.as_str(),
                unformatted.as_str(),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "includes": ["test.js"], "formatter": { "lineWidth": 120 }, "javascript": { "formatter": { "quoteStyle": "single" } } },
    { "includes": ["test2.js"], "formatter": { "lineWidth": 120, "indentStyle": "space" }, "javascript": { "formatter": { "semicolons": "asNeeded" } } },
    { "includes": ["test.css"], "formatter": { "lineWidth": 120, "indentStyle": "space" }, "css": { "formatter": { "quoteStyle": "single" } } }
   ]
}
"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());
    let test_css = Utf8Path::new("test.css");
    fs.insert(test_css.into(), CSS_UNFORMATTED_QUOTES.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                test.as_str(),
                test2.as_str(),
                test_css.as_str(),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "includes": ["test.js"], "formatter": { "lineWidth": 120 }, "javascript": { "formatter": { "quoteStyle": "single" } } },
    {
        "includes": ["test2.js"],
        "formatter": { "lineWidth": 120, "indentStyle": "space" },
        "javascript": { "formatter": { "semicolons": "asNeeded" } },
        "json": { "formatter": { "indentStyle": "space", "lineWidth": 20, "indentWidth": 4 } }
    },
    {
        "includes": ["test3.json"],
        "formatter": { "lineWidth": 120, "indentStyle": "space" },
        "json": { "formatter": { "indentStyle": "space", "lineWidth": 20, "indentWidth": 4 } }
    }
  ]
}

"#
            .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let json_file = Utf8Path::new("test3.json");
    fs.insert(json_file.into(), UNFORMATTED_JSON.as_bytes());

    let css_file = Utf8Path::new("test4.css");
    fs.insert(css_file.into(), UNFORMATTED_CSS.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                test.as_str(),
                test2.as_str(),
                json_file.as_str(),
                css_file.as_str(),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
        "formatter": { "lineWidth": 20, "indentStyle": "space" },
  "overrides": [
    { "includes": ["test.js"], "linter": { "enabled": false } }
  ]
}

"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test.as_str(), test2.as_str()].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
        "javascript": { "formatter": { "quoteStyle": "single" } },
  "overrides": [
    { "includes": ["test.js"], "linter": { "enabled": false } }
  ]
}

"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test.as_str(), test2.as_str()].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
    "javascript": { "formatter": { "lineWidth": 20 } },
  "overrides": [
    { "includes": ["test.js"], "linter": { "enabled": false } }
  ]
}

"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", test.as_str(), test2.as_str()].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "javascript": { "formatter": { "quoteStyle": "single" } },
  "overrides": [
    { "includes": ["*.js"], "javascript": { "formatter": { "quoteStyle": "double" } } },
    { "includes": ["test.js"], "javascript": { "formatter": { "indentWidth": 4 } } }
  ]
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", test.as_str(), test2.as_str()].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "overrides": [
                {
                    "includes": ["*.js"],
                    "formatter": { "enabled": false }
                }, {
                    "includes": ["*.js"],
                    "formatter": { "enabled": true }
                }
            ]
        }"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", test.as_str()].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "overrides": [
                {
                    "includes": [
                        "**/*.json"
                    ],
                    "formatter": { "enabled": false }
                }
            ]
        }"#
        .as_bytes(),
    );

    let tsconfig = Utf8Path::new("tsconfig.json");
    fs.insert(
        tsconfig.into(),
        r#"{
    // This is a comment
    "compilerOptions": {},
}"#,
    );

    let other_json = Utf8Path::new("other.json");
    fs.insert(
        other_json.into(),
        r#"{
    "asta": ["lorem", "ipsum", "first", "second"]
}"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", other_json.as_str(), tsconfig.as_str()].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "formatter": {
                "indentStyle": "space",
                "indentWidth": 4
            },
            "overrides": [
                {
                    "includes": [
                        "**/*.json"
                    ],
                    "json": { "parser": { "allowTrailingCommas": true } }
                }
            ]
        }"#
        .as_bytes(),
    );

    let tsconfig = Utf8Path::new("tsconfig.json");
    fs.insert(
        tsconfig.into(),
        r#"{
    // This is a comment
    "compilerOptions": {},
}"#,
    );

    let vscode_settings = Utf8Path::new(".vscode/settings.json");
    fs.insert(
        vscode_settings.into(),
        r#"{
    // This is a comment
    "editor.rulers": [80, 100],
}"#,
    );

    let vscode_text_file = Utf8Path::new(".vscode/any.text");
    fs.insert(vscode_text_file.into(), "any text");

    let other_json = Utf8Path::new("other.json");
    fs.insert(
        other_json.into(),
        r#"{
    "asta": ["lorem", "ipsum", "first", "second"],
}"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", other_json.as_str(), tsconfig.as_str(), ".vscode/"].as_slice()),
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
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "formatter": {
                "indentStyle": "space",
                "indentWidth": 4
            },
            "overrides": [
                {
                    "includes": [
                        "**/*.json"
                    ],
                    "json": { "parser": { "allowComments": false } }
                }
            ]
        }"#
        .as_bytes(),
    );

    let tsconfig = Utf8Path::new("tsconfig.json");
    fs.insert(
        tsconfig.into(),
        r#"{
    // This is a comment
    "compilerOptions": {}
}"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", tsconfig.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "disallow_comments_on_well_known_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn overrides_default_formatter_for_package_json() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let biome_json = Utf8Path::new("biome.json");
    fs.insert(
        biome_json.into(),
        r#"{
            "overrides": [
                {
                    "includes": ["package.json"],
                    "json": { "formatter": { "expand": "followSource" } }
                }
            ]
        }"#,
    );
    let file_path = Utf8Path::new("package.json");
    fs.insert(
        file_path.into(),
        r#"{ "name": "foo", "dependencies": { "foo": "latest" } }"#.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "overrides_default_formatter_for_package_json",
        fs,
        console,
        result,
    ));
}

#[test]
fn overrides_grit_formatting_options() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "formatter": {
                "indentStyle": "tab"
            },
            "overrides": [
                {
                    "includes": [
                        "file.grit"
                    ],
                    "grit": { "formatter": { "indentStyle": "space", "indentWidth": 8 }  }
                }
            ]
        }"#
        .as_bytes(),
    );

    let js_file = Utf8Path::new("file.js");
    fs.insert(js_file.into(), r#"function name() { return "hello"; }"#);

    let grit_file = Utf8Path::new("file.grit");
    fs.insert(
        grit_file.into(),
        r#"`console.$_($content)` where { $content <: contains `secret` until `sanitized($_)` }"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", js_file.as_str(), grit_file.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "overrides_grit_formatting_options",
        fs,
        console,
        result,
    ));
}
