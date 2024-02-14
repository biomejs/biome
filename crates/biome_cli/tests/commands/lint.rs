use bpaf::Args;
use std::env::temp_dir;
use std::fs::{create_dir, create_dir_all, remove_dir_all, File};
use std::io::Write;
#[cfg(target_family = "unix")]
use std::os::unix::fs::symlink;
#[cfg(target_os = "windows")]
use std::os::windows::fs::{symlink_dir, symlink_file};
use std::path::{Path, PathBuf};

use crate::configs::{
    CONFIG_FILE_SIZE_LIMIT, CONFIG_IGNORE_SYMLINK, CONFIG_LINTER_AND_FILES_IGNORE,
    CONFIG_LINTER_DISABLED, CONFIG_LINTER_DISABLED_JSONC, CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC,
    CONFIG_LINTER_IGNORED_FILES, CONFIG_LINTER_SUPPRESSED_GROUP, CONFIG_LINTER_SUPPRESSED_RULE,
    CONFIG_LINTER_UPGRADE_DIAGNOSTIC, CONFIG_RECOMMENDED_GROUP,
};
use crate::snap_test::{assert_file_contents, markup_to_string, SnapshotPayload};
use crate::{assert_cli_snapshot, run_cli, FORMATTED, LINT_ERROR, PARSE_ERROR};
use biome_console::{markup, BufferConsole, LogLevel, MarkupBuf};
use biome_fs::{ErrorEntry, FileSystemExt, MemoryFileSystem, OsFileSystem};
use biome_service::DynRef;

const ERRORS: &str = r#"
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
"#;

const NO_DEBUGGER: &str = "debugger;";
const NEW_SYMBOL: &str = "new Symbol(\"\");";

const FIX_BEFORE: &str = "(1 >= -0)";
const FIX_AFTER: &str = "(1 >= 0)";

const APPLY_SUGGESTED_BEFORE: &str = "let a = 4;
debugger;
console.log(a);
";

const APPLY_SUGGESTED_AFTER: &str = "const a = 4;\nconsole.log(a);\n";

const NO_DEBUGGER_BEFORE: &str = "debugger;\n";
const NO_DEBUGGER_AFTER: &str = "debugger;\n";

const UPGRADE_SEVERITY_CODE: &str = r#"if(!cond) { exprA(); } else { exprB() }"#;

const NURSERY_UNSTABLE: &str = r#"if(a = b) {}"#;

#[test]
fn lint_help() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), "--help"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_help",
        fs,
        console,
        result,
    ));
}

#[test]
fn ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
}

#[test]
fn ok_read_only() {
    let mut fs = MemoryFileSystem::new_read_only();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
}

#[test]
fn parse_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), PARSE_ERROR.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn maximum_diagnostics() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), ERRORS.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let messages = &console.out_buffer;

    assert_eq!(
        messages
            .iter()
            .filter(|m| m.level == LogLevel::Error)
            .count(),
        20_usize
    );

    assert!(messages
        .iter()
        .filter(|m| m.level == LogLevel::Log)
        .any(|m| {
            let content = format!("{:?}", m.content);
            content.contains("The number of diagnostics exceeds the number allowed by Biome")
                && content.contains("Diagnostics not shown")
                && content.contains("76")
        }));

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "maximum_diagnostics",
        fs,
        console,
        result,
    ));
}

#[test]
fn apply_ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, FIX_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn apply_noop() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), FIX_AFTER.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_noop",
        fs,
        console,
        result,
    ));
}

#[test]
fn apply_suggested_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), APPLY_SUGGESTED_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                ("--apply"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_suggested_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn apply_suggested() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), APPLY_SUGGESTED_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, APPLY_SUGGESTED_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_suggested",
        fs,
        console,
        result,
    ));
}

#[test]
fn apply_unsafe_with_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // last line doesn't have code fix
    let source = "let a = 4;
debugger;
console.log(a);
function f() { arguments; }
";

    let expected = "const a = 4;
