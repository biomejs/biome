<!-- should not generate diagnostics -->

<script>
	let { isActive } = $props();
</script>

<!-- biome-ignore lint/nursery/noSvelteClassDirective: this component also supports older Svelte versions -->
<div class:active={isActive}></div>
