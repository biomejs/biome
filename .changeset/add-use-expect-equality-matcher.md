---
"@biomejs/biome": patch
---

Added the nursery rule [`useExpectEqualityMatcher`](https://biomejs.dev/linter/rules/use-expect-equality-matcher/), which reports assertions that compare two values with `===` or `!==` inside `expect()` and check the result against a boolean. Using an equality matcher instead makes failing tests show both values.

```js
// Invalid
expect(total === 10).toBe(true);
// Valid
expect(total).toBe(10);
```
