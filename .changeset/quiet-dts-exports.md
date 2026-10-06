---
"@biomejs/biome": patch
---

Fixed [#8603](https://github.com/biomejs/biome/issues/8603): [`noUselessEmptyExport`](https://biomejs.dev/linter/rules/no-useless-empty-export/) no longer reports `export {}` in TypeScript declaration files (`.d.ts`), where it keeps declarations that aren't exported from being exported anyway.
