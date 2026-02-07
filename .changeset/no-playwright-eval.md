---
"@biomejs/biome": patch
---

Added the nursery rule [`noPlaywrightEval`](https://biomejs.dev/linter/rules/no-playwright-eval/). Disallows `page.$eval()` and `page.$$eval()` methods.

```js
await page.$eval('.btn', el => el.textContent);
```
