<!-- should not generate diagnostics -->
<script>
	export let expanded = false;
	export let person;

	// Each reactive declaration declares a component variable named like a global.
	$: length = expanded ? 24 : 4;
	$: (parent = expanded);
	$: ({ name, status } = person);
	$: [top, self] = [1, 2];
</script>

<p>{length} {parent} {name} {status} {top} {self}</p>
