---
"@biomejs/biome": patch
---

Fixed [#7289](https://github.com/biomejs/biome/issues/7289). [`useImportType`](https://biomejs.dev/uk/linter/rules/use-import-type/) now inline `import type` into `import { type }` when the `style` option is set to `inlineType`.
