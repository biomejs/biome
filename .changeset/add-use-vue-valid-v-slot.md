---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueValidVSlot`](https://biomejs.dev/linter/rules/use-vue-valid-v-slot/), which reports misplaced, duplicated, or malformed `v-slot` directives in Vue templates.

For example, the rule reports two `<template>` elements that fill the same slot:

```vue
<MyComponent>
  <template #header>First title</template>
  <template #header>Second title</template>
</MyComponent>
```
