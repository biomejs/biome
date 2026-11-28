/* should generate diagnostics */
await page.locator('.my-element');
await page.locator('button').first();
await page.locator('button').or(page.locator('input'));

const btn = page.locator('button');
await btn.first();

async function testLocator(locator) {
    await locator.first();
    await locator.or(other);
}

