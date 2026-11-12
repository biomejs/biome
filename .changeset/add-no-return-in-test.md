---
"@biomejs/biome": patch
---

Added the nursery rule [`noReturnInTest`](https://biomejs.dev/linter/rules/no-return-in-test/), which reports `return` statements directly inside the body of a test function. The rule belongs to the `test` domain.

```js
test("adds two numbers", () => {
    return expect(add(1, 2)).toBe(3);
});
```
