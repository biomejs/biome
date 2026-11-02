---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueBooleanPropTypeFirst`](https://biomejs.dev/linter/rules/use-vue-boolean-prop-type-first/), which requires `Boolean` to come first in the list of types of a Vue prop. When `String` comes before `Boolean`, a prop written in a template without a value is set to an empty string instead of `true`.

```vue
<script setup>
defineProps({
  disabled: [String, Boolean],
});
</script>
```
