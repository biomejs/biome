---
"@biomejs/biome": patch
---

Added the nursery rule [`useExpectToContain`](https://biomejs.dev/linter/rules/use-expect-to-contain/), which reports tests that compare the result of `includes()` to a boolean, and suggests using `toContain()` instead.

```js
expect(fruits.includes("apple")).toBe(true);
```
