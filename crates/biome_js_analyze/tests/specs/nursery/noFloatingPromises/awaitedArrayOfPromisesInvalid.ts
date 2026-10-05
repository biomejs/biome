// should generate diagnostics
async function asyncValues(): Promise<Array<Promise<void>>> {
	return [];
}
function syncValues(): Promise<Array<Promise<void>>> {
	return Promise.resolve([]);
}
declare const directValues: Promise<Array<Promise<void>>>;
declare const values: Array<Promise<void>>;

asyncValues();
syncValues();
directValues;
values;

await asyncValues();
await syncValues();
await directValues;
await await asyncValues();
await values;
