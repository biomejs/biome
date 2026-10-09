<!-- should not generate diagnostics -->
<script module>
	import { SvelteSet as Set } from "svelte/reactivity";
</script>

<script>
	const set = new Set();
	set.add(1);
</script>
