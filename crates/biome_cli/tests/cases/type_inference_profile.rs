use crate::run_cli_with_server_workspace;
use crate::snap_test::markup_to_string;
use biome_console::{BufferConsole, markup};
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use serial_test::serial;

fn run_lint(extra_arguments: &[&str]) -> String {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let file_path = "file.ts";
    fs.insert(
        file_path.into(),
        "async function returnsPromise() {}\nreturnsPromise();\n",
    );
    let mut arguments = vec!["lint", "--only=nursery/noFloatingPromises"];
    arguments.extend_from_slice(extra_arguments);
    arguments.push(file_path);

    let (_fs, _result) =
        run_cli_with_server_workspace(fs, &mut console, Args::from(arguments.as_slice()));

    console
        .out_buffer
        .iter()
        .map(|message| markup_to_string(markup! {{ message.content }}))
        .collect()
}

fn run_profile(extra_arguments: &[&str]) -> String {
    let mut arguments = vec!["--profile-type-inference"];
    arguments.extend_from_slice(extra_arguments);
    run_lint(&arguments)
}

#[test]
#[serial]
fn reports_compact_profile_with_relative_text_ranges() {
    let output = run_profile(&[]);

    assert!(output.contains("Type inference profile"), "{output}");
    assert!(output.contains("Showing ranked aggregates"), "{output}");
    assert!(output.contains("Requests by consumer"), "{output}");
    assert!(output.contains("Query bodies"), "{output}");
    assert!(output.contains("file.ts:"), "{output}");
    assert!(!output.contains("Hot inference queries"), "{output}");
    assert!(!output.contains("Code references"), "{output}");
}

#[test]
#[serial]
fn verbose_profile_reports_every_source_record() {
    for arguments in [&["--verbose"][..], &["--reporter=summary", "--verbose"][..]] {
        let output = run_profile(arguments);

        assert!(output.contains("Hot request origins"), "{output}");
        assert!(output.contains("Hot inference queries"), "{output}");
        assert!(output.contains("Interpretation"), "{output}");
        assert!(output.contains("Code references"), "{output}");
        assert!(!output.contains("Showing ranked aggregates"), "{output}");
    }
}

#[test]
#[serial]
fn rule_profiler_is_scoped_to_requested_run() {
    biome_analyze::profiling::disable();
    biome_analyze::profiling::reset();

    let profiled_output = run_lint(&["--profile-rules"]);
    assert!(
        profiled_output.contains("Rule execution time"),
        "{profiled_output}"
    );

    let regular_output = run_lint(&[]);
    assert!(
        !regular_output.contains("Rule execution time"),
        "{regular_output}"
    );
    assert!(!biome_analyze::profiling::is_enabled());
}

#[test]
#[serial]
fn reports_profile_to_console_with_a_file_reporter() {
    let output = run_profile(&["--reporter=json", "--reporter-file=report.json"]);

    assert!(output.contains("Type inference profile"), "{output}");
}

#[test]
#[serial]
fn reports_profile_once_with_multiple_reporters() {
    let output = run_profile(&["--reporter=summary", "--reporter=concise"]);

    assert_eq!(
        output.matches("Type inference profile").count(),
        1,
        "{output}"
    );
}

#[test]
#[serial]
fn rejects_unsupported_execution_modes() {
    for arguments in [
        vec![
            "lint",
            "--profile-type-inference",
            "--use-server",
            "file.ts",
        ],
        vec![
            "lint",
            "--profile-type-inference",
            "--stdin-file-path=file.ts",
        ],
    ] {
        let fs = MemoryFileSystem::default();
        fs.insert("file.ts".into(), "const value = 1;\n");
        let mut console = BufferConsole::default();
        let (_fs, result) =
            run_cli_with_server_workspace(fs, &mut console, Args::from(arguments.as_slice()));
        assert!(
            matches!(
                result,
                Err(biome_cli::CliDiagnostic::IncompatibleArguments(_))
            ),
            "unsupported mode must report incompatible arguments: {arguments:?}"
        );
    }
}
