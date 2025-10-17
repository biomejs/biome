test('example', async ({ page }) => {
    await page.click('button');
    await page.waitForSelector('.result');
});

