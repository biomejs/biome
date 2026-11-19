// should not generate diagnostics
// A read in a closure created after a test sees the test, as in TypeScript,
// so the left operand of each `||` below can't be nullish.

function tested(value: string | null) {
	if (value === null) return;
	return () => value || "none";
}

function constant(record: { a: number } | undefined) {
	if (record) {
		[1].forEach(() => record || { a: 0 });
	}
}
