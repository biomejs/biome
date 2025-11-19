---
"@biomejs/biome": patch
---

Fixed [#7876](https://github.com/biomejs/biome/issues/7876): The [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) rule now ignores imports that are used by @linkcode and @linkplain (previously supported @link and @see).

The following code will no longer be a false positive:

```js
import type { a } from "a"

/**
 * {@linkcode a}
 */
function func() {}
```
