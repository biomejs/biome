/* should generate diagnostics */
export function createCounter() {
	let count = $state(0);
	$inspect(count);
	return {
		get count() {
			return count;
		},
	};
}
