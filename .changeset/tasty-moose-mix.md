---
"@biomejs/biome": patch
---

Added new nursery [`useExplicitTestAssertions`](https://biomejs.dev/linter/rules/use-explicit-test-assertions/) rule, inspired by [`expect-expect`](https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/expect-expect.md). Require all test cases to use `expect()` (Vitest/Jest) or `assert()` (node:assert).

**Invalid examples**

```js
test("myLogic", () => {
  console.log("myLogic");
});
```

```js
test("myLogic", () => {});
```

**Valid examples**

```js
test("myLogic", () => {
  const actual = myLogic();
  expect(actual).toBe(true);
});
```
