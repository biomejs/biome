use crate::run_cli;
use crate::snap_test::{CliSnapshot, SnapshotPayload};
use bpaf::Args;
use rome_cli::CliDiagnostic;
use rome_console::{BufferConsole, Console};
use rome_fs::{FileSystem, MemoryFileSystem};
use rome_service::DynRef;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use std::{env, fs};

#[test]
fn ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_rage(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("rage")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "rage_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn with_configuration() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Path::new("rome.json").to_path_buf(),
        r#"{
  "formatter": {
    "enabled": false
  }
}"#,
    );

    let result = run_rage(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("rage")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn with_malformed_configuration() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Path::new("rome.json").to_path_buf(),
        r#"{
  "formatter": {
    "enabled":
  }
}"#,
    );

    let result = run_rage(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("rage")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_malformed_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn with_server_logs() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = {
        let log_dir = TestLogDir::new("rome-test-logs");
        fs::create_dir_all(&log_dir.path).expect("Failed to create test log directory");

        fs::write(log_dir.path.join("server.log.2022-10-14-16"), r#"
┐rome_cli::commands::daemon::Running Server{pid=195434}
├─2547ms INFO rome_lsp::server Starting Rome Language Server...
├─15333ms INFO rome_lsp::server Starting Rome Language Server...
├─15347ms INFO rome_lsp::server Attempting to load the configuration from 'rome.json' file
├─15347ms INFO rome_service::configuration Attempting to load the configuration file at path "/home/micha/git/ant-design/rome.json"
├─15347ms ERROR rome_service::configuration Could not find the file configuration at "/home/micha/git/ant-design/rome.json"
├─15347ms ERROR rome_service::configuration Reason: Os { code: 2, kind: NotFound, message: "No such file or directory" }
├─┐rome_js_parser::parse::parse{file_id=FileId(0)}
├─┘
├─┐rome_js_parser::parse::parse{file_id=FileId(1)}
├─┘
├─16108ms INFO rome_lsp::server Starting Rome Language Server...
├─41801ms INFO rome_lsp::server Starting Rome Language Server...
├─41802ms INFO rome_lsp::server Sending shutdown signal
INFO rome_cli::commands::daemon Received shutdown signal
├─41802ms ERROR tower_lsp::transport failed to encode message: failed to encode response: Broken pipe (os error 32)
┘
┐rome_cli::commands::daemon::Running Server{pid=197796}
├─2822ms INFO rome_lsp::server Starting Rome Language Server...
├─7550ms INFO rome_lsp::server Starting Rome Language Server...
├─7551ms INFO rome_lsp::server Attempting to load the configuration from 'rome.json' file
├─7551ms INFO rome_service::configuration Attempting to load the configuration file at path "/home/micha/git/ant-design/rome.json"
├─7551ms ERROR rome_service::configuration Could not find the file configuration at "/home/micha/git/ant-design/rome.json"
├─7551ms ERROR rome_service::configuration Reason: Os { code: 2, kind: NotFound, message: "No such file or directory" }
├─┐rome_js_parser::parse::parse{file_id=FileId(0)}
├─┘
├─┐rome_js_parser::parse::parse{file_id=FileId(1)}
├─┘
├─7897ms INFO rome_lsp::server Starting Rome Language Server...
"#,
        ).expect("Failed to write log file");

        fs::write(
            log_dir.path.join("server.log.2022-10-14-15"),
            r#"
Not most recent log file
"#,
        )
        .expect("Failed to write configuration file");

        run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from([("rage")].as_slice()),
        )
    };

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_server_logs",
        fs,
        console,
        result,
    ));
}

/// Runs the `rage` command mocking out the log directory.
fn run_rage<'app>(
    fs: DynRef<'app, dyn FileSystem>,
    console: &'app mut dyn Console,
    args: Args,
) -> Result<(), CliDiagnostic> {
    let _test_dir = TestLogDir::new("rome-rage-test");
    run_cli(fs, console, args)
}

fn assert_rage_snapshot(payload: SnapshotPayload<'_>) {
    let test_name = payload.test_name;
    let module_path = payload.module_path;

    let mut snapshot = CliSnapshot::from(payload);

    // Replace any platform specific content that may yield unstable results.
    for message in snapshot.messages.iter_mut() {
        *message = message
            .lines()
            .map(|line| match line.trim_start().split_once(':') {
                Some((
                    "CPU Architecture" | "OS" | "NO_COLOR" | "TERM" | "BIOME_LOG_DIR"
                    | "Color support",
                    value,
                )) => line.replace(value.trim_start(), "**PLACEHOLDER**"),
                _ => line.to_string(),
            })
            .collect::<Vec<_>>()
            .join("\n");
    }

    let content = snapshot.emit_content_snapshot();

    let module_path = module_path.replace("::", "_");
    let snapshot_path = PathBuf::from("../snapshots").join(module_path);

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => snapshot_path
    }, {
        insta::assert_snapshot!(test_name, content);

    });
}

/// Mutex to guarantee that the `rage` tests run sequentially. Necessary to avoid race conditions
/// when reading the server logs.
static RAGE_GUARD: Mutex<()> = Mutex::new(());

/// Mocks out the directory from which `rage` reads the server logs. Ensures that the test directory
/// gets removed at the end of the test.
struct TestLogDir {
    path: PathBuf,
    _guard: MutexGuard<'static, ()>,
}

impl TestLogDir {
    fn new(name: &str) -> Self {
        let guard = RAGE_GUARD.lock().unwrap();
        let path = env::temp_dir().join(name);

        env::set_var("BIOME_LOG_DIR", &path);

        Self {
            path,
            _guard: guard,
        }
    }
}

impl Drop for TestLogDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).ok();
        env::remove_var("BIOME_LOG_DIR");
    }
}
