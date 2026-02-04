/* should generate diagnostics */

// Basic test.skip
test.skip("skipped test", async ({ page }) => {
    await page.click("button");
});

// it.skip variant
it.skip("skipped it", async ({ page }) => {
    await page.click("button");
});

// Bracket notation
test["skip"]("bracket notation skip", async ({ page }) => {
    await page.click("button");
});
