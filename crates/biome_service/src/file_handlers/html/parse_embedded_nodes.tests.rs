use crate::Workspace;
use crate::settings::ModuleGraphResolutionKind;
use crate::test_utils::setup_workspace_and_open_project;
use crate::workspace::{FileContent, OpenFileParams, UpdateSettingsParams};
use biome_configuration::{Configuration, HtmlConfiguration};
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_parser::prelude::ParseDiagnostic;
use camino::{Utf8Path, Utf8PathBuf};

fn prepare(file: &str, content: &str) -> Vec<ParseDiagnostic> {
    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(file), content);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                html: Some(HtmlConfiguration {
                    experimental_full_support_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(file),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
        })
        .unwrap();

    workspace
        .get_parse_diagnostics(Utf8Path::new(file))
        .unwrap()
}

fn assert_no_diagnostics(file: &str, content: &str) {
    let diagnostics = prepare(file, content);
    assert!(
        diagnostics.is_empty(),
        "Expected no parse errors for typed Svelte snippet, got: {diagnostics:#?}"
    );
}

fn assert_diagnostics(file: &str, content: &str) {
    let diagnostics = prepare(file, content);

    assert!(
        !diagnostics.is_empty(),
        "Expected diagnostics, but none were emitted"
    );
}

#[test]
fn snippet_svelte_ts_parsing() {
    const FILE_PATH: &str = "/project/file.svelte";
    const FILE_CONTENT: &str = r#"<script lang="ts">
	let name = $state('world');
</script>

<h1>Hello {name}!</h1>

{#snippet add(a: any, b: float)}
	{a} + {b} = {a + b}
{/snippet}

{@render add(1, 2)}
"#;

    assert_no_diagnostics(FILE_PATH, FILE_CONTENT);
}

#[test]
fn snippet_svelte_incorrect() {
    const FILE_PATH: &str = "/project/file.svelte";
    const FILE_CONTENT: &str = r#"<script lang="ts">
	let name = $state('world');
</script>

<h1>Hello {name}!</h1>

{#snippet add}
	{a} + {b} = {a + b}
{/snippet}

{@render add(1, 2)}
"#;

    assert_diagnostics(FILE_PATH, FILE_CONTENT);
}

#[test]
fn svelte_each_with_correct_method_call_key() {
    const FILE_PATH: &str = "/project/file.svelte";
    const FILE_CONTENT: &str = r#"<script lang="ts">
    const numbers = [1, 2, 3, 4];
</script>
{#each numbers as number, index (number.toString())}
  <p>{number}</p>
{/each}
"#;

    assert_no_diagnostics(FILE_PATH, FILE_CONTENT);
}

#[test]
fn svelte_each_with_incorrect_method_call_key() {
    const FILE_PATH: &str = "/project/file.svelte";
    const FILE_CONTENT: &str = r#"<script lang="ts">
    const numbers = [1, 2, 3, 4];
</script>
{#each numbers as number, index (number.toString(})}
  <p>{number}</p>
{/each}
"#;

    assert_diagnostics(FILE_PATH, FILE_CONTENT);
}

#[test]
fn svelte_interpolation_ranges_are_brace_and_string_aware() {
    use super::svelte_interpolation_ranges;
    use biome_rowan::{TextRange, TextSize};

    fn slices(input: &str) -> Vec<&str> {
        svelte_interpolation_ranges(input)
            .into_iter()
            .map(|r| &input[r])
            .collect()
    }

    assert_eq!(slices("top: {top}px"), vec!["top"]);
    assert_eq!(slices("a {x} b {y} c"), vec!["x", "y"]);
    // A `}` inside a string doesn't end the group.
    assert_eq!(slices("{ ok ? 'a}b' : c }"), vec![" ok ? 'a}b' : c "]);
    // Nested braces.
    assert_eq!(slices("{ {x: 1} }"), vec![" {x: 1} "]);
    assert!(slices("plain text").is_empty());
    // Ranges point at the inner expression, not the braces.
    assert_eq!(
        svelte_interpolation_ranges("ab{cd}"),
        vec![TextRange::new(TextSize::from(3), TextSize::from(5))]
    );
}
