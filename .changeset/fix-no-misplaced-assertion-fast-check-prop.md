---
"@biomejs/biome": patch
---

Fixed [#11454](https://github.com/biomejs/biome/issues/11454): [`noMisplacedAssertion`](https://biomejs.dev/linter/rules/no-misplaced-assertion/) now recognises `@fast-check/vitest`'s `test.prop(...)` (and `.concurrent.prop`, `.skip.prop`, etc.) as a test function, the same way it already recognises `test.each`.

For example, Biome no longer reports the assertion below as misplaced:

```js
import { fc, test } from "@fast-check/vitest";

test.prop([fc.string()])("round-trips", (s) => {
  expect(s).toBe(s);
});
```
