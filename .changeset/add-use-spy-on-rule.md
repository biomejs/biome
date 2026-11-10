---
"@biomejs/biome": patch
---

Added the nursery rule [`useSpyOn`](https://biomejs.dev/linter/rules/use-spy-on/), which reports properties that are overwritten with `jest.fn()` or `vi.fn()`. Overwriting a property loses its original value, while `jest.spyOn()` and `vi.spyOn()` keep it so it can be restored after the test.

```js
Date.now = jest.fn(() => 10);
```
