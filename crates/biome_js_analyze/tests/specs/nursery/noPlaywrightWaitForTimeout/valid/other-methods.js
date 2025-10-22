await page.waitForLoadState();
await page.waitForURL('/home');
await page.waitForFunction(() => window.innerWidth < 100);

