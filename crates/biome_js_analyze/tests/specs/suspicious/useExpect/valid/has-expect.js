/* should not generate diagnostics */

test("has assertion", async ({ page }) => {
    await expect(page).toHaveTitle("Title");
});

test("soft assertion", async ({ page }) => {
    await expect.soft(page.locator("h1")).toBeVisible();
});

test.only("poll assertion", async ({ page }) => {
    await expect.poll(() => page.title()).toBe("Title");
});

it("it with assertion", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator("h1")).toBeVisible();
});

// describe blocks don't need assertions
test.describe("suite", () => {
    test("inner test", async ({ page }) => {
        await expect(page).toHaveURL("/home");
    });
});

// describe blocks without direct expects are valid
test.describe("empty suite", () => {});

test.describe("suite with setup only", () => {
    // no tests inside, just setup code
});

test.describe.only("describe.only is not a test", () => {});
test.describe.skip("describe.skip is not a test", () => {});

// hooks are not test calls
test.beforeEach(async ({ page }) => {
    await page.goto("/");
});

test.afterEach(async ({ page }) => {
    await page.close();
});

test.beforeAll(async () => {
    // setup
});

test.afterAll(async () => {
    // teardown
});
