// should generate diagnostics
async function asyncValues(): Promise<Array<Promise<void>>> {
	return [];
}
function syncValues(): Promise<Array<Promise<void>>> {
	return Promise.resolve([]);
}
declare const directValues: Promise<Array<Promise<void>>>;

await asyncValues();
await syncValues();
await directValues;
await await asyncValues();
