/* should not generate diagnostics */
// Valid: awaited bracket notation
test('awaited bracket notation for test.step', async ({ page }) => {
    await test["step"]('do something', async () => {
        await page.click('button');
    });
});

test('awaited template literal for test.step', async ({ page }) => {
    await test[`step`]('do something', async () => {
        await page.click('button');
    });
});

test('awaited bracket notation for expect.soft', async ({ page }) => {
    await expect["soft"](page).toBeVisible();
});

test('awaited template literal for expect.soft', async ({ page }) => {
    await expect[`soft`](page).toBeVisible();
});

test('awaited bracket notation for matcher', async ({ page }) => {
    await expect(page)["toBeVisible"]();
});

test('awaited template literal for matcher', async ({ page }) => {
    await expect(page)[`toBeVisible`]();
});
