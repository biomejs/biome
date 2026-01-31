---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueArrowFuncInWatch`](https://biomejs.dev/linter/rules/no-vue-arrow-func-in-watch/). This rule forbids using arrow functions in watchers in Vue components, because arrow functions do not give access to the component instance (via `this`), while regular functions do.
