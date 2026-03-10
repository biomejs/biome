/* should generate diagnostics */
test('example', async ({ page }) => {
    Promise.all([
        expect(page.locator('.one')).toBeVisible(),
        expect(page.locator('.two')).toBeVisible()
    ]);
});

/* Promise.all with .then() chain — should diagnose */
test('example', async ({ page }) => {
    Promise.all([
        expect(page.locator('body')).toBeVisible(),
    ]).then(() => {});
});

