<!-- should generate diagnostics -->
<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";

	// allowReassign: true, the wrapped call still initializes `map`, which is reassigned — no fix
	// offered.
	let map = $state(new SvelteMap()) as SvelteMap<string, number>;
	map = new SvelteMap();
</script>
