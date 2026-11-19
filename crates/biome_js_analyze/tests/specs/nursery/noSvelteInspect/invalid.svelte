<!-- should generate diagnostics -->
<script>
	let count = $state(0);
	let message = $state("hello");

	$inspect(count);
	$inspect(count, message);
	$inspect(count).with(console.trace);
	$inspect(message).with((type, value) => {
		if (type === "update") {
			debugger;
		}
	});

	$effect(() => {
		$inspect.trace();
	});

	function doubled() {
		$inspect.trace("doubled");
		return count * 2;
	}
</script>

<p>{count} {message}</p>
