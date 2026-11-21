---
"@biomejs/biome": patch
---

Fixed [#10609](https://github.com/biomejs/biome/issues/10609): [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) no longer reports `const` members of a namespace exported with `export =`, such as `import { StrictMode } from "react"`. It now also reports members of a non-ambient namespace that aren't exported with `export`.
