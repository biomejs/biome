/* should NOT generate diagnostics - test with options object containing callbacks */
test("name", { retry: 2 }, async () => {
    await expect(page).toHaveTitle("Title");
});

test("with retry callback", { retry: () => 2 }, async ({ page }) => {
    await expect(page).toBeVisible();
});
