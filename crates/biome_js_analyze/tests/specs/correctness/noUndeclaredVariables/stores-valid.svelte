<!-- should not generate diagnostics -->
<script>
	import { writable } from "svelte/store";

	const count = writable(1);
	$count;
</script>
