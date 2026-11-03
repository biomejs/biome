/* should not generate diagnostics */
type X = number;
const value = 10;

await page.evaluate(() => {
	const x = 10 as X;
	return Promise.resolve(x);
});

await page.addInitScript(() => {
	const foo = (bar: X) => bar;
	return Promise.resolve(foo(10));
});

await page.evaluate(() => {
	const x: X = 10;
	return Promise.resolve(x);
});

await page.evaluate(() => {
	const x: typeof value = 10;
	return x;
});

declare const ambientValue: number;
await page.evaluate(() => ambientValue);

declare global {
	var globalAugmented: string;
}
await page.evaluate(() => globalAugmented);
