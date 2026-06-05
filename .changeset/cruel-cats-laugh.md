---
"@biomejs/biome": patch
---

Fixed [`useVueConsistentVBindStyle`](https://biomejs.dev/linter/rules/use-vue-consistent-v-bind-style/): The rule no longer reports argument-less `v-bind` directives because they cannot be converted to shorthand syntax.
