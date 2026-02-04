/* should generate diagnostics */

// test.step.skip
test("test with skipped step", async ({ page }) => {
    await test.step.skip("skipped step", async () => {
        await page.click("button");
    });
});
