use crate::TestArgs as Args;
use crate::snap_test::SnapshotPayload;
use crate::{assert_cli_snapshot, run_cli};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use std::process::Command;

#[test]
fn completion_prints_script_for_shell() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["completion", "zsh"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "completion_prints_script_for_shell",
        fs,
        console,
        result,
    ));
}

#[test]
fn completion_write_installs_script() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["completion", "--write", "zsh"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "completion_write_installs_script",
        fs,
        console,
        result,
    ));
}

// This test is needed to keep bpaf scripts inline with ours. If bpaf changes them, this fails
// and we need to update our scripts.
#[test]
fn completion_scripts_match_bpaf() {
    let biome = env!("CARGO_BIN_EXE_biome");
    let stdout = |args: &[&str]| {
        let output = Command::new(biome).args(args).output().unwrap();
        assert!(output.status.success(), "biome {args:?} failed: {output:?}");
        String::from_utf8(output.stdout).unwrap()
    };

    for shell in ["bash", "zsh", "fish", "elvish"] {
        // bpaf names the program after the executable file, which has an
        // `.exe` extension on Windows.
        let expected =
            stdout(&[&format!("--bpaf-complete-style-{shell}")]).replace("biome.exe", "biome");
        assert_eq!(
            stdout(&["completion", shell]),
            expected,
            "script for {shell}"
        );
    }
}