console.log(a);
function f() { arguments; }
";

    let test1 = Path::new("test1.js");
    fs.insert(test1.into(), source.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), source.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply-unsafe"),
                test1.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test1, expected);
    assert_file_contents(&fs, test2, expected);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_unsafe_with_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_lint_if_linter_is_disabled_when_run_apply() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

    let config_path = Path::new("biome.json");
    fs.insert(config_path.into(), CONFIG_LINTER_DISABLED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, FIX_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_lint_if_linter_is_disabled_when_run_apply",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_lint_if_linter_is_disabled_when_run_apply_biome_jsonc() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

    let config_path = Path::new("biome.jsonc");
    fs.insert(config_path.into(), CONFIG_LINTER_DISABLED_JSONC.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, FIX_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_lint_if_linter_is_disabled_when_run_apply_biome_jsonc",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_lint_if_linter_is_disabled() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

    let config_path = Path::new("biome.json");
    fs.insert(config_path.into(), CONFIG_LINTER_DISABLED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, FIX_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_lint_if_linter_is_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_disable_a_rule() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), NO_DEBUGGER_BEFORE.as_bytes());

    let config_path = Path::new("biome.json");
    fs.insert(config_path.into(), CONFIG_LINTER_SUPPRESSED_RULE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, NO_DEBUGGER_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_disable_a_rule",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_disable_a_rule_group() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

    let config_path = Path::new("biome.json");
    fs.insert(
        config_path.into(),
        CONFIG_LINTER_SUPPRESSED_GROUP.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, "(1 >= -0)");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_disable_a_rule_group",
        fs,
        console,
        result,
    ));
}

#[test]
fn downgrade_severity() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC.as_bytes(),
    );

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), NO_DEBUGGER.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    println!("{console:?}");

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let messages = &console.out_buffer;

    assert_eq!(
        messages
            .iter()
            .filter(|m| m.level == LogLevel::Error)
            .filter(|m| {
                let content = format!("{:#?}", m.content);
                content.contains("suspicious/noDebugger")
            })
            .count(),
        1
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "downgrade_severity",
        fs,
        console,
        result,
    ));
}

#[test]
fn upgrade_severity() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        CONFIG_LINTER_UPGRADE_DIAGNOSTIC.as_bytes(),
    );

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), UPGRADE_SEVERITY_CODE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let messages = &console.out_buffer;

    let error_count = messages
        .iter()
        .filter(|m| m.level == LogLevel::Error)
        .filter(|m| {
            let content = format!("{:?}", m.content);
            content.contains("style/noNegationElse")
        })
        .count();

    assert_eq!(
        error_count, 1,
        "expected 1 error-level message in console buffer, found {error_count:?}:\n{:?}",
        console.out_buffer
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "upgrade_severity",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_lint_when_file_is_ignored() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_LINTER_IGNORED_FILES.as_bytes());

    let file_path = Path::new("test.js");
    fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, FIX_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_lint_when_file_is_ignored",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_lint_if_files_are_listed_in_ignore_option() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_LINTER_AND_FILES_IGNORE.as_bytes());

    let file_path_test1 = Path::new("test1.js");
    fs.insert(file_path_test1.into(), FIX_BEFORE.as_bytes());

    let file_path_test2 = Path::new("test2.js");
    fs.insert(file_path_test2.into(), FIX_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                file_path_test1.as_os_str().to_str().unwrap(),
                file_path_test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path_test1)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, FIX_BEFORE);

    let mut buffer = String::new();
    fs.open(file_path_test2)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, FIX_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_lint_if_files_are_listed_in_ignore_option",
        fs,
        console,
        result,
    ));
}

/// Creating a symbolic link will fail on Windows if the current process is
/// unprivileged. Since running tests as administrator is uncommon and
/// constraining, this error gets silently ignored if we're not running on CI
/// (the workflows are being being run with the correct permissions on CI)
#[cfg(target_os = "windows")]
macro_rules! check_windows_symlink {
    ($result:expr) => {
        match $result {
            Ok(res) => res,
            Err(err) if option_env!("CI") == Some("1") => panic!("failed to create symlink: {err}"),
            Err(_) => return,
        }
    };
}

