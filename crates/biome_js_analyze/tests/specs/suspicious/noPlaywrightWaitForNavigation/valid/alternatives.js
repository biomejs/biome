/* should not generate diagnostics */
await page.waitForURL('/home');
await page.waitForLoadState('load');
await page.goto('/home');

