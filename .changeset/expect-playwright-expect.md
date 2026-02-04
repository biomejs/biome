---
"@biomejs/biome": patch
---

Added the rule [`expectPlaywrightExpect`](https://biomejs.dev/linter/rules/expect-playwright-expect/). This rule ensures that Playwright test functions contain at least one `expect()` assertion.

```js
// Invalid - test without assertion
test("no assertion", async ({ page }) => {
    await page.goto("/");
});

// Valid - test with assertion
test("has assertion", async ({ page }) => {
    await expect(page).toHaveTitle("Title");
});
```
