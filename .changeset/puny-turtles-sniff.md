---
"@biomejs/biome": patch
---

Fixed [#7205](https://github.com/biomejs/biome/issues/7205): The [`noDuplicateTestHooks`](https://biomejs.dev/linter/rules/no-duplicate-test-hooks/) rule now treats chained describe variants (e.g., describe.each/for/todo) as proper describe scopes, eliminating false positives.

The following code will no longer be a false positive:

```js
describe("foo", () => {
  describe.for([])("baz", () => {
    beforeEach(() => {});
  });

  describe.todo("qux", () => {
    beforeEach(() => {});
  });

  describe.todo.each([])("baz", () => {
    beforeEach(() => {});
  });
});
```
