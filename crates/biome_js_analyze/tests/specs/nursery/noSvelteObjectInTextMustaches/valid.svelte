<!-- should not generate diagnostics -->
<script>
	import MyComponent from './MyComponent.svelte';
	let a = 'hello!';
	let items = [a];
</script>

{a}
{JSON.stringify({ a })}
{[a].join(', ')}
{{ a }.a}
{(() => a)()}
<input class="{a} a" />
<input class="{{ a }}" />
<input class:foo={{ a }} />
<MyComponent prop={{ a }} />
<MyComponent {...{ a }} />
<button onclick={() => a}>a</button>
<div style={{ color: 'red' }}></div>
{#each items as item}{item}{/each}
{#if [a].length}{a}{/if}
