---
"@biomejs/biome": patch
---

Biome now parses and formats the `generic` attribute of Vue `<script setup lang="ts">` blocks, such as `generic="TData extends RowData"`. The declared type parameters are no longer reported by [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/), [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) and [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) also take the parameters into account now.

```diff
- <script setup lang="ts" generic="T   extends   Item,U">
+ <script setup lang="ts" generic="T extends Item, U">
```
