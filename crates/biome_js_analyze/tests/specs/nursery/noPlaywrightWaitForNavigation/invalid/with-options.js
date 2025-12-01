/* should generate diagnostics */
await page.click('button');
await page.waitForNavigation({ waitUntil: 'networkidle' });

