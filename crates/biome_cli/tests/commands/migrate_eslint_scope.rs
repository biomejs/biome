use crate::TestArgs as Args;
use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

#[test]
fn migrate_scope_decisions() {
    let spec: serde_json::Value = serde_json::from_str(include_str!(
        "../specs/migrate_eslint/scopeDecisions/basic.jsonc"
    ))
    .unwrap();
    let eslint = serde_json::to_vec(&spec["eslint"]).unwrap();

    for (name, args) in [
        ("default", vec!["migrate", "eslint", "--write"]),
        (
            "include_inspired",
            vec!["migrate", "eslint", "--include-inspired", "--write"],
        ),
        (
            "include_inspired_and_nursery",
            vec![
                "migrate",
                "eslint",
                "--include-inspired",
                "--include-nursery",
                "--write",
            ],
        ),
        (
            "include_nursery",
            vec!["migrate", "eslint", "--include-nursery", "--write"],
        ),
    ] {
        let fs = MemoryFileSystem::default();
        fs.insert(Utf8Path::new("biome.json").into(), b"{}");
        fs.insert(Utf8Path::new(".eslintrc.json").into(), eslint.as_slice());
        let mut console = BufferConsole::default();
        let (fs, result) = run_cli(fs, &mut console, Args::from(args.as_slice()));
        assert!(result.is_ok(), "run_cli returned {result:?}");
        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            name,
            fs,
            console,
            result,
        ));
    }
}
