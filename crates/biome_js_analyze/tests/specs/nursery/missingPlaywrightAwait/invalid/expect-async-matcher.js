test('example', async ({ page }) => {
    expect(page.locator('body')).toBeVisible();
});

