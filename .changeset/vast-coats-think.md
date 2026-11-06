---
"@biomejs/biome": patch
---

Fixed [`noMissingVarFunction`](https://biomejs.dev/linter/rules/no-missing-var-function/) so it also checks declarations inside at-rules such as `@page` and `@font-face`. For example, `@page { margin: --page-margin; }` is now reported when `--page-margin` is declared in `:root`.