#[test]
fn fs_error_dereferenced_symlink() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let root_path = temp_dir().join("lint_rome_test_broken_symlink");
    let subdir_path = root_path.join("prefix");

    let _ = remove_dir_all(&root_path);
    create_dir(&root_path).unwrap();
    create_dir(subdir_path).unwrap();

    #[cfg(target_family = "unix")]
    {
        symlink(root_path.join("null"), root_path.join("broken_symlink")).unwrap();
    }

    #[cfg(target_os = "windows")]
    {
        check_windows_symlink!(symlink_file(
            root_path.join("null"),
            root_path.join("broken_symlink")
        ));
    }

    let result = run_cli(
        DynRef::Owned(Box::new(OsFileSystem::new(root_path.clone()))),
        &mut console,
        Args::from([("lint"), root_path.display().to_string().as_str()].as_slice()),
    );

    remove_dir_all(root_path).unwrap();

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_dereferenced_symlink",
        fs,
        console,
        result,
    ));
}

#[test]
fn fs_error_infinite_symlink_expansion_to_dirs() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let root_path = temp_dir().join("lint_rome_test_infinite_symlink_expansion_to_dirs");
    let subdir1_path = root_path.join("prefix");
    let subdir2_path = root_path.join("foo").join("bar");

    let _ = remove_dir_all(&root_path);
    create_dir_all(&subdir1_path).unwrap();
    create_dir_all(&subdir2_path).unwrap();

    #[cfg(target_family = "unix")]
    {
        symlink(&subdir2_path, subdir1_path.join("symlink1")).unwrap();
        symlink(subdir1_path, subdir2_path.join("symlink2")).unwrap();
    }

    #[cfg(target_os = "windows")]
    {
        check_windows_symlink!(symlink_dir(&subdir2_path, &subdir1_path.join("symlink1")));
        check_windows_symlink!(symlink_dir(subdir1_path, subdir2_path.join("symlink2")));
    }

    let result = run_cli(
        DynRef::Owned(Box::new(OsFileSystem::new(root_path.clone()))),
        &mut console,
        Args::from([("lint"), (root_path.display().to_string().as_str())].as_slice()),
    );

    remove_dir_all(root_path).unwrap();

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_infinite_symlink_expansion_to_dirs",
        fs,
        console,
        result,
    ));
}

#[test]
fn fs_error_infinite_symlink_expansion_to_files() {
    let mut console = BufferConsole::default();

    let root_path = temp_dir().join("lint_rome_test_infinite_symlink_expansion_to_files");
    let subdir1_path = root_path.join("prefix");
    let subdir2_path = root_path.join("foo").join("bar");

    let _ = remove_dir_all(&root_path);
    create_dir_all(&subdir1_path).unwrap();
    create_dir_all(&subdir2_path).unwrap();

    let symlink1_path = subdir1_path.join("symlink1");
    let symlink2_path = subdir2_path.join("symlink2");

    #[cfg(target_family = "unix")]
    {
        symlink(&symlink2_path, &symlink1_path).unwrap();
        symlink(&symlink1_path, &symlink2_path).unwrap();
    }

    #[cfg(target_os = "windows")]
    {
        check_windows_symlink!(symlink_dir(&symlink2_path, &symlink1_path));
        check_windows_symlink!(symlink_dir(&symlink1_path, &symlink2_path));
    }

    let result = run_cli(
        DynRef::Owned(Box::new(OsFileSystem::new(root_path.clone()))),
        &mut console,
        Args::from([("lint"), (root_path.display().to_string().as_str())].as_slice()),
    );

    remove_dir_all(root_path).unwrap();

    assert!(result.is_err(), "run_cli returned {result:?}");

    // Don't use a snapshot here, since the diagnostics can be reported in
    // arbitrary order:
    assert!(console
        .out_buffer
        .iter()
        .flat_map(|msg| msg.content.0.iter())
        .any(|node| node.content.contains("Deeply nested symlink expansion")));
    assert!(console
        .out_buffer
        .iter()
        .flat_map(|msg| msg.content.0.iter())
        .any(|node| node.content.contains(&symlink1_path.display().to_string())));
    assert!(console
        .out_buffer
        .iter()
        .flat_map(|msg| msg.content.0.iter())
        .any(|node| node.content.contains(&symlink2_path.display().to_string())));
}

