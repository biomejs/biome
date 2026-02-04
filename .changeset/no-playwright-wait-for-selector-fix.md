---
"@biomejs/biome": patch
---

Added an auto-fix to [`noPlaywrightWaitForSelector`](https://biomejs.dev/linter/rules/no-playwright-wait-for-selector/). The fix replaces `waitForSelector(selector)` with `locator(selector).waitFor()`.

```js
// Before
await page.waitForSelector('#dialog', { state: 'visible' });
// After
await page.locator('#dialog').waitFor({ state: 'visible' });
```
