---
"@biomejs/biome": patch
---

Added the rule [`noPlaywrightMissingAwait`](https://biomejs.dev/linter/rules/no-playwright-missing-await/). Enforces awaiting async Playwright APIs.

```js
const el = page.locator('.btn');
el.click(); // Missing await
```
