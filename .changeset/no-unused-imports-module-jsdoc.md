---
"@biomejs/biome": patch
---

Fixed [#8762](https://github.com/biomejs/biome/issues/8762): [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) now considers the types referenced by the module comment of a file: a JSDoc comment with a `@module` tag that is the first comment in the file. `@module` comments anywhere else are still ignored.

The following code is no longer reported:

```ts
/**
 * Helpers for working with {@linkcode Foo}.
 * @module
 */

import type { Foo } from "./foo";
```
