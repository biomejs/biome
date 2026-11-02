<!-- should generate diagnostics -->
<script lang="ts">
	import { writable } from "svelte/store";

	const foo = writable<number>(0);

	foo.subscribe((value: number) => {});
	foo.subscribe(() => {}) as unknown;
	foo!.subscribe(() => {});
</script>
