use crate::TestArgs as Args;
use crate::run_cli_with_dyn_fs;
use crate::snap_test::{
    SnapshotPayload, assert_cli_snapshot, assert_file_contents, markup_to_string,
};
use biome_console::{BufferConsole, markup};
use biome_fs::TemporaryFs;
use camino::Utf8Path;

const UNFORMATTED: &str = "  statement(  )  ";

#[test]
fn honors_nested_gitignore_negation() {
    let unformatted = "{\n\t\"a\": [\n\t\t\"b\"\n\t]\n}\n";
    let formatted = "{\n\t\"a\": [\"b\"]\n}\n";

    for (case, target) in [
        ("file", "project.inlang/settings.json"),
        ("directory", "project.inlang"),
        ("project", "."),
    ] {
        let mut fs = TemporaryFs::new(&format!("honors_nested_gitignore_negation_{case}"));
        let mut console = BufferConsole::default();
        fs.create_file(
            "biome.json",
            r#"{
                "vcs": {
                    "enabled": true,
                    "clientKind": "git",
                    "useIgnoreFile": true
                },
                "files": { "includes": ["**"] },
                "formatter": { "indentStyle": "tab" }
            }"#,
        );
        fs.create_file(".gitignore", "dist/\n");
        fs.create_file("project.inlang/.gitignore", "*\n!settings.json\n");
        let included = fs.create_file("project.inlang/settings.json", unformatted);
        let ignored_files = [
            fs.create_file("project.inlang/ignored.json", unformatted),
            fs.create_file("project.inlang/ignored/settings.json", unformatted),
            fs.create_file("dist/settings.json", unformatted),
        ];
        let target_path = fs.working_directory.join(target);

        let result = run_cli_with_dyn_fs(
            Box::new(fs.create_os()),
            &mut console,
            Args::from(["check", "--write", target_path.as_str()].as_slice()),
        );

        assert!(
            result.is_ok(),
            "target {target}: run_cli returned {result:?}"
        );
        assert_eq!(
            std::fs::read_to_string(included).unwrap(),
            formatted,
            "target {target} must process the re-included file"
        );
        for ignored in ignored_files {
            assert_eq!(
                std::fs::read_to_string(&ignored).unwrap(),
                unformatted,
                "target {target} must leave {ignored} ignored"
            );
        }

        let snapshot_name = format!("honors_nested_gitignore_negation_{case}");
        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            &snapshot_name,
            fs.create_mem(),
            console,
            result,
        ));
    }
}

#[test]
fn nested_gitignore_cannot_reinclude_file_below_ignored_parent() {
    let mut fs = TemporaryFs::new("nested_gitignore_cannot_reinclude_file_below_ignored_parent");
    let mut console = BufferConsole::default();
    fs.create_file(
        "biome.json",
        r#"{
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true,
                "root": "."
            }
        }"#,
    );
    fs.create_file(".gitignore", "ignored/\n");
    fs.create_file("ignored/.gitignore", "!file.js\n");
    let ignored = fs.create_file("ignored/file.js", UNFORMATTED);
    let kept = fs.create_file("kept.js", UNFORMATTED);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", "--write", ignored.as_str(), kept.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_eq!(std::fs::read_to_string(ignored).unwrap(), UNFORMATTED);
    assert_ne!(std::fs::read_to_string(kept).unwrap(), UNFORMATTED);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "nested_gitignore_cannot_reinclude_file_below_ignored_parent",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn combines_gitignore_and_ignore_files_at_every_scope() {
    let mut fs = TemporaryFs::new("combines_gitignore_and_ignore_files_at_every_scope");
    let mut console = BufferConsole::default();
    fs.create_file(
        ".gitignore",
        "packages/app/root-git.js\npackages/app/root-precedence.js\n",
    );
    fs.create_file(
        ".ignore",
        "packages/app/root-ignore.js\n!packages/app/root-precedence.js\n",
    );
    fs.create_file(
        "packages/.gitignore",
        "app/ancestor-git.js\napp/ancestor-precedence.js\n",
    );
    fs.create_file(
        "packages/.ignore",
        "app/ancestor-ignore.js\n!app/ancestor-precedence.js\n",
    );
    fs.create_file(
        "packages/app/.gitignore",
        "nested-git.js\nnested-precedence.js\n",
    );
    fs.create_file(
        "packages/app/.ignore",
        "nested-ignore.js\n!nested-precedence.js\n",
    );
    fs.create_file(
        "packages/app/biome.json",
        r#"{
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true,
                "root": "../.."
            }
        }"#,
    );
    let included = [
        fs.create_file("packages/app/included.js", UNFORMATTED),
        fs.create_file("packages/app/root-precedence.js", UNFORMATTED),
        fs.create_file("packages/app/ancestor-precedence.js", UNFORMATTED),
        fs.create_file("packages/app/nested-precedence.js", UNFORMATTED),
    ];
    let ignored = [
        fs.create_file("packages/app/root-git.js", UNFORMATTED),
        fs.create_file("packages/app/root-ignore.js", UNFORMATTED),
        fs.create_file("packages/app/ancestor-git.js", UNFORMATTED),
        fs.create_file("packages/app/ancestor-ignore.js", UNFORMATTED),
        fs.create_file("packages/app/nested-git.js", UNFORMATTED),
        fs.create_file("packages/app/nested-ignore.js", UNFORMATTED),
    ];
    fs.append_to_working_directory("packages/app");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", "--write", fs.cli_path()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    for path in included {
        assert_eq!(std::fs::read_to_string(path).unwrap(), "statement();\n");
    }
    for path in ignored {
        assert_eq!(std::fs::read_to_string(path).unwrap(), UNFORMATTED);
    }
}

