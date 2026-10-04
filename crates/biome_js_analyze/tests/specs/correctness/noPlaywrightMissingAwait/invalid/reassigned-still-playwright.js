/* EC-001 negative: const binding (never reassigned) — should still flag */

test("const playwright locator", async ({ page }) => {
    const loc = page.locator(".item");
    expect(loc).toBeVisible();
});
