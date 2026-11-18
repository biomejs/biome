/* should generate diagnostics */
const x = 10;

// The function has no parameter, so the existing argument isn't used.
await page.evaluate(() => x, 1);

// There's a parameter but no argument.
await page.evaluate((a) => a + x);

await page.evaluate((...args) => args.length + x, 1);

await page.evaluate((() => x));

// The parameter's default value uses the variable.
await page.evaluate((a = x) => a, 1);
