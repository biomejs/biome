use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const UNORGANIZED: &str = r#"import * as something from "../something";
import { lorem, foom, bar } from "foo";"#;
const ORGANIZED: &str = r#"import { bar, foom, lorem } from "foo";
import * as something from "../something";"#;

#[test]
fn does_handle_included_file_and_disable_organize_imports() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "includes": ["test.js", "special/**"]
  },
  "overrides": [{ "includes": ["special/**"], "assist": { "enabled": false } }]
}

"#
        .as_bytes(),
    );

    let test = Utf8Path::new("test.js");
    fs.insert(test.into(), UNORGANIZED.as_bytes());

    let test2 = Utf8Path::new("special/test2.js");
    fs.insert(test2.into(), UNORGANIZED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--write",
                "--formatter-enabled=false",
                "--linter-enabled=false",
                test.as_str(),
                test2.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, UNORGANIZED);
    assert_file_contents(&fs, test, ORGANIZED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_handle_included_file_and_disable_organize_imports",
        fs,
        console,
        result,
    ));
}
