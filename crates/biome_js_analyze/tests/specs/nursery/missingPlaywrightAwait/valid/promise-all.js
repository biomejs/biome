test('example', async ({ page }) => {
    await Promise.all([
        expect(page.locator('.one')).toBeVisible(),
        expect(page.locator('.two')).toBeVisible()
    ]);
});

