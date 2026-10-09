<!-- should not generate diagnostics -->
<script>
	import { SvelteMap as Map } from "svelte/reactivity";

	const map = new Map([[1, "one"]]);
	map.set(2, "two");

	function scoped() {
		class Set {
			add() {}
		}
		const set = new Set();
		set.add(1);
	}
</script>
