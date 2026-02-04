---
"@biomejs/biome": patch
---

Added the rule [`noPlaywrightNetworkidle`](https://biomejs.dev/linter/rules/no-playwright-networkidle/). Disallows deprecated `networkidle` wait option.

```js
await page.goto(url, { waitUntil: 'networkidle' });
```
