<script lang="ts" generics="T extends { selected?: boolean; name?: string }">
	let { items }: { items: T[] } = $props();
</script>

<style data-theme="{ dark }">
	p { color: red; }
</style>

<div title="{items.length} items"></div>

<svelte:head>
	<script async src="https://example.com/gtag.js?id={id}"></script>
	<style data-theme="{theme}"></style>
</svelte:head>
