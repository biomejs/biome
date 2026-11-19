<!-- should generate diagnostics -->
<script>
	let a = $state({ b: 1 });
	let list = $state([1, 2, 3]);

	const concise = $derived.by(() => a.b);

	const blockReturn = $derived.by(() => {
		return a.b;
	});

	const functionExpression = $derived.by(function () {
		return a.b;
	});

	const object = $derived.by(() => ({ value: a.b }));

	const sequence = $derived.by(() => (a.b, list.length));

	const nestedFunction = $derived.by(() => () => a.b);

	// `this` inside a nested function belongs to that function.
	const nestedThis = $derived.by(function () {
		return list.map(function () {
			return this;
		});
	});

	const multiline = $derived.by(() =>
		list
			.filter((item) => item > a.b)
			.map((item) => item * 2),
	);

	// The fix is withheld because these comments would be removed.
	const commentInBlock = $derived.by(() => {
		// explain why
		return a.b;
	});

	const commentAfterReturn = $derived.by(() => {
		return a.b; // explain why
	});

	// Comments inside the expression are kept.
	const commentInExpression = $derived.by(() => a.b /* kept */ + 1);
</script>

{concise}
