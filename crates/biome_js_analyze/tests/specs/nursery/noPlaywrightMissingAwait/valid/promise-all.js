/* should not generate diagnostics */
test('example', async ({ page }) => {
    await Promise.all([
        expect(page.locator('.one')).toBeVisible(),
        expect(page.locator('.two')).toBeVisible()
    ]);
});

/* Promise.all with awaited .then() chain — should NOT diagnose */
test('example', async ({ page }) => {
    await Promise.all([
        expect(page.locator('body')).toBeVisible(),
    ]).then(() => {});
});

