mod spec_test;

use biome_html_factory::syntax::HtmlElement;
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_html_syntax::ScriptType;
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;

mod ok {
    tests_macros::gen_tests! {"tests/html_specs/ok/**/*.{html,svg,astro,vue,svelte}", crate::spec_test::run, "ok"}
}

mod error {
    tests_macros::gen_tests! {"tests/html_specs/error/**/*.{html,svg,astro,vue,svelte}", crate::spec_test::run, "error"}
}

#[test]
fn svelte_runes_mode_parser_baseline() {
    // Each component compiles with svelte@5.59.1 and runes: true, including the scriptless one.
    for source in [
        r#"<script>
	let count = $state(0);
	let doubled = $derived(count * 2);
</script>

<input type="number" bind:value={count} />
<button class:active={count > 0} onclick={() => count++}>
	Double: {doubled}
</button>
"#,
        r#"<script module lang="ts">
	export type Item = { id: number; label: string };
</script>

<script lang="ts">
	let { items }: { items: Item[] } = $props();
</script>

{#snippet row<T extends Item>(item: T)}
	<li>{item.label}</li>
{/snippet}

{#if items.length}
	<ul>
		{#each items as item (item.id)}
			{@render row(item)}
		{/each}
	</ul>
{:else}
	<p>No items</p>
{/if}
"#,
        r#"<article>
	<h1>News</h1>
	<p>Updates from the <strong>team</strong>.</p>
</article>
"#,
    ] {
        let parsed = parse_html(source, HtmlParserOptions::from(&HtmlFileSource::svelte()));
        assert!(
            parsed.diagnostics().is_empty(),
            "{:#?}",
            parsed.diagnostics()
        );
    }
}

#[test]
fn svelte_restricted_expressions_ignore_keywords_in_javascript_literals() {
    let source = r#"{#each " as " as character}
	{character}
{/each}

{#each ` as ` as character}
	{character}
{/each}

{#each / as /.exec("value") ?? [] as match}
	{match}
{/each}

{#each values /* as */ as value}
	{value}
{/each}

{#each values // as
 as value}
	{value}
{/each}

{#await " then " then value}
	{value}
{/await}

{#await value /* then */ then result}
	{result}
{/await}

{#await " catch " catch error}
	{error}
{/await}

{#await value /* catch */ catch error}
	{error}
{/await}
"#;
    let parsed = parse_html(source, HtmlParserOptions::from(&HtmlFileSource::svelte()));
    assert!(
        parsed.diagnostics().is_empty(),
        "{:#?}",
        parsed.diagnostics()
    );
}

#[test]
fn test_is_javascript_tag() {
    let html = r#"
        <script type="text/javascript">
        </script>
        "#;
    let syntax = parse_html(html, HtmlParserOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(element.is_javascript_tag());

    let html = r#"
        <script type="application/javascript">
        </script>
        "#;
    let syntax = parse_html(html, HtmlParserOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(element.is_javascript_tag());
    assert_eq!(element.get_script_type(), Some(ScriptType::Classic));

    let html = r#"
        <script type="application/ecmascript">
        </script>
        "#;
    let syntax = parse_html(html, HtmlParserOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(element.is_javascript_tag());
    assert_eq!(element.get_script_type(), Some(ScriptType::Classic));

    let html = r#"
        <script type="module">
        </script>
        "#;
    let syntax = parse_html(html, HtmlParserOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(element.is_javascript_tag());
    assert_eq!(element.get_script_type(), Some(ScriptType::Module));

    // FIXME: Uncomment when the parser supports unquoted attribute values.
    //let html = r#"
    //    <script type=module></script>
    //    "#;
    //let syntax = parse_html(html, HtmlParserOptions::default());
    //let element = syntax
    //    .tree()
    //    .syntax()
    //    .descendants()
    //    .find_map(HtmlElement::cast)
    //    .unwrap();
    //
    //assert!(element.is_javascript_tag());
    //assert_eq!(element.get_script_type(), Some(ScriptType::Module));

    let html = r#"
        <script></script>
        "#;
    let syntax = parse_html(html, HtmlParserOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(element.is_javascript_tag());
    assert_eq!(element.get_script_type(), Some(ScriptType::Classic));

    let html = r#"
        <script type="importmap"></script>
        "#;
    let syntax = parse_html(html, HtmlParserOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(!element.is_javascript_tag());
    assert_eq!(element.get_script_type(), Some(ScriptType::ImportMap));
}
