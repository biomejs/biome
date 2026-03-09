---
"@biomejs/biome": patch
---

Added the nursery rule [`useImportsFirst`](https://biomejs.dev/linter/rules/use-imports-first/) that enforces all import statements appear before any non-import statements in a module. Inspired by the eslint-plugin-import [`import/first`](https://github.com/import-js/eslint-plugin-import/blob/HEAD/docs/rules/first.md) rule.

```js
// Invalid
import { foo } from "foo";
const bar = 1;
import { baz } from "baz"; // ← flagged

// Valid
import { foo } from "foo";
import { baz } from "baz";
const bar = 1;
```
