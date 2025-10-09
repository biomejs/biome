use crate::snap_test::SnapshotPayload;
use crate::{assert_cli_snapshot, run_cli};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

// Tests for --json-parse-allow-comments flag

#[test]
fn check_json_parse_allow_comments_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.json");
    // JSON with comments
    fs.insert(
        file_path.into(),
        "{\n  // This is a comment\n  \"key\": \"value\"\n}".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--json-parse-allow-comments=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_json_parse_allow_comments_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_json_parse_allow_comments_false() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.json");
    // JSON with comments
    fs.insert(
        file_path.into(),
        "{\n  // This is a comment\n  \"key\": \"value\"\n}".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--json-parse-allow-comments=false",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_json_parse_allow_comments_false",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_json_parse_allow_comments_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.json");
    // JSON with comments
    fs.insert(
        file_path.into(),
        "{\n  // This is a comment\n  \"key\": \"value\"\n}".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--json-parse-allow-comments=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_json_parse_allow_comments_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_json_parse_allow_comments_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.json");
    // JSON with comments
    fs.insert(
        file_path.into(),
        "{\n  // This is a comment\n  \"key\": \"value\"\n}".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--json-parse-allow-comments=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_json_parse_allow_comments_true",
        fs,
        console,
        result,
    ));
}

// Tests for --json-parse-allow-trailing-commas flag

#[test]
fn check_json_parse_allow_trailing_commas_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.json");
    // JSON with trailing comma
    fs.insert(file_path.into(), "{\n  \"key\": \"value\",\n}".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--json-parse-allow-trailing-commas=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_json_parse_allow_trailing_commas_true",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_json_parse_allow_trailing_commas_false() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.json");
    // JSON with trailing comma
    fs.insert(file_path.into(), "{\n  \"key\": \"value\",\n}".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--json-parse-allow-trailing-commas=false",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_json_parse_allow_trailing_commas_false",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_json_parse_allow_trailing_commas_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.json");
    // JSON with trailing comma
    fs.insert(file_path.into(), "{\n  \"key\": \"value\",\n}".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--json-parse-allow-trailing-commas=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_json_parse_allow_trailing_commas_true",
        fs,
        console,
        result,
    ));
}

// Combined tests

#[test]
fn check_combined_json_parser_flags() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.json");
    // JSON with both comments and trailing comma
    fs.insert(
        file_path.into(),
        "{\n  // Comment\n  \"key\": \"value\",\n}".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--json-parse-allow-comments=true",
                "--json-parse-allow-trailing-commas=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_combined_json_parser_flags",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_json_parse_allow_comments_true() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.json");
    // JSON with comments
    fs.insert(
        file_path.into(),
        "{\n  // This is a comment\n  \"key\": \"value\"\n}".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--json-parse-allow-comments=true", file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_json_parse_allow_comments_true",
        fs,
        console,
        result,
    ));
}

// Config override tests

#[test]
fn check_json_parser_flags_override_config() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Config that disallows comments
    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "json": {
    "parser": {
      "allowComments": false
    }
  }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.json");
    fs.insert(
        file_path.into(),
        "{\n  // Comment\n  \"key\": \"value\"\n}".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--json-parse-allow-comments=true",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_json_parser_flags_override_config",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_json_parse_respects_config_allow_comments() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Config that allows comments
    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "json": {
    "parser": {
      "allowComments": true
    }
  }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.json");
    fs.insert(
        file_path.into(),
        "{\n  // Comment\n  \"key\": \"value\"\n}".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_json_parse_respects_config_allow_comments",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_json_parse_respects_config_allow_trailing_commas() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Config that allows trailing commas
    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
  "json": {
    "parser": {
      "allowTrailingCommas": true
    }
  }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.json");
    fs.insert(file_path.into(), "{\n  \"key\": \"value\",\n}".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file_path.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_json_parse_respects_config_allow_trailing_commas",
        fs,
        console,
        result,
    ));
}
