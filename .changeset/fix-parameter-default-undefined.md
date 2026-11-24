---
"@biomejs/biome": patch
---

Fixed [#13392](https://github.com/biomejs/biome/issues/13392): [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) no longer reports `flag || other` when `flag` is a parameter whose default value removes `undefined` from its type, such as `flag: boolean | undefined = false`.
