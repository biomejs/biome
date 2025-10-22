const button = page.locator('button');
await button.click();

const buttons = page.locator('.btn');
await expect(buttons).toHaveCount(3);

await page.getByRole('button', { name: 'Submit' }).click();

