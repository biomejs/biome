---
"@biomejs/biome": patch
---

Added the rule [`noPlaywrightWaitForSelector`](https://biomejs.dev/linter/rules/no-playwright-wait-for-selector/). Prefers locators over deprecated `waitForSelector()`.

```js
await page.waitForSelector('.btn');
```
