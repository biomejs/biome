<!-- should generate diagnostics -->
<script>
	import { SvelteMap } from "svelte/reactivity";

	// allowReassign: true, the template reassigns the variable — no fix offered.
	let map = $state(new SvelteMap());
</script>

<button onclick={() => (map = new SvelteMap())}>Reset</button>
