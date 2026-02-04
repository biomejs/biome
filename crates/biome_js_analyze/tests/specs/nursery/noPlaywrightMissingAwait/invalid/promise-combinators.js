/* should generate diagnostics */
test('Promise.allSettled', async ({ page }) => {
    Promise.allSettled([
        expect(page.locator('.foo')).toBeVisible(),
        expect(page.locator('.bar')).toHaveText('foo')
    ]);
});

test('Promise.race', async ({ page }) => {
    Promise.race([
        expect(page.locator('.foo')).toBeVisible(),
        expect(page.locator('.bar')).toHaveText('foo')
    ]);
});

test('Promise.any', async ({ page }) => {
    Promise.any([
        expect(page.locator('.foo')).toBeVisible(),
        expect(page.locator('.bar')).toHaveText('foo')
    ]);
});
