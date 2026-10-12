---
"@biomejs/biome": patch
---

Added the new nursery rule [`useLowercaseTestTitle`](https://biomejs.dev/linter/rules/use-lowercase-test-title/), which requires test, test suite, and benchmark titles to start with a lowercase letter.

```js
// Invalid
it("Returns the cached value", () => {});
```
