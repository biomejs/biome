<!-- should not generate diagnostics -->
<script>
	import { SvelteMap } from "svelte/reactivity";

	// allowReassign: true — $state is permitted because the variable is later reassigned.
	let map = $state(new SvelteMap());
	map = new SvelteMap([[1, "a"]]);
</script>
