---
"@biomejs/biome": patch
---

Fixed [#10870](https://github.com/biomejs/biome/issues/10870): [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) no longer reports false positives such as `import type { NextRequest } from "next/server"`.
