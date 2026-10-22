---
"@biomejs/biome": patch
---

Added the new nursery rule [`noPlaywrightNthMethods`](https://biomejs.dev/linter/rules/no-playwright-nth-methods/), which disallows selecting Playwright elements by position with `first()`, `last()`, and `nth()`. A test that relies on the position of an element can act on a different element when the page changes.

```js
await page.locator("button").first().click();
```
