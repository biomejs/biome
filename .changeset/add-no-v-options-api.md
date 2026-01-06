---
"@biomejs/biome": patch
---

Added a new nursery rule [`noVueOptionsApi`](https://biomejs.dev/linter/rules/no-vue-options-api/).

Biome now reports Vue Options API usage, which is incompatible with Vue 3.6's Vapor Mode.
This rule detects Options API patterns in `<script>` blocks, `defineComponent()`, and `createApp()` calls,
helping prepare codebases for Vapor Mode adoption.

For example, the following now triggers this rule:

```vue
<script>
export default {
  data() {
    return { count: 0 };
  }
}
</script>
```
