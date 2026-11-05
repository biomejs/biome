---
"@biomejs/biome": patch
---

Fixed [#12688](https://github.com/biomejs/biome/issues/12688): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports Svelte `<script>` variables that are only updated or assigned in markup expressions (such as `count++` or `total += 1`).
