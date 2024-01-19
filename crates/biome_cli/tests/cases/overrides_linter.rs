use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

const FIX_BEFORE: &str = "(1 >= -0)";
const FIX_AFTER: &str = "(1 >= 0)";

const DEBUGGER_BEFORE: &str = "debugger";
const DEBUGGER_AFTER: &str = "";

const SIMPLE_NUMBERS_BEFORE: &str = "({ 0x1: 1 });";
const SIMPLE_NUMBERS_AFTER: &str = "({ 1: 1 });";

#[test]
fn does_handle_included_file_and_disable_linter() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "include": ["test.js", "special/**"]
  },
  "overrides": [{ "include": ["special/**"], "linter": { "enabled": false } }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), FIX_BEFORE.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), FIX_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FIX_BEFORE);
    assert_file_contents(&fs, test, FIX_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_handle_included_file_and_disable_linter",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_rules() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [{ "include": ["special/**"], "linter": { "rules": {
    "suspicious": { "noDebugger": "off" }
  } } }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_linting_and_applies_all_of_them() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
    "overrides": [
        {
            "include": [
                "special/**"
            ],
            "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "off"
                    }
                }
            }
        },
        {
            "include": [
                "special/**"
            ],
            "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "error"
                    }
                }
            }
        }
    ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, DEBUGGER_AFTER);
    assert_file_contents(&fs, test, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_linting_and_applies_all_of_them",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_overrides() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
    "overrides": [
        {
            "include": [
                "test.js"
            ],
            "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "off"
                    }
                }
            }
        },
        {
            "include": [
                "test2.js"
            ],
            "linter": {
                "rules": {
                    "complexity": {
                        "useSimpleNumberKeys": "error"
                    }
                }
            }
        }
    ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), SIMPLE_NUMBERS_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test2, SIMPLE_NUMBERS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_overrides",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_override_the_rules() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
    "overrides": [
        {
            "include": [
                "test.js"
            ],
            "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "off"
                    }
                }
            }
        }
    ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test2, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_override_the_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_change_linting_settings() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
        "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "off"
                    }
                }
            },
  "overrides": [
    { "include": ["test.js"], "formatter": { "enabled": false } }
  ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test2, DEBUGGER_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_change_linting_settings",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_override_recommended() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "linter": {
                "rules": {
                    "recommended": true
                }
            },
            "overrides": [
                {
                    "include": ["test.js"],
                    "linter": {
                        "rules": {
                            "recommended": false
                        }
                    }
                }
            ]
        }"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(["lint", "--apply-unsafe", "."].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test2, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_override_recommended",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_override_groupe_recommended() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "linter": {
                "rules": {
                    "suspicious": {
                        "recommended": true
                    }
                }
            },
            "overrides": [
                {
                    "include": ["test.js"],
                    "linter": {
                        "rules": {
                            "suspicious": {
                                "recommended": false
                            }
                        }
                    }
                }
            ]
        }"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(["lint", "--apply-unsafe", "."].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test2, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_override_groupe_recommended",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_preserve_group_recommended_when_override_global_recommened() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "linter": {
                "rules": {
                    "suspicious": {
                        "recommended": false
                    }
                }
            },
            "overrides": [
                {
                    "include": ["test.js"],
                    "linter": {
                        "rules": {
                            "recommended": true
                        }
                    }
                }
            ]
        }"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(["lint", "--apply-unsafe", "."].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test2, DEBUGGER_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_preserve_group_recommended_when_override_global_recommened",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_preserve_individually_diabled_rules_in_overrides() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "off"
                    }
                }
            },
            "overrides": [
                {
                    "include": ["test.js"],
                    "linter": {
                        "rules": {
                            "suspicious": {}
                        }
                    }
                }
            ]
        }"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(["lint", "--apply-unsafe", "."].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test2, DEBUGGER_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_preserve_individually_diabled_rules_in_overrides",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_merge_all_overrides() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
            "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "error"
                    }
                }
            },
            "overrides": [
                {
                    "include": ["*.js"],
                    "linter": {
                        "rules": {
                            "suspicious": {
                                "noDebugger": "warn"
                            }
                        }
                    }
                }, {
                    "include": ["test.js"],
                    "linter": {
                        "rules": {
                            "suspicious": {
                                "noDebugger": "off"
                            }
                        }
                    }
                }
            ]
        }"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), DEBUGGER_BEFORE.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), DEBUGGER_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(["lint", "--apply-unsafe", "."].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, DEBUGGER_BEFORE);
    assert_file_contents(&fs, test2, DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_merge_all_overrides",
        fs,
        console,
        result,
    ));
}
