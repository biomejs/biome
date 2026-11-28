<!-- should generate diagnostics -->
<script lang="ts">
	let element: HTMLDivElement | undefined = $state();

	function remove() {
		element!.remove();
		element?.replaceChildren();
		(element as HTMLDivElement).remove();
	}
</script>

<div bind:this={element}></div>
