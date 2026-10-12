---
"@biomejs/biome": patch
---

Fixed a false positive in [`useVueConsistentDefinePropsDeclaration`](https://biomejs.dev/linter/rules/use-vue-consistent-define-props-declaration/). The rule no longer reports calls such as `a.b.c(0)` or calls to a locally imported `defineProps` function. It now only checks calls to Vue's `defineProps` macro.
