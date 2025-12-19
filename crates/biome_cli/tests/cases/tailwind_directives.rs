use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn should_parse_tailwind_directive() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_content = r#"@theme {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
    "formatter": {
        "enabled": true
    },
    "css": {
        "parser": {
            "tailwindDirectives": true
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--css-formatter-enabled=true", css_file.as_str()].as_slice()),
    );

    // should format the file
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_parse_tailwind_directive",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_parse_tailwind_directive_when_disabled() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_content = r#"@theme {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
    "formatter": {
        "enabled": true
    },
    "css": {
        "parser": {
            "tailwindDirectives": false
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", css_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_parse_tailwind_directive_when_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn tw_should_not_show_unknown_at_rule_diagnostic() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_content = r#"@theme {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
    "linter": {
        "enabled": true,
        "rules": {
            "recommended": false,
            "suspicious": {
                "noUnknownAtRules": "warn"
            }
        }
    },
    "css": {
        "parser": {
            "tailwindDirectives": true
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", css_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "tw_should_not_show_unknown_at_rule_diagnostic",
        fs,
        console,
        result,
    ));
}
