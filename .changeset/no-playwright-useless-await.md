---
"@biomejs/biome": patch
---

Added the rule [`noPlaywrightUselessAwait`](https://biomejs.dev/linter/rules/no-playwright-useless-await/). Disallows unnecessary `await` on synchronous Playwright methods.

```js
const loc = await page.locator('.btn');
```
