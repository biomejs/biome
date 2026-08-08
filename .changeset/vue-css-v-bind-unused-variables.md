---
"@biomejs/biome": patch
---

Fixed [#11157](https://github.com/biomejs/biome/issues/11157): [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports variables that are only referenced by `v-bind()` inside a Vue component's `<style>` block.
