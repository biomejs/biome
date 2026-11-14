use crate::configs::CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC;
use crate::run_cli;
use crate::snap_test::{assert_file_contents, markup_to_string};
use biome_console::{BufferConsole, LogLevel, markup};
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn clean_input_succeeds() {
    for (command, write) in [
        ("lint", false),
        ("check", false),
        ("format", false),
        ("lint", true),
        ("check", true),
        ("format", true),
    ] {
        for disk_content in [None, Some("export const value = 2;\n"), Some("debugger;\n")] {
            let fs = MemoryFileSystem::default();
            let mut console = BufferConsole::default();
            let source = "export const value = 1;\n";
            let path = Utf8Path::new("example.ts");
            if let Some(content) = disk_content {
                fs.insert(path.into(), content.as_bytes());
            }
            console.in_buffer.push(source.into());

            let mut args = vec![command, "--stdin-file-path=example.ts"];
            if write {
                args.push("--write");
            }
            let (fs, result) = run_cli(fs, &mut console, Args::from(args.as_slice()));

            assert!(result.is_ok(), "{command}: {result:?}");
            assert_eq!(console.out_buffer.len(), 1);
            let message = &console.out_buffer[0];
            assert_eq!(message.level, LogLevel::Log);
            assert_eq!(markup_to_string(markup! {{message.content}}), source);
            if let Some(content) = disk_content {
                assert_file_contents(&fs, path, content);
            }
        }
    }
}

#[test]
fn formatting_differences() {
    let source = "export const value=1";
    for (command, options, succeeds, output) in [
        ("lint", vec![], true, source),
        ("check", vec![], false, source),
        ("check", vec!["--formatter-enabled=false"], true, source),
        ("check", vec!["--write"], true, "export const value = 1;\n"),
        ("format", vec![], true, "export const value = 1;\n"),
    ] {
        let fs = MemoryFileSystem::default();
        fs.insert("example.ts".into(), "export const value = 2;\n");
        let mut console = BufferConsole::default();
        console.in_buffer.push(source.into());
        let mut args = vec![command, "--stdin-file-path=example.ts"];
        args.extend(options);
        let (fs, result) = run_cli(fs, &mut console, Args::from(args.as_slice()));

        assert_eq!(result.is_ok(), succeeds, "{args:?}: {result:?}");
        assert_eq!(console.out_buffer.len(), 1);
        let message = &console.out_buffer[0];
        assert_eq!(message.level, LogLevel::Log);
        assert_eq!(markup_to_string(markup! {{message.content}}), output);
        assert_file_contents(
            &fs,
            Utf8Path::new("example.ts"),
            "export const value = 2;\n",
        );
    }
}

#[test]
fn errors_fail_even_when_diagnostics_are_capped() {
    for (command, write) in [
        ("lint", false),
        ("check", false),
        ("lint", true),
        ("check", true),
    ] {
        for source in ["debugger;\n", "const = ;\n"] {
            for max_diagnostics in ["--max-diagnostics=0", "--max-diagnostics=20"] {
                let fs = MemoryFileSystem::default();
                let mut console = BufferConsole::default();
                console.in_buffer.push(source.into());
                let mut args = vec![command, "--stdin-file-path=example.ts", max_diagnostics];
                if write {
                    args.push("--write");
                }
                let (_, result) = run_cli(fs, &mut console, Args::from(args.as_slice()));

                assert!(result.is_err(), "{command}: {result:?}");
                let message = &console.out_buffer[0];
                assert_eq!(message.level, LogLevel::Log);
                assert_eq!(markup_to_string(markup! {{message.content}}), source);
                if max_diagnostics == "--max-diagnostics=20" {
                    assert!(
                        console
                            .out_buffer
                            .iter()
                            .any(|message| message.level == LogLevel::Error)
                    );
                }
            }
        }
    }
}

#[test]
fn warnings_respect_error_on_warnings() {
    for command in ["lint", "check"] {
        for error_on_warnings in [false, true] {
            let fs = MemoryFileSystem::default();
            fs.insert("biome.json".into(), CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC);
            let mut console = BufferConsole::default();
            console.in_buffer.push("debugger;\n".into());
            let mut args = vec![command, "--stdin-file-path=example.ts"];
            if error_on_warnings {
                args.push("--error-on-warnings");
            }
            let (_, result) = run_cli(fs, &mut console, Args::from(args.as_slice()));

            assert_eq!(result.is_err(), error_on_warnings, "{command}: {result:?}");
            assert!(console.out_buffer.iter().any(|message| {
                message.level == LogLevel::Error
                    && markup_to_string(markup! {{message.content}})
                        .contains("suspicious/noDebugger")
            }));
        }
    }
}

#[test]
fn check_reports_diagnostics_against_original_input() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    console.in_buffer.push("\n\n  debugger;".into());
    let (_, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--stdin-file-path=example.ts"].as_slice()),
    );

    assert!(result.is_err());
    assert!(console.out_buffer.iter().any(|message| {
        markup_to_string(markup! {{message.content}}).contains("example.ts:3:3")
    }));
}

#[test]
fn format_rejects_parse_errors() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    console.in_buffer.push("const = ;\n".into());
    let (_, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--stdin-file-path=example.ts"].as_slice()),
    );

    assert!(result.is_err());
    assert!(console.out_buffer.iter().any(|message| {
        message.level == LogLevel::Error
            && markup_to_string(markup! {{message.content}}).contains("parse")
    }));
}

