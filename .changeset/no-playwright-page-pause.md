---
"@biomejs/biome": patch
---

Added the nursery rule [`noPlaywrightPagePause`](https://biomejs.dev/linter/rules/no-playwright-page-pause/). Disallows `page.pause()` debugging calls in committed code.

```js
await page.pause();
```
