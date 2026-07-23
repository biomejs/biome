<!-- should generate diagnostics -->
<script>
	import { SvelteMap } from "svelte/reactivity";

	// allowReassign: true, but never reassigned — fix still offered.
	const map = $state(new SvelteMap());

	// allowReassign: true, variable is reassigned — diagnostic fires, but no fix offered.
	let map2 = $state(new SvelteMap());
	map2 = new SvelteMap([[1, "a"]]);
</script>
