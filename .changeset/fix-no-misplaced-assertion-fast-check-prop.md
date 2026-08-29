---
"@biomejs/biome": patch
---

Fixed [#11454](https://github.com/biomejs/biome/issues/11454): [`noMisplacedAssertion`](https://biomejs.dev/linter/rules/no-misplaced-assertion/) now recognises `@fast-check/vitest`'s `test.prop(...)` (and `.concurrent.prop`, `.skip.prop`, etc.) as a test function, the same way it already recognises `test.each`. The JS formatter picks up the same recognition, so a curried `test.prop(...)(...)` call is now formatted with the regular breakable argument layout used for `test.each`/`test.for`, instead of the single-line-hugging layout used for plain `it`/`test` calls.

For example, Biome no longer reports the assertion below as misplaced:

```js
import { fc, test } from "@fast-check/vitest";

test.prop([fc.string()])("round-trips", (s) => {
  expect(s).toBe(s);
});
```
