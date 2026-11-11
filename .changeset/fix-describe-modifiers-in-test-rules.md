---
"@biomejs/biome": patch
---

Fixed the detection of `describe` blocks in the test rules [`noIdenticalTestTitle`](https://biomejs.dev/linter/rules/no-identical-test-title/), [`useTestHooksOnTop`](https://biomejs.dev/linter/rules/use-test-hooks-on-top/), and [`useTestHooksInOrder`](https://biomejs.dev/linter/rules/use-test-hooks-in-order/). These rules treated `describe.todo`, `describe.shuffle`, `test.describe.parallel`, and `test.describe.serial` as tests, and they didn't recognize conditional blocks such as `describe.skipIf(condition)("title", () => {})`. As a result, they didn't check the tests and hooks inside those blocks.

For example, `noIdenticalTestTitle` now reports the second test:

```js
test.describe.serial("checkout", () => {
  test("pays", async ({ page }) => {});
  test("pays", async ({ page }) => {});
});
```

The nursery rule [`useValidTestTitle`](https://biomejs.dev/linter/rules/use-valid-test-title/) now also checks the titles of tagged-template tables such as ``test.each`table`("title", () => {})`` and of conditional calls such as `test.skipIf(condition)("title", () => {})`.
