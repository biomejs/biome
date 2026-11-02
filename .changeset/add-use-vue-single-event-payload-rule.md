---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueSingleEventPayload`](https://biomejs.dev/linter/rules/use-vue-single-event-payload/), which reports Vue component events that are emitted or declared with more than one value. A listener written inline in a template, such as `@change="onChange($event)"`, only receives the first value.

```vue
<script setup>
const emit = defineEmits(["change"]);
emit("change", value, extra);
</script>
```
