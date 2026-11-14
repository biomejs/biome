---
"@biomejs/biome": patch
---

Added the nursery rule [`useExpectToHaveBeenCalledTimes`](https://biomejs.dev/linter/rules/use-expect-to-have-been-called-times/), which reports tests that check the length of `mock.calls` with `toHaveLength()`, and suggests using `toHaveBeenCalledTimes()` instead.

```js
expect(onSave.mock.calls).toHaveLength(1);
```
