---
"@biomejs/biome": patch
---

Fixed [#9155](https://github.com/biomejs/biome/issues/9155): Biome no longer reports a parse error for typed slot props in Vue files, such as `v-slot="{ value }: { value: ValueType }"`. Types used in slot props annotations are now correctly detected as used by [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/).
