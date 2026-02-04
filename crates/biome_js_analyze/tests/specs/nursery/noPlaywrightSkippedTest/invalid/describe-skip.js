/* should generate diagnostics */

// test.describe.skip
test.describe.skip("skipped suite", () => {
    test("test", async ({ page }) => {
        await page.click("button");
    });
});

// describe.skip
describe.skip("skipped describe", () => {
    test("test", async ({ page }) => {
        await page.click("button");
    });
});

// test.describe.parallel.skip
test.describe.parallel.skip("parallel skipped", () => {
    test("test", async ({ page }) => {});
});

// test.describe.serial.skip
test.describe.serial.skip("serial skipped", () => {
    test("test", async ({ page }) => {});
});
