---
"@biomejs/biome": patch
---

Fixed [#8603](https://github.com/biomejs/biome/issues/8603): [`noUselessEmptyExport`](https://biomejs.dev/linter/rules/no-useless-empty-export/) no longer reports `export {}` in an ambient context (a declaration file, or the body of a `declare module` or `declare namespace`) that has no other export declaration or export assignment. There, `export {}` keeps the declarations without the `export` modifier from being exported implicitly.
