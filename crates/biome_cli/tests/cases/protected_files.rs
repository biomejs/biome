use crate::TestArgs as Args;
use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, markup_to_string};
use biome_console::{BufferConsole, markup};
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

#[test]
fn not_process_file_from_stdin_format() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(r#"{ "name": "test" }"#.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path=package-lock.json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_stdin_format",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_file_from_stdin_lint() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(r#"{ "name": "test" }"#.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--stdin-file-path=package.json"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_stdin_lint",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_file_from_stdin_verbose_format() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(r#"{ "name": "test" }"#.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--verbose", "--stdin-file-path=package-lock.json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_stdin_verbose_format",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_file_from_stdin_verbose_lint() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console.in_buffer.push(r#"{ "name": "test" }"#.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--verbose", "--stdin-file-path=package-lock.json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_stdin_verbose_lint",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_file_from_cli() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("package-lock.json");
    fs.insert(file_path.into(), r#"{ "name": "test" }"#.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_file_from_cli_verbose() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("package-lock.json");
    fs.insert(file_path.into(), r#"{ "name": "test" }"#.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--verbose", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_from_cli_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn pnpm_lockfiles_not_processed_from_cli() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for path in ["pnpm-lock.yaml", "packages/app/pnpm-lock.yaml"] {
        fs.insert(path.into(), b"lockfileVersion:    '9.0'\n");
    }

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--verbose",
                "pnpm-lock.yaml",
                "packages/app/pnpm-lock.yaml",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "pnpm_lockfiles_not_processed_from_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_ignored_file_from_cli_verbose() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("package.json");
    fs.insert(file_path.into(), r#"{ "name": "test" }"#.as_bytes());

    let file_path = Utf8Path::new("other.json");
    fs.insert(file_path.into(), r#"{}"#.as_bytes());

    let file_path1 = Utf8Path::new("biome.json");
    fs.insert(
        file_path1.into(),
        r#"{ "files": { "includes": ["**", "!package.json"] } }"#.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--verbose", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_ignored_file_from_cli_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn not_process_file_linter_disabled_from_cli_verbose() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("package.json");
    fs.insert(file_path.into(), r#"{ "name": "test" }"#.as_bytes());

    let file_path = Utf8Path::new("other.json");
    fs.insert(file_path.into(), r#"{}"#.as_bytes());

    let file_path1 = Utf8Path::new("biome.json");
    fs.insert(
        file_path1.into(),
        r#"{ "linter": { "enabled": false } }"#.as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--verbose", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "not_process_file_linter_disabled_from_cli_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_return_the_content_of_protected_files_via_stdin() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    console
        .in_buffer
        .push(r#"{ "name": "something" }"#.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path", "package-lock.json"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, r#"{ "name": "something" }"#);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_return_the_content_of_protected_files_via_stdin",
        fs,
        console,
        result,
    ));
}

#[test]
fn pnpm_lockfile_unchanged_from_stdin() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let source = "lockfileVersion:    '9.0'\n";
    console.in_buffer.push(source.to_string());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path=pnpm-lock.yaml"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");
    assert_eq!(markup_to_string(markup! { {message.content} }), source);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "pnpm_lockfile_unchanged_from_stdin",
        fs,
        console,
        result,
    ));
}
