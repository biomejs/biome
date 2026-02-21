---
"@biomejs/biome": patch
---

Fixed [#9172](https://github.com/biomejs/biome/issues/9172) and [#9168](https://github.com/biomejs/biome/issues/9168):
Biome now considers more constructs as valid test assertions.

Previously, [`assert`](https://vitest.dev/api/assert.html), [`expectTypeOf`](https://vitest.dev/api/expect-typeof.html) and [`assertType`](https://vitest.dev/api/assert-type.html)
were not recognized as valid assertions by Biome's linting rules, producing false positives in [`lint/nursery/useExpect`](https://biomejs.dev/linter/rules/use-expect) and other similar rules.

Now, these rules will no longer produce errors in test cases that used these constructs instead of `expect`:
```ts
import { expectTypeOf, assert, assertType } from 'vitest';

const myStr = "Hello from vitest!";
it('should be a string', () => {
  expectTypeOf(myStr).toBeString();
});
test("should still be a string", () => {
  assertType<string>(myStr);
});
it.todo("should still still be a string", () => {
  assert(typeof myStr === "string");
});
```
