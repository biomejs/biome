/* should generate diagnostics */
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

test('expect inside inner function is not returned by outer return', async ({ page }) => {
    return async function() {
        expect(page).toBeVisible();
    }
});
