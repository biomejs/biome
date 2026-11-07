---
"@biomejs/biome": patch
---

Fixed [#8762](https://github.com/biomejs/biome/issues/8762): [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) now considers the types referenced by the module comment of a file: the first JSDoc comment with a `@module` tag in the file header, before any code. Comments such as a license banner may come before it. `@module` comments after the first line of code are still ignored.

The following code is no longer reported:

```ts
/**
 * Helpers for working with {@linkcode Foo}.
 * @module
 */

import type { Foo } from "./foo";
```
