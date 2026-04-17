/* should not generate diagnostics */

test("expect.assertions is a valid assertion check", () => {
    expect.assertions(1);
    expect(true).toBe(true);
});

test("expect.hasAssertions is a valid assertion check", () => {
    expect.hasAssertions();
    expect(1).toBe(1);
});

test("expect.assertions alone counts as assertion-aware", () => {
    expect.assertions(0);
});

test("expect.soft is a valid assertion", async ({ page }) => {
    await expect.soft(page.locator("h1")).toBeVisible();
});

test("expect.poll is a valid assertion", async ({ page }) => {
    await expect.poll(() => page.title()).toBe("Title");
});
