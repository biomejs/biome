---
"@biomejs/biome": patch
---

Added the rule [`noPlaywrightElementHandle`](https://biomejs.dev/linter/rules/no-playwright-element-handle/). Prefers locators over element handles.

```js
const el = await page.$('.btn');
```
