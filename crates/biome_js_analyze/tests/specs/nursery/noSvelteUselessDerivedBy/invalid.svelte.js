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

const sequence = $der\u0069ved.by(() => { return first(), /* keep */ second(); });
const indirect = $derived.by(function compute() { return eval?.("compute"); });
const nested = $derived.by(function () { return { value(x = arguments) { return x; } }; });
const commented = $derived./* keep */by(() => 1);
