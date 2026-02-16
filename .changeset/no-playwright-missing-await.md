---
"@biomejs/biome": patch
---

Added the nursery rule [`noPlaywrightMissingAwait`](https://biomejs.dev/linter/rules/no-playwright-missing-await/). Enforces awaiting async Playwright APIs.

```js
const el = page.locator('.btn');
el.click(); // Missing await
```
