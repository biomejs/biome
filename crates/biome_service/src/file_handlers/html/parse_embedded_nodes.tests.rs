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
            editor_features: None,
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
fn vue_v_on_accepts_inline_statements_and_expression_handlers() {
    // Mirrors Vue's v-on handler cases:
    // https://github.com/vuejs/core/blob/86ad0764fd9f7b01cef75b4fc941b03419306bf8/packages/compiler-core/__tests__/transforms/vOn.spec.ts#L148-L161
    // https://github.com/vuejs/core/blob/86ad0764fd9f7b01cef75b4fc941b03419306bf8/packages/compiler-core/__tests__/transforms/vOn.spec.ts#L163-L179
    // https://github.com/vuejs/core/blob/86ad0764fd9f7b01cef75b4fc941b03419306bf8/packages/compiler-core/__tests__/transforms/vOn.spec.ts#L181-L197
    // https://github.com/vuejs/core/blob/86ad0764fd9f7b01cef75b4fc941b03419306bf8/packages/compiler-core/__tests__/transforms/vOn.spec.ts#L260-L273
    // https://github.com/vuejs/core/blob/86ad0764fd9f7b01cef75b4fc941b03419306bf8/packages/compiler-core/__tests__/transforms/vOn.spec.ts#L305-L320
    // https://github.com/vuejs/core/blob/86ad0764fd9f7b01cef75b4fc941b03419306bf8/packages/compiler-core/__tests__/transforms/vOn.spec.ts#L347-L370
    // https://github.com/vuejs/core/blob/86ad0764fd9f7b01cef75b4fc941b03419306bf8/packages/compiler-core/__tests__/transforms/vOn.spec.ts#L372-L385
    const FILE_PATH: &str = "/project/file.vue";
    const FILE_CONTENT: &str = r#"<script setup>
let counter = 0;
function foo() {}
function bar() {}
const a = { b: foo };
const c = "b";
</script>
<template>
  <button @click="counter++; counter++;"></button>
  <button @click="foo();bar()"></button>
  <button @click="
    counter++;
    counter++;
  "></button>
  <button @click="foo"></button>
  <button @click="a['b' + c]"></button>
  <button @click="$event => foo($event)"></button>
  <button @click="async $event => foo($event)"></button>
  <button @click="function($event) { foo($event) }"></button>
  <button v-on:click="counter++; counter++;"></button>
</template>
"#;

    assert_no_diagnostics(FILE_PATH, FILE_CONTENT);
}

#[test]
fn vue_v_on_accepts_typescript_function_handler() {
    // Mirrors Vue's TypeScript function-expression v-on handler case:
    // https://github.com/vuejs/core/blob/86ad0764fd9f7b01cef75b4fc941b03419306bf8/packages/compiler-core/__tests__/transforms/vOn.spec.ts#L275-L303
    const FILE_PATH: &str = "/project/file.vue";
    const FILE_CONTENT: &str = r#"<script lang="ts">
let counter = 0;
</script>
<template>
  <button @click="($event: MouseEvent) => counter++"></button>
</template>
"#;

    assert_no_diagnostics(FILE_PATH, FILE_CONTENT);
}

#[test]
fn vue_v_on_rejects_function_handler_with_trailing_statement() {
    // Enforces Vue's split between function-expression handlers and inline
    // statement handlers by combining these upstream cases:
    // https://github.com/vuejs/core/blob/86ad0764fd9f7b01cef75b4fc941b03419306bf8/packages/compiler-core/__tests__/transforms/vOn.spec.ts#L260-L273
    // https://github.com/vuejs/core/blob/86ad0764fd9f7b01cef75b4fc941b03419306bf8/packages/compiler-core/__tests__/transforms/vOn.spec.ts#L163-L179
    const FILE_PATH: &str = "/project/file.vue";
    const FILE_CONTENT: &str = r#"<template>
  <button @click="() => {counter++}; counter++;"></button>
</template>
"#;

    assert_diagnostics(FILE_PATH, FILE_CONTENT);
}

#[test]
fn vue_non_event_directives_stay_expression_only() {
    const FILE_PATH: &str = "/project/file.vue";
    const FILE_CONTENT: &str = r#"<script setup>
const isActive = true;
const foo = "foo";
const bar = "bar";
const duration = 100;
</script>
<template>
  <div :class="{ active: isActive }"></div>
  <div v-bind="{ id: foo, class: bar }"></div>
  <div>{{ { duration } }}</div>
</template>
"#;

    assert_no_diagnostics(FILE_PATH, FILE_CONTENT);
}