#[test]
fn fs_error_read_only() {
    let mut fs = MemoryFileSystem::new_read_only();
    let mut console = BufferConsole::default();

    let file_path = Path::new("test.js");
    fs.insert(file_path.into(), *b"content");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--apply"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    // Do not store the content of the file in the snapshot
    fs.remove(file_path);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_read_only",
        fs,
        console,
        result,
    ));
}

#[test]
fn fs_error_unknown() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert_error(PathBuf::from("prefix/ci.js"), ErrorEntry::UnknownFileType);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), ("prefix")].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_unknown",
        fs,
        console,
        result,
    ));
}

// Symbolic link ignore pattern test
//
// Verifies, that ignore patterns to symbolic links are allowed.
//
// ├── biome.json
// ├── hidden_nested
// │   └── test
// │       └── symlink_testcase1_2 -> hidden_testcase1
// ├── hidden_testcase1
// │   └── test
// │       └── test.js // ok
// ├── hidden_testcase2
// │   ├── test1.ts // ignored
// │   ├── test2.ts // ignored
// │   └── test.js  // ok
// └── src
//     ├── symlink_testcase1_1 -> hidden_nested
//     ├── symlink_testcase1_3 -> hidden_testcase1/test/test.js
//     └── symlink_testcase2 -> hidden_testcase2
#[test]
#[ignore = "It regresses on linux since we added the ignore crate, to understand why"]
fn fs_files_ignore_symlink() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let root_path = temp_dir().join("lint_rome_test_files_ignore_symlink");
    let src_path = root_path.join("src");

    let testcase1_path = root_path.join("hidden_testcase1");
    let testcase1_sub_path = testcase1_path.join("test");
    let testcase1_sub_file_path = testcase1_sub_path.join("test.js");
    let testcase2_path = root_path.join("hidden_testcase2");

    let nested_path = root_path.join("hidden_nested");
    let nested_sub_path = nested_path.join("test");

    let _ = remove_dir_all(&root_path);
    create_dir(&root_path).unwrap();
    create_dir(&src_path).unwrap();
    create_dir_all(testcase1_sub_path.clone()).unwrap();
    create_dir(testcase2_path.clone()).unwrap();
    create_dir_all(nested_sub_path.clone()).unwrap();

    // src/symlink_testcase1_1
    let symlink_testcase1_1_path = src_path.join("symlink_testcase1_1");
    // hidden_nested/test/symlink_testcase1_2
    let symlink_testcase1_2_path = nested_sub_path.join("symlink_testcase1_2");
    // src/symlink_testcase1_3
    let symlink_testcase1_3_path = src_path.join("symlink_testcase1_3");
    // src/symlink_testcase2
    let symlink_testcase2_path = src_path.join("symlink_testcase2");

    #[cfg(target_family = "unix")]
    {
        // src/symlink_testcase1_1 -> hidden_nested
        symlink(nested_path, symlink_testcase1_1_path).unwrap();
        // hidden_nested/test/symlink_testcase1_2 -> hidden_testcase1
        symlink(testcase1_path, symlink_testcase1_2_path).unwrap();
        // src/symlink_testcase1_3 -> hidden_testcase1/test/test.js
        symlink(testcase1_sub_file_path, symlink_testcase1_3_path).unwrap();
        // src/symlink_testcase2 -> hidden_testcase2
        symlink(&testcase2_path, symlink_testcase2_path).unwrap();
    }

    #[cfg(target_os = "windows")]
    {
        check_windows_symlink!(symlink_dir(nested_path, symlink_testcase1_1_path));
        check_windows_symlink!(symlink_dir(testcase1_path, symlink_testcase1_2_path));
        check_windows_symlink!(symlink_dir(
            testcase1_sub_file_path,
            symlink_testcase1_3_path
        ));
        check_windows_symlink!(symlink_dir(&testcase2_path, symlink_testcase2_path));
    }

    let config_path = root_path.join("rome.json");
    let mut config_file = File::create(config_path).unwrap();
    config_file
        .write_all(CONFIG_IGNORE_SYMLINK.as_bytes())
        .unwrap();

    let files: [PathBuf; 4] = [
        testcase1_sub_path.join("test.js"), // ok
        testcase2_path.join("test.js"),     // ok
        testcase2_path.join("test1.ts"),    // ignored
        testcase2_path.join("test2.ts"),    // ignored
    ];

    for file_path in files {
        let mut file = File::create(file_path).unwrap();
        file.write_all(APPLY_SUGGESTED_BEFORE.as_bytes()).unwrap();
    }

    let result = run_cli(
        DynRef::Owned(Box::new(OsFileSystem::new(root_path.clone()))),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--config-path"),
                (root_path.display().to_string().as_str()),
                ("--apply-unsafe"),
                (src_path.display().to_string().as_str()),
            ]
            .as_slice(),
        ),
    );

    remove_dir_all(root_path).unwrap();

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_files_ignore_symlink",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), "statement();\n".repeat(80660).as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    // Do not store the content of the file in the snapshot
    fs.remove(file_path);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large_config_limit() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(PathBuf::from("biome.json"), CONFIG_FILE_SIZE_LIMIT);

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large_config_limit",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large_cli_limit() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--files-max-size=16"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large_cli_limit",
        fs,
        console,
        result,
    ));
}

