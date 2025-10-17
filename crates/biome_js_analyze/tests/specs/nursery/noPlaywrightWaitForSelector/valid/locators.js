await page.locator('.submit-button').click();
await expect(page.locator('#dialog')).toBeVisible();
const button = page.getByRole('button', { name: 'Submit' });
await button.click();

