<!-- should generate diagnostics -->
<script>
	import { SvelteMap as ReactiveMap } from "svelte/reactivity";
	import * as reactivity from "svelte/reactivity";

	// A renamed import of a reactive class.
	const map = $state(new ReactiveMap());
	// A reactive class read from a namespace import.
	const set = $state(new reactivity.SvelteSet());
</script>
