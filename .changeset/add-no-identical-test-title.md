---
"@biomejs/biome": patch
---

Added the [`nursery/noIdenticalTestTitle`](https://biomejs.dev/linter/rules/no-identical-test-title) lint rule. This rule disallows using the same title for two `describe` blocks or two test cases at the same nesting level.

```js
describe('foo', () => {});
describe('foo', () => { // invalid: same title as previous describe block
  test('baz', () => {});
  test('baz', () => {}); // invalid: same title as previous test case
});
```
