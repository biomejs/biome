use bpaf::Args;
use camino::Utf8Path;

use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;

use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};

/// Regression test for https://github.com/biomejs/biome/issues/9180
///
/// This issue was caused by noRedundantUseStrict's fix replacing the directive with a directive that was syntactically blank.
/// When fixes are applied, they also get formatted by our formatter. The formatter would try to format the
/// directive, but it expects the directive to have quotes, and since it was syntactically blank, it would panic
/// when trying to trim the quotes off.
///
/// Our unit tests didn't pick it up because linter unit tests don't include the formatting step.
///
/// This issue was fixed by changing the rule's fix to remove the node and transfer the trivia to the next token.
#[test]
fn issue_9180() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file = Utf8Path::new("test.js");
    fs.insert(
        js_file.into(),
        "// foo\n'use strict';\r\nconsole.log('test');\n".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", js_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_9180",
        fs,
        console,
        result,
    ));
}

#[test]
fn issue_9180_2() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file = Utf8Path::new("test.js");
    fs.insert(js_file.into(), "// foo\n'use strict';\n".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", js_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_9180_2",
        fs,
        console,
        result,
    ));
}
