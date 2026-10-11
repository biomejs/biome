---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueVText`](https://biomejs.dev/linter/rules/no-vue-v-text/), which disallows Vue's `v-text` directive in favor of text interpolation.

For example, the following snippet triggers the rule:

```vue
<div v-text="message"></div>
```
