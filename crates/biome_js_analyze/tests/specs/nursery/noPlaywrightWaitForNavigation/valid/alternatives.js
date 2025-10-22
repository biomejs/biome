await page.waitForURL('/home');
await page.waitForLoadState('load');
await page.goto('/home');