#[test]
fn write_fails_when_errors_remain_after_fixes() {
    for command in ["lint", "check"] {
        for max_diagnostics in ["--max-diagnostics=0", "--max-diagnostics=20"] {
            let fs = MemoryFileSystem::default();
            fs.insert(
                "biome.json".into(),
                r#"{
                "assist": { "enabled": false },
                "linter": { "rules": {
                    "recommended": false,
                    "complexity": { "noUselessRename": "error" },
                    "suspicious": { "noDebugger": "error" }
                } }
            }"#,
            );
            fs.insert("example.ts".into(), "export const disk = 1;\n");
            let mut console = BufferConsole::default();
            console
                .in_buffer
                .push("import { a as a } from \"mod\"; export { a }; debugger;\n".into());

            let (fs, result) = run_cli(
                fs,
                &mut console,
                Args::from(
                    [
                        command,
                        "--write",
                        "--stdin-file-path=example.ts",
                        max_diagnostics,
                    ]
                    .as_slice(),
                ),
            );

            assert!(result.is_err(), "{command}: {result:?}");
            let message = &console.out_buffer[0];
            assert_eq!(message.level, LogLevel::Log);
            let output = markup_to_string(markup! {{message.content}});
            let (expected, location) = if command == "lint" {
                (
                    "import { a } from \"mod\"; export { a }; debugger;\n",
                    "example.ts:1:40",
                )
            } else {
                (
                    "import { a } from \"mod\";\nexport { a };\ndebugger;\n",
                    "example.ts:3:1",
                )
            };
            assert_eq!(output, expected);
            if max_diagnostics == "--max-diagnostics=20" {
                assert!(console.out_buffer.iter().any(|message| {
                    let diagnostic = markup_to_string(markup! {{message.content}});
                    message.level == LogLevel::Error
                        && diagnostic.contains("suspicious/noDebugger")
                        && diagnostic.contains(location)
                }));
            }
            assert_file_contents(&fs, Utf8Path::new("example.ts"), "export const disk = 1;\n");
        }
    }
}

#[test]
fn write_succeeds_when_all_errors_are_fixed() {
    for command in ["lint", "check"] {
        let fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();
        console.in_buffer.push("debugger;\n".into());
        let (_, result) = run_cli(
            fs,
            &mut console,
            Args::from(
                [
                    command,
                    "--write",
                    "--unsafe",
                    "--stdin-file-path=example.ts",
                ]
                .as_slice(),
            ),
        );

        assert!(result.is_ok(), "{command}: {result:?}");
        assert_eq!(console.out_buffer.len(), 1);
        let message = &console.out_buffer[0];
        assert_eq!(message.level, LogLevel::Log);
        assert!(
            markup_to_string(markup! {{message.content}})
                .trim()
                .is_empty()
        );
    }
}

#[test]
fn write_warnings_respect_error_on_warnings() {
    for command in ["lint", "check"] {
        for error_on_warnings in [false, true] {
            let fs = MemoryFileSystem::default();
            fs.insert(
                "biome.json".into(),
                r#"{
                "assist": { "enabled": false },
                "linter": { "rules": {
                    "recommended": false,
                    "complexity": { "noUselessRename": "error" },
                    "suspicious": { "noDebugger": "warn" }
                } }
            }"#,
            );
            let mut console = BufferConsole::default();
            console
                .in_buffer
                .push("import { a as a } from \"mod\"; export { a }; debugger;\n".into());
            let mut args = vec![command, "--write", "--stdin-file-path=example.ts"];
            if error_on_warnings {
                args.push("--error-on-warnings");
            }
            let (_, result) = run_cli(fs, &mut console, Args::from(args.as_slice()));

            assert_eq!(result.is_err(), error_on_warnings, "{command}: {result:?}");
            let message = &console.out_buffer[0];
            let output = markup_to_string(markup! {{message.content}});
            assert!(!output.contains("as a"));
            assert!(output.contains("debugger;"));
            assert!(console.out_buffer.iter().any(|message| {
                message.level == LogLevel::Error
                    && markup_to_string(markup! {{message.content}})
                        .contains("suspicious/noDebugger")
            }));
        }
    }
}

#[test]
fn skip_parse_errors_preserves_input() {
    for command in ["lint", "check"] {
        for write in [false, true] {
            for max_diagnostics in ["--max-diagnostics=0", "--max-diagnostics=20"] {
                let fs = MemoryFileSystem::default();
                let mut console = BufferConsole::default();
                let source = "const = ;\ndebugger;\n";
                console.in_buffer.push(source.into());
                let mut args = vec![
                    command,
                    "--stdin-file-path=example.ts",
                    "--skip-parse-errors",
                    max_diagnostics,
                ];
                if write {
                    args.extend(["--write", "--unsafe"]);
                }
                let (_, result) = run_cli(fs, &mut console, Args::from(args.as_slice()));

                assert!(result.is_ok(), "{args:?}: {result:?}");
                assert_eq!(console.out_buffer.len(), 1);
                let message = &console.out_buffer[0];
                assert_eq!(message.level, LogLevel::Log);
                assert_eq!(markup_to_string(markup! {{message.content}}), source);
            }
        }
    }
}