#[test]
fn files_max_size_parse_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--files-max-size=-1"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "files_max_size_parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn max_diagnostics_default() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Creates 40 diagnostics.
    for i in 0..40 {
        let file_path = PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, LINT_ERROR.as_bytes());
    }

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), ("src")].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut diagnostic_count = 0;
    let mut filtered_messages = Vec::new();

    for msg in console.out_buffer {
        let MarkupBuf(nodes) = &msg.content;
        let is_diagnostic = nodes.iter().any(|node| {
            node.content.contains("useWhile")
                || node.content.contains("useBlockStatements")
                || node.content.contains("noConstantCondition")
                || node.content.contains("lint")
        });

        if is_diagnostic {
            diagnostic_count += 1;
        } else {
            filtered_messages.push(msg);
        }
    }

    console.out_buffer = filtered_messages;

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics_default",
        fs,
        console,
        result,
    ));
    assert_eq!(diagnostic_count, 20);
}

#[test]
fn max_diagnostics() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..20 {
        let file_path = PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, LINT_ERROR.as_bytes());
    }

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--max-diagnostics"),
                ("10"),
                Path::new("src").as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    let mut diagnostic_count = 0;
    let mut filtered_messages = Vec::new();

    for msg in console.out_buffer {
        let MarkupBuf(nodes) = &msg.content;
        let is_diagnostic = nodes.iter().any(|node| {
            node.content.contains("useWhile")
                || node.content.contains("useBlockStatements")
                || node.content.contains("noConstantCondition")
                || node.content.contains("lint")
        });

        if is_diagnostic {
            diagnostic_count += 1;
        } else {
            filtered_messages.push(msg);
        }
    }

    console.out_buffer = filtered_messages;

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics",
        fs,
        console,
        result,
    ));

    assert_eq!(diagnostic_count, 10);
}

#[test]
fn no_supported_file_found() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), "."].as_slice()),
    );

    eprintln!("{:?}", console.out_buffer);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_supported_file_found",
        fs,
        console,
        result,
    ));
}

#[test]
fn deprecated_suppression_comment() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("file.js");
    fs.insert(
        file_path.into(),
        *b"// rome-ignore lint(suspicious/noDoubleEquals): test
a == b;",
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "deprecated_suppression_comment",
        fs,
        console,
        result,
    ));
}

#[test]
fn print_verbose() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--verbose"),
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "print_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn unsupported_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.txt");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unsupported_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn unsupported_file_verbose() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.txt");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                "--verbose",
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unsupported_file_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppression_syntax_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), *b"// rome-ignore(:\n");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppression_syntax_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn config_recommended_group() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_RECOMMENDED_GROUP.as_bytes());

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), NEW_SYMBOL.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "config_recommended_group",
        fs,
        console,
        result,
    ));
}

