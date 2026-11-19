/* should not generate diagnostics */
await page.goto('https://example.com');
await page.fill('input', 'text');
await page.screenshot();

