await page.waitForLoadState('load');
await page.goto('https://example.com');
await page.locator('.content').waitFor();


