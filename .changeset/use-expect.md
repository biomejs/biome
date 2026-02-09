---
"@biomejs/biome": patch
---

Added the nursery rule [`useExpect`](https://biomejs.dev/linter/rules/use-expect/). This rule ensures that test functions contain at least one `expect()` assertion.

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
