use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::{BufferConsole, LogLevel};
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

const TEST_CONTENTS: &str = "debugger;";

#[test]
fn logs_the_appropriate_messages_according_to_set_diagnostics_level() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "include": ["test.js"]
  },
  "linter": {
    "rules": {
        "suspicious": {
            "noDebugger": "warn"
        }
    }
  }
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), TEST_CONTENTS.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--diagnostic-level=error"),
                test.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let messages = &console.out_buffer;

    assert!(messages
        .iter()
        .filter(|m| m.level == LogLevel::Log)
        .any(|m| {
            let content = format!("{:?}", m.content);

            !content.contains("noDebugger")
        }));

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "logs_the_appropriate_messages_according_to_set_diagnostics_level",
        fs,
        console,
        result,
    ));
}
