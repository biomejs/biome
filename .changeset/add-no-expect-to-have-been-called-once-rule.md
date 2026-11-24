---
"@biomejs/biome": patch
---

Added the nursery rule [`noExpectToHaveBeenCalledOnce`](https://biomejs.dev/linter/rules/no-expect-to-have-been-called-once/), which reports `toHaveBeenCalledOnce()` and `toBeCalledOnce()` assertions and suggests `toHaveBeenCalledTimes(1)` and `toBeCalledTimes(1)` instead. The rule belongs to the `test` domain.

```js
expect(sendEmail).toHaveBeenCalledOnce();
```
