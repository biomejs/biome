---
"@biomejs/biome": patch
---

Added the nursery rule [`noPlaywrightElementHandle`](https://biomejs.dev/linter/rules/no-playwright-element-handle/). Prefers locators to element handles.

```js
const el = await page.$('.btn');
```
