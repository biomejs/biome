/* should not generate diagnostics */
// Valid: awaited Promise combinators
test('awaited Promise.all', async ({ page }) => {
    await Promise.all([
        expect(page.locator('.foo')).toBeVisible(),
        expect(page.locator('.bar')).toHaveText('foo')
    ]);
});

test('awaited Promise.allSettled', async ({ page }) => {
    await Promise.allSettled([
        expect(page.locator('.foo')).toBeVisible(),
        expect(page.locator('.bar')).toHaveText('foo')
    ]);
});

test('returned Promise.race', async ({ page }) => {
    return Promise.race([
        expect(page.locator('.foo')).toBeVisible(),
        expect(page.locator('.bar')).toHaveText('foo')
    ]);
});

test('returned Promise.any', async ({ page }) => {
    return Promise.any([
        expect(page.locator('.foo')).toBeVisible(),
        expect(page.locator('.bar')).toHaveText('foo')
    ]);
});
