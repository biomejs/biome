---
"@biomejs/biome": patch
---

Fixed [#8762](https://github.com/biomejs/biome/issues/8762): [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) now considers the types referenced by JSDoc comments that contain a `@module` tag. These comments document the whole file, so they usually aren't attached to a declaration.

The following code is no longer reported:

```ts
/**
 * Helpers for working with {@linkcode Foo}.
 * @module
 */

import type { Foo } from "./foo";
```
