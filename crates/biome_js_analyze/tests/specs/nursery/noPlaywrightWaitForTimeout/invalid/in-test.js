test('wait', async ({ page }) => {
    await page.click('button');
    await page.waitForTimeout(1000);
});

