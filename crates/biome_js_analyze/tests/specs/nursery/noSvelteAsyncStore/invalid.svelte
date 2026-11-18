<!-- should generate diagnostics -->
<script>
	import { writable } from "svelte/store";

	const user = writable(null, async (set) => {
		set(await fetchUser());
	});
</script>

<p>{$user}</p>
