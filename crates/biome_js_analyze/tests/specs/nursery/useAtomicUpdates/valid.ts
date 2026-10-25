/* should not generate diagnostics */
let total: number = 0;

async function add(getAmount: () => Promise<number>) {
	const amount: number = await getAmount();
	(total as number) += amount;
}

async function addLocal(getAmount: () => Promise<number>) {
	let localTotal: number = 0;
	(localTotal as number) += await getAmount();
	return localTotal;
}

let config: { url: string } | undefined;

async function reload(fetchJson: <T>(url: string) => Promise<T>) {
	config = await fetchJson<typeof config>("/config.json");
}
