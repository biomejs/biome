use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn should_error_when_interpolation_is_disabled() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let html_file = Utf8Path::new("file.html");
    fs.insert(
        html_file.into(),
        r#"<div>{{ $interpolation }}</div>
"#
        .as_bytes(),
    );
    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
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
        Args::from(["format", html_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_error_when_interpolation_is_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_error_when_interpolation_is_enabled() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let html_file = Utf8Path::new("file.html");
    fs.insert(
        html_file.into(),
        r#"<div>{{ $interpolation }}</div>
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
        "parser": {
            "interpolation": true
        },
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
        Args::from(["format", html_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_error_when_interpolation_is_enabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_format_indent_embedded_languages() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let html_file = Utf8Path::new("file.html");
    fs.insert(
        html_file.into(),
        r#"<script>function lorem() { return "ipsum" }</script>
<style>#id .class div > p { background-color: red; align: center; padding: 0; } </style>
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
        "formatter": {
            "enabled": true,
            "indentScriptAndStyle": true
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", html_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_format_indent_embedded_languages",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_format_indent_embedded_languages_with_language_options() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let html_file = Utf8Path::new("file.html");
    fs.insert(
        html_file.into(),
        r#"<script>function lorem() { return "ipsum" }</script>
<style>#id .class div > p { background-color: red; align: center; padding: 0; } </style>
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
        "formatter": {
            "enabled": true,
            "indentScriptAndStyle": true
        }
    },
    "javascript": {
        "formatter": {
            "quoteStyle": "single"
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", html_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_format_indent_embedded_languages_with_language_options",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_pull_diagnostics_from_embedded_languages_when_formatting() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let html_file = Utf8Path::new("file.html");
    fs.insert(
        html_file.into(),
        r#"<script>function () { return "ipsum" }</script>
<style>#id .class div > { background-color: ; align: center; padding: 0; } </style>
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
        "formatter": {
            "enabled": true,
            "indentScriptAndStyle": true
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", html_file.as_str()].as_slice()),
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

    let html_file = Utf8Path::new("file.html");
    fs.insert(
        html_file.into(),
        r#"<script>debugger</script>
<script>import z from "zod"</script>
<style>#id .class div { background-color: red; background-color: red;  } </style>
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
        "formatter": {
            "enabled": true,
            "indentScriptAndStyle": true
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
        Args::from(["lint", html_file.as_str()].as_slice()),
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

    let html_file = Utf8Path::new("file.html");
    fs.insert(
        html_file.into(),
        r#"<script type="module">import z from "zod"; import _ from "lodash";

        debugger

        let schema = z.object({}).optional().nullable();

        </script>
<style>#id .class div { background-color: red; background-color: red;  } </style>
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
        "formatter": {
            "enabled": true,
            "indentScriptAndStyle": true
        },
        "linter": {
            "enabled": true
        },
        "assist": {
            "enabled": true
        }
    }
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--unsafe", html_file.as_str()].as_slice()),
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
fn should_lint_a_html_file() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let html_file = Utf8Path::new("file.html");
    fs.insert(
        html_file.into(),
        r#"<div scope="col"></div>
"#
        .as_bytes(),
    );

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
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
        Args::from(["lint", html_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_lint_a_html_file",
        fs,
        console,
        result,
    ));
}
