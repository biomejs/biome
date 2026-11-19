const values = [1, 2, 3];

values.map(async (value) => value);
values.map((value) => Promise.resolve(value));

declare const pending: Promise<void>[];
pending;

async function nested(): Promise<Array<Promise<void>>> {
	return [];
}

await nested();
Promise.all(values.map(async (value) => value));

await Promise.all(values.map(async (value) => value));
void pending;
