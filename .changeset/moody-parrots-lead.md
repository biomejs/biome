---
"@biomejs/biome": patch
---

Fixed [#12085](https://github.com/biomejs/biome/issues/12085): stale [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) diagnostics after workspace links under `node_modules` change. The language server refreshes import resolution and open-file diagnostics when workspace links are created, replaced, or removed.
