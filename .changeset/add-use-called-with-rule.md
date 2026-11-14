---
"@biomejs/biome": patch
---

Added the nursery rule [`useCalledWith`](https://biomejs.dev/linter/rules/use-called-with/), which reports `toHaveBeenCalled()`, `toBeCalled()`, and `toHaveBeenCalledOnce()` assertions, because they don't check the arguments that the function was called with. The rule belongs to the `test` domain.

```js
expect(sendEmail).toHaveBeenCalled();
```
