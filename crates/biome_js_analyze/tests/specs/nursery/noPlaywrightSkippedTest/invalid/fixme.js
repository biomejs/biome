/* should generate diagnostics */

// test.fixme
test.fixme("needs fixing", async ({ page }) => {
    await page.click("button");
});

// it.fixme
it.fixme("it needs fixing", async ({ page }) => {
    await page.click("button");
});

// test.describe.fixme
test.describe.fixme("suite needs fixing", () => {
    test("test", async ({ page }) => {});
});

// test.describe.parallel.fixme
test.describe.parallel.fixme("parallel fixme", () => {
    test("test", async ({ page }) => {});
});
