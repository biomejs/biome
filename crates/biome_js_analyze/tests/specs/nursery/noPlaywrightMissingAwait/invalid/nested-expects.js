// Test nested expect calls and chaining
test('nested not operator', async ({ page }) => {
    expect(page).not.toBeVisible();
});

test('chained not and soft', async ({ page }) => {
    expect.soft(page).not.toHaveText("foo");
});

test('multiple not calls', async ({ page }) => {
    expect(page).not.not.toBeEnabled();
});
