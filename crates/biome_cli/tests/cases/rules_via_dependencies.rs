//! Here, we put test cases where lint rules are enabled via package.json dependencies

use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use std::path::Path;

#[test]
fn enables_rules_via_dependencies() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("package.json");
    fs.insert(
        file_path.into(),
        r#"{
    "dependencies": {
        "react": "16.0.0"
    }
}
"#
        .as_bytes(),
    );

    let content = r#"
import { useCallback } from "react";

function Component2() {
    const [local,SetLocal] = useState(0);
    const handle = useCallback(() => {
      console.log(local);
    }, [local, local]);
}
    "#;
    let test = Path::new("test.jsx");
    fs.insert(test.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "enables_rules_via_dependencies",
        fs,
        console,
        result,
    ));
}

#[test]
fn enables_rules_via_dependencies_but_disable_rule_from_config() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("package.json");
    fs.insert(
        file_path.into(),
        r#"{
    "dependencies": {
        "react": "latest"
    }
}
"#
        .as_bytes(),
    );

    let content = r#"
import { useCallback } from "react";

function Component2() {
    const [local,SetLocal] = useState(0);
    const handle = useCallback(() => {
      console.log(local);
    }, [local, local]);
}
    "#;
    let test = Path::new("test.jsx");
    fs.insert(test.into(), content.as_bytes());

    let config = Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "correctness": {
                "useExhaustiveDependencies": "off"
            }
        }
    }
}
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "enables_rules_via_dependencies_but_disable_rule_from_config",
        fs,
        console,
        result,
    ));
}
