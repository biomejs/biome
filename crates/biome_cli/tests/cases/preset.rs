use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const DEBUGGER_BEFORE: &str = "debugger;\n";
const DEBUGGER_AFTER: &str = "\n";

// --- preset: "recommended" at the top-level rules ---

#[test]
fn preset_recommended_enables_recommended_rules() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "preset": "recommended"
        }
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", test.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    // noDebugger is recommended → debugger statement should be removed
    assert_file_contents(&fs, test, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "preset_recommended_enables_recommended_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn preset_recommended_does_not_enable_nursery_rules() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "preset": "recommended"
        }
    }
}"#
        .as_bytes(),
    );

    // `continue` triggers nursery/noContinue if enabled
    let test = Utf8Path::new("test.js");
    fs.insert(
        test.into(),
        r#"for (let i = 0; i < 10; i++) {
    if (i === 5) {
        continue;
    }
}
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // No nursery errors → should pass cleanly
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "preset_recommended_does_not_enable_nursery_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn preset_none_disables_all_rules() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "preset": "none"
        }
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // preset: "none" disables everything → no lint errors
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "preset_none_disables_all_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn preset_all_enables_all_rules() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "preset": "all"
        }
    }
}"#
        .as_bytes(),
    );

    // noNegationElse is a non-recommended rule in style.
    // It should only trigger when "all" rules are enabled.
    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), b"if (!cond) { f(); } else { g(); }\n");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // preset: "all" enables all rules including non-recommended → errors expected
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "preset_all_enables_all_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn preset_all_does_not_enable_nursery_rules() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "preset": "all"
        }
    }
}"#
        .as_bytes(),
    );

    // noContinue is a nursery rule — should NOT be triggered by top-level preset: "all"
    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), NURSERY_CONTINUE_CODE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // preset: "all" at top level should not enable nursery rules
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "preset_all_does_not_enable_nursery_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn preset_none_with_group_preset_all() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "preset": "none",
            "suspicious": {
                "preset": "all"
            }
        }
    }
}"#
        .as_bytes(),
    );

    // noEmptyBlockStatements is a non-recommended suspicious rule → should trigger.
    // useLiteralKeys is a recommended complexity rule → should NOT trigger
    // because top-level preset is "none".
    let test = Utf8Path::new("test.js");
    fs.insert(
        test.into(),
        br#"function emptyFunctionBody() {}
a["b"] = 42;
"#,
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "preset_none_with_group_preset_all",
        fs,
        console,
        result,
    ));
}

// --- preset at the group level ---

#[test]
fn group_preset_recommended_enables_group_rules() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "recommended": false,
            "suspicious": {
                "preset": "recommended"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", test.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    // Only suspicious recommended rules should fire → noDebugger removes debugger
    assert_file_contents(&fs, test, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "group_preset_recommended_enables_group_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn group_preset_none_disables_group_rules() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "recommended": true,
            "suspicious": {
                "preset": "none"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // suspicious preset is "none" → noDebugger should NOT fire, debugger stays
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "group_preset_none_disables_group_rules",
        fs,
        console,
        result,
    ));
}

// --- preset with individual rule overrides ---

#[test]
fn group_preset_none_with_rule_enabled() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "recommended": false,
            "suspicious": {
                "preset": "none",
                "noDebugger": "error"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // preset is "none" but noDebugger is explicitly enabled → should see diagnostic
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "group_preset_none_with_rule_enabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn group_preset_all_with_rule_off() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "recommended": false,
            "suspicious": {
                "preset": "all",
                "noDebugger": "off"
            }
        }
    }
}"#
        .as_bytes(),
    );

    // debugger; triggers noDebugger (off) — should NOT fire.
    // function foo() {} triggers noEmptyBlockStatements (non-recommended) — should fire
    // via preset: "all", proving the preset works.
    let test = Utf8Path::new("test.js");
    fs.insert(
        test.into(),
        b"debugger;\nfunction foo() {}\n",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // noEmptyBlockStatements fires (preset "all"), but noDebugger does not (explicitly off)
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "group_preset_all_with_rule_off",
        fs,
        console,
        result,
    ));
}

// --- preset and recommended interaction ---

#[test]
fn preset_recommended_same_as_recommended_true() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    // Use preset: "recommended" (equivalent to recommended: true)
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "preset": "recommended",
            "suspicious": {
                "noDebugger": "off"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // preset is recommended but noDebugger is explicitly off → no error
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "preset_recommended_same_as_recommended_true",
        fs,
        console,
        result,
    ));
}

// --- nursery rules and presets ---

const NURSERY_CONTINUE_CODE: &str = r#"for (let i = 0; i < 10; i++) {
    if (i >= 5) {
        continue;
    }
    console.log(i);
}
"#;

#[test]
fn nursery_preset_all_does_not_enable_rules() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "nursery": {
                "preset": "all"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), NURSERY_CONTINUE_CODE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // nursery preset "all" should NOT enable rules — nursery rules must be enabled individually
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "nursery_preset_all_does_not_enable_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn nursery_rule_enabled_individually() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "nursery": {
                "noContinue": "error"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), NURSERY_CONTINUE_CODE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // noContinue explicitly enabled → should see diagnostic
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "nursery_rule_enabled_individually",
        fs,
        console,
        result,
    ));
}

// --- empty/partial rules should still enable recommended by default ---

#[test]
fn empty_rules_object_still_enables_recommended() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    // "rules": {} — no recommended, no preset. Recommended should be the default.
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {}
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", test.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    // noDebugger is recommended and should fire even with empty rules object
    assert_file_contents(&fs, test, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "empty_rules_object_still_enables_recommended",
        fs,
        console,
        result,
    ));
}

#[test]
fn customising_one_group_still_enables_recommended() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    // Only customising one rule in one group — other recommended rules should still fire.
    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "style": {
                "noNegationElse": "error"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", test.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    // noDebugger is recommended (suspicious) and should still fire
    assert_file_contents(&fs, test, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "customising_one_group_still_enables_recommended",
        fs,
        console,
        result,
    ));
}

// --- nursery is never enabled by preset: "recommended" at top level ---

#[test]
fn recommended_true_does_not_enable_nursery_rules() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let config = Utf8Path::new("biome.json");
    fs.insert(
        config.into(),
        r#"{
    "linter": {
        "rules": {
            "recommended": true
        }
    }
}"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(
        test.into(),
        r#"for (let i = 0; i < 10; i++) {
    if (i === 5) {
        continue;
    }
}
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", test.as_str()].as_slice()),
    );

    // recommended: true should NOT enable nursery rules
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "recommended_true_does_not_enable_nursery_rules",
        fs,
        console,
        result,
    ));
}
