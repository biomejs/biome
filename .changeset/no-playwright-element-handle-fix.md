---
"@biomejs/biome": patch
---

Added an auto-fix to [`noPlaywrightElementHandle`](https://biomejs.dev/linter/rules/no-playwright-element-handle/). The fix replaces `$()` and `$$()` with `locator()`.

```js
// Before
const button = await page.$('button');
// After
const button = await page.locator('button');
```