#[test]
fn nursery_unstable() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), NURSERY_UNSTABLE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "nursery_unstable",
        fs,
        console,
        result,
    ));
}

#[test]
fn all_rules() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "linter": {
            "rules": { "all": true }
        }
    }"#;

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

    let config_path = Path::new("biome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "all_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn top_level_all_down_level_not_all() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "linter": {
            "rules": {
                "all": true,
                "style": {
                    "all": false
                }
            }
        }
    }"#;

    // style/noArguments
    // style/noShoutyConstants
    // style/useSingleVarDeclarator
    let code = r#"
    function f() {arguments;}
    const FOO = "FOO";
    var x, y;
    "#;

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), code.as_bytes());

    let config_path = Path::new("biome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "top_level_all_down_level_not_all",
        fs,
        console,
        result,
    ));
}

#[test]
fn top_level_not_all_down_level_all() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "linter": {
            "rules": {
                "all": false,
                "style": {
                    "all": true
                }
            }
        }
    }"#;

    // style/noArguments
    // style/noShoutyConstants
    // style/useSingleVarDeclarator
    let code = r#"
    function f() {arguments;}
    const FOO = "FOO";
    var x, y;
    "#;

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), code.as_bytes());

    let config_path = Path::new("biome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "top_level_not_all_down_level_all",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignore_configured_globals() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "javascript": {
            "globals": ["foo", "bar"]
        },
        "linter": {
            "rules": {
                "correctness": {
                    "noUndeclaredVariables": "error"
                }
            }
        }
    }"#;

    // style/useSingleVarDeclarator
    let code = r#"foo.call(); bar.call();"#;

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), code.as_bytes());

    let config_path = Path::new("biome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_configured_globals",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignore_vcs_ignored_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "vcs": {
            "enabled": true,
            "clientKind": "git",
            "useIgnoreFile": true
        }
    }"#;

    let git_ignore = r#"
