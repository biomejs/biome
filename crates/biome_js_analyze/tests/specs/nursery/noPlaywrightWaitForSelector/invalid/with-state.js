/* should generate diagnostics */
await page.waitForSelector('#dialog', { state: 'visible' });
await page.click('#dialog .button');

