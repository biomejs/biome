---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueDeprecatedVBindSync`](https://biomejs.dev/linter/rules/no-vue-deprecated-v-bind-sync/), which reports the `.sync` modifier on Vue `v-bind` directives. Vue 3 removed this modifier, so the rule suggests `v-model` with the same argument instead.

```vue
<MyComponent :title.sync="pageTitle" />
```