#[test]
fn honors_nested_gitignore_in_project_inside_scanner_ignored_ancestor() {
    let mut fs =
        TemporaryFs::new("honors_nested_gitignore_in_project_inside_scanner_ignored_ancestor");
    let mut console = BufferConsole::default();
    fs.create_file(
        ".cache/project/biome.json",
        r#"{
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true
            }
        }"#,
    );
    fs.create_file(".cache/project/.gitignore", "dist/\n");
    fs.create_file(".cache/project/sub/.gitignore", "generated/\n");
    let included = fs.create_file(".cache/project/sub/index.js", UNFORMATTED);
    let ignored = fs.create_file(".cache/project/sub/generated/index.js", UNFORMATTED);
    fs.append_to_working_directory(".cache/project");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", "--write", fs.cli_path()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_eq!(std::fs::read_to_string(included).unwrap(), "statement();\n");
    assert_eq!(
        std::fs::read_to_string(ignored).unwrap(),
        UNFORMATTED,
        "the nested .gitignore must apply even though an ancestor of the project is named .cache"
    );
}

#[test]
fn honors_root_gitignore_directory_negation() {
    let mut fs = TemporaryFs::new("honors_root_gitignore_directory_negation");
    let mut console = BufferConsole::default();
    fs.create_file(
        "biome.jsonc",
        r#"{
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true,
                "defaultBranch": "main"
            }
        }"#,
    );
    fs.create_file(".gitignore", "/*\n!/src\n!/biome.jsonc\n!/.gitignore\n");
    fs.create_file("src/included.js", UNFORMATTED);
    fs.create_file("excluded/ignored.js", UNFORMATTED);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", "--write", fs.cli_path()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    let fs = fs.create_mem();
    assert_file_contents(&fs, Utf8Path::new("src/included.js"), "statement();\n");
    assert_file_contents(&fs, Utf8Path::new("excluded/ignored.js"), UNFORMATTED);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "honors_root_gitignore_directory_negation",
        fs,
        console,
        result,
    ));
}

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
fn ignore_git_info_exclude() {
    let mut fs = TemporaryFs::new("ignore_git_info_exclude");
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

    fs.create_file(".gitignore", "!file2.js\n");
    fs.create_file(".git/info/exclude", "file2.js\nfile3.js\n");

    fs.create_file("file1.js", UNFORMATTED);
    fs.create_file("file2.js", UNFORMATTED);
    fs.create_file("file3.js", UNFORMATTED);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", "--write", fs.cli_path()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_git_info_exclude",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn ignore_git_common_info_exclude() {
    let mut fs = TemporaryFs::new("ignore_git_common_info_exclude");
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

    fs.create_file(".gitignore", "");
    fs.create_file(".git", "gitdir: actual.git/worktrees/current\n");
    fs.create_file("actual.git/worktrees/current/commondir", "../..\n");
    fs.create_file("actual.git/info/exclude", "file2.js\n");

    fs.create_file("file1.js", UNFORMATTED);
    fs.create_file("file2.js", UNFORMATTED);

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", "--write", fs.cli_path()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_git_common_info_exclude",
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
fn should_resolve_gitignore_using_vcs_root() {
    let mut fs = TemporaryFs::new("should_resolve_gitignore_using_vcs_root");
    let mut console = BufferConsole::default();

    // .gitignore is at the repo root (one level up from the workspace)
    fs.create_file(".gitignore", "**/file2.js\n");

    // biome.json is in a subdirectory (workspace), with vcs.root pointing to parent
    fs.create_file(
        "workspace/biome.json",
        r#"{
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true,
                "root": ".."
            }
        }"#,
    );

    // This file should be formatted (not ignored)
    fs.create_file("workspace/file1.js", UNFORMATTED);
    // This file should be ignored by .gitignore at the root
    fs.create_file("workspace/file2.js", UNFORMATTED);

    // Run CLI from the workspace subdirectory
    fs.append_to_working_directory("workspace");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", fs.cli_path()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_resolve_gitignore_using_vcs_root",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn processes_paths_outside_the_vcs_root() {
    let mut fs = TemporaryFs::new("processes_paths_outside_the_vcs_root");
    let mut console = BufferConsole::default();

    fs.create_file("engine/.gitignore", "ignored.js\n");
    fs.create_file(
        "engine/biome.json",
        r#"{
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true,
                "root": "."
            }
        }"#,
    );
    fs.create_file("outside/file.js", UNFORMATTED);
    fs.append_to_working_directory("engine");
    let outside_path = fs.working_directory.join("../outside/file.js");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", "--write", outside_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    let fs = fs.create_mem();
    assert_file_contents(&fs, Utf8Path::new("outside/file.js"), "statement();\n");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "processes_paths_outside_the_vcs_root",
        fs,
        console,
        result,
    ));
}

