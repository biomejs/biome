---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedScopedSlots`](https://biomejs.dev/linter/rules/no-vue-deprecated-scoped-slots/). It reports deprecated `$scopedSlots` references in Vue templates and component objects, and offers an unsafe replacement with `$slots`. For example, Biome now reports `this.$scopedSlots.default` inside a Vue component.
