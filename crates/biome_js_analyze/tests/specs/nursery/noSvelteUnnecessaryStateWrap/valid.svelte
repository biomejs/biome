<!-- should not generate diagnostics -->
<script>
	import { SvelteMap, SvelteSet, SvelteDate } from "svelte/reactivity";

	// No $state wrapper: correct usage.
	const map = new SvelteMap();
	const set = new SvelteSet();
	const date = new SvelteDate();

	// Plain $state on a primitive: fine.
	let count = $state(0);
	let name = $state("hello");

	// SvelteMap from a different module: not covered by this rule.
	import { SvelteMap as FakeMap } from "./my-lib";
	const fake = $state(new FakeMap());
</script>
