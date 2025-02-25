//! Here, we put test cases where lint rules are enabled via package.json dependencies

use crate::run_cli_with_dyn_fs;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::TemporaryFs;
use bpaf::Args;

#[test]
fn enables_react_rules_via_dependencies() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("enables_react_rules_via_dependencies");
    fs.create_file(
        "package.json",
        r#"{
    "dependencies": {
        "react": "^16.0.0"
    }
}
"#,
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
    fs.create_file("test.jsx", content);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "enables_react_rules_via_dependencies",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn enables_test_globals_via_dependencies() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("enables_test_globals_via_dependencies");
    fs.create_file(
        "package.json",
        r#"{
    "dependencies": {
        "mocha": "10.0.0"
    }
}
"#,
    );

    let content = r#"
describe("foo", () => {
	beforeEach(() => {
	});
	beforeEach(() => {
	});
	test("bar", () => {
		someFn();
	});
});
    "#;
    fs.create_file("test.js", content);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "enables_test_globals_via_dependencies",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn enables_rules_via_dependencies_but_disable_rule_from_config() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("enables_rules_via_dependencies_but_disable_rule_from_config");
    fs.create_file(
        "package.json",
        r#"{
    "dependencies": {
        "react": "latest"
    }
}
"#,
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
    fs.create_file("test.jsx", content);

    fs.create_file(
        "biome.json",
        r#"{
    "linter": {
        "rules": {
            "correctness": {
                "useExhaustiveDependencies": "off"
            }
        }
    }
}
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", fs.cli_path()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "enables_rules_via_dependencies_but_disable_rule_from_config",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn enables_next_rules_via_dependencies() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("enables_next_rules_via_dependencies");
    fs.create_file(
        "package.json",
        r#"{
    "dependencies": {
        "next": ">=14.0.0"
    }
}"#,
    );

    fs.create_file(
        "test.jsx",
        r#"import React from 'react';

function IndexPage() {
    return (
        <div>
            <img alt="Foo" />
            <p>Some content</p>
        </div>
    );
}

export default IndexPage;
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", fs.cli_path()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "enables_next_rules_via_dependencies",
        fs.create_mem(),
        console,
        result,
    ));
}
