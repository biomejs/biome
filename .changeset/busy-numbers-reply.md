---
"@biomejs/biome": patch
---

Fixed consider more constructs as valid test assertions

Previously, [`assert`](https://vitest.dev/api/assert.html), [`expectTypeOf`](https://vitest.dev/api/expect-typeof.html) and [`assertType`](https://vitest.dev/api/assert-type.html) were not recognized as valid assertions by Biome's linting rules.

Now, linting rules like `useExpect` will no longer produce false positives in tests that used these constructs instead of `expect` variants:
```ts
import { expectTypeOf, assert, assertType } from 'vitest';

const myStr = "Hello from vitest!";
it('should be a string', () => {
  expectTypeOf<myStr>().toBeString();
});
it("should still be a string", () => {
  assertType<string>(myStr);
});
it("should still still be a string", () => {
  assert(typeof myStr === "string");
});
```
