// should generate diagnostics
// `count && 1` is `number` when `count` is falsy, because `NaN` and `-0` are
// falsy numbers too, so it is never `null`.

function compare(count: number) {
	return (count && 1) === null;
}
