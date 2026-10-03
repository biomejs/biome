---
"@biomejs/biome": patch
---

Biome now parses the `generic` attribute of Vue `<script setup lang="ts">` blocks, such as `generic="TData extends RowData"`. The declared type parameters are no longer reported by [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/) when used in the script or the template, and unused ones are reported by [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/).
