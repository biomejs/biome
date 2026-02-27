/* should not generate diagnostics */
await page.locator('button').click();
await page.locator('check').check();
await page.locator('input').fill('text');


