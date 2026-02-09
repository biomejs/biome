---
"@biomejs/biome": patch
---

Added the nursery rule [`noConditionalExpect`](https://biomejs.dev/linter/rules/no-conditional-expect/). This rule disallows conditional `expect()` calls inside tests, which can lead to tests that silently pass when assertions never run.

```js
// Invalid - conditional expect may not run
test("conditional", async ({ page }) => {
    if (someCondition) {
        await expect(page).toHaveTitle("Title");
    }
});

// Valid - unconditional expect
test("unconditional", async ({ page }) => {
    await expect(page).toHaveTitle("Title");
});
```
