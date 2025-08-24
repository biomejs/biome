---
"@biomejs/biome": patch
---

Fixed [#7289](https://github.com/biomejs/biome/issues/7289). The rule [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) now inlines `import type` into `import { type }` when the `style` option is set to `inlineType`.

Example:

```ts
import type { T } from "mod";
// becomes
import { type T } from "mod";
```
