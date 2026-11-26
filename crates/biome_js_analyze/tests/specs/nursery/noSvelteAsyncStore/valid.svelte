<!-- should not generate diagnostics -->
<script>
	import { writable } from "svelte/store";

	const user = writable(null, (set) => {
		fetchUser().then(set);
	});
</script>

<p>{$user}</p>
