/* should generate diagnostics */
let total: number = 0;

async function addAsType(getAmount: () => Promise<number>) {
	(total as number) += await getAmount();
}

async function addNonNull(getAmount: () => Promise<number>) {
	total! = total + (await getAmount());
}

class Counter {
	constructor(private readonly state: { count: number }) {}

	async increment(state: { count: number }) {
		state.count = state.count + (await Promise.resolve(1));
	}
}
