---
"@biomejs/biome": patch
---

Fixed [#8603](https://github.com/biomejs/biome/issues/8603): [`noUselessEmptyExport`](https://biomejs.dev/linter/rules/no-useless-empty-export/) no longer reports `export {}` in declaration files. In a `.d.ts` file, the empty export stops the declarations without the `export` keyword from being exported implicitly, so removing it changed the public API of the file.