file2.js
"#;

    let code2 = r#"foo.call(); bar.call();"#;
    let code1 = r#"array.map(sentence => sentence.split(' ')).flat();"#;

    // ignored files
    let file_path1 = Path::new("file1.js");
    fs.insert(file_path1.into(), code1.as_bytes());
    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), code2.as_bytes());

    // configuration
    let config_path = Path::new("biome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    // git ignore file
    let ignore_file = Path::new(".gitignore");
    fs.insert(ignore_file.into(), git_ignore.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                file_path1.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_vcs_ignored_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignore_vcs_os_independent_parse() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "vcs": {
            "enabled": true,
            "clientKind": "git",
            "useIgnoreFile": true
        }
    }"#;

    let git_ignore = "something.js\nfile2.js\r\nfile3.js";

    let code3 = r#"console.log('biome is cool');"#;
    let code2 = r#"foo.call(); bar.call();"#;
    let code1 = r#"blah.call();"#;

    let file_path1 = Path::new("file1.js");
    fs.insert(file_path1.into(), code1.as_bytes());

    // ignored files
    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), code2.as_bytes());
    let file_path3 = Path::new("file3.js");
    fs.insert(file_path3.into(), code3.as_bytes());

    // configuration
    let config_path = Path::new("biome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    // git ignore file
    let ignore_file = Path::new(".gitignore");
    fs.insert(ignore_file.into(), git_ignore.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                file_path1.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
                file_path3.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_vcs_os_independent_parse",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignore_vcs_ignored_file_via_cli() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let git_ignore = r#"
file2.js
"#;

    let code2 = r#"foo.call(); bar.call();"#;
    let code1 = r#"array.map(sentence => sentence.split(' ')).flat();"#;

    // ignored files
    let file_path1 = Path::new("file1.js");
    fs.insert(file_path1.into(), code1.as_bytes());
    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), code2.as_bytes());

    // git folder
    let git_folder = Path::new("./.git");
    fs.insert(git_folder.into(), "".as_bytes());

    // git ignore file
    let ignore_file = Path::new("./.gitignore");
    fs.insert(ignore_file.into(), git_ignore.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                ("--vcs-enabled=true"),
                ("--vcs-client-kind=git"),
                ("--vcs-use-ignore-file=true"),
                ("--vcs-root=."),
                file_path1.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_vcs_ignored_file_via_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_stdin_apply_successfully() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push("import {a as a} from 'mod'; function f() {return{a}} class Foo {}".to_string());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), "--apply", ("--stdin-file-path"), ("mock.js")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(
        content,
        "import {a} from 'mod'; function f() {return{a}} class Foo {}"
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_stdin_apply_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_stdin_apply_unsafe_successfully() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    console
        .in_buffer
        .push("function f() {var x=1; return{x}} class Foo {}".to_string());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                "--apply-unsafe",
                ("--stdin-file-path"),
                ("mock.js"),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let message = console
        .out_buffer
        .first()
        .expect("Console should have written a message");

    let content = markup_to_string(markup! {
        {message.content}
    });

    assert_eq!(content, "function f() {const x=1; return{x}} class Foo {}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_stdin_apply_unsafe_successfully",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_apply_correct_file_source() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("file.ts");
    fs.insert(
        file_path.into(),
        "type A = { a: string }; type B = Partial<A>".as_bytes(),
    );

    let config_path = Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
    	"linter": {
    		"rules": {
    			"recommended": true,
    			"correctness": {
    				"noUndeclaredVariables": "error"
    			}
    		}
    	}
    }"#
        .as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_apply_correct_file_source",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_enable_all_recommended_rules() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"	{
		"organizeImports": {
		"enabled": false
	},
		"linter": {
		"enabled": true,
		"rules": {
			"recommended": false,
			"a11y": {},
			"complexity": {},
			"correctness": {},
			"performance": {},
			"security": {},
			"style": {},
			"suspicious": {}
		}
	}
	}"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let file_path = Path::new("fix.js");
    fs.insert(
        file_path.into(),
        r#"
    		LOOP: for (const x of xs) {
    		    if (x > 0) {
    		        break;
    		    }
    		    f(x);
    		}
		"#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_enable_all_recommended_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_disable_recommended_rules_for_a_group() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"	{
  "organizeImports": {
    "enabled": false
  },
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": true,
      "complexity": {
        "noUselessSwitchCase": "off"
      }
    }
  }
}"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let file_path = Path::new("fix.js");
    fs.insert(
        file_path.into(),
        r#"const array = ["split", "the text", "into words"];
// next line should error because of the recommended rule
array.map((sentence) => sentence.split(" ")).flat();
		"#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_disable_recommended_rules_for_a_group",
        fs,
        console,
        result,
    ));
}

