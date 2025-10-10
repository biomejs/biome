use crate::snap_test::SnapshotPayload;
use crate::{assert_cli_snapshot, run_cli};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

// Tests for --css-parse-css-modules flag

#[test]
fn check_css_parse_css_modules_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.module.css");
    // CSS Modules specific syntax
    fs.insert(
        file_path.into(),
        ".className { composes: other from './other.module.css'; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--css-parse-css-modules=true", file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_css_parse_css_modules_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_css_parse_css_modules_false() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.module.css");
    // CSS Modules specific syntax
    fs.insert(
        file_path.into(),
        ".className { composes: other from './other.module.css'; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--css-parse-css-modules=false", file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_css_parse_css_modules_false",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_css_parse_css_modules_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.module.css");
    // CSS Modules specific syntax
    fs.insert(
        file_path.into(),
        ".className { composes: other from './other.module.css'; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--css-parse-css-modules=true", file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_css_parse_css_modules_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_css_parse_css_modules_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.module.css");
    // CSS Modules specific syntax
    fs.insert(
        file_path.into(),
        ".className { composes: other from './other.module.css'; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--css-parse-css-modules=true", file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_css_parse_css_modules_true",
        fs,
        console,
        result,
    ));
}

// Tests for --css-parse-tailwind-directives flag

#[test]
fn check_css_parse_tailwind_directives_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.css");
    // Tailwind CSS 4.0 directive
    fs.insert(
        file_path.into(),
        "@import 'tailwindcss';\n.foo { color: red; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--css-parse-tailwind-directives=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_css_parse_tailwind_directives_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_css_parse_tailwind_directives_false() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.css");
    // Tailwind CSS 4.0 directive
    fs.insert(
        file_path.into(),
        "@import 'tailwindcss';\n.foo { color: red; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--css-parse-tailwind-directives=false",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_css_parse_tailwind_directives_false",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_css_parse_tailwind_directives_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.css");
    // Tailwind CSS 4.0 directive
    fs.insert(
        file_path.into(),
        "@import 'tailwindcss';\n.foo { color: red; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--css-parse-tailwind-directives=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_css_parse_tailwind_directives_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_css_parse_tailwind_directives_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.css");
    // Tailwind CSS 4.0 directive
    fs.insert(
        file_path.into(),
        "@import 'tailwindcss';\n.foo { color: red; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "ci",
                "--css-parse-tailwind-directives=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_css_parse_tailwind_directives_true",
        fs,
        console,
        result,
    ));
}

// Combined tests

#[test]
fn check_combined_css_parser_flags() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.module.css");
    // CSS Modules with Tailwind directives
    fs.insert(
        file_path.into(),
        "@import 'tailwindcss';\n.className { composes: other from './other.module.css'; }"
            .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--css-parse-css-modules=true",
                "--css-parse-tailwind-directives=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_combined_css_parser_flags",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_combined_format_with_errors_and_css_modules() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.module.css");
    // CSS Modules with syntax error
    fs.insert(
        file_path.into(),
        ".className { composes: other from './other.module.css';".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--format-with-errors=true",
                "--css-parse-css-modules=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_combined_format_with_errors_and_css_modules",
        fs,
        console,
        result,
    ));
}

// Config override tests

#[test]
fn check_css_parser_flags_override_config() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Config that disables CSS Modules
    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "css": {
    "parser": {
      "cssModules": false
    }
  }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.module.css");
    fs.insert(
        file_path.into(),
        ".className { composes: other from './other.module.css'; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--css-parse-css-modules=true", file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_css_parser_flags_override_config",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_css_parse_respects_config_css_modules() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Config that enables CSS Modules
    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "css": {
    "parser": {
      "cssModules": true
    }
  }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.module.css");
    fs.insert(
        file_path.into(),
        ".className { composes: other from './other.module.css'; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_css_parse_respects_config_css_modules",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_css_parse_respects_config_tailwind_directives() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Config that enables Tailwind directives
    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "css": {
    "parser": {
      "tailwindDirectives": true
    }
  }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.css");
    fs.insert(
        file_path.into(),
        "@import 'tailwindcss';\n.foo { color: red; }".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_css_parse_respects_config_tailwind_directives",
        fs,
        console,
        result,
    ));
}
