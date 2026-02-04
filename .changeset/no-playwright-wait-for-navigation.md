---
"@biomejs/biome": patch
---

Added the rule [`noPlaywrightWaitForNavigation`](https://biomejs.dev/linter/rules/no-playwright-wait-for-navigation/). Prefers modern navigation APIs over deprecated `waitForNavigation()`.

```js
await page.waitForNavigation();
```
