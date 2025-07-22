
[1, 2, 3].map(async (x) => x + 1);

async function floatingArray() {
	[1, 2, 3].map((x) => Promise.resolve(x + 1));
}
