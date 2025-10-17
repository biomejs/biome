test('example', async ({ page }) => {
    Promise.all([
        expect(page.locator('.one')).toBeVisible(),
        expect(page.locator('.two')).toBeVisible()
    ]);
});

