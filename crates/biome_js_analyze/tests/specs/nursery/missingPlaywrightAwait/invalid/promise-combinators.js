test('Promise.allSettled', async ({ page }) => {
    Promise.allSettled([
        expect(page).toBeVisible(),
        expect(page).toHaveText('foo')
    ]);
});

test('Promise.race', async ({ page }) => {
    Promise.race([
        expect(page).toBeVisible(),
        expect(page).toHaveText('foo')
    ]);
});

test('Promise.any', async ({ page }) => {
    Promise.any([
        expect(page).toBeVisible(),
        expect(page).toHaveText('foo')
    ]);
});
