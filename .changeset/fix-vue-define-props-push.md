---
"@biomejs/biome": patch
---

Fixed [#8682](https://github.com/biomejs/biome/issues/8682): [`useVueConsistentDefinePropsDeclaration`](https://biomejs.dev/linter/rules/use-vue-consistent-define-props-declaration/) no longer reports ordinary chained function calls, such as `array.push()`, as `defineProps()` declarations.
