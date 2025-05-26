---
"@biomejs/biome": minor
---

Added [`useExplicitTestAssertions`](https://biomejs.dev/linter/rules/no-explicit-test-assertions/) rule, inspired by expect-expect. Require all test cases to use `expect()` (Vitest/Jest) or `assert()` (node:assert).

## Examples

### Invalid

```js
test("myLogic", () => {
  console.log("myLogic");
});
```

```js
test("myLogic", () => {});
```

### Valid

```js
test("myLogic", () => {
  const actual = myLogic();
  expect(actual).toBe(true);
});
```
