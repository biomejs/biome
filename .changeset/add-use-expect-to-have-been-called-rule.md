---
"@biomejs/biome": patch
---

Added the nursery rule [`useExpectToHaveBeenCalled`](https://biomejs.dev/linter/rules/use-expect-to-have-been-called/), which reports tests that compare the number of calls of a mock function to zero, and suggests using `toHaveBeenCalled()` instead.

```js
expect(sendEmail).toHaveBeenCalledTimes(0);
```
