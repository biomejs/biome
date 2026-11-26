// should not generate diagnostics
// TypeScript infers `0 | 1`, but `count && 1` returns `NaN` or `-0` when
// `count` is either of them, so the annotation is not wider than the result.

function first(count: number): number {
	return count && 1;
}
