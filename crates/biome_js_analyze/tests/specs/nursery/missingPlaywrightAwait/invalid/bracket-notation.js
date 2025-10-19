// Test bracket notation for various Playwright async APIs
test('bracket notation for test.step', async ({ page }) => {
    test["step"]('do something', async () => {
        await page.click('button');
    });
});

test('template literal bracket notation for test.step', async ({ page }) => {
    test[`step`]('do something', async () => {
        await page.click('button');
    });
});

test('bracket notation for expect.soft', async ({ page }) => {
    expect["soft"](page).toBeVisible();
});

test('template literal for expect.soft', async ({ page }) => {
    expect[`soft`](page).toBeVisible();
});

test('bracket notation for matcher', async ({ page }) => {
    expect(page)["toBeVisible"]();
});

test('template literal for matcher', async ({ page }) => {
    expect(page)[`toBeVisible`]();
});
