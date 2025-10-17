await page.waitForURL('/home');
await page.waitForLoadState('networkidle');
await page.goto('/home');

