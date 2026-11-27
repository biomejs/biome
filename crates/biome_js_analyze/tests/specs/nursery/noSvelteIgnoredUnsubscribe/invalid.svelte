<!-- should generate diagnostics -->
<script>
	import { onMount } from "svelte";
	import { writable } from "svelte/store";

	const foo = writable(0);

	foo.subscribe(() => {
		console.log("foo changed");
	});

	foo?.subscribe(() => {});
	foo.subscribe?.(() => {});
	foo["subscribe"](() => {});
	(foo.subscribe)(() => {});
	(foo.subscribe(() => {}));
	stores.foo.subscribe(() => {});

	onMount(() => {
		foo.subscribe(() => {});
	});
</script>
