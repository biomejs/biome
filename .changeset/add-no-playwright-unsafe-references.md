---
"@biomejs/biome": patch
---

Added the new nursery rule [`noPlaywrightUnsafeReferences`](https://biomejs.dev/linter/rules/no-playwright-unsafe-references/), which reports functions passed to `page.evaluate()` or `page.addInitScript()` that use variables declared outside of them. Playwright runs these functions in the browser page, where those variables don't exist.

```js
const x = 7;
await page.evaluate(() => x * 2); // reported: `x` must be passed as an argument
```
