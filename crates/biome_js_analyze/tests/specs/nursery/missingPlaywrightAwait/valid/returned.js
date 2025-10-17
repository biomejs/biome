test('example', async ({ page }) => {
    return expect(page.locator('body')).toBeVisible();
});

test('arrow', async ({ page }) => expect(page.locator('body')).toBeVisible());

