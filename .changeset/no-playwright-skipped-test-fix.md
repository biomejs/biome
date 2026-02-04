---
"@biomejs/biome": patch
---

Added an auto-fix to [`noPlaywrightSkippedTest`](https://biomejs.dev/linter/rules/no-playwright-skipped-test/). The fix removes `.skip` or `.fixme` annotations.

```js
// Before
test.skip("test", async ({ page }) => {});
// After
test("test", async ({ page }) => {});
```
