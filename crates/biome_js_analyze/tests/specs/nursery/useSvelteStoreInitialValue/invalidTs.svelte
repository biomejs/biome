<!-- should generate diagnostics -->
<script lang="ts">
	import { writable } from "svelte/store";
	const count = writable<number>();
</script>

<p>{$count}</p>
