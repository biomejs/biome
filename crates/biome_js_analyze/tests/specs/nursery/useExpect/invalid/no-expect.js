/* should generate diagnostics */

test("no assertion", async ({ page }) => {
    await page.goto("/");
});

test.skip("skipped without assertion", async ({ page }) => {
    await page.click("button");
});

it("it function without assertion", async ({ page }) => {
    await page.fill("input", "value");
});
