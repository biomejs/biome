/* should generate diagnostics */
const x = 10;
const y = 12;

await page.evaluate((value: number) => value + y, x);

await page.evaluate(() => x as number);

await page.evaluate(<T,>() => x);
