---
"@biomejs/biome": patch
---

Added the nursery rule [`useExpectToHaveLength`](https://biomejs.dev/linter/rules/use-expect-to-have-length/), which reports tests that check the `length` property with `toBe()`, `toEqual()`, or `toStrictEqual()`, and suggests using `toHaveLength()` instead.

```js
expect(files.length).toBe(1);
```
