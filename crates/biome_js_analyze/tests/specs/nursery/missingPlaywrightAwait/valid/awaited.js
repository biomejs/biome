test('example', async ({ page }) => {
    await expect(page).toBeVisible();
    await test.step('step', async () => {});
    await expect.poll(() => foo).toBe(true);
});

