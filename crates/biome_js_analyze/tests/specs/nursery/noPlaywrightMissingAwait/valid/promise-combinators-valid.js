/* should not generate diagnostics */
// Valid: awaited Promise combinators
test('awaited Promise.all', async ({ page }) => {
    await Promise.all([
        expect(page).toBeVisible(),
        expect(page).toHaveText('foo')
    ]);
});

test('awaited Promise.allSettled', async ({ page }) => {
    await Promise.allSettled([
        expect(page).toBeVisible(),
        expect(page).toHaveText('foo')
    ]);
});

test('returned Promise.race', async ({ page }) => {
    return Promise.race([
        expect(page).toBeVisible(),
        expect(page).toHaveText('foo')
    ]);
});

test('returned Promise.any', async ({ page }) => {
    return Promise.any([
        expect(page).toBeVisible(),
        expect(page).toHaveText('foo')
    ]);
});
