---
"@biomejs/biome": patch
---

Added the nursery rule [`noPlaywrightWaitForSelector`](https://biomejs.dev/linter/rules/no-playwright-wait-for-selector/). Prefers locators over deprecated `waitForSelector()`.

```js
await page.waitForSelector('.btn');
```
