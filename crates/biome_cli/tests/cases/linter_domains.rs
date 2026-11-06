use crate::TestArgs as Args;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use crate::{run_cli, run_cli_with_server_workspace};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

#[test]
fn does_enable_test_rules() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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
    let fs = MemoryFileSystem::default();
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

#[test]
fn does_enable_test_rules_when_recommended_rules_are_disabled_but_domain_is_enabled() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "recommended": false
        },
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
        Args::from(["lint", test1.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_enable_test_rules_when_recommended_rules_are_disabled_but_domain_is_enabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_enable_domain_via_cli() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "recommended": false
        },
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
        Args::from(["lint", "--only=test", test1.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_enable_domain_via_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_disable_domain_via_cli() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
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
        Args::from(["lint", "--skip=test", test1.as_str(), test2.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_disable_domain_via_cli",
        fs,
        console,
        result,
    ));
}

/// Regression test for https://github.com/biomejs/biome/issues/9258
/// Skipping a domain should not report suppression comments for rules in that
/// domain as unused.
#[test]
fn skip_domain_does_not_report_unused_suppression() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
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
        r#"// biome-ignore lint/suspicious/noFocusedTests: reason
describe.only("bar", () => {});
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--skip=test", test1.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "skip_domain_does_not_report_unused_suppression",
        fs,
        console,
        result,
    ));
}

/// Verifies that explicit domain enables are additive with whole-group enables:
/// the plain group contributes only non-domain rules, and the domain adds its
/// domain-tagged rules back explicitly.
#[test]
fn group_enable_and_domain_enable_are_additive() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        br#"{
    "linter": {
        "rules": {
            "recommended": false,
            "correctness": "error"
        },
        "domains": {
            "react": "all"
        }
    }
}
"#,
    );
    let test1 = Utf8Path::new("test1.jsx");
    fs.insert(
        test1.into(),
        br#"import { useEffect, useState } from "react";

export function Component() {
    const [local, setLocal] = useState(0);

    useEffect(() => {
        console.log(local);
    }, []);

    return <button onClick={() => setLocal(1)}>increment</button>;
}
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test1.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "group_enable_and_domain_enable_are_additive",
        fs,
        console,
        result,
    ));
}

/// A rule that belongs to several domains stays enabled when one of its domains
/// is disabled but another one is enabled.
#[test]
fn disabled_domain_does_not_disable_rules_of_enabled_domain() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        br#"{
    "linter": {
        "domains": {
            "react": "all",
            "next": "none"
        }
    }
}
"#,
    );
    let test1 = Utf8Path::new("test1.jsx");
    fs.insert(
        test1.into(),
        br#"import { useEffect } from "react";

export function Clock({ tick, onTick }) {
    useEffect(() => {
        onTick(tick);
    }, []);
    return null;
}
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test1.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "disabled_domain_does_not_disable_rules_of_enabled_domain",
        fs,
        console,
        result,
    ));
}

/// A rule enabled explicitly stays enabled when its domain is disabled.
#[test]
fn explicitly_enabled_rule_is_not_disabled_by_domain() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        br#"{
    "linter": {
        "rules": {
            "correctness": {
                "useExhaustiveDependencies": "error"
            }
        },
        "domains": {
            "react": "none"
        }
    }
}
"#,
    );
    let test1 = Utf8Path::new("test1.jsx");
    fs.insert(
        test1.into(),
        br#"import { useEffect } from "react";

export function Clock({ tick, onTick }) {
    useEffect(() => {
        onTick(tick);
    }, []);
    return null;
}
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test1.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "explicitly_enabled_rule_is_not_disabled_by_domain",
        fs,
        console,
        result,
    ));
}

#[test]
fn explicitly_enabled_override_rule_is_not_disabled_by_domain() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8Path::new("biome.json").into(),
        br#"{
    "linter": {
        "domains": {
            "react": "none"
        }
    },
    "overrides": [{
        "includes": ["test.jsx"],
        "linter": {
            "rules": {
                "correctness": {
                    "useExhaustiveDependencies": "error"
                }
            }
        }
    }]
}
"#,
    );
    let test = Utf8Path::new("test.jsx");
    fs.insert(
        test.into(),
        br#"import { useEffect } from "react";

export function Clock({ tick, onTick }) {
    useEffect(() => {
        onTick(tick);
    }, []);
    return null;
}
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "explicitly_enabled_override_rule_is_not_disabled_by_domain",
        fs,
        console,
        result,
    ));
}

#[test]
fn rules_enabled_in_overrides_enable_project_and_types_domains() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8Path::new("biome.json").into(),
        br#"{
    "overrides": [{
        "includes": ["src/**"],
        "linter": {
            "rules": {
                "correctness": { "noUnresolvedImports": "error" },
                "nursery": { "noFloatingPromises": "error" },
                "suspicious": { "noConsole": "error" }
            }
        }
    }]
}
"#,
    );
    let index = Utf8Path::new("src/index.ts");
    fs.insert(
        index.into(),
        br#"import { missing } from "./other.ts";

async function load(): Promise<string> {
	return "data";
}

export function main(): void {
	console.log(missing);
	load();
}
"#,
    );
    fs.insert(
        Utf8Path::new("src/other.ts").into(),
        b"export const other = 1;\n",
    );

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["check", index.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "rules_enabled_in_overrides_enable_project_and_types_domains",
        fs,
        console,
        result,
    ));
}

#[test]
fn unrelated_override_rule_does_not_enable_rules_from_disabled_domain() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8Path::new("biome.json").into(),
        br#"{
    "linter": {
        "rules": { "correctness": "error" },
        "domains": { "solid": "none" }
    },
    "overrides": [{
        "includes": ["**"],
        "linter": {
            "rules": {
                "correctness": { "noUnusedVariables": "off" }
            }
        }
    }]
}
"#,
    );
    let card = Utf8Path::new("card.tsx");
    fs.insert(
        card.into(),
        br#"export function Card({ title }: { title: string }) {
    return <div>{title}</div>;
}
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", card.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unrelated_override_rule_does_not_enable_rules_from_disabled_domain",
        fs,
        console,
        result,
    ));
}
