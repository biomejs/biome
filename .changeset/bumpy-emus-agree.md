---
"@biomejs/biome": patch
---

Fixed [#11157](https://github.com/biomejs/biome/issues/11157): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports Vue `<script setup>` bindings used by CSS `v-bind()` as unused.