#[test]
fn apply_bogus_argument() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("fix.js");
    fs.insert(
        file_path.into(),
        "function _13_1_3_fun(arguments) { }".as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                file_path.as_os_str().to_str().unwrap(),
                ("--apply-unsafe"),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_bogus_argument",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignores_unknown_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Path::new("test.txt");
    fs.insert(file_path1.into(), *b"content");

    let file_path2 = Path::new("test.js");
    fs.insert(file_path2.into(), *b"console.log('bar');\n");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                file_path1.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
                "--files-ignore-unknown=true",
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignores_unknown_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_json_files() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Path::new("test.json");
    fs.insert(
        file_path1.into(),
        r#"{ "foo": true, "foo": true }"#.as_bytes(),
    );

    let configuration = Path::new("biome.json");
    fs.insert(
        configuration.into(),
        r#"{
	"linter": {
		"rules": {
			"nursery": {
				"noDuplicateJsonKeys": "error"
			}
		}
	}
	 }"#
        .as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path1.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_json_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn doesnt_error_if_no_files_were_processed() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), "--no-errors-on-unmatched", ("file.js")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "doesnt_error_if_no_files_were_processed",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_pass_if_there_are_only_warnings() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"
{
  "linter": {
    "rules": {
        "recommended": true,
        "suspicious": {
            "noClassAssign": "warn"
        }
    }
  }
}
        "#
        .as_bytes(),
    );

    let file_path = Path::new("file.js");
    fs.insert(
        file_path.into(),
        r#"class A {};
A = 0;
"#
        .as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), "--apply-unsafe", ("file.js")].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_pass_if_there_are_only_warnings",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_error_with_only_warnings() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"
{
  "linter": {
    "rules": {
        "recommended": true,
        "suspicious": {
            "noClassAssign": "warn"
        }
    }
  }
}
        "#
        .as_bytes(),
    );

    let file_path = Path::new("file.js");
    fs.insert(
        file_path.into(),
        r#"class A {};
A = 0;
"#
        .as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                "--apply-unsafe",
                "--error-on-warnings",
                file_path.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_error_with_only_warnings",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_only_processes_changed_files_when_changed_flag_is_set() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    fs.set_on_get_changed_files(Box::new(|| vec![String::from("file.js")]));

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), r#"console.log('file');"#.as_bytes());

    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), r#"console.log('file2');"#.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                "--changed",
                "--since=main",
                file_path.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_only_processes_changed_files_when_changed_flag_is_set",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_error_if_changed_flag_is_used_without_since_or_default_branch_config() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    fs.set_on_get_changed_files(Box::new(|| vec![String::from("file.js")]));

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), r#"console.log('file');"#.as_bytes());

    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), r#"console.log('file2');"#.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                "--changed",
                file_path.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_error_if_changed_flag_is_used_without_since_or_default_branch_config",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_process_changed_files_if_changed_flag_is_set_and_default_branch_is_configured() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    fs.set_on_get_changed_files(Box::new(|| vec![String::from("file.js")]));

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"
{
    "vcs": {
        "defaultBranch": "main"
    }
}
        "#
        .as_bytes(),
    );

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), r#"console.log('file');"#.as_bytes());

    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), r#"console.log('file2');"#.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                "--changed",
                file_path.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_process_changed_files_if_changed_flag_is_set_and_default_branch_is_configured",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_error_if_since_arg_is_used_without_changed() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    fs.set_on_get_changed_files(Box::new(|| vec![String::from("file.js")]));

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), r#"console.log('file');"#.as_bytes());

    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), r#"console.log('file2');"#.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                "--since=main",
                file_path.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_error_if_since_arg_is_used_without_changed",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_only_process_changed_file_if_its_included() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    fs.set_on_get_changed_files(Box::new(|| {
        vec![String::from("file.js"), String::from("file2.js")]
    }));

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"
{
    "files": {
        "include": ["file.js"]
    },
    "vcs": {
        "defaultBranch": "main"
    }
}
        "#
        .as_bytes(),
    );

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), r#"console.log('file');"#.as_bytes());

    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), r#"console.log('file2');"#.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                "--changed",
                "--since=main",
                file_path.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_only_process_changed_file_if_its_included",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_not_process_ignored_file_even_if_its_changed() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();

    fs.set_on_get_changed_files(Box::new(|| vec![String::from("file.js")]));

    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"
{
    "files": {
        "ignore": ["file.js"]
    },
    "vcs": {
        "defaultBranch": "main"
    }
}
        "#
        .as_bytes(),
    );

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), r#"console.log('file');"#.as_bytes());

    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), r#"console.log('file2');"#.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("lint"),
                "--changed",
                "--since=main",
                file_path.as_os_str().to_str().unwrap(),
                file_path2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_process_ignored_file_even_if_its_changed",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_syntax_rules() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), r#"class A { #foo; #foo }"#.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_syntax_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_unused_dependencies() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"	{
  "linter": {
    "enabled": true,
    "rules": {
      "all": false,
      "nursery": {
        "noUndeclaredDependencies": "error"
      }
    }
  }
}"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let package_json = r#"	{
  "dependencies": { "react": "latest", "react-dom": "^17.0.0" }
}"#;
    let package_json_path = Path::new("package.json");
    fs.insert(package_json_path.into(), package_json.as_bytes());

    let file_path = Path::new("fix.js");
    fs.insert(
        file_path.into(),
        r#"import "react";
import "lodash";
		"#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("lint"), file_path.as_os_str().to_str().unwrap()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_dependencies",
        fs,
        console,
        result,
    ));
}
