<script>
	export let expanded = false;

	// Only a top-level `$: name = ...` declares a variable. These assign to the globals.
	$: {
		length = expanded ? 24 : 4;
	}
	$: if (expanded) parent = 1;
	$: x = (status = 1);
	$: self += 1;
	// Only `name` is declared here; `top` is the global.
	$: name = top++;

	function update() {
		$: top = 1;
	}
</script>
