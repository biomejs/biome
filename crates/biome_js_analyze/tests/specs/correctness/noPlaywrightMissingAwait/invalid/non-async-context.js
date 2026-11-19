/* should generate diagnostics but NOT autofix (not in async context) */
test('example', ({ page }) => {
    expect(page.locator('body')).toBeVisible();
});

/* sync function expression callback — should diagnose but NOT autofix */
test('example', function({ page }) {
    expect(page.locator('body')).toBeVisible();
});
