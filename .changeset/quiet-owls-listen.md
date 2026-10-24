---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedListeners`](https://biomejs.dev/linter/rules/no-vue-deprecated-listeners/). It reports references to `$listeners`, which Vue 3 removed, in Vue templates and component objects. For example, Biome now reports `<div v-on="$listeners" />` and `this.$listeners` inside a Vue component.
