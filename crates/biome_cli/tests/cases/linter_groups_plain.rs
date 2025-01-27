use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn enables_all_rules_when_group_is_on_with_default_severity() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "style": "on"
        }
    }
}
"#
        .as_bytes(),
    );
    let test1 = Utf8Path::new("test1.js");
    fs.insert(
        test1.into(),
        r#"function f() { console.log(arguments); }
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test1.as_str()].as_slice()),
    );

    // style rules have warnings
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "enables_all_rules_when_group_is_on_with_default_severity",
        fs,
        console,
        result,
    ));
}

#[test]
fn enables_all_rules_when_group_is_error() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "style": "error"
        }
    }
}
"#
        .as_bytes(),
    );
    let test1 = Utf8Path::new("test1.js");
    fs.insert(
        test1.into(),
        r#"function f() { console.log(arguments); }
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test1.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "enables_all_rules_when_group_is_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn disable_all_rules_when_group_is_off() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "a11y": "off"
        }
    }
}
"#
        .as_bytes(),
    );
    let test1 = Utf8Path::new("test1.jsx");
    // img without alt should trigger a recommended rule, but not in this case
    fs.insert(
        test1.into(),
        r#"<img src="foo.png" />
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test1.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "disable_all_rules_when_group_is_off",
        fs,
        console,
        result,
    ));
}
