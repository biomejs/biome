---
"@biomejs/biome": patch
---

Added the nursery rule [`noPlaywrightWaitForTimeout`](https://biomejs.dev/linter/rules/no-playwright-wait-for-timeout/). Disallows hard-coded timeouts with `waitForTimeout()`.

```js
await page.waitForTimeout(5000);
```
