//! Here, we put test cases where lint rules are enabled via package.json dependencies

use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn does_enable_test_rules() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "domains": {
            "test": "all"
        }
    }
}
"#
        .as_bytes(),
    );
    let test1 = Utf8Path::new("test1.js");
    fs.insert(
        test1.into(),
        r#"describe.only("bar", () => {});
"#
        .as_bytes(),
    );

    let content = r#"
describe("foo", () => {
	beforeEach(() => {});
    beforeEach(() => {});
    test("bar", () => {
        someFn();
    });
});
    "#;
    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test1.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_enable_test_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_disable_test_rules() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "domains": {
            "test": "none"
        }
    }
}
"#
        .as_bytes(),
    );
    let test1 = Utf8Path::new("test1.js");
    fs.insert(
        test1.into(),
        r#"describe.only("bar", () => {});
"#
        .as_bytes(),
    );

    let content = r#"
describe("foo", () => {
	beforeEach(() => {});
    beforeEach(() => {});
    test("bar", () => {
        someFn();
    });
});
    "#;
    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test1.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_disable_test_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn enable_test_rules_via_overrides() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "domains": {
            "test": "none"
        }
    },
    "overrides": [{
        "includes": ["test1.js"],
        "linter": {
            "domains": {
                "test": "all"
            }
        }
    }]
}
"#
        .as_bytes(),
    );
    let test1 = Utf8Path::new("test1.js");
    fs.insert(
        test1.into(),
        r#"describe.only("bar", () => {});
"#
        .as_bytes(),
    );

    let content = r#"
describe("foo", () => {
	beforeEach(() => {});
    beforeEach(() => {});
    test("bar", () => {
        someFn();
    });
});
    "#;
    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test1.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "enable_test_rules_via_overrides",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_enable_test_rules_and_skip() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "domains": {
            "test": "all"
        }
    }
}
"#
        .as_bytes(),
    );
    let test1 = Utf8Path::new("test1.js");
    fs.insert(
        test1.into(),
        r#"describe.only("bar", () => {});
"#
        .as_bytes(),
    );

    let content = r#"
describe("foo", () => {
	beforeEach(() => {});
    beforeEach(() => {});
    test("bar", () => {
        someFn();
    });
});
    "#;
    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                test1.as_str(),
                test2.as_str(),
                "--skip=suspicious/noDuplicateTestHooks",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_enable_test_rules_and_skip",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_enable_test_rules_and_only() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "domains": {
            "test": "all"
        }
    }
}
"#
        .as_bytes(),
    );
    let test1 = Utf8Path::new("test1.js");
    fs.insert(
        test1.into(),
        r#"
        debugger;
        describe.only("bar", () => {});
"#
        .as_bytes(),
    );

    let content = r#"
describe("foo", () => {
	beforeEach(() => {});
    beforeEach(() => {});
    test("bar", () => {
        someFn();
    });
});
    "#;
    let test2 = Utf8Path::new("test2.js");
    fs.insert(test2.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                test1.as_str(),
                test2.as_str(),
                "--only=suspicious/noDebugger",
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_enable_test_rules_and_only",
        fs,
        console,
        result,
    ));
}
