---
"@biomejs/biome": patch
---

Added the nursery rule [`noPlaywrightForceOption`](https://biomejs.dev/linter/rules/no-playwright-force-option/). Disallows the `force` option on user interactions.

```js
await locator.click({ force: true });
```
