use crate::run_cli_with_dyn_fs;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::TemporaryFs;
use bpaf::Args;

const UNFORMATTED: &str = "  statement(  )  ";

#[test]
fn include_vcs_ignore_cascade() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("include_vcs_ignore_cascade");

    fs.create_file(".gitignore", r#"file4.js"#);
    fs.create_file(
        "biome.json",
        r#"{
        "vcs": {
            "enabled": true,
            "clientKind": "git",
            "useIgnoreFile": true
        },
        "files": {
            "includes": ["**", "!file2.js"]
        },
        "formatter": {
          "includes": ["file1.js", "file2.js", "file4.js", "!file3.js"]
        }
    }"#,
    );

    // Only `file1.js` will be formatted:
    // - `file2.js` is ignored at top-level
    // - `file3.js` is ignored at formatter-level
    // - `file4.js` is ignored in `.gitignore`
    let files = [
        ("file1.js", true),
        ("file2.js", false),
        ("file3.js", false),
        ("file4.js", false),
    ];
    for (file_path, _) in files {
        fs.create_file(file_path, UNFORMATTED);
    }

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", fs.cli_path(), "--write"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "include_vcs_ignore_cascade",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn ignore_vcs_os_independent_parse() {
    let mut fs = TemporaryFs::new("ignore_vcs_os_independent_parse");
    let mut console = BufferConsole::default();

    fs.create_file(
        "biome.json",
        r#"{
        "vcs": {
            "enabled": true,
            "clientKind": "git",
            "useIgnoreFile": true
        }
    }"#,
    );

    fs.create_file(".gitignore", "something.js\nfile2.js\r\nfile3.js");

    fs.create_file("file3.js", r#"console.log('biome is cool');"#);
    fs.create_file("file2.js", r#"foo.call(); bar.call();"#);
    fs.create_file("file1.js", r#"blah.call();"#);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["check", "--write", fs.cli_path()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_vcs_os_independent_parse",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn ignore_vcs_ignored_file_via_cli() {
    let mut fs = TemporaryFs::new("ignore_vcs_ignored_file_via_cli");
    let mut console = BufferConsole::default();

    fs.create_file(
        ".gitignore",
        r#"
file2.js
"#,
    );

    fs.create_file("file2.js", r#"foo.call(); bar.call();"#);
    fs.create_file(
        "file1.js",
        r#"array.map(sentence => sentence.split(' ')).flat();"#,
    );

    // git folder
    fs.create_folder("git");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(
            [
                "lint",
                "--vcs-enabled=true",
                "--vcs-client-kind=git",
                "--vcs-use-ignore-file=true",
                "--vcs-root=.",
                fs.cli_path(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_vcs_ignored_file_via_cli",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn ignores_file_inside_directory() {
    let mut fs = TemporaryFs::new("ignores_file_inside_directory");
    let mut console = BufferConsole::default();

    fs.create_file(
        ".gitignore",
        r#"
**/ignored/
"#,
    );

    fs.create_file(
        "ignored/file1.js",
        r#"array.map(sentence => sentence.split('    ')).flat();"#,
    );
    fs.create_file("ignored/file2.js", r#"foo.call(); bar.call();"#);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(
            [
                "check",
                "--vcs-enabled=true",
                "--vcs-client-kind=git",
                "--vcs-use-ignore-file=true",
                "--vcs-root=.",
                "--write",
                "--unsafe",
                fs.cli_path(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignores_file_inside_directory",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn use_root_gitignore_when_running_from_subdirectory() {
    let mut fs = TemporaryFs::new("use_root_gitignore_when_running_from_subdirectory");
    let mut console = BufferConsole::default();

    fs.create_file(".gitignore", r#"dist/"#);
    fs.create_file(
        "biome.json",
        r#"{
            "files": {
                "includes": ["packages/**"]
            },
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true
            }
        }"#,
    );

    fs.create_file("packages/lib/dist/out.js", r#"foo.call(); bar.call();"#);
    fs.create_file("packages/lib/src/in.js", r#"foo.call(); bar.call();"#);

    fs.append_to_working_directory("packages/lib");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format"].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "use_root_gitignore_when_running_from_subdirectory",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_fail_when_ignore_file_is_absent() {
    let mut fs = TemporaryFs::new("should_fail_when_ignore_file_is_absent");
    let mut console = BufferConsole::default();

    fs.create_file(
        "biome.json",
        r#"{
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true
            }
        }"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_fail_when_ignore_file_is_absent",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn should_ignore_absolute_paths_in_ignore_file_with_glob() {
    let mut fs = TemporaryFs::new("should_ignore_absolute_paths_in_ignore_file_with_glob");
    let mut console = BufferConsole::default();
    fs.create_file(".gitignore", r#"dist/"#);
    fs.create_file(
        "biome.json",
        r#"{
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true
            }
        }"#,
    );
    fs.create_file("in.js", r#"foo.call(); bar.call();"#);
    let out_path = fs.create_file("dist/out.js", r#"foo.call(); bar.call();"#);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", out_path.as_str()].as_slice()),
    );

    // No files processed, which is what we want
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_ignore_absolute_paths_in_ignore_file_with_glob",
        fs.create_mem(),
        console,
        result,
    ));
}
