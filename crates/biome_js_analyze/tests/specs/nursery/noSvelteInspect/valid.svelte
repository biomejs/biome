<!-- should not generate diagnostics -->
<script>
	let count = $state(0);
	let doubled = $derived(count * 2);

	$effect(() => {
		console.log(count);
	});

	const obj = { $inspect: () => {} };
	obj.$inspect(count);
	const inspect = "$inspect";
</script>

<p>{count} {doubled}</p>