#[test]
fn honors_vcs_ignore_for_sibling_paths() {
    let mut fs = TemporaryFs::new("honors_vcs_ignore_for_sibling_paths");
    let mut console = BufferConsole::default();

    fs.create_file(".gitignore", "plans/root-ignored.js\n");
    fs.create_file("plans/.gitignore", "nested-ignored.js\n");
    fs.create_file(
        "engine/biome.json",
        r#"{
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true,
                "root": ".."
            }
        }"#,
    );
    fs.create_file("plans/included.js", UNFORMATTED);
    fs.create_file("plans/root-ignored.js", UNFORMATTED);
    fs.create_file("plans/nested-ignored.js", UNFORMATTED);
    fs.append_to_working_directory("engine");
    let plans_path = fs.working_directory.join("../plans");

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", "--write", plans_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    let fs = fs.create_mem();
    assert_file_contents(&fs, Utf8Path::new("plans/included.js"), "statement();\n");
    assert_file_contents(&fs, Utf8Path::new("plans/root-ignored.js"), UNFORMATTED);
    assert_file_contents(&fs, Utf8Path::new("plans/nested-ignored.js"), UNFORMATTED);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "honors_vcs_ignore_for_sibling_paths",
        fs,
        console,
        result,
    ));
}

#[test]
fn normalizes_stdin_paths_before_vcs_matching() {
    let mut fs = TemporaryFs::new("normalizes_stdin_paths_before_vcs_matching");
    let mut console = BufferConsole::default();

    fs.create_file(".gitignore", "dist/\n");
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
    console.in_buffer.push(UNFORMATTED.to_string());

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["format", "--stdin-file-path", "missing/../dist/file.js"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    let output = console
        .out_buffer
        .first()
        .expect("the original content should be printed");
    assert_eq!(markup_to_string(markup!({ output.content })), UNFORMATTED);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "normalizes_stdin_paths_before_vcs_matching",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn processes_symlink_with_target_outside_the_project() {
    let mut fs = TemporaryFs::new("processes_symlink_with_target_outside_the_project");
    let mut console = BufferConsole::default();

    fs.create_file("project/.gitignore", "dist\n");
    fs.create_file(
        "project/biome.json",
        r#"{
            "vcs": {
                "enabled": true,
                "clientKind": "git",
                "useIgnoreFile": true
            },
            "linter": {
                "rules": {
                    "suspicious": {
                        "noDebugger": "warn"
                    }
                }
            }
        }"#,
    );
    let target = fs.create_file("shared/file.js", "debugger;\n");
    let link = fs.working_directory.join("project/file.js");

    #[cfg(unix)]
    std::os::unix::fs::symlink(&target, &link).unwrap();
    #[cfg(windows)]
    if let Err(error) = std::os::windows::fs::symlink_file(&target, &link) {
        if option_env!("CI") == Some("1") {
            panic!("failed to create symlink: {error}");
        }
        return;
    }

    fs.append_to_working_directory("project");
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    let emitted = console
        .out_buffer
        .iter()
        .map(|message| markup_to_string(markup!({ &message.content })))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(emitted.contains("file.js:1:1"), "{emitted}");
    assert!(!emitted.contains("shared/file.js"), "{emitted}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "processes_symlink_with_target_outside_the_project",
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
