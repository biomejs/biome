await page.locator('.my-element').click();
await page.goto('https://example.com');
await expect(page.locator('.foo')).toBeVisible();
await expect.poll(() => foo).toBe(true);

