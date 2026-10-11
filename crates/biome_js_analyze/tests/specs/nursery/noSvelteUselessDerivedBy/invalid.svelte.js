/* should generate diagnostics */
export function createCounter() {
	let count = $state(0);
	const doubled = $derived.by(() => count * 2);
	return {
		get doubled() {
			return doubled;
		},
	};
}

export class Counter {
	count = $state(0);
	doubled = $derived.by(() => this.count * 2);
}
