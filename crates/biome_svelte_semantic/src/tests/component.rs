use super::{component, component_at};
use crate::{
    Access, Resolution, RouteFile, Rune, RuneTarget, StaticValue, StoreCreator, SymbolKind,
};

const TODO_LIST: &str = r#"<script module lang="ts">
  export const prerender = true;
</script>

<script lang="ts">
  import { writable } from 'svelte/store';
  import Item from './Item.svelte';

  interface Props { title: string; items: Todo[]; onselect?: (id: number) => void }
  let { title, items = $bindable([]), ...rest }: Props = $props();
  let filter = $state('');
  let visible = $derived(items.filter((item) => item.text.includes(filter)));
  const count = writable(0);
  const prefix = '/todos/';
</script>

<h1 {title}>{title} ({$count})</h1>
<input bind:value={filter} />
{#each visible as { id, text }, index (id)}
  <Item {text} href="{prefix}{id}" onclick={() => rest.onselect?.(id)} />
  <input bind:value={visible[index].text} />
{:else}
  <p>{@render empty()}</p>
{/each}

{#snippet empty()}
  {@const message = 'Nothing to do'}
  <em>{message}</em>
{/snippet}
"#;

#[test]
fn worked_example() {
    insta::assert_snapshot!(component(TODO_LIST).to_string());
}

/// Renders the references of a model, one per line.
fn references(code: &str) -> String {
    component(code)
        .to_string()
        .lines()
        .filter(|line| line.starts_with("ref "))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn runes_mode_depends_on_runes_options_and_await() {
    assert!(!component("<script>let count = 0;</script>{count}").is_runes_mode());
    assert!(component("<script>let count = $state(0);</script>{count}").is_runes_mode());
    assert!(component("<p>{$inspect(1)}</p>").is_runes_mode());
    assert!(component("<svelte:options runes />").is_runes_mode());
    assert!(component("<svelte:options runes={true} />").is_runes_mode());
    assert!(
        !component("<svelte:options runes={false} /><script>let a = $state(0);</script>")
            .is_runes_mode()
    );
    assert!(component("<script>const data = await load();</script>").is_runes_mode());
    assert!(!component("<script>async function f() { await load(); }</script>").is_runes_mode());
}

#[test]
fn template_variables_are_scoped_to_their_block() {
    insta::assert_snapshot!(references(
        r#"<script>let item = $state(); let list = $state([]);</script>
{#each list as item, i (item.id)}{item}{i}{:else}{item}{/each}
{#await list then { item: value = item }}{value}{:catch error}{error}{/await}
{#if list}{@const total = list.length}{total}{:else}{total}{/if}
"#
    ));
}

#[test]
fn snippets_are_hoisted_and_visible_to_the_instance_script() {
    insta::assert_snapshot!(references(
        r#"<script>
  const rows = $state([]);
  export { row };
  const render = () => row;
</script>
{@render row(rows[0])}
{#snippet row(value, fallback = rows)}<p>{value}{fallback}{@render cell()}</p>{/snippet}
{#snippet cell()}<td />{/snippet}
<div>{#snippet hidden()}{/snippet}</div>
"#
    ));
}

#[test]
fn module_bindings_are_visible_to_the_instance_but_not_the_reverse() {
    insta::assert_snapshot!(references(
        r#"<script module>
  export const shared = 1;
  console.log(local);
</script>
<script>
  let local = shared;
</script>
{shared}{local}
"#
    ));
}

#[test]
fn store_subscriptions_resolve_to_the_store() {
    let model = component(
        r#"<script>
  import { writable } from 'svelte/store';
  import * as stores from 'svelte/store';
  const count = writable(0);
  const other = stores.readable(1);
  $count = 2;
</script>
<button onclick={() => $count++}>{$count}{$other}</button>
"#,
    );
    let count = model
        .symbols()
        .find(|symbol| symbol.name() == "count")
        .unwrap();
    assert_eq!(count.store_creator(), Some(StoreCreator::Writable));
    assert!(!count.is_reassigned());
    let subscriptions: Vec<_> = count
        .uses()
        .filter(|reference| reference.is_store_subscription())
        .map(|reference| (reference.name().to_string(), reference.access()))
        .collect();
    assert_eq!(
        subscriptions,
        [
            ("$count".to_string(), Access::Write),
            ("$count".to_string(), Access::Write),
            ("$count".to_string(), Access::Read),
        ]
    );
    let other = model
        .symbols()
        .find(|symbol| symbol.name() == "other")
        .unwrap();
    assert_eq!(other.store_creator(), Some(StoreCreator::Readable));
}

#[test]
fn rune_calls_record_their_target() {
    let model = component(
        r#"<script>
  class Counter {
    count = $state(0);
    constructor() { this.total = $state.raw(0); }
  }
  let { a, b: renamed = 1 } = $props();
  const doubled = $derived.by(() => a * 2);
  $effect(() => console.log(doubled));
  $effect.pre(() => {});
  $state.foo();
</script>
"#,
    );
    let calls: Vec<_> = model
        .rune_calls()
        .map(|call| (call.rune(), call.target().clone(), call.symbols().count()))
        .collect();
    assert_eq!(
        calls,
        [
            (Rune::State, RuneTarget::ClassField("count".into()), 0),
            (
                Rune::StateRaw,
                RuneTarget::ConstructorField("total".into()),
                0
            ),
            (Rune::Props, RuneTarget::Variable, 2),
            (Rune::DerivedBy, RuneTarget::Variable, 1),
            (Rune::Effect, RuneTarget::Other, 0),
            (Rune::EffectPre, RuneTarget::Other, 0),
        ]
    );
    let renamed = model
        .symbols()
        .find(|symbol| symbol.name() == "renamed")
        .unwrap();
    assert_eq!(renamed.rune(), Some(Rune::Props));
}

#[test]
fn props_come_from_the_pattern_and_the_type() {
    let model = component(
        r#"<script lang="ts">
  interface Base { id: string }
  type Props = Base & { label: string; onclick(): void };
  let { label, value = $bindable(), ...rest }: Props = $props();
</script>
{label}{rest.id}
"#,
    );
    insta::assert_snapshot!(
        model
            .to_string()
            .lines()
            .filter(|line| line.starts_with("prop"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn imported_prop_types_are_open() {
    let model = component(
        r#"<script lang="ts">
  import type { Props } from './types';
  let { label }: Props = $props();
</script>
"#,
    );
    assert!(!model.props_declaration().unwrap().is_complete());
}

#[test]
fn props_object_members_are_uses_of_the_props() {
    let model = component(
        r#"<script>
  let props = $props();
</script>
<p>{props.title}</p>
"#,
    );
    let declaration = model.props_declaration().unwrap();
    assert_eq!(declaration.object().unwrap().name(), "props");
    let reference = declaration.object().unwrap().uses().next().unwrap();
    assert_eq!(reference.path().collect::<Vec<_>>(), ["title"]);
}

#[test]
fn markup_names_refer_to_bindings() {
    insta::assert_snapshot!(references(
        r#"<script>
  import { fade } from 'svelte/transition';
  import * as ui from './ui';
  import tooltip from './tooltip';
  let value = $state('');
  let active = $state(false);
</script>
<ui.Button.Primary transition:fade use:tooltip.show={'x'} />
<input bind:value class:active class:other-name style:color />
"#
    ));
}

#[test]
fn bind_values_are_writes() {
    insta::assert_snapshot!(references(
        r#"<script>
  let user = $state({});
  let element;
  let value = $state();
</script>
<input bind:value={user.name} bind:this={element} />
<input bind:value={() => value, (next) => (value = next)} />
"#
    ));
}

#[test]
fn changes_through_each_items_change_the_list() {
    let model = component(
        r#"<script>
  let todos = $state([]);
  let data = $state({ rows: [] });
</script>
{#each todos as todo}<input bind:checked={todo.done} />{/each}
{#each data.rows as row}<button onclick={() => row.items.push(1)}>{row}</button>{/each}
"#,
    );
    let todos = model
        .symbols()
        .find(|symbol| symbol.name() == "todos")
        .unwrap();
    let todo = model
        .symbols()
        .find(|symbol| symbol.name() == "todo")
        .unwrap();
    assert_eq!(todo.each_source(), Some(todos.clone()));
    let changes: Vec<_> = todos
        .uses()
        .filter(|reference| reference.via().is_some())
        .map(|reference| reference.access())
        .collect();
    assert_eq!(changes, [Access::Mutate]);
    let data = model
        .symbols()
        .find(|symbol| symbol.name() == "data")
        .unwrap();
    assert_eq!(
        data.uses()
            .filter(|reference| reference.via().is_some())
            .count(),
        1
    );
}

#[test]
fn values_follow_constants_across_the_template() {
    let model = component(
        r#"<script module>
  const base = '/app';
</script>
<script>
  const page = `${base}/page`;
  const mode = $state('dark');
  let theme = 'light';
</script>
<a href={page} title={active ? 'on' : 'off'} data-mode={mode} data-theme={theme} />
<a href={`${page}/${id}`} />
<a href={base + id} />
"#,
    );
    let facts: Vec<_> = model
        .snippets()
        .filter(|snippet| snippet.attr().is_some())
        .map(|snippet| snippet.facts().clone())
        .collect();
    assert_eq!(
        facts[0].value(),
        Some(&StaticValue::String("/app/page".into()))
    );
    assert_eq!(
        facts[1].possible_strings(),
        Some(&["on".into(), "off".into()][..])
    );
    assert_eq!(facts[2].value(), None);
    assert_eq!(facts[3].value(), None);
    assert_eq!(facts[4].prefix(), Some("/app/page/"));
    assert_eq!(facts[4].value(), None);
    assert_eq!(facts[5].prefix(), Some("/app"));
}

#[test]
fn references_know_whether_they_run_at_the_top_level() {
    insta::assert_snapshot!(references(
        r#"<script>
  const width = window.innerWidth;
  function measure() { return window.innerHeight; }
</script>
<p>{window.location}</p>
<button onclick={() => window.alert()} />
{#snippet later()}{window.name}{/snippet}
"#
    ));
}

#[test]
fn elements_know_how_often_they_render() {
    insta::assert_snapshot!(
        component(
            r#"<script>import Card from './Card.svelte';</script>
<div id="once" />
{#if a}<div id="maybe" />{#each b as c}<div id="many" />{/each}{/if}
<Card><div id="children" /></Card>
{#key k}<div id="keyed" />{/key}
<svelte:element this="p" /><svelte:window onresize={r} />
"#
        )
        .to_string()
        .lines()
        .filter(|line| line.starts_with("element") || line.starts_with("  attr"))
        .collect::<Vec<_>>()
        .join("\n")
    );
}

#[test]
fn exports_are_recorded() {
    insta::assert_snapshot!(
        component(
            r#"<script module lang="ts">
  export const load = () => {};
  export function helper() {}
  export type Kind = 'a' | 'b';
  const local = 1;
  export { local as renamed, local };
  export { other } from './other';
  export * as all from './all';
</script>
"#
        )
        .to_string()
        .lines()
        .filter(|line| line.starts_with("export"))
        .collect::<Vec<_>>()
        .join("\n")
    );
}

#[test]
fn route_files_come_from_the_path() {
    let route = |path| component_at("", path).route_file();
    assert_eq!(
        route("src/routes/+page.svelte"),
        Some(RouteFile::PageComponent)
    );
    assert_eq!(
        route("/app/src/routes/(group)/blog/+layout@.svelte"),
        Some(RouteFile::LayoutComponent)
    );
    assert_eq!(
        route("src/routes/+error.svelte"),
        Some(RouteFile::ErrorComponent)
    );
    assert_eq!(route("src/lib/+page.svelte"), None);
    assert_eq!(route("src/routes/Page.svelte"), None);
}

#[test]
fn script_metadata_is_read() {
    let model = component(
        r#"<script lang="ts" generics="T extends Item">let a = 1;</script>
<script context="module">let b = 2;</script>
<style lang="scss">p { color: red }</style>
<div><style>nested {}</style></div>
"#,
    );
    let scripts: Vec<_> = model
        .scripts()
        .map(|script| {
            (
                script.kind(),
                script.lang().map(str::to_string),
                script.has_generics(),
            )
        })
        .collect();
    assert_eq!(
        scripts,
        [
            (crate::ScriptKind::Instance, Some("ts".to_string()), true),
            (crate::ScriptKind::Module, None, false),
        ]
    );
    assert_eq!(model.styles().count(), 1);
    assert_eq!(model.styles().next().unwrap().lang(), Some("scss"));
}

#[test]
fn guards_are_found_through_branches() {
    let model = component(
        r#"<script>import { browser } from '$app/environment';</script>
{#if browser}<p>{window.innerWidth}</p>{:else}<p>{window.name}</p>{/if}
"#,
    );
    let reference = model
        .references()
        .find(|reference| reference.name() == "window")
        .unwrap();
    let branch = model
        .branches_containing(reference.host_range())
        .next()
        .unwrap();
    let condition = branch.condition().unwrap();
    let browser = condition.references().next().unwrap();
    assert!(
        browser
            .symbol()
            .unwrap()
            .is_import_of("$app/environment", "browser")
    );
    assert_eq!(
        browser.resolution(),
        Resolution::Symbol(browser.symbol().unwrap().id())
    );
}

#[test]
fn symbol_kinds_of_the_template() {
    let model = component(
        r#"{#each list as [first, ...others], i}{/each}
{#await p}{:then value}{:catch error}{/await}
{#snippet s(param)}{@const c = 1}{let d = 2}{/snippet}
"#,
    );
    let kinds: Vec<_> = model
        .symbols()
        .map(|symbol| (symbol.name().to_string(), symbol.kind()))
        .collect();
    assert_eq!(
        kinds,
        [
            ("first".to_string(), SymbolKind::EachItem),
            ("others".to_string(), SymbolKind::EachItem),
            ("i".to_string(), SymbolKind::EachIndex),
            ("value".to_string(), SymbolKind::AwaitValue),
            ("error".to_string(), SymbolKind::AwaitError),
            ("s".to_string(), SymbolKind::Snippet),
            ("param".to_string(), SymbolKind::SnippetParam),
            ("c".to_string(), SymbolKind::ConstTag),
            ("d".to_string(), SymbolKind::DeclarationTag),
        ]
    );
}

#[test]
fn changes_through_nested_each_items_change_every_list() {
    let model = component(
        r#"<script>
  let groups = $state([]);
</script>
{#each groups as group}{#each group.items as item}<input bind:value={item.name} />{/each}{/each}
"#,
    );
    let changed: Vec<_> = model
        .references()
        .filter(|reference| reference.site() == crate::RefSite::EachItem)
        .map(|reference| {
            (
                reference.name().to_string(),
                reference.via().unwrap().name().to_string(),
            )
        })
        .collect();
    assert_eq!(
        changed,
        [
            ("group".to_string(), "item".to_string()),
            ("groups".to_string(), "group".to_string()),
        ]
    );
}

#[test]
fn operands_split_on_the_outermost_or() {
    let model = component("{#if a && (b || c)}{:else if c || d && e}{/if}");
    let groups: Vec<Vec<_>> = model
        .blocks()
        .flat_map(|block| block.branches().collect::<Vec<_>>())
        .filter_map(|branch| branch.condition())
        .map(|condition| {
            condition
                .operands()
                .iter()
                .map(|operand| operand.or_group())
                .collect()
        })
        .collect();
    assert_eq!(groups, [vec![0, 0], vec![0, 1, 1]]);
}

#[test]
fn else_if_branches_have_their_condition() {
    let model = component("{#if a}<p />{:else if b}<p />{:else}<p />{/if}");
    let branches: Vec<_> = model
        .blocks()
        .next()
        .unwrap()
        .branches()
        .map(|branch| {
            (
                branch.kind(),
                branch
                    .condition()
                    .map(|condition| condition.references().count() == 1),
            )
        })
        .collect();
    assert_eq!(
        branches,
        [
            (crate::BranchKind::If, Some(true)),
            (crate::BranchKind::ElseIf, Some(true)),
            (crate::BranchKind::Else, None),
        ]
    );
}

#[test]
fn private_fields_keep_their_hash() {
    let model = component(
        r#"<script>
  class Counter {
    #count = $state(0);
    constructor() { this.#total = $state(0); }
  }
</script>
"#,
    );
    let targets: Vec<_> = model
        .rune_calls()
        .map(|call| call.target().clone())
        .collect();
    assert_eq!(
        targets,
        [
            RuneTarget::ClassField("#count".into()),
            RuneTarget::ConstructorField("#total".into()),
        ]
    );
}

#[test]
fn untyped_props_read_as_a_whole_are_open() {
    let complete = |code| component(code).props_declaration().unwrap().is_complete();
    assert!(complete("<script>let { a } = $props();</script>"));
    assert!(!complete("<script>let props = $props();</script>"));
    assert!(!complete("<script>let { a, ...rest } = $props();</script>"));
    assert!(complete(
        "<script lang=\"ts\">let { a, ...rest }: { a: string } = $props();</script>"
    ));
}

#[test]
fn event_handlers_have_an_expression_value() {
    let model = component(
        r#"<script>import Button from './Button.svelte';</script>
<button onclick={go} onfocus="go()" onblur {onkeydown} on:mouseover={go} />
<Button onclick={go} />
"#,
    );
    let handlers: Vec<_> = model
        .elements()
        .flat_map(|element| element.attrs().collect::<Vec<_>>())
        .filter(|attr| attr.is_event_handler())
        .map(|attr| attr.name().unwrap().to_string())
        .collect();
    assert_eq!(handlers, ["onclick", "onkeydown"]);
}

#[test]
fn numbers_become_strings_only_when_written_canonically() {
    let model = component("<p class={`w-${1}`} title={`w-${1.0}`} />");
    let values: Vec<_> = model
        .snippets()
        .map(|snippet| snippet.facts().value().cloned())
        .collect();
    assert_eq!(values, [Some(StaticValue::String("w-1".into())), None]);
}

#[test]
fn the_module_script_does_not_subscribe_to_stores() {
    let model = component(
        r#"<script module>
  import { writable } from 'svelte/store';
  const count = writable(0);
  console.log($count);
</script>
"#,
    );
    let reference = model
        .references()
        .find(|reference| reference.name() == "$count")
        .unwrap();
    assert_eq!(reference.resolution(), Resolution::Global);
}

#[test]
fn wrapped_rune_calls_still_initialize_their_variable() {
    let model = component(
        r#"<script lang="ts">
  let map = ($state(new Map()) as Map<string, number>)!;
</script>
"#,
    );
    let call = model.rune_calls().next().unwrap();
    assert_eq!(call.target(), &RuneTarget::Variable);
    assert_eq!(
        call.symbols()
            .map(|symbol| symbol.name().to_string())
            .collect::<Vec<_>>(),
        ["map"]
    );
}
