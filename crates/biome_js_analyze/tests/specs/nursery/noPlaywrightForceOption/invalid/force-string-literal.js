/* should generate diagnostics */
// Test string literal syntax for force option
await page.locator('button').click({ "force": true });

await page.locator('input').fill('text', { 'force': true });
