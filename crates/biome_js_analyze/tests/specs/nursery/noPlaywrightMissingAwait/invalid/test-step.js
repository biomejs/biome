test('example', async ({ page }) => {
    test.step('clicks button', async () => {
        await page.click('button');
    });
});

