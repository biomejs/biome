---
"@biomejs/biome": patch
---

Fixed [#11587](https://github.com/biomejs/biome/issues/11587): [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) no longer reports Deno `jsr:` package specifiers such as `jsr:@std/assert` or `jsr:@std/assert@^1`, which Deno downloads on demand.
