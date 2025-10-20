mod cases;
mod commands;
mod configs;
#[cfg(test)]
mod snap_test;

#[cfg(test)]
use snap_test::assert_cli_snapshot;

use biome_cli::{BiomeCli, BiomeCommand, CliDiagnostic, CliSession};
use biome_console::{BufferConsole, Console, ConsoleExt, markup};
use biome_fs::{MemoryFileSystem, OsFileSystem};
use biome_resolver::FsWithResolverProxy;
use biome_service::App;
use clap::Parser;
use std::sync::Arc;

const UNFORMATTED: &str = "  statement(  )  ";
const FORMATTED: &str = "statement();\n";

const PARSE_ERROR: &str = "if\n";
const LINT_ERROR: &str = "for(;true;);\n";

const CUSTOM_FORMAT_BEFORE: &str = r#"
function f() {
return { something }
}
"#;

mod help {
    use super::*;

    #[test]
    fn unknown_command() {
        let mut console = BufferConsole::default();
        let fs = MemoryFileSystem::default();

        let (_, result) = run_cli(fs, &mut console, &["unknown", "--help"]);

        assert!(result.is_ok(), "run_cli returned {result:?}");
    }
}

mod main {
    use super::*;

    #[test]
    fn unknown_command() {
        let mut console = BufferConsole::default();
        let fs = MemoryFileSystem::default();

        let (_, result) = run_cli(fs, &mut console, &["unknown"]);
        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn unexpected_argument() {
        let mut console = BufferConsole::default();
        let fs = MemoryFileSystem::default();

        let (_, result) = run_cli(fs, &mut console, &["format", "--unknown", "file.js"]);

        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn empty_arguments() {
        let mut console = BufferConsole::default();
        let fs = MemoryFileSystem::default();

        let (_, result) = run_cli(fs, &mut console, &["format"]);

        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn missing_argument() {
        let mut console = BufferConsole::default();
        let fs = MemoryFileSystem::default();

        let (_, result) = run_cli(fs, &mut console, &["format", "--write"]);
        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn incorrect_value() {
        let mut console = BufferConsole::default();
        let fs = MemoryFileSystem::default();

        let (_, result) = run_cli(fs, &mut console, &["check", "--max-diagnostics=foo"]);

        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn overflow_value() {
        let mut console = BufferConsole::default();
        let fs = MemoryFileSystem::default();

        let (_, result) = run_cli(fs, &mut console, &["check", "--max-diagnostics=500"]);

        assert!(result.is_err(), "run_cli returned {result:?}");
    }
    //
    // #[test]
    // fn no_colors() {
    //     let mut args = ["--colors=off"]);
    //     let result = color_from_arguments(&mut args);
    //
    //     assert!(result.is_ok(), "run_cli returned {result:?}");
    // }
    //
    // #[test]
    // fn force_colors() {
    //     let mut args = ["--colors=force"]);
    //     let result = color_from_arguments(&mut args);
    //
    //     assert!(result.is_ok(), "run_cli returned {result:?}");
    // }
    //
    // #[test]
    // fn invalid_colors() {
    //     let mut args = ["--colors=other"]);
    //     let result = color_from_arguments(&mut args);
    //     assert!(result.is_err(), "run_cli returned {result:?}");
    // }
}

mod configuration {
    use super::*;
    use crate::configs::{
        CONFIG_ALL_FIELDS, CONFIG_BAD_LINE_WIDTH, CONFIG_INCORRECT_GLOBALS,
        CONFIG_INCORRECT_GLOBALS_V2, CONFIG_LINTER_WRONG_RULE,
    };
    use crate::snap_test::SnapshotPayload;
    use biome_console::BufferConsole;
    use biome_fs::MemoryFileSystem;

    use camino::Utf8Path;

    #[test]
    fn correct_root() {
        let fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();
        let file_path = Utf8Path::new("biome.json");
        fs.insert(file_path.into(), CONFIG_ALL_FIELDS.as_bytes());

        let (fs, result) = run_cli(fs, &mut console, &["format", "file.js"]);

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "correct_root",
            fs,
            console,
            result,
        ));
    }

    #[test]
    fn line_width_error() {
        let fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Utf8Path::new("biome.json");
        fs.insert(file_path.into(), CONFIG_BAD_LINE_WIDTH.as_bytes());

        let (fs, result) = run_cli(fs, &mut console, &["format", "file.js"]);

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "line_width_error",
            fs,
            console,
            result,
        ));
    }

    #[test]
    fn incorrect_rule_name() {
        let fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Utf8Path::new("biome.json");
        fs.insert(file_path.into(), CONFIG_LINTER_WRONG_RULE.as_bytes());

        let (fs, result) = run_cli(fs, &mut console, &["check", "file.js"]);

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "incorrect_rule_name",
            fs,
            console,
            result,
        ));
    }

    #[test]
    fn incorrect_globals() {
        let fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Utf8Path::new("biome.json");
        fs.insert(file_path.into(), CONFIG_INCORRECT_GLOBALS.as_bytes());

        let (fs, result) = run_cli(fs, &mut console, &["check", "file.js"]);

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "incorrect_globals",
            fs,
            console,
            result,
        ));
    }

    #[test]
    fn ignore_globals() {
        let fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        fs.insert(
            Utf8Path::new("biome.json").into(),
            CONFIG_INCORRECT_GLOBALS_V2.as_bytes(),
        );
        fs.insert(Utf8Path::new("file.js").into(), UNFORMATTED.as_bytes());

        let (_, result) = run_cli(fs, &mut console, &["check", "file.js"]);

        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn override_globals() {
        let fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        fs.insert(
            Utf8Path::new("biome.json").into(),
            r#"{
                "linter": {
                    "enabled": true,
                    "rules": {
                        "correctness": {
                            "noUndeclaredVariables": "error"
                        }
                    }
                },
                "javascript": {
                    "globals": ["React"]
                },
                "overrides": [{
                    "includes": ["tests/**"],
                    "javascript": {
                        "globals": ["test", "it"]
                    }
                }]
            }"#
            .as_bytes(),
        );
        fs.insert(
            Utf8Path::new("tests/test.js").into(),
            r#"test("globals", () => {
    it("uses React", () => {
        React.useMemo();
    });
});"#
                .as_bytes(),
        );

        let (fs, result) = run_cli(fs, &mut console, &["lint", "tests/test.js"]);

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "override_globals",
            fs,
            console,
            result,
        ));
    }
}

