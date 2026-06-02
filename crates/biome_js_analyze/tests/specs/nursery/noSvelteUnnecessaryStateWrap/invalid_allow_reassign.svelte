<!-- should generate diagnostics -->
<script>
	import { SvelteMap } from "svelte/reactivity";

	// allowReassign: true, but this variable is never reassigned — still flagged.
	const map = $state(new SvelteMap());
</script>
