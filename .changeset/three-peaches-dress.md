---
"@biomejs/biome": patch
---

Fixed [#9626](https://github.com/biomejs/biome/issues/9626): [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) no longer reports false positives for named imports from packages that have a corresponding `@types/*` package installed. For example, `import { useState } from "react"` with `@types/react` installed is now correctly recognised.
