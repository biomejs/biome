---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueToggleInsideTransition`](https://biomejs.dev/linter/rules/use-vue-toggle-inside-transition/), which requires the element inside a Vue `<Transition>` to be shown and hidden with `v-if` or `v-show`. Without one of them, the element is always shown and the transition never plays.

```vue
<template>
  <Transition>
    <div>Content</div>
  </Transition>
</template>
```
