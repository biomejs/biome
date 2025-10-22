test('example', async ({ page }) => {
    await expect(page.locator('body')).toBeVisible();
    await test.step('step', async () => {});
    await expect.poll(() => foo).toBe(true);
});

