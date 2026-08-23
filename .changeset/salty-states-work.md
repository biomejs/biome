---
"@biomejs/biome": patch
---

Fixed: Variables and imports used as custom Vue directives are no longer reported as unused.

For example:

```vue
<script setup>
const vHighlight = {
  mounted: (element) => {
    element.style.color = "red";
  },
};
</script>

<template>
  <p v-highlight>Hello</p>
</template>
```
