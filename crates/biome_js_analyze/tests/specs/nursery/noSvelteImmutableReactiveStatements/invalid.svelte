<!-- should generate diagnostics -->
<script>
	import { PI } from "./constants.js";

	const base = 1;
	const factor = 2;

	// `const` primitive dependencies: this never re-runs.
	$: doubled = base * factor;

	// imported binding: immutable.
	$: area = PI * base;

	// reactive block referencing only immutable values.
	$: {
		sum = base + PI;
	}
</script>
