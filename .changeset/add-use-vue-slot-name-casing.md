---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueSlotNameCasing`](https://biomejs.dev/linter/rules/use-vue-slot-name-casing/), which enforces a consistent casing for the names of `<slot>` elements in Vue templates. Slot names must be in camelCase by default. The `casing` option can require kebab-case or a single lowercase word instead.

```vue
<template>
  <slot name="foo-bar" />
</template>
```
