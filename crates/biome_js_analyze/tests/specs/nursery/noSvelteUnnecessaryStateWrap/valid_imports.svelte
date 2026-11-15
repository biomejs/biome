<!-- should not generate diagnostics -->
<script>
	import { SvelteDate, SvelteMap, SvelteURL } from "svelte/reactivity";
	import * as lib from "./my-lib";

	// A namespace import of another module.
	const other = $state(new lib.SvelteMap());

	// `$state.raw` is a different rune.
	const raw = $state.raw(new SvelteMap());

	// Static methods return plain values, which `$state` makes reactive.
	let now = $state(SvelteDate.now());
	let url = $state(SvelteURL.createObjectURL(blob));
</script>
