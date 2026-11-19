/* should generate diagnostics */
const x = 10;
const y = 12;

await page.evaluate(() => {
	return Promise.resolve(x);
});

await page.addInitScript(() => {
	return Promise.resolve(x);
});

await page.evaluate(function () {
	return Promise.resolve(x);
});

await page.evaluate(() => {
	return Promise.resolve(x);
}, []);

await page.evaluate(() => Promise.resolve(x + y + x));

await page.evaluate(() => {
	const z = 20;
	return Promise.resolve(x + z);
});

test("test", async () => {
	const local = 10;
	await page.evaluate(() => Promise.resolve(x + local), []);
});

await page.evaluate(([x]) => Promise.resolve(x + y), [x]);

await page.evaluate((x) => Promise.resolve(x + y), x);

await page.evaluate(async (value) => value + y, x);

await page.evaluate(([a]) => a + y, [x, y]);

await page.evaluate(({ a }) => a + y, { a: x });

await page.evaluate((a = 1) => a + y, x);

await page.evaluate(async () => x);

await page["evaluate"](() => x);

await page[`evaluate`](() => x);

await myPage.evaluate(() => x);

await context.page.evaluate(() => x);

await frame.evaluate(() => x);

await page.evaluate(() => {
	x = 5;
});

await page.evaluate(() => {
	function inner() {
		return y;
	}
	return inner();
});

function helper() {}
await page.evaluate(() => helper());

await page.evaluate(value => value + y, x);

await page.evaluate(
	() => x,
);
