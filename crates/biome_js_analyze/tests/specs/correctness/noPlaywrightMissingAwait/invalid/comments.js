/* should generate diagnostics */
test('call', async ({ page }) => {
    // a leading comment
    expect(page.locator('body')).toBeVisible(); // a trailing comment
});

test('combinator', async ({ page }) => {
    // a leading comment
    Promise.all([
        expect(page.locator('.one')).toBeVisible(),
    ]); // a trailing comment
});