/// Create an [App] instance using the provided [FileSystem] and [Console]
/// instance, and using an in-process "remote" instance of the workspace
pub(crate) fn run_cli(
    fs: MemoryFileSystem,
    console: &mut dyn Console,
    args: &[&str],
) -> (MemoryFileSystem, Result<(), CliDiagnostic>) {
    let files = fs.files.clone();

    let result = run_cli_with_dyn_fs(Box::new(fs), console, args);

    // This is a little bit of a workaround to allow us to easily create
    // a snapshot of the files even though the original file system was
    // consumed by the workspace.
    let fs = MemoryFileSystem::from_files(files);

    (fs, result)
}

/// Create an [App] instance using the provided [FileSystem] and [Console]
/// instance, and using an in-process "remote" instance of the workspace
pub(crate) fn run_cli_with_dyn_fs(
    fs: Box<dyn FsWithResolverProxy>,
    console: &mut dyn Console,
    args: &[&str],
) -> Result<(), CliDiagnostic> {
    use biome_cli::SocketTransport;
    use biome_lsp::ServerFactory;
    use biome_service::{WorkspaceRef, workspace};
    use tokio::{
        io::{duplex, split},
        runtime::Runtime,
    };

    let factory = ServerFactory::new_with_fs(Arc::new(OsFileSystem::new(
        fs.working_directory().unwrap_or_default(),
    )));
    let connection = factory.create();

    let runtime = Runtime::new().expect("failed to create runtime");

    let (client, server) = duplex(4096);
    let (stdin, stdout) = split(server);
    runtime.spawn(connection.accept(stdin, stdout));

    let (client_read, client_write) = split(client);
    let transport = SocketTransport::open(runtime, client_read, client_write);

    let workspace = workspace::client(transport, fs).unwrap();
    let app = App::new(console, WorkspaceRef::Owned(workspace));

    let mut session = CliSession { app };
    let cli = BiomeCli::try_parse_from(args);
    match cli {
        Ok(cli) => session.run(cli),
        Err(failure) => {
            if let clap::error::ErrorKind::DisplayHelp = failure.kind() {
                let console = &mut session.app.console;
                console.log(markup! {{failure.to_string()}});
                Ok(())
            } else {
                Err(CliDiagnostic::parse_error_clap(failure))
            }
        }
    }
}

/// Create an [App] instance using the provided [FileSystem] and [Console]
/// instance, and using an in-process server instance of the workspace
pub(crate) fn run_cli_with_server_workspace(
    fs: MemoryFileSystem,
    console: &mut dyn Console,
    args: &[&str],
) -> (MemoryFileSystem, Result<(), CliDiagnostic>) {
    use biome_service::{WorkspaceRef, workspace};

    let files = fs.files.clone();

    let workspace = workspace::server(Arc::new(fs), None);
    let app = App::new(console, WorkspaceRef::Owned(workspace));

    let mut session = CliSession { app };
    let cli = BiomeCli::try_parse_from(args);
    let result = match cli {
        Ok(cli) => session.run(cli),
        Err(failure) => {
            if let clap::error::ErrorKind::DisplayHelp = failure.kind() {
                let console = &mut session.app.console;
                console.log(markup! {{failure.to_string()}});
                Ok(())
            } else {
                Err(CliDiagnostic::parse_error_clap(failure))
            }
        }
    };

    // This is a little bit of a workaround to allow us to easily create
    // a snapshot of the files even though the original file system was
    // consumed by the workspace.
    let fs = MemoryFileSystem::from_files(files);

    (fs, result)
}
