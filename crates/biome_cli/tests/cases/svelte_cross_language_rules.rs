use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const BIOME_CONFIG_HTML_FULL_SUPPORT: &str =
    r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#;

#[test]
fn no_undeclared_variables_not_triggered_for_exported_module_script_functions() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.svelte");
    fs.insert(
        file.into(),
        r#"
<script lang="ts" module>
	export const someFunction = () => {
		console.log("hai!");
	};

	const someOtherFunction = () => {
		console.log("heyyy");
	};
</script>

<script lang="ts">
	someFunction();
	someOtherFunction();
</script>
"#
        .as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUndeclaredVariables", file.as_str()].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_undeclared_variables_not_triggered_for_exported_module_script_functions",
        fs,
        console,
        result,
    ));
}
