use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use crate::{run_cli, run_cli_with_dyn_fs};
use biome_console::BufferConsole;
use biome_fs::{MemoryFileSystem, TemporaryFs};
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

// ── noUndeclaredStyles ────────────────────────────────────────────────────────

const BIOME_JSON_HTML_LINTER: &str = r#"{ "linter": { "enabled": true }, "html": { "linter": { "enabled": true }, "experimentalFullSupportEnabled": true } }"#;

/// Undeclared classes are flagged in several scenarios: a single unknown class
/// in a `<style>` block, a mix of known/unknown in a multi-class string,
/// unknown classes across multiple `<style>` blocks, unknown classes on
/// different element types, and a class absent from a linked stylesheet.
#[test]
fn no_undeclared_styles_reports_undeclared_classes() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_undeclared_styles_reports_undeclared_classes");

    fs.create_file("biome.json", BIOME_JSON_HTML_LINTER);
    // .card is declared; .header is not → flagged
    // .title is declared; .footer is not → flagged
    // Classes span two separate <style> blocks; .unknown is also flagged
    // external stylesheet declares .hero; .missing-class is not → flagged
    fs.create_file("styles.css", ".hero { font-size: 2rem; }");
    fs.create_file(
        "file.html",
        r#"<style>.card { border: 1px solid; } .title { font-weight: bold; }</style>
<style>.container { max-width: 1200px; } .btn { padding: 8px; }</style>
<div class="card header">Single unknown</div>
<div class="card header title footer">Multi unknown</div>
<div class="header footer unknown">Multi-block unknown</div>
<body class="container">
  <main class="main-content">
    <button class="btn unknown-btn">Element types</button>
  </main>
</body>
"#,
    );
    fs.create_file(
        "index.html",
        r#"<!DOCTYPE html>
<html>
<head><link rel="stylesheet" href="./styles.css" /></head>
<body><h1 class="hero missing-class">Linked stylesheet</h1></body>
</html>
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUndeclaredStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_undeclared_styles_reports_undeclared_classes",
        fs.create_mem(),
        console,
        result,
    ));
}

/// Declared classes produce no diagnostics across all supported declaration
/// sources: inline `<style>` block, pseudo-selector variants (`:hover`,
/// `::before`), and a linked external stylesheet.
#[test]
fn no_undeclared_styles_passes_when_declared() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_undeclared_styles_passes_when_declared");

    fs.create_file("biome.json", BIOME_JSON_HTML_LINTER);
    fs.create_file("styles.css", ".hero { font-size: 2rem; }");
    fs.create_file(
        "file.html",
        r#"<style>.card { border: 1px solid; } .btn:hover { color: darkblue; } .btn::before { content: ''; }</style>
<div class="card">Inline style</div>
<button class="btn">Pseudo-selector declared</button>
"#,
    );
    fs.create_file(
        "index.html",
        r#"<!DOCTYPE html>
<html>
<head><link rel="stylesheet" href="./styles.css" /></head>
<body><h1 class="hero">Linked stylesheet</h1></body>
</html>
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUndeclaredStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_undeclared_styles_passes_when_declared",
        fs.create_mem(),
        console,
        result,
    ));
}

/// A file with no `<style>` blocks and no linked stylesheets must never emit
/// diagnostics — the rule silently passes to avoid false positives.
#[test]
fn no_undeclared_styles_silent_without_style_info() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_undeclared_styles_silent_without_style_info");

    fs.create_file("biome.json", BIOME_JSON_HTML_LINTER);
    fs.create_file("file.html", r#"<div class="anything">Content</div>"#);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUndeclaredStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_undeclared_styles_silent_without_style_info",
        fs.create_mem(),
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
