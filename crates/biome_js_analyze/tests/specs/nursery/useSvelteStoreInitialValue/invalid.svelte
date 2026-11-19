<!-- should generate diagnostics -->
<script>
	import { writable, readable, derived } from "svelte/store";
	const w = writable();
	const r = readable();
	const d = derived([a, b], () => {});
</script>

<p>{$w} {$r} {$d}</p>
